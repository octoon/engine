use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::core::*;
use super::super::spectrum::*;
use super::super::scene::{ SceneNode, SceneSubData };

#[derive(Debug)]
pub struct SkyLight 
{
	pub node:SceneNode,
	pub spectrum:Spectrum,
	pub intensity:f32,
}

impl SkyLight 
{
	#[inline]
	pub fn new(irradiance:Arc<Texture>, radiance:Arc<Texture>) -> Self 
	{
		let userdata = LightData
		{
			kind:LightType::Sky,
			color:Spectrum::one(),
			intensity:1.0,
			radiance:Some(irradiance),
			irradiance:Some(radiance),
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
		}
	}

	#[inline(always)]
	pub fn builder(irradiance:Arc<Texture>, radiance:Arc<Texture>) -> SkyLightBuilder
	{
		SkyLightBuilder::new(irradiance, radiance)
	}

	#[inline(always)]
	pub fn irradiance(&self) -> Arc<Texture>
	{
		match self.node.data.borrow().user_data.downcast_ref::<LightData>()
		{
			Some(data) => { data.irradiance.as_ref().unwrap().clone() },
			None => { panic!("No irradiance map"); },
		}
	}

	#[inline(always)]
	pub fn radiance(&self) -> Arc<Texture>
	{
		match self.node.data.borrow().user_data.downcast_ref::<LightData>()
		{
			Some(data) => { data.radiance.as_ref().unwrap().clone() },
			None => { panic!("No radiance map"); },
		}
	}
}

impl Light for SkyLight
{
	fn kind(&self) -> LightType
	{
		LightType::Sky
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

impl Resource for SkyLight
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Object for SkyLight
{
}

impl Downcast for SkyLight
{
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for SkyLight
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for SkyLight
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<SkyLight> for Box<Light + 'static>
{
	fn from(light:SkyLight) -> Self
	{
		Box::new(light)
	}
}

impl From<SkyLight> for Rc<Light + 'static>
{
	fn from(light:SkyLight) -> Self
	{
		Rc::new(light)
	}
}

impl From<SkyLight> for Arc<Light + 'static>
{
	fn from(light:SkyLight) -> Self
	{
		Arc::new(light)
	}
}

impl From<SkyLight> for Rc<RefCell<Light + 'static>>
{
	fn from(light:SkyLight) -> Self
	{
		Rc::new(RefCell::new(light))
	}
}

impl From<SkyLight> for Arc<RefCell<Light + 'static>>
{
	fn from(light:SkyLight) -> Self
	{
		Arc::new(RefCell::new(light))
	}
}

pub struct SkyLightBuilder
{
	light:SkyLight
}

impl SkyLightBuilder
{
	#[inline]
	pub fn new(irradiance:Arc<Texture>, radiance:Arc<Texture>) -> Self
	{
		Self
		{
			light:SkyLight::new(irradiance, radiance)
		}
	}

	#[inline]
	pub fn build(self) -> SkyLight
	{
		self.light
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