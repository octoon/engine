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
uniform float3 albedo;
uniform float3 specular;
uniform float3 emissive;
uniform float3 emissiveIntensity;
uniform float smoothness;
uniform float metalness;

uniform sampler2D albedo_map;
uniform sampler2D normal_map;
uniform sampler2D emissive_map;

uniform bool albedo_map_enable;
uniform bool normal_map_enable;
uniform bool emissive_map_enable;
uniform bool smoothness_map_enable;
uniform bool metalness_map_enable;
uniform bool occlusion_map_enable;

Gbuffer main_fs(Args args)
{
	Gbuffer buffer;
	buffer.albedo = albedo;
	buffer.specular = specular;
	buffer.emissive = emissive;
	buffer.smoothness = smoothness;
	buffer.metalness = metalness;
	buffer.normal = normalize(args.normal);

	if (emissive_map_enable)
	{ 
		buffer.emissive *= pow(texture2D(emissive_map, args.coord.xy).xyz, float3(2.2));
	}

	if (albedo_map_enable)
	{
		buffer.albedo *= pow(texture2D(albedo_map, args.coord.xy).xyz, float3(2.2));
	}

	return buffer;
}"#;


#[derive(Debug)]
pub struct StandardMaterial
{
	pub uuid: uuid::Uuid,
	pub state:RenderState,
	pub attribs:Vec<VertexAttrib>,
	pub uniforms:HashMap<String, Variant>
}

impl StandardMaterial
{
	pub fn new() -> Self
	{
		let mut params = HashMap::new();
		params.insert("albedo".to_string(), float3::one().into());
		params.insert("specular".to_string(), float3::new(0.5,0.5,0.5).into());
		params.insert("emissive".to_string(), float3::zero().into());
		params.insert("emissive_intensity".to_string(), float1::zero().into());
		params.insert("smoothness".to_string(), float1::zero().into());
		params.insert("metalness".to_string(), float1::zero().into());
		params.insert("occlusion".to_string(), float1::one().into());
		params.insert("albedo_map".to_string(), None.into());
		params.insert("normal_map".to_string(), None.into());
		params.insert("emissive_map".to_string(), None.into());
		params.insert("smoothness_map".to_string(), None.into());
		params.insert("metalness_map".to_string(), None.into());
		params.insert("occlusion_map".to_string(), None.into());
		params.insert("albedo_map_enable".to_string(), float1::zero().into());
		params.insert("normal_map_enable".to_string(), float1::zero().into());
		params.insert("emissive_map_enable".to_string(), float1::zero().into());
		params.insert("smoothness_map_enable".to_string(), float1::zero().into());
		params.insert("metalness_map_enable".to_string(), float1::zero().into());
		params.insert("occlusion_map_enable".to_string(), float1::zero().into());

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
	pub fn builder() -> StandardMaterialBuilder
	{
		StandardMaterialBuilder::new()
	}

	pub fn set_albedo(&mut self, value:float3) -> &mut Self
	{
		self.set_uniform("albedo", value.into());
		self
	}

	pub fn set_specular(&mut self, value:float3) -> &mut Self
	{
		self.set_uniform("specular", value.into());
		self
	}

	pub fn set_emissive(&mut self, value:float3) -> &mut Self
	{
		self.set_uniform("emissive", value.into());
		self
	}

	pub fn set_emissive_intensity(&mut self, value:float1) -> &mut Self
	{
		self.set_uniform("emissive_intensity", value.into());
		self
	}

	pub fn set_smoothness(&mut self, value:float1) -> &mut Self
	{
		self.set_uniform("smoothness", value.into());
		self
	}

	pub fn set_metalness(&mut self, value:float1) -> &mut Self
	{
		self.set_uniform("metalness", value.into());
		self
	}

	pub fn set_occlusion(&mut self, value:float1) -> &mut Self
	{
		self.set_uniform("occlusion", value.into());
		self
	}

	pub fn set_albedo_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("albedo_map_enable", value.is_some().into());
		self.set_uniform("albedo_map", value.into());
		self
	}

	pub fn set_normal_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("normal_map_enable", value.is_some().into());
		self.set_uniform("normal_map", value.into());
		self
	}

	pub fn set_emissive_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("emissive_map_enable", value.is_some().into());
		self.set_uniform("emissive_map", value.into());
		self
	}

	pub fn set_smoothness_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("smoothness_map_enable", value.is_some().into());
		self.set_uniform("smoothness_map", value.into());
		self
	}

	pub fn set_metalness_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("metalness_map_enable", value.is_some().into());
		self.set_uniform("metalness_map", value.into());
		self
	}

	pub fn set_occlusion_map(&mut self, value:Option<Arc<Texture>>) -> &mut Self
	{
		self.set_uniform("occlusion_map_enable", value.is_some().into());
		self.set_uniform("occlusion_map", value.into());
		self
	}
}

impl Material for StandardMaterial 
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
}

impl Resource for StandardMaterial
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl AsRef<RenderState> for StandardMaterial
{
	fn as_ref(&self) -> &RenderState
	{
		&self.state
	}
}

impl AsMut<RenderState> for StandardMaterial
{
	fn as_mut(&mut self) -> &mut RenderState
	{
		&mut self.state
	}
}

impl From<StandardMaterial> for Rc<Material + 'static>
{
	fn from(material:StandardMaterial) -> Self
	{
		Rc::new(material)
	}
}

impl From<StandardMaterial> for Arc<Material + 'static>
{
	fn from(material:StandardMaterial) -> Self
	{
		Arc::new(material)
	}
}

impl From<StandardMaterial> for Rc<RefCell<Material + 'static>>
{
	fn from(material:StandardMaterial) -> Self
	{
		Rc::new(RefCell::new(material))
	}
}

impl From<StandardMaterial> for Arc<RefCell<Material + 'static>>
{
	fn from(material:StandardMaterial) -> Self
	{
		Arc::new(RefCell::new(material))
	}
}

pub struct StandardMaterialBuilder
{
	material:StandardMaterial
}

impl StandardMaterialBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			material:StandardMaterial::new()
		}
	}

	#[inline]
	pub fn build(self) -> StandardMaterial
	{
		self.material
	}

	pub fn set_albedo(mut self, value:float3) -> Self
	{
		self.material.set_albedo(value);
		self
	}

	pub fn set_specular(mut self, value:float3) -> Self
	{
		self.material.set_specular(value);
		self
	}

	pub fn set_emissive(mut self, value:float3) -> Self
	{
		self.material.set_emissive(value);
		self
	}

	pub fn set_emissive_intensity(mut self, value:float1) -> Self
	{
		self.material.set_emissive_intensity(value);
		self
	}

	pub fn set_smoothness(mut self, value:float1) -> Self
	{
		self.material.set_smoothness(value);
		self
	}

	pub fn set_metalness(mut self, value:float1) -> Self
	{
		self.material.set_metalness(value);
		self
	}

	pub fn set_occlusion(mut self, value:float1) -> Self
	{
		self.material.set_occlusion(value);
		self
	}

	pub fn set_albedo_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_albedo_map(value);
		self
	}

	pub fn set_normal_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_normal_map(value);
		self
	}

	pub fn set_emissive_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_emissive_map(value);
		self
	}

	pub fn set_smoothness_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_smoothness_map(value);
		self
	}

	pub fn set_metalness_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_metalness_map(value);
		self
	}

	pub fn set_occlusion_map(mut self, value:Option<Arc<Texture>>) -> Self
	{
		self.material.set_occlusion_map(value);
		self
	}
}