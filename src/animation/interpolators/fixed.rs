use std::fmt::Debug;
use super::super::interpolator::*;

#[derive(Debug, Copy, Clone)]
pub struct FixedInterpolator<T = f32>
{
	pub value:T
}

impl<T> FixedInterpolator<T>
{
	pub fn new(value:T) -> Self
	{
		Self
		{
			value
		}
	}
}

impl<T> Interpolator<T> for FixedInterpolator<T> where T:Debug + Copy
{
	fn interpolator(&self, _:T) -> T
	{
		self.value
	}
}