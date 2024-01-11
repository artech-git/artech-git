

pub mod btree; 
pub mod header; 
pub mod types; 

pub use types::*;
pub use btree::*; 
pub use header::*;

use anyhow::{bail, Error, Result};
use tokio::io::{AsyncSeekExt, AsyncReadExt};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    usize,
};

use btree::{BtreePage, RecordValue};

use crate::{error::BackendResult, structure::{DBLayout, DatabaseHeader}};


pub async fn read_sqlite_schema(file: &mut DBLayout) -> BackendResult<Vec<TableSchema>> {

    let k = (file.get_mut_file()).seek(SeekFrom::Start(0)).await?;
    
    let mut first_page = {
        let header = file.get_headers();
        let mut first_page = vec![0; header.page_size as usize];
        first_page
    };
    
    let i = (file.get_mut_file()).read_exact(&mut first_page).await?;
    
    let header = file.get_headers();
    let text_encoding = TextEncoding::try_from(header.text_encoding)?; 

    let page = BtreePage::new(&first_page, &text_encoding)?;
    let mut result = vec![];

    for cell in page.cells.into_iter() {

        let mut vals = cell.record.values.into_iter();

        result.push(TableSchema {
            _type: if let Some(RecordValue::String(val)) = vals.next() {
                val
            } else {
                panic!()
            },
            name: if let Some(RecordValue::String(val)) = vals.next() {
                val
            } else {
                panic!()
            },
            _tbl_name: if let Some(RecordValue::String(val)) = vals.next() {
                val
            } else {
                panic!()
            },
            _rootpage: if let Some(RecordValue::I8(val)) = vals.next() {
                val as usize
            } else {
                panic!()
            },
            _sql: if let Some(RecordValue::String(val)) = vals.next() {
                val
            } else {
                panic!()
            },
        });
    }

    Ok(result)
}


#[derive(Debug)]
pub struct TableSchema {
    _type: String,
    pub name: String,
    _tbl_name: String,
    _rootpage: usize,
    _sql: String,
}

pub enum TextEncoding {
    Utf8,
    Utf16le,
    Utf16be,
}


impl TryFrom<u32> for TextEncoding {
    
    type Error = crate::error::BackendError;

    fn try_from(value: u32) -> BackendResult<TextEncoding> {
        match value {
            1 => Ok(TextEncoding::Utf8),
            2 => Ok(TextEncoding::Utf16le),
            3 => Ok(TextEncoding::Utf16be),
            _ => Err(anyhow::anyhow!("No text encoding corresponds to {value:x}").into()),
        }

    }
}