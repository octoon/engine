use std::fmt::Debug;
use crate::math::{float3, float4x4};
use super::{Resource, Downcast};
use super::super::scene::{SceneNode};

pub trait Object : 
	  AsRef<SceneNode> 
	+ AsMut<SceneNode> 
	+ Debug
	+ Downcast 
	+ Resource
{
	#[inline(always)]
	fn is_visible(&self) -> bool
	{
		self.as_ref().is_visible()
	}

	#[inline(always)]
	fn set_visible(&mut self, visible: bool)
	{
		self.as_mut().set_visible(visible);
	}

	#[inline(always)]
	fn name(&self) -> String
	{
		self.as_ref().name()
	}

	#[inline(always)]
	fn set_name(&mut self, name: &str)
	{
		self.as_mut().set_name(name);
	}

	#[inline(always)]
	fn translate(&self) -> float3
	{
		self.as_ref().translate()
	}
	
	#[inline(always)]
	fn set_translate(&mut self, pos:float3) -> &mut SceneNode
	{
		self.as_mut().set_translate(pos);
		self.as_mut()
	}

	#[inline(always)]
	fn scale(&self) -> float3
	{
		self.as_ref().scale()
	}

	#[inline(always)]
	fn set_scale(&mut self, sz:float3) -> &mut SceneNode
	{
		self.as_mut().set_scale(sz);
		self.as_mut()
	}
	
	#[inline(always)]
	fn rotation(&self) -> float3
	{
		self.as_ref().rotation()
	}
	
	#[inline(always)]
	fn set_rotation(&mut self, rot:float3) -> &mut SceneNode
	{
		self.as_mut().set_rotation(rot);
		self.as_mut()
	}

	#[inline(always)]
	fn transform(&self) -> float4x4
	{
		self.as_ref().transform()
	}
	
	#[inline(always)]
	fn transform_inverse(&self) -> float4x4
	{
		self.as_ref().transform_inverse()
	}
	
	#[inline(always)]
	fn up(&self) -> float3
	{
		self.transform().up()
	}

	#[inline(always)]
	fn set_up(&mut self, speed:float3) -> &mut SceneNode
	{
		let up = self.up();
		self.set_translate(up * speed);
		self.as_mut()
	}

	#[inline(always)]
	fn right(&self) -> float3
	{
		self.transform().right()
	}

	#[inline(always)]
	fn set_right(&mut self, speed:float3) -> &mut SceneNode
	{
		let right = self.right();
		self.set_translate(right * speed);
		self.as_mut()
	}

	#[inline(always)]
	fn forward(&self) -> float3
	{
		self.transform().forward()
	}

	#[inline(always)]
	fn set_forward(&mut self, speed:float3) -> &mut SceneNode
	{
		let forward = self.forward();
		self.set_translate(forward * speed);
		self.as_mut()
	}
}