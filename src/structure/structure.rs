use std::io::SeekFrom;

use serde::{Serialize, Deserialize};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::{io::FileIo, error::BackendResult, btree::{btree::{BtreePage, RecordValue}, TextEncoding, TableSchema}};

use super::header::{DatabaseHeader, self};

// #[derive(Debug,)]
pub struct DBLayout { 
    file_io: FileIo,
    headers: DatabaseHeader,
    btree_page: BtreePage,
    table_schemas: Vec<TableSchema>    
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

        let btree_pages = {
    
            let k = (file).seek(SeekFrom::Start(0)).await?;
    
            let mut first_page = {
                // let header = db_headers.clone();
                let mut first_page = vec![0; db_headers.page_size as usize];
                first_page
            };
            
            let i = (file).read_exact(&mut first_page).await?;
            
            // let num = &db_headers.text_encoding; 
            let text_encoding = TextEncoding::try_from(db_headers.text_encoding)?; 
            let page = BtreePage::new(&first_page, &text_encoding)?;
            page    
        };

        let table_schema = { 

            let mut result = vec![];

                for cell in btree_pages.cells.iter() {

                    let mut vals = (cell.clone()).record.values.into_iter();

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

                result
        };

        Ok(Self{ 
            file_io: file, 
            headers: db_headers, 
            btree_page: btree_pages,
            table_schemas: table_schema
        })
    }

    pub async fn print_dbinfo(&self) { 

        println!("database page size: {}", self.headers.page_size);

        let num_tables = {
            self.table_schemas.len()
        };

        println!("number of tables: {}", num_tables);
    }

    
    pub async fn print_tables(&self) {


        for name in self.table_schemas.iter() { 
            print!("{} ", name.name); 
        }
    }

}