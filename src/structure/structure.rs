use serde::{Serialize, Deserialize};
use tokio::io::AsyncReadExt;

use crate::{io::FileIo, error::BackendResult};

use super::header::{DatabaseHeader, self};


#[derive(Debug,)]
pub struct DBLayout { 
    file_io: FileIo,
    headers: DatabaseHeader,
}


impl DBLayout { 

    pub async fn init(file_path: impl AsRef<std::path::Path>) -> BackendResult<Self> {

        let mut file = FileIo::create(file_path, None).await?; 

        let mut buf = Vec::with_capacity(100);
        let mut res = file.read_buf(&mut buf).await?; 

        let mut array_buf = { 
            let mut array_buf = [0_u8; 100];
            for (idx, val) in buf.into_iter().enumerate() { 
                array_buf[idx] = val;
            }
            array_buf
        };

        let headers = DatabaseHeader::try_from(array_buf).unwrap();

        Ok(Self{ 
            file_io: file, 
            headers: headers
        })
    }

    pub async fn print_data(&mut self) { 

        println!("database page size: {}", self.headers.page_size);

        let mut buf = Vec::with_capacity(8); 
        let mut res = self.file_io.read_buf(&mut buf).await.unwrap(); 

        let num_tables = u16::from_be_bytes([buf[3], buf[4]]);
        println!("number of tables: {}", num_tables);
    }
}