
use crate::error::BackendResult;

use super::TextEncoding;

#[derive(Debug)]
pub struct BtreePageHeader {
    _page_type: BtreePageType,
    _freeblock_idx: u16,
    cell_count: u16,
    _cell_content_idx: u16,
    _free_byte_count: u8,
    _final_pointer: Option<u32>,
}

#[derive(Debug)]
enum BtreePageType {
    InteriorIndex,
    InteriorTable,
    LeafIndex,
    LeafTable,
}


impl TryFrom<u8> for BtreePageType {

    type Error = crate::error::BackendError;

    fn try_from(value: u8) -> BackendResult<Self> {
        match value {
            0x02 => Ok(BtreePageType::InteriorIndex),
            0x05 => Ok(BtreePageType::InteriorTable),
            0x0a => Ok(BtreePageType::LeafIndex),
            0x0d => Ok(BtreePageType::LeafTable),
            _ => Err((anyhow::anyhow!("Invalid BtreePageType value {value:x}")).into()),
        }
    }
}


impl BtreePageHeader {

    fn new(buf: &[u8]) -> BackendResult<Self> {

        let page_type = buf[100].try_into()?;

        let final_pointer = match page_type {

            BtreePageType::InteriorIndex => {
                Some(u32::from_be_bytes([buf[108], buf[109], buf[110], buf[111]]))
            }

            BtreePageType::InteriorTable => {
                Some(u32::from_be_bytes([buf[108], buf[109], buf[110], buf[111]]))
            }

            BtreePageType::LeafIndex => None,

            BtreePageType::LeafTable => None,
        };

        Ok(BtreePageHeader {
            _page_type: page_type,
            _freeblock_idx: u16::from_be_bytes([buf[101], buf[102]]),
            cell_count: u16::from_be_bytes([buf[103], buf[104]]),
            _cell_content_idx: u16::from_be_bytes([buf[105], buf[106]]),
            _free_byte_count: buf[107],
            _final_pointer: final_pointer,
        })
    }
    
}


#[derive(Debug, Clone)]
pub struct TableBtreeLeafCell {
    _size: u64,
    _row_id: u64,
    pub record: Record,
}

impl TableBtreeLeafCell {
    fn new(buf: &[u8], text_encoding: &TextEncoding) -> BackendResult<Self> {
        let (size, size_len) = super::parse_varint(buf);
        let (row_id, id_len) = super::parse_varint(&buf[size_len..]);
        let idx = size_len + id_len;
        Ok(TableBtreeLeafCell {
            _size: size,
            _row_id: row_id,
            record: Record::new(&buf[idx..idx + size as usize], text_encoding)?,
        })
    }
}

pub struct BtreePage {
    pub _header: BtreePageHeader,
    pub cells: Vec<TableBtreeLeafCell>,
}

impl BtreePage {
    pub fn new(buf: &[u8], text_encoding: &TextEncoding) -> BackendResult<Self> {

        let header = BtreePageHeader::new(buf)?;
        let mut cells = vec![];

        for i in 0..header.cell_count {

            let cell_pointer =
                u16::from_be_bytes([buf[(108 + i * 2) as usize], buf[(108 + i * 2 + 1) as usize]])
                    as usize;

            cells.push(TableBtreeLeafCell::new(
                &buf[cell_pointer..],
                text_encoding,
            )?);

        }

        Ok(BtreePage {
            _header: header,
            cells,
        })

    }
}


#[derive(Debug, Clone)]
pub struct Record {
    _header: RecordHeader,
    pub values: Vec<RecordValue>,
}

