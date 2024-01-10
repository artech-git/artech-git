

use std::array::TryFromSliceError;
use std::convert::TryInto;

use crate::error::BackendResult;

#[derive(Debug)]
pub struct DatabaseHeader {
    signature: [u8; 16],
    page_size: u16,
    file_format_write_version: u8,
    file_format_read_version: u8,
    reserved_space: u8,
    max_embedded_payload_fraction: u8,
    min_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_counter: u32,
    database_size_pages: u32,
    first_freelist_trunk_page: u32,
    freelist_pages: u32,
    schema_cookie: u32,
    schema_format_number: u32,
    default_page_cache_size: u32,
    page_number_of_largest_root_btree_page: u32,
    database_text_encoding: u32,
    user_version: u32,
    incremental_vacuum_mode: u32,
    application_id: u32,
    reserved_for_expansion: [u8; 20],
    version_valid_for: u32,
    sqlite_version_number: u32,
}

impl DatabaseHeader {

    fn read_from_buffer(file_buffer: &[u8]) -> BackendResult<Self> {
        // let mut file = File::open(file_path).map_err(convert_to_header_err)?;
        // let mut buffer = [0u8; 100]; // Size of the SQLite database header is 100 bytes
        // file.read_exact(&mut buffer).map_err(convert_to_header_err)?;

        if file_buffer.len() != 100 { 
            return Err(HeaderError::IncorrectBufferLength.into()); 
        }

        let mut buffer = [0_u8; 100];
        
        for (idx, val) in  file_buffer.iter().enumerate() {
            buffer[idx] = *val; 
        } 


        let header = DatabaseHeader {
            signature: buffer[0..16].try_into().map_err(convert_to_header_err)?,
            page_size: u16::from_le_bytes(buffer[16..18].try_into().map_err(convert_to_header_err)?),
            file_format_write_version: buffer[18],
            file_format_read_version: buffer[19],
            reserved_space: buffer[20],
            max_embedded_payload_fraction: buffer[21],
            min_embedded_payload_fraction: buffer[22],
            leaf_payload_fraction: buffer[23],
            file_change_counter: u32::from_le_bytes(buffer[24..28].try_into().map_err(convert_to_header_err)?),
            database_size_pages: u32::from_le_bytes(buffer[28..32].try_into().map_err(convert_to_header_err)?),
            first_freelist_trunk_page: u32::from_le_bytes(buffer[32..36].try_into().map_err(convert_to_header_err)?),
            freelist_pages: u32::from_le_bytes(buffer[36..40].try_into().map_err(convert_to_header_err)?),
            schema_cookie: u32::from_le_bytes(buffer[40..44].try_into().map_err(convert_to_header_err)?),
            schema_format_number: u32::from_le_bytes(buffer[44..48].try_into().map_err(convert_to_header_err)?),
            default_page_cache_size: u32::from_le_bytes(buffer[48..52].try_into().map_err(convert_to_header_err)?),
            page_number_of_largest_root_btree_page: u32::from_le_bytes(buffer[52..56].try_into().map_err(convert_to_header_err)?),
            database_text_encoding: u32::from_le_bytes(buffer[56..60].try_into().map_err(convert_to_header_err)?),
            user_version: u32::from_le_bytes(buffer[60..64].try_into().map_err(convert_to_header_err)?),
            incremental_vacuum_mode: u32::from_le_bytes(buffer[64..68].try_into().map_err(convert_to_header_err)?),
            application_id: u32::from_le_bytes(buffer[68..72].try_into().map_err(convert_to_header_err)?),
            reserved_for_expansion: buffer[72..92].try_into().map_err(convert_to_header_err)?,
            version_valid_for: u32::from_le_bytes(buffer[92..96].try_into().map_err(convert_to_header_err)?),
            sqlite_version_number: u32::from_le_bytes(buffer[96..100].try_into().map_err(convert_to_header_err)?),
        };

        Ok(header)
    }
}


#[derive(Debug, thiserror::Error)]
pub enum HeaderError { 
    #[error("Incorrect buffer length recieved")]
    IncorrectBufferLength, 

    #[error("Incorrect buffer length recieved")]
    SliceCastError(#[from] TryFromSliceError), 
}

#[inline(always)]
fn convert_to_header_err(val: impl Into<HeaderError>) -> HeaderError {
    val.into()
}


#[cfg(test)]
mod tests { 
    use super::*;

    // #[test]
    // fn read_sample_sqlite_file() {
    //     if let Ok(header) = DatabaseHeader::read_from_file("./sample.db") {
    //         println!(" 🥺Header data {:#.map_err(convert_to_header_err)?}", header); // Print the header for demonstration
    //     } else {
    //         println!("Error reading the database header.");
    //     }
    // }
    
}