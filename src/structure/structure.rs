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

    pub fn get_mut_file(&mut self) -> &mut FileIo { 
        &mut self.file_io
    }

    pub fn get_headers(&self) -> &DatabaseHeader { 
        &self.headers
    }

    pub async fn init(file_path: impl AsRef<std::path::Path>) -> BackendResult<Self> {
        let mut file = FileIo::create(file_path, None).await?; 

        let mut buf = [0_u8; 100];
        let mut res = file.read_exact(&mut buf).await?; 

        let db_headers = DatabaseHeader::try_from(buf)?; 

        Ok(Self{ 
            file_io: file, 
            headers: db_headers, 
        })
    }

    pub async fn print_dbinfo(&mut self) { 

        println!("database page size: {}", self.headers.page_size);

        let mut buf = Vec::with_capacity(8); 
        let mut res = self.file_io.read_buf(&mut buf).await.unwrap(); 

        let num_tables = u16::from_be_bytes([buf[3], buf[4]]);
        println!("number of tables: {}", num_tables);
    }

    
    pub async fn print_tables(&mut self) {
        println!()
    }

}