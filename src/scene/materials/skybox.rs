use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::hash_map::HashMap;

use crate::math::*;
use crate::math::type_size::*;
use super::super::core::*;
use super::super::util::uuid::OsRandNewV4;

pub static SHADER_VERTEX:&'static str = r#"
Result main_vs(Args args)
{
	Result result;
	result.position = transform(args.position);
	result.normal = args.normal;
	result.coord = args.coord;
	return result;
}
"#;

pub static SHADER_FRAGMENT:&'static str = r#"
uniform float3 color;
uniform sampler2D texture;

uniform bool texture_enable;

Gbuffer main_fs(Args args)
{
	Gbuffer buffer;
	buffer.albedo = float3(1e-16,1e-16,1e-16);
	buffer.specular = float3(0.0,0.0,0.0);
	buffer.emissive = color;
	buffer.smoothness = 0.0;
	buffer.metalness = 0.0;
	buffer.normal = normalize(args.normal);

	if (texture_enable)
	{
		buffer.emissive *= pow(texture2D(texture, ComputeSphereCoord(buffer.normal)).xyz, float3(2.2));
	}

	return buffer;
}"#;


#[derive(Debug)]
pub struct SkyboxMaterial
{
	pub uuid: uuid::Uuid,
	pub state:RenderState,
	pub attribs:Vec<VertexAttrib>,
	pub uniforms:HashMap<String, Variant>
}

impl SkyboxMaterial
{
	pub fn new() -> Self
	{
		let mut params = HashMap::new();
		params.insert("color".to_string(), float3::one().into());
		params.insert("texture".to_string(), None.into());
		params.insert("texture_enable".to_string(), float1::zero().into());

		let mut attribs = Vec::new();
		let stride = (float3::type_size() + float3::type_size() + float2::type_size()) as u8;
		attribs.push(VertexAttrib::new(0, Format::RGBSFloat(8,8,8), stride, 0));
		attribs.push(VertexAttrib::new(1, Format::RGBSFloat(8,8,8), stride, float3::type_size() as _));
		attribs.push(VertexAttrib::new(2, Format::RGSFloat(8,8), stride, (float3::type_size() * 2) as _));

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			state:RenderState::new(),
			uniforms:params,
			attribs:attribs
		}
	}

	#[inline]
	pub fn builder() -> SkyboxMaterialBuilder
	{
		SkyboxMaterialBuilder::new()
	}

	pub fn set_color(&mut self, value:float3) -> &mut Self
	{
		self.set_uniform("color", value.into());
		self
	}

	pub fn set_texture(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("texture_enable", value.is_some().into());
		self.set_uniform("texture", value.into());
		self
	}
}

impl Material for SkyboxMaterial 
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
		if let Some(x) = self.uniforms.get_mut(name) {
			*x = value;
		}
	}

	fn vs(&self) -> &str
	{
		SHADER_VERTEX
	}

	fn fs(&self) -> &str
	{
		SHADER_FRAGMENT		
	}

	fn cull_mode(&self) -> CullMode
	{
		return CullMode::None;
	}
}

impl Resource for SkyboxMaterial
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl AsRef<RenderState> for SkyboxMaterial
{
	fn as_ref(&self) -> &RenderState
	{
		&self.state
	}
}

impl AsMut<RenderState> for SkyboxMaterial
{
	fn as_mut(&mut self) -> &mut RenderState
	{
		&mut self.state
	}
}

impl From<SkyboxMaterial> for Rc<Material + 'static>
{
	fn from(material:SkyboxMaterial) -> Self
	{
		Rc::new(material)
	}
}

impl From<SkyboxMaterial> for Arc<Material + 'static>
{
	fn from(material:SkyboxMaterial) -> Self
	{
		Arc::new(material)
	}
}

impl From<SkyboxMaterial> for Rc<RefCell<Material + 'static>>
{
	fn from(material:SkyboxMaterial) -> Self
	{
		Rc::new(RefCell::new(material))
	}
}

impl From<SkyboxMaterial> for Arc<RefCell<Material + 'static>>
{
	fn from(material:SkyboxMaterial) -> Self
	{
		Arc::new(RefCell::new(material))
	}
}

pub struct SkyboxMaterialBuilder
{
	material:SkyboxMaterial
}

impl SkyboxMaterialBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			material:SkyboxMaterial::new()
		}
	}

	#[inline]
	pub fn build(self) -> SkyboxMaterial
	{
		self.material
	}

	pub fn set_color(mut self, value:float3) -> Self
	{
		self.material.set_color(value);
		self
	}

	pub fn set_texture(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_texture(value);
		self
	}
}