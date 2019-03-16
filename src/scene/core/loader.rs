use super::super::{Result, Scene};

pub trait Loader
{
	fn can_read(&self, buf:&[u8]) -> bool;

	fn do_load(&self, buf:&[u8]) -> Result<Scene>;
}