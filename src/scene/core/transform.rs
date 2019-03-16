use std::cell::RefCell;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use crate::math::{float3, Quaternion, float4x4, One, Zero};

#[derive(Debug, Default, Clone)]
pub struct Transform 
{
	translate:float3,
	scale:float3,
	rotation:float3,
	transform:RefCell<float4x4>,
	transform_inverse:RefCell<float4x4>,
	need_update:RefCell<bool>
}

impl Transform 
{
	pub fn new() -> Self 
	{
		Self
		{
			translate:float3::zero(),
			scale:float3::one(),
			rotation:float3::zero(),
			transform:RefCell::new(float4x4::one()),
			transform_inverse:RefCell::new(float4x4::one()),
			need_update:RefCell::new(true)
		}
	}

	#[inline(always)]
	pub fn translate(&self) -> float3
	{
		return self.translate;
	}

	#[inline(always)]
	pub fn scale(&self) -> float3
	{
		return self.scale;
	}

	#[inline(always)]
	pub fn rotation(&self) -> float3
	{
		return self.rotation;
	}

	#[inline(always)]
	pub fn transform(&self) -> float4x4
	{
		self.update();
		return *self.transform.borrow();
	}

	#[inline(always)]
	pub fn transform_inverse(&self) -> float4x4
	{
		self.update();
		return *self.transform_inverse.borrow();
	}

	#[inline]
	pub fn set_translate(&mut self, pos:float3)
	{
		if self.translate != pos
		{
			self.translate = pos;
			self.need_update.replace(true);
		}
	}

	#[inline]
	pub fn set_scale(&mut self, sz:float3)
	{
		if self.scale != sz
		{
			self.scale = sz;
			self.need_update.replace(true);
		}
	}

	#[inline]
	pub fn set_rotation(&mut self, rot:float3)
	{
		if self.rotation != rot
		{
			self.rotation = rot;
			self.need_update.replace(true);
		}
	}

	#[inline]
	fn update(&self)
	{
		if *self.need_update.borrow()
		{
			let translate = float4x4::translate(self.translate.x, self.translate.y, self.translate.z);
			let rotation:float4x4 = Quaternion::euler_xyz(&self.rotation).into();
			let scale = float4x4::scale(self.scale.x, self.scale.y, self.scale.z);
			let transform = translate * rotation * scale;

			self.transform.replace(transform);
			self.transform_inverse.replace(transform.transform_inverse());

			self.need_update.replace(false);
		}
	}
}

impl Serialize for Transform
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut s = serializer.serialize_struct("transform", 3)?;
		s.serialize_field("translate", &self.translate)?;
		s.serialize_field("rotation", &self.rotation)?;
		s.serialize_field("scale", &self.scale)?;
		s.end()
	}
}