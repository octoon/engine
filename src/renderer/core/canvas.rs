use crate::scene::Scene;

pub trait Canvas
{
	fn width(&self) -> u32;
	fn height(&self) -> u32;

	fn render(&mut self, scene: &Scene);
}