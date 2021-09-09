use thiserror::Error;

#[derive(Debug, Error)]
pub enum GdlError {
    #[error("Io error: {0}")]
    IoError(std::io::Error),
    #[error("Serde error: {0}")]
    SerdeError(serde_json::Error),
    #[error("Program doesn't exist : {0}")]
    WhichError(which::Error),
    #[error("Invalid node content : {0}")]
    InvalidNodeContent(&'static str),
}

impl From<std::io::Error> for GdlError {
    fn from(err : std::io::Error) -> Self {
        Self::IoError(err)
    }
}
impl From<serde_json::Error> for GdlError {
    fn from(err : serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}

impl From<which::Error> for GdlError {
    fn from(err : which::Error) -> Self {
        Self::WhichError(err)
    }
}
