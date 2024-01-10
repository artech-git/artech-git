
use tokio::io::BufStream; 
use tokio::fs::File;

use crate::error::BackendResult; 


#[derive(Debug)]
pub struct FileIo {
    file: BufStream<File>,
    cap: usize
}

impl FileIo {
    async fn create(path: impl AsRef<std::path::Path>, cap: Option<usize> ) -> BackendResult<Self> {
        
        let cap = cap.unwrap_or(1_00_000); 
        let fs = File::open(path.as_ref()).await.map_err(|e| FileError::from(e) )?; 
        let buf_stream = BufStream::with_capacity( cap, cap, fs);
        
        Ok(Self { 
            file: buf_stream, 
            cap: cap
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FileError  { 

    #[error("File could not be opened : {0:?}")]
    FileOpeningError(#[from] tokio::io::Error),

}

fn convert_to_file_error(err: impl Into<FileError>) -> FileError { 
    err.into()
}