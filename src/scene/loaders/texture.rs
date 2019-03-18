use std::io::{Seek, BufRead};
use image::{ImageResult, GenericImageView, DynamicImage};
use super::super::core::Result;
use super::super::core::{Texture, ColorType};

#[derive(Debug)]
pub struct TextureLoader {}

impl TextureLoader
{
	fn load_from_image(img:ImageResult<DynamicImage>, name:Option<String>) -> Result<Texture>
	{
		let color_type;
		let image = img?;

		match image.color()
		{
			image::ColorType::Gray(n) => { color_type = ColorType::Gray(n); },
			image::ColorType::GrayA(n) => { color_type = ColorType::GrayA(n); },
			image::ColorType::RGBA(n) => { color_type = ColorType::RGBA(n); },
			image::ColorType::RGB(n) => { color_type = ColorType::RGB(n); },
			image::ColorType::BGRA(n) => { color_type = ColorType::BGRA(n); },
			image::ColorType::BGR(n) => { color_type = ColorType::BGR(n); },
			image::ColorType::Palette(n) => { color_type = ColorType::Palette(n); },
		};

		let mut texture = Texture::new(color_type, image.width(), image.height(), image.raw_pixels());
		match name
		{
			Some(name) => texture.set_name(&name),
			None => {},
		}
		Ok(texture)
	}

	pub fn load<P: AsRef<std::path::Path>>(path:P) -> Result<Texture>
	{
		let mut texture = TextureLoader::load_from_image(image::open(&path), None)?;
		match path.as_ref().to_str()
		{
			Some(name) => texture.set_name(name),
			None => {},
		}
		Ok(texture)
	}

	pub fn load_from_buf<R:BufRead + Seek>(r:R, format:image::ImageFormat, name:Option<String>) -> Result<Texture>
	{
		TextureLoader::load_from_image(image::load(r, format), name)
	}

	pub fn load_from_memory(buffer:&[u8], name:Option<String>) -> Result<Texture>
	{
		TextureLoader::load_from_image(image::load_from_memory(buffer), name)
	}
}