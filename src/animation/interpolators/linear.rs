use std::fmt::Debug;
use super::super::interpolator::*;

#[derive(Debug, Copy, Clone)]
pub struct LinearInterpolator {}

impl LinearInterpolator
{
	pub fn new() -> Self
	{
		Self {}
	}
}

impl<T> Interpolator<T> for LinearInterpolator where T:Debug + Copy
{
	fn interpolator(&self, t:T) -> T
	{
		t
	}
}