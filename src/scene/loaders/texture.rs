use std::io::{Seek, BufRead};
use image::{ImageResult, GenericImageView, DynamicImage};
use super::super::core::Result;
use super::super::core::{Texture, ColorType};

#[derive(Debug)]
pub struct TextureLoader {}

impl TextureLoader
{
	fn load_from_image(img:ImageResult<DynamicImage>) -> Result<Texture>
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

		Ok(Texture::new(color_type, image.width(), image.height(), image.raw_pixels()))
	}

	pub fn load<P: AsRef<std::path::Path>>(path:P) -> Result<Texture>
	{
		TextureLoader::load_from_image(image::open(path))
	}

	pub fn load_from_buf<R:BufRead + Seek>(r:R, format:image::ImageFormat) -> Result<Texture>
	{
		TextureLoader::load_from_image(image::load(r, format))
	}

	pub fn load_from_memory(buffer:&[u8]) -> Result<Texture>
	{
		TextureLoader::load_from_image(image::load_from_memory(buffer))
	}
}