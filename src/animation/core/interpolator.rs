use std::fmt::Debug;

pub trait Interpolator<T = f32> : Debug
{
	fn interpolator(&self, t:T) -> T;
}