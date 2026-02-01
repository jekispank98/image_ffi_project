use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File I/O error '{path}': {source}")]
    IO {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Image process error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Plugin error: {0}")]
    Plugin(#[from] libloading::Error),

    #[error("Params error (null-byte): {0}")]
    Utf8(#[from] std::ffi::NulError),

    #[error("Execution error: {0}")]
    Generic(String),
}