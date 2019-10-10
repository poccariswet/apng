use failure::Fail;
use std::io::Error as IOError;

pub type APNGResult<T> = Result<T, APNGError>;

#[derive(Fail, Debug)]
pub enum APNGError {
  #[fail(display = "IO error: {}", 0)]
  Io(IOError),
  #[fail(display = "images are not found")]
  ImagesNotFound,
  #[fail(display = "wrong data size, expected {} got {}", 0, 1)]
  WrongDataSize(usize, usize),
}

macro_rules! define_error {
  ($source:ty, $kind:tt) => {
    impl From<$source> for APNGError {
      fn from(error: $source) -> APNGError {
        APNGError::$kind(error)
      }
    }
  };
}

define_error!(std::io::Error, Io);

pub type AppResult<T> = Result<T, AppError>;

#[derive(Fail, Debug)]
pub enum AppError {
  #[fail(display = "png decode error: {}", 0)]
  PNGImage(png::DecodingError),
}

macro_rules! define_error {
  ($source:ty, $kind:ident) => {
    impl From<$source> for AppError {
      fn from(error: $source) -> AppError {
        AppError::$kind(error)
      }
    }
  };
}

define_error!(png::DecodingError, PNGImage);
