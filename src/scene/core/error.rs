use std::fmt;
use std::io;
use crate::models;

pub enum Error {
    IoError(io::Error),
    ImageError(image::ImageError),
    ModelError(models::Error),
    LoaderError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IoError {{ {} }}", err),
            Error::ImageError(ref err) => write!(f, "ImageError {{ {} }}", err),
            Error::ModelError(ref err) => write!(f, "ModelError {{ {} }}", err),
            Error::LoaderError(ref err) => write!(f, "LoaderError {{ {} }}", err),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(_) => "I/O Error",
            Error::ImageError(_) => "Image Error",
            Error::ModelError(_) => "Model Error",
            Error::LoaderError(_) => "Loader Error",
        }
    }

}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<image::ImageError> for Error
{
    fn from(err: image::ImageError) -> Self
    {
        Error::ImageError(err)
    }
}

impl From<models::Error> for Error
{
    fn from(err: models::Error) -> Self
    {
        Error::ModelError(err)
    }
}

impl From<String> for Error
{
    fn from(err: String) -> Self
    {
        Error::LoaderError(err)
    }
}

impl From<std::string::FromUtf8Error> for Error
{
    fn from(_err: std::string::FromUtf8Error) -> Error
    {
        Error::LoaderError("Failed to convert the Utf8".to_string())
    }
}

impl From<std::string::FromUtf16Error> for Error
{
    fn from(_err: std::string::FromUtf16Error) -> Error
    {
        Error::LoaderError("Failed to convert the Utf16".to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;