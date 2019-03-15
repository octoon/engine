use super::Canvas;

pub trait UpdateEvent
{
	fn update(&mut self, canvas:&Canvas);
}

pub trait StateEvent
{
    fn animate(&self, time:f32);
}