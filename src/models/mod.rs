pub mod core;
pub mod loaders;

pub use self::core::*;
pub use self::loaders::*;

use std::io::prelude::*;
use std::fs::File;

pub fn load_from_memory(buf:&[u8]) -> Result<Model>
{
	let loaders = vec![Box::new(PMXLoader::new())];

	for loader in loaders
	{
		if loader.can_read(buf)
		{
			return Ok(loader.do_load(buf)?);
		}
	}

	Err(Error("Not supported yet".to_string()))
}

pub fn load_from_buf<R:BufRead + Seek>(mut r:R) -> Result<Model>
{
	load_from_memory(r.fill_buf()?)
}

pub fn open<P: AsRef<std::path::Path>>(path:P) -> Result<Model>
{
	let mut buffer = Vec::new();
	File::open(path)?.read_to_end(&mut buffer)?;
	load_from_memory(&buffer)
}