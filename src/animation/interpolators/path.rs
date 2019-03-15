use super::super::interpolator::*;

#[derive(Debug, Copy, Clone)]
pub struct PathInterpolator<T = f32>
{
	pub xa:T,
	pub xb:T,
	pub ya:T,
	pub yb:T,
}

impl<T> PathInterpolator<T>
{
	pub fn new(xa:T,xb:T,ya:T,yb:T) -> Self
	{
		Self
		{
			xa,
			xb,
			ya,
			yb
		}
	}
}

impl Interpolator for PathInterpolator
{
	fn interpolator(&self, t:f32) -> f32
	{
		let mut min = 0.0;
		let mut max = 1.0;

		let mut ct = t;

		loop
		{
			let x11 = self.xa * ct;
			let x12 = self.xa + (self.xb - self.xa) * ct;
			let x13 = self.xb + (1.0 - self.xb) * ct;

			let x21 = x11 + (x12 - x11) * ct;
			let x22 = x12 + (x13 - x12) * ct;

			let x3 = x21 + (x22 - x21) * ct;

			if f32::abs(x3 - t) < 0.0001
			{
				let y11 = self.ya * ct;
				let y12 = self.ya + (self.yb - self.ya) * ct;
				let y13 = self.yb + (1.0 - self.yb) * ct;

				let y21 = y11 + (y12 - y11) * ct;
				let y22 = y12 + (y13 - y12) * ct;

				let y3 = y21 + (y22 - y21) * ct;

				return y3;
			}
			else if x3 < t
			{
				min = ct;
			}
			else
			{
				max = ct;
			}

			ct = (min + max) * 0.5;
		}
	}
}