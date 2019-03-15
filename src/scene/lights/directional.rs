use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::core::*;
use super::super::spectrum::*;
use super::super::scene::{ SceneNode, SceneSubData };

#[derive(Debug)]
pub struct DirectionalLight 
{
	pub node:SceneNode,
	pub spectrum:Spectrum,
	pub intensity:f32,
	pub direction:float3
}

impl DirectionalLight 
{
	#[inline]
	pub fn new() -> Self 
	{
		let userdata = LightData
		{
			kind:LightType::Directional,
			color:Spectrum::one(),
			intensity:1.0,
			radiance:None,
			irradiance:None,
			cos_angle:0.0,
			direction:-float3::unit_y()
		};

		let mut node = SceneNode::new(SceneSubData::Light);
		node.set_user_data(Box::new(userdata));

		Self
		{
			node:node,
			spectrum:Spectrum::one(),
			intensity:1.0,
			direction:-float3::unit_y(),
		}
	}

	#[inline(always)]
	pub fn builder() -> DirectionalLightBuilder
	{
		DirectionalLightBuilder::new()
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
}

impl Light for DirectionalLight
{
	fn kind(&self) -> LightType
	{
		LightType::Directional
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

impl Object for DirectionalLight
{
}

impl Resource for DirectionalLight
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Downcast for DirectionalLight
{
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for DirectionalLight
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for DirectionalLight
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<DirectionalLight> for Box<Light + 'static>
{
	fn from(light:DirectionalLight) -> Self
	{
		Box::new(light)
	}
}

impl From<DirectionalLight> for Rc<Light + 'static>
{
	fn from(light:DirectionalLight) -> Self
	{
		Rc::new(light)
	}
}

impl From<DirectionalLight> for Arc<Light + 'static>
{
	fn from(light:DirectionalLight) -> Self
	{
		Arc::new(light)
	}
}

impl From<DirectionalLight> for Rc<RefCell<Light + 'static>>
{
	fn from(light:DirectionalLight) -> Self
	{
		Rc::new(RefCell::new(light))
	}
}

impl From<DirectionalLight> for Arc<RefCell<Light + 'static>>
{
	fn from(light:DirectionalLight) -> Self
	{
		Arc::new(RefCell::new(light))
	}
}

pub struct DirectionalLightBuilder
{
	light:DirectionalLight
}

impl DirectionalLightBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			light:DirectionalLight::new()
		}
	}

	#[inline]
	pub fn build(self) -> DirectionalLight
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