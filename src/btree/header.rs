

// use crate::{TextEncoding, error::BackendResult};
// use anyhow::Result;


// pub struct SQLiteHeader {
//     pub page_size: usize,
//     pub text_encoding: TextEncoding,
// }

// impl SQLiteHeader {

//     pub fn new(buf: &[u8]) -> BackendResult<Self> {
//         Ok(SQLiteHeader {
//             page_size: u16::from_be_bytes([buf[16], buf[17]]) as usize,
//             text_encoding: u32::from_be_bytes([buf[56], buf[57], buf[58], buf[59]]).try_into()?,
//         })
//     }

// }