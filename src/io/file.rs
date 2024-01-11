
use tokio::io::BufStream; 
use tokio::fs::File;

use crate::error::BackendResult; 


#[derive(Debug)]
pub struct FileIo {
    file: BufStream<File>,
    cap: usize,
    pos: usize
}


impl FileIo {

    pub async fn create(path: impl AsRef<std::path::Path>, cap: Option<usize> ) -> BackendResult<Self> {
        
        let cap = cap.unwrap_or(1_00_000); 
        let pos = 0_usize; 

        let fs = File::open(path.as_ref()).await.map_err(|e| FileError::from(e))?; 
        
        let buf_stream = BufStream::with_capacity( cap, cap, fs);
        
        Ok(Self { 
            file: buf_stream, 
            cap: cap,
            pos: pos,
        })
    }

}

impl std::ops::Deref for FileIo {
    type Target = BufStream<File>;
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl std::ops::DerefMut for FileIo { 
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (self.file)
    }
}

use thiserror::Error; 

#[derive(Debug, Error)]
pub enum FileError {
     
    #[error("File could not be opened : {0:?}")]
    FileOpeningError(#[from] tokio::io::Error),
    
    // #[error("std file opening: {0:?}")]
    // StdFileError(#[from] std::io::Error)
}

fn convert_to_file_error(err: impl Into<FileError>) -> FileError { 
    err.into()
}