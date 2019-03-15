pub trait Animation<T = f32>
{
	fn animation(&self, rhs: &Self, t:T) -> Self; 
}

impl Animation<f32> for f32
{
	fn animation(&self, rhs: &Self, t:f32) -> Self
	{
		self * (1.0 - t) + rhs * t
	}
}

impl Animation<f64> for f64
{
	fn animation(&self, rhs: &Self, t:f64) -> Self
	{
		self * (1.0 - t) + rhs * t
	}
}