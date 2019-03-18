use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use super::{Resource, Sampler, SamplerWrap, SamplerFilter};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum ColorType
{
	Gray(u8),
	GrayA(u8),
	RGB(u8),
	RGBA(u8),
	BGR(u8),
	BGRA(u8),
	Palette(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
pub struct Texture
{
	pub uuid:uuid::Uuid,
	pub sampler:Sampler,
	pub width:u32,
	pub height:u32,
	pub color:ColorType,
	pub raw_pixels:Vec<u8>,
	pub name:String
}

impl Texture
{
	pub fn new(color_type:ColorType, width:u32, height:u32, raw_pixels:Vec<u8>) -> Self
	{
		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			sampler:Sampler::new(SamplerWrap::Repeat,SamplerFilter::Nearest,SamplerFilter::Nearest),
			color:color_type,
			width:width,
			height:height,
			raw_pixels:raw_pixels,
			name:String::new()
		}
	}

	#[inline(always)]
	pub fn width(&self) -> u32
	{
		self.width
	}

	#[inline(always)]
	pub fn height(&self) -> u32
	{
		self.height
	}

	#[inline(always)]
	pub fn color_type(&self) -> ColorType
	{
		self.color
	}

	#[inline(always)]
	pub fn raw_pixels(&self) -> &[u8]
	{
		&self.raw_pixels
	}

	#[inline(always)]
	pub fn name(&self) -> &str
	{
		&self.name
	}

	#[inline(always)]
	pub fn set_name(&mut self, name:&str)
	{
		self.name = name.to_string()
	}
}

impl Resource for Texture
{
	#[inline(always)]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<Texture> for Rc<RefCell<Texture>>
{
	#[inline(always)]
	fn from(texture:Texture) -> Self
	{
		Rc::new(RefCell::new(texture))
	}
}

impl From<Texture> for Arc<RefCell<Texture>>
{
	#[inline(always)]
	fn from(texture:Texture) -> Self
	{
		Arc::new(RefCell::new(texture))
	}
}

impl Serialize for Texture
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut s = serializer.serialize_struct("texture", 4)?;
		s.serialize_field("uuid", &self.uuid)?;
		s.serialize_field("name", &self.name)?;
		s.serialize_field("sampler", &self.sampler)?;
		s.end()
	}
}