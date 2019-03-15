use super::super::animations::{AnimationClip};
use super::super::core::{Animation};

#[derive(Debug)]
pub struct Animator<Elem = f32, Time = f32> 
{
	pub name:String,
	pub clips:Vec<AnimationClip<Elem, Time>>,
}

impl<Elem, Time> Animator<Elem, Time>
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			clips:Vec::new()
		}
	}

	pub fn add_clip(&mut self, clip:AnimationClip<Elem, Time>)
	{
		self.clips.push(clip);
	}

	pub fn add_clips(&mut self, clips:Vec<AnimationClip<Elem, Time>>)
	{
		for clip in clips
		{
			self.clips.push(clip);			
		}
	}

	pub fn empty(&self) -> bool
	{
		if self.clips.len() > 0 { true } else { false }
	}

	pub fn len(&self) -> usize
	{
		self.clips.len()
	}
}

impl<T> Animator<T> where T:Animation + Copy
{
	pub fn evaluate(&self, time:f32)
	{
		for clip in self.clips.iter()
		{
			clip.evaluate(time);
		}
	}
}