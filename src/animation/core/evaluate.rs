pub trait Evaluate<T = f32, Time = f32>
{
	fn evaluate(&self, time:Time) -> T;
}