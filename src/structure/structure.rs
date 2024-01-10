use serde::{Serialize, Deserialize};

use crate::{io::FileIo, error::BackendResult};

use super::header::DatabaseHeader;



#[derive(Debug,)]
pub struct DBLayout { 

    file_io: FileIo,
    headers: DatabaseHeader, 

}


impl DBLayout { 
    fn init() -> BackendResult<Self> {
        todo!()
    }
}