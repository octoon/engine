use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::core::*;
use super::super::spectrum::*;
use super::super::scene::{ SceneNode, SceneSubData };

#[derive(Debug)]
pub struct SpotLight 
{
	pub node:SceneNode,
	pub spectrum:Spectrum,
	pub intensity:f32,
	pub direction:float3,
	pub angle:f32
}

impl SpotLight 
{
	#[inline]
	pub fn new() -> Self 
	{
		let userdata = LightData
		{
			kind:LightType::Spot,
			color:Spectrum::one(),
			intensity:1.0,
			radiance:None,
			irradiance:None,
			cos_angle:70.0,
			direction:-float3::unit_y()
		};

		let mut node = SceneNode::new(SceneSubData::Light);
		node.set_user_data(Box::new(userdata));

		Self
		{
			node:SceneNode::new(SceneSubData::Light),
			spectrum:Spectrum::one(),
			intensity:1.0,
			direction:-float3::unit_y(),
			angle:70.0
		}
	}

	#[inline(always)]
	pub fn builder() -> SpotLightBuilder
	{
		SpotLightBuilder::new()
	}

	#[inline(always)]
	pub fn direction(&self) -> float3
	{
		self.direction
	}

	#[inline(always)]
	pub fn set_direction(&mut self, dir:float3) -> &mut Self
	{
		self.direction = dir;

		match self.node.data.borrow_mut().user_data.downcast_mut::<LightData>()
		{
			Some(data) => { data.direction = self.direction; },
			None => {},
		}

		self
	}

	#[inline(always)]
	pub fn angle(&self) -> f32
	{
		self.angle
	}

	#[inline(always)]
	pub fn cos_angle(&self) -> f32
	{
		self.angle.to_degrees().cos()
	}

	#[inline(always)]
	pub fn set_angle(&mut self, angle:f32) -> &mut Self
	{
		self.angle = angle;

		match self.node.data.borrow_mut().user_data.downcast_mut::<LightData>()
		{
			Some(data) => { data.cos_angle = self.cos_angle(); },
			None => {},
		}

		self
	}
}

impl Light for SpotLight
{
	fn kind(&self) -> LightType
	{
		LightType::Spot
	}

	fn color(&self) -> Spectrum
	{
		self.spectrum
	}

	fn intensity(&self) -> f32
	{
		self.intensity
	}

	fn set_color(&mut self, spectrum:Spectrum)
	{
		self.spectrum = spectrum;

		match self.node.data.borrow_mut().user_data.downcast_mut::<LightData>()
		{
			Some(data) => { data.color = self.spectrum; },
			None => {},
		}
	}

	fn set_intensity(&mut self, intensity:f32)
	{
		self.intensity = intensity;

		match self.node.data.borrow_mut().user_data.downcast_mut::<LightData>()
		{
			Some(data) => { data.intensity = self.intensity; },
			None => {},
		}
	}
}

impl Object for SpotLight
{
}

impl Resource for SpotLight
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Downcast for SpotLight
{
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for SpotLight
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for SpotLight
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<SpotLight> for Box<Light + 'static>
{
	fn from(light:SpotLight) -> Self
	{
		Box::new(light)
	}
}

impl From<SpotLight> for Rc<Light + 'static>
{
	fn from(light:SpotLight) -> Self
	{
		Rc::new(light)
	}
}

impl From<SpotLight> for Arc<Light + 'static>
{
	fn from(light:SpotLight) -> Self
	{
		Arc::new(light)
	}
}

impl From<SpotLight> for Rc<RefCell<Light + 'static>>
{
	fn from(light:SpotLight) -> Self
	{
		Rc::new(RefCell::new(light))
	}
}

impl From<SpotLight> for Arc<RefCell<Light + 'static>>
{
	fn from(light:SpotLight) -> Self
	{
		Arc::new(RefCell::new(light))
	}
}

pub struct SpotLightBuilder
{
	light:SpotLight
}

impl SpotLightBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			light:SpotLight::new()
		}
	}

	#[inline]
	pub fn build(self) -> SpotLight
	{
		self.light
	}

	#[inline]
	pub fn set_direction(mut self, dir:float3) -> Self
	{
		self.light.set_direction(dir);
		self
	}

	#[inline]
	pub fn set_angle(mut self, angle:f32) -> Self
	{
		self.light.set_angle(angle);
		self
	}

	#[inline]
	pub fn set_color(mut self, spectrum:Spectrum) -> Self
	{
		self.light.set_color(spectrum);
		self
	}

	#[inline]
	pub fn set_intensity(mut self, intensity:f32) -> Self
	{
		self.light.set_intensity(intensity);
		self
	}

	#[inline]
	pub fn set_translate(mut self, pos:float3) -> Self
	{
		self.light.set_translate(pos);
		self
	}

	#[inline]
	pub fn set_scale(mut self, sz:float3) -> Self
	{
		self.light.set_scale(sz);
		self
	}

	#[inline]
	pub fn set_rotation(mut self, rot:float3) -> Self
	{
		self.light.set_rotation(rot);
		self
	}
}