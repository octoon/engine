use super::super::keyframes::Keyframe;
use super::super::core::{Interpolator, Animation, Evaluate};
use super::super::interpolators::{LinearInterpolator};

#[derive(Debug)]
pub struct AnimationCurve<Elem = f32, Time = f32> 
{
	pub frames:Vec<Keyframe<Elem, Time>>,
	pub interpolator:Box<Interpolator + 'static>,
}

impl<Elem, Time> AnimationCurve<Elem, Time>
{
	pub fn new() -> Self
	{
		Self
		{
			frames:Vec::new(),
			interpolator:Box::new(LinearInterpolator::new())
		}
	}

	pub fn new_with(frames:Vec<Keyframe<Elem, Time>>, interpolator:Option<Box<Interpolator + 'static>>) -> Self
	{
		Self
		{
			frames:frames,
			interpolator:interpolator.unwrap_or(Box::new(LinearInterpolator::new()))
		}
	}

	pub fn add(&mut self, key:Keyframe<Elem, Time>)
	{
		self.frames.push(key);
	}

	pub fn add_keyframe(&mut self, time:Time, value:Elem, interpolator:Option<Box<Interpolator + 'static>>)
	{
		self.frames.push(Keyframe::new(time, value, interpolator));
	}
}

impl<T> Evaluate<T> for AnimationCurve<T> where T:Animation + Copy
{
	fn evaluate(&self, time:f32) -> T
	{
		let index = self.frames.binary_search_by(
			|key|
			{
				key.time.partial_cmp(&time).unwrap()
			}
		);

		let x;
		match index
		{
			Ok(value) => { x = value }
			Err(value) => { x = value }
		}

		if x < self.frames.len() - 1
		{
			let anim0 = &self.frames[x];
			let anim1 = &self.frames[x + 1];
			let t = (time - anim0.time) / (anim1.time - anim0.time);

			if anim0.interpolator.is_some()
			{
				let interpolator = anim0.interpolator.as_ref().unwrap();
				anim0.value.animation(&anim1.value, interpolator.interpolator(t))
			}
			else
			{
				anim0.value.animation(&anim1.value, self.interpolator.interpolator(t))
			}
		}
		else
		{
			if x >= self.frames.len() - 1
			{
				self.frames[self.frames.len() - 1].value
			}
			else
			{
				self.frames[0].value
			}
		}
	}
}