use std::collections::HashMap;
use std::ops::Fn;
use std::fmt::Debug;

use super::super::animations::{AnimationCurve};
use super::super::core::{Animation, Evaluate};

pub struct AnimationClip<Elem = f32, Time = f32> 
{
	pub name:String,
	pub curves:HashMap<String, AnimationCurve<Elem, Time>>,
	pub events:Vec<Box<Fn(&str, &Elem)>>
}

impl<Elem, Time> AnimationClip<Elem, Time>
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			curves:HashMap::new(),
			events:Vec::new(),
		}
	}

	#[inline(always)]
	pub fn set_name(&mut self, name:&str)
	{
		self.name = name.to_string();
	}

	#[inline(always)]
	pub fn set_curve(&mut self, name:&str, curve:AnimationCurve<Elem, Time>)
	{
		self.curves.insert(name.to_string(), curve);
	}

	#[inline(always)]
	pub fn empty(&self) -> bool
	{
		if self.curves.len() > 0 { true } else { false }
	}

	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.curves.len()
	}

	#[inline(always)]
	pub fn add_event<T>(&mut self, method:T) -> &mut Self where T:Fn(&str, &Elem) + 'static
	{
		self.events.push(Box::new(method));
		self
	}
}

impl<T> AnimationClip<T> where T:Animation + Copy
{
	pub fn evaluate(&self, time:f32)
	{
		for (key, curve) in self.curves.iter()
		{
			let value = curve.evaluate(time);

			for event in self.events.iter()
			{
				event(key, &value)
			}
		}
	}
}

impl<Elem:Debug, Time:Debug> std::fmt::Debug for AnimationClip<Elem, Time>
{
	fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{:?},{:?}",
			self.name,
			self.curves,
		)
	}
}