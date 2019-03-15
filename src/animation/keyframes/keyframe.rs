use std::boxed::Box;
use super::super::core::{Interpolator};

#[derive(Debug, Default)]
pub struct Keyframe<Elem = f32, Time = f32>
{
	pub time:Time,
	pub value:Elem,
	pub interpolator:Option<Box<Interpolator + 'static>>
}

impl<Elem, Time> Keyframe<Elem, Time>
{
	pub fn new(time:Time, value:Elem, interpolator:Option<Box<Interpolator + 'static>>) -> Self
	{
		Self
		{
			time:time,
			value:value,
			interpolator:interpolator
		}
	}
}