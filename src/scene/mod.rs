pub mod core;
pub mod spectrum;
pub mod cameras;
pub mod geometries;
pub mod lights;
pub mod shapes;
pub mod materials;
pub mod scene;
pub mod loaders;
pub mod util;

pub use self::core::*;
pub use self::spectrum::*;
pub use self::cameras::*;
pub use self::geometries::*;
pub use self::lights::*;
pub use self::shapes::*;
pub use self::materials::*;
pub use self::scene::*;
pub use self::loaders::*;
pub use self::util::*;

use std::io::prelude::*;
use std::fs::File;

pub fn load_from_memory(buf:&[u8]) -> Result<Scene>
{
	let loaders = vec![Box::new(PMMLoader::new())];

	for loader in loaders
	{
		if loader.can_read(buf)
		{
			return Ok(loader.do_load(buf)?);
		}
	}

	Err(Error::LoaderError("Not supported yet".to_string()))
}

pub fn load_from_buf<R:BufRead + Seek>(mut r:R) -> Result<Scene>
{
	load_from_memory(r.fill_buf()?)
}

pub fn open<P: AsRef<std::path::Path>>(path:P) -> Result<Scene>
{
	let mut buffer = Vec::new();
	File::open(path)?.read_to_end(&mut buffer)?;
	load_from_memory(&buffer)
}