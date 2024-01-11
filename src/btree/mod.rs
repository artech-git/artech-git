

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


#[derive(Debug)]
pub struct TableSchema {
    pub _type: String,
    pub name: String,
    pub _tbl_name: String,
    pub _rootpage: usize,
    pub _sql: String,
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