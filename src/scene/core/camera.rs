use std::f32;
use crate::math::{float4x4, One};
use super::Object;

#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub enum Dimensions
{
	Automatic,
	Sized(f32),
}

#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub enum CameraType
{
	Main,
	Shadow,
	Custom
}

impl CameraType
{
	pub fn as_int(&self) -> u32
	{
		match self
		{
			CameraType::Main => 0,
			CameraType::Shadow => 1,
			CameraType::Custom => 2,
		}
	}

	pub fn cmp(&self, other:&Self) -> std::cmp::Ordering
	{
		self.as_int().cmp(&other.as_int())
	}
}

#[derive(Debug, Copy, Clone)]
pub struct CameraData
{
	pub kind:CameraType,
	pub view:float4x4,
	pub view_inverse:float4x4,
	pub projection:float4x4,
	pub projection_inverse:float4x4,
	pub view_projection:float4x4,
	pub view_projection_inverse:float4x4,
}

impl CameraData
{
	pub fn new() -> Self
	{
		Self
		{
			kind:CameraType::Custom,
			view:float4x4::one(),
			view_inverse:float4x4::one(),
			projection:float4x4::one(),
			projection_inverse:float4x4::one(),
			view_projection:float4x4::one(),
			view_projection_inverse:float4x4::one(),
		}
	}
}

pub trait Camera : Object
{
	fn kind(&self) -> CameraType;
	
	fn width(&self) -> f32;
	fn height(&self) -> f32;
	fn ratio(&self) -> f32 { self.width() / self.height() }

	fn focal_length(&self) -> f32;
	fn focal_distance(&self) -> f32;

	fn exposure(&self) -> f32;

	fn clear_color(&self) -> (f32,f32,f32,f32);
	fn viewport(&self) -> (f32,f32,f32,f32);

	fn view(&self) -> float4x4;
	fn view_inverse(&self) -> float4x4;
	fn view_projection(&self) -> float4x4;
	fn view_projection_inverse(&self) -> float4x4;
	fn projection(&self) -> float4x4;
	fn projection_inverse(&self) -> float4x4;

	fn set_clear_color(&mut self, r:f32, g:f32, b:f32, a:f32);
	fn set_viewport(&mut self, x:f32, y:f32, z:f32, w:f32);
}