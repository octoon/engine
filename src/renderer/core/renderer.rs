use crate::scene::Scene;
use super::Canvas;

pub trait Renderer : Canvas
{
	fn set_width(&mut self, width:u32);
	fn set_height(&mut self, height:u32);

	fn render(&mut self, scene: &Scene);
}