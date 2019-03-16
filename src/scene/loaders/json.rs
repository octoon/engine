use super::super::{Result, Loader, Scene};

#[derive(Debug)]
pub struct JsonLoader {}

impl JsonLoader
{
	pub fn new() -> Self
	{
		Self
		{
		}
	}
}

impl Loader for JsonLoader
{
	fn can_read(&self, _buf:&[u8]) -> bool
	{
		false
	}

	fn do_load(&self, _buf:&[u8]) -> Result<Scene>
	{
		Ok(Scene::new())
	}

	fn do_save(&self, scene:&Scene) -> Result<Vec<u8>>
	{
    	let serialized = serde_json::to_string(scene).unwrap();
    	Ok(serialized.into_bytes())
	}
}