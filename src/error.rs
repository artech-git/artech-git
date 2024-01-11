

pub type BackendResult<T> = Result<T, BackendError> ; 

#[derive(Debug)]
pub struct BackendError { 
    errors: ErrorEnum 
}

impl<T> std::convert::From<T> for BackendError
where T: Into<ErrorEnum> 
{ 
    #[track_caller]
    fn from(val: T) -> Self { 
        Self { 
            errors: val.into()
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorEnum { 

    #[error(transparent)]
    CommandError(#[from] crate::commands::CommandErrors), 

    #[error(transparent)]
    HeaderError(#[from] crate::structure::HeaderError), 

    #[error(transparent)]
    FileError(#[from] crate::io::FileError), 

    #[error("std file opening: {0:?}")]
    StdFileError(#[from] std::io::Error),
    
    #[error(transparent)]
    UtfConvError(#[from] std::str::Utf8Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error)


}