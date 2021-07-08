use std::io::Error as IOError;
use thiserror::Error as ThisError;

pub type APNGResult<T> = Result<T, APNGError>;

#[derive(ThisError, Debug)]
pub enum APNGError {
    #[error("IO error: {0}")]
    Io(#[from] IOError),
    #[error("images are not found")]
    ImagesNotFound,
    #[error("wrong data size, expected {0} got {1}")]
    WrongDataSize(usize, usize),
    #[error("wrong frames nums, expected {0} got {1}")]
    WrongFrameNums(usize, usize),
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("png decode error: {0}")]
    PNGImage(png::DecodingError),
}