impl Record {
    fn new(buf: &[u8], text_encoding: &TextEncoding) -> BackendResult<Self> {
        let header = RecordHeader::new(buf);
        let mut values = vec![];
        let mut idx = header.len as usize;
        for col_type in header.col_types.iter() {
            let value = match col_type {
                RecordSerialType::Null => RecordValue::Null,
                RecordSerialType::I8 => RecordValue::I8(buf[idx] as i8),
                RecordSerialType::I16 => {
                    RecordValue::I16(i16::from_be_bytes([buf[idx], buf[idx + 1]]))
                }
                RecordSerialType::I24 => RecordValue::I24(i32::from_be_bytes([
                    buf[idx],
                    buf[idx + 1],
                    buf[idx + 2],
                    buf[idx + 3],
                ])),
                RecordSerialType::I32 => RecordValue::I32(i32::from_be_bytes([
                    buf[idx],
                    buf[idx + 1],
                    buf[idx + 2],
                    buf[idx + 3],
                ])),
                RecordSerialType::I48 => RecordValue::I48(i64::from_be_bytes([
                    buf[idx],
                    buf[idx + 1],
                    buf[idx + 2],
                    buf[idx + 3],
                    buf[idx + 4],
                    buf[idx + 5],
                    buf[idx + 6],
                    buf[idx + 7],
                ])),
                RecordSerialType::I64 => RecordValue::I64(i64::from_be_bytes([
                    buf[idx],
                    buf[idx + 1],
                    buf[idx + 2],
                    buf[idx + 3],
                    buf[idx + 4],
                    buf[idx + 5],
                    buf[idx + 6],
                    buf[idx + 7],
                ])),
                RecordSerialType::F64 => RecordValue::F64(f64::from_be_bytes([
                    buf[idx],
                    buf[idx + 1],
                    buf[idx + 2],
                    buf[idx + 3],
                    buf[idx + 4],
                    buf[idx + 5],
                    buf[idx + 6],
                    buf[idx + 7],
                ])),
                RecordSerialType::Zero => RecordValue::Zero,
                RecordSerialType::One => RecordValue::One,
                RecordSerialType::SQLiteInternal => unimplemented!(),
                RecordSerialType::Blob(len) => {
                    RecordValue::Blob(buf[idx..idx + *len as usize].to_vec())
                }
                RecordSerialType::String(len) => {
                    let string = match text_encoding {
                        TextEncoding::Utf8 => std::str::from_utf8(&buf[idx..idx + *len as usize])?,
                        _ => unimplemented!(),
                    };
                    RecordValue::String(string.to_string())
                }
            };
            values.push(value);
            idx += col_type.len();
        }
        Ok(Record {
            _header: header,
            values,
        })

    }
}


#[derive(Debug, Clone)]
struct RecordHeader {
    len: u64,
    col_types: Vec<RecordSerialType>,
}

impl RecordHeader {
    fn new(buf: &[u8]) -> Self {
        let (len, len_len) = super::parse_varint(buf);
        let mut idx = len_len;
        let header = &buf[..len as usize];
        let mut col_types: Vec<RecordSerialType> = vec![];
        while idx != len as usize {
            let (type_id, len) = super::parse_varint(&header[idx..]);
            col_types.push(type_id.into());
            idx += len;
        }
        RecordHeader { len, col_types }
    }
}

#[derive(Debug, Clone)]
enum RecordSerialType {
    Null,
    I8,
    I16,
    I24,
    I32,
    I48,
    I64,
    F64,
    Zero,
    One,
    SQLiteInternal,
    Blob(u64),
    String(u64),
}

impl RecordSerialType {
    fn len(&self) -> usize {
        match &self {
            RecordSerialType::Null => 0,
            RecordSerialType::I8 => 1,
            RecordSerialType::I16 => 2,
            RecordSerialType::I24 => 3,
            RecordSerialType::I32 => 4,
            RecordSerialType::I48 => 6,
            RecordSerialType::I64 => 8,
            RecordSerialType::F64 => 8,
            RecordSerialType::Zero => 0,
            RecordSerialType::One => 0,
            RecordSerialType::SQLiteInternal => unimplemented!(),
            RecordSerialType::Blob(len) => *len as usize,
            RecordSerialType::String(len) => *len as usize,
        }
    }
}

impl From<u64> for RecordSerialType {
    fn from(value: u64) -> Self {
        match value {
            0 => RecordSerialType::Null,
            1 => RecordSerialType::I8,
            2 => RecordSerialType::I16,
            3 => RecordSerialType::I24,
            4 => RecordSerialType::I32,
            5 => RecordSerialType::I48,
            6 => RecordSerialType::I64,
            7 => RecordSerialType::F64,
            8 => RecordSerialType::Zero,
            9 => RecordSerialType::One,
            10..=11 => RecordSerialType::SQLiteInternal,
            _ => {
                if value / 2 == 0 {
                    RecordSerialType::Blob((value - 12) / 2)
                } else {
                    RecordSerialType::String((value - 13) / 2)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RecordValue {
    Null,
    I8(i8),
    I16(i16),
    I24(i32),
    I32(i32),
    I48(i64),
    I64(i64),
    F64(f64),
    Zero,
    One,
    Blob(Vec<u8>),
    String(String),
}