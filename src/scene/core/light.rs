use std::sync::Arc;
use crate::math::float3;
use super::{Object, Texture};
use super::super::spectrum::*;

#[derive(Debug, Copy, PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum LightType
{
	Sky,
	Directional,
	Point,
	Spot,
}

#[derive(Debug, Clone)]
pub struct LightData
{
	pub kind:LightType,
	pub color:Spectrum,
	pub intensity:f32,
	pub radiance:Option<Arc<Texture>>,
	pub irradiance:Option<Arc<Texture>>,
	pub direction:float3,
	pub cos_angle:f32
}

impl LightData
{
	pub fn intensity(&self) -> f32
	{
		self.intensity
	}

	pub fn cos_angle(&self) -> f32
	{
		self.cos_angle
	}

	pub fn direction(&self) -> float3
	{
		self.direction
	}
}

pub trait Light : Object
{
	fn kind(&self) -> LightType;

	fn color(&self) -> Spectrum;
	fn intensity(&self) -> f32;

	fn set_color(&mut self, spectrum:Spectrum);
	fn set_intensity(&mut self, cd:f32);
}