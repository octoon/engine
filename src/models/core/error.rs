use std::error;
use std::fmt;
use std::io;

pub struct Error(pub String);

impl fmt::Display for Error
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl fmt::Debug for Error
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
	{
		fmt::Display::fmt(self, f)
	}
}

impl std::error::Error for Error
{
	fn description(&self) -> &str
	{
		"I/O Error"
	}

	fn cause(&self) -> Option<&error::Error>
	{
		Some(self)
	}
}

impl From<io::Error> for Error
{
	fn from(err: io::Error) -> Error
	{
		if let Some(inner_err) = err.get_ref() {
			Error(format!("Inner error: {:?}", inner_err))
		} else {
			Error(format!("No inner error"))
		}
	}
}

impl From<std::string::FromUtf8Error> for Error
{
	fn from(_err: std::string::FromUtf8Error) -> Error
	{
		Error("Failed to convert the Utf8".to_string())
	}
}

impl From<std::string::FromUtf16Error> for Error
{
	fn from(_err: std::string::FromUtf16Error) -> Error
	{
		Error("Failed to convert the Utf16".to_string())
	}
}

pub type Result<T> = std::result::Result<T, Error>;