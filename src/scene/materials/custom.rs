use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use crate::math::*;
use crate::math::type_size::*;
use super::super::core::*;
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug)]
pub struct CustomMaterial
{
	pub uuid: uuid::Uuid,
	pub state:RenderState,
	pub attribs:Vec<VertexAttrib>,
	pub uniforms:HashMap<String, Variant>
}

impl CustomMaterial
{
	pub fn new() -> Self
	{
		let mut attribs = Vec::new();
		let stride = (float3::type_size() + float3::type_size() + float2::type_size()) as u8;
		attribs.push(VertexAttrib::new(0, Format::RGBSFloat(8,8,8), stride, 0));
		attribs.push(VertexAttrib::new(1, Format::RGBSFloat(8,8,8), stride, float3::type_size() as _));
		attribs.push(VertexAttrib::new(2, Format::RGSFloat(8,8), stride, (float3::type_size() * 2) as _));

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			state:RenderState::new(),
			uniforms:HashMap::new(),
			attribs:attribs
		}
	}
}

impl Material for CustomMaterial 
{
	fn input_layout(&self) -> &[VertexAttrib]
	{
		&self.attribs
	}

	fn uniforms(&self) -> &HashMap<String, Variant>
	{
		&self.uniforms
	}

	fn set_uniform(&mut self, name:&str, value:Variant)
	{
		if let Some(x) = self.uniforms.get_mut(name)
		{
			*x = value;
		}
	}

	fn cull_mode(&self) -> CullMode
	{
		return CullMode::None;
	}
}

impl Resource for CustomMaterial
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl AsRef<RenderState> for CustomMaterial
{
	fn as_ref(&self) -> &RenderState
	{
		&self.state
	}
}

impl AsMut<RenderState> for CustomMaterial
{
	fn as_mut(&mut self) -> &mut RenderState
	{
		&mut self.state
	}
}

impl From<CustomMaterial> for Rc<Material + 'static>
{
	fn from(material:CustomMaterial) -> Self
	{
		Rc::new(material)
	}
}

impl From<CustomMaterial> for Arc<Material + 'static>
{
	fn from(material:CustomMaterial) -> Self
	{
		Arc::new(material)
	}
}

impl From<CustomMaterial> for Rc<RefCell<Material + 'static>>
{
	fn from(material:CustomMaterial) -> Self
	{
		Rc::new(RefCell::new(material))
	}
}

impl From<CustomMaterial> for Arc<RefCell<Material + 'static>>
{
	fn from(material:CustomMaterial) -> Self
	{
		Arc::new(RefCell::new(material))
	}
}