use super::Canvas;

pub trait UpdateEvent
{
	fn update(&mut self, canvas:&Canvas);
}