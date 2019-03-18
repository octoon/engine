use std::fmt::Debug;
use std::collections::hash_map::HashMap;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use super::{Resource, Format, Variant};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearFlags
{
	Color(f32,f32,f32,f32),
	Depth(f32),
	Stencil(u8)
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendFactor
{
	Zero,
	One,
	DstCol,
	SrcColor,
	SrcAlpha,
	DstAlpha,
	OneMinusSrcCol,
	OneMinusDstCol,
	OneMinusSrcAlpha,
	OneMinusDstAlpha,
	ConstantColor,
	ConstantAlpha,
	OneMinusConstantColor,
	OneMinusConstantAlpha,
	SrcAlphaSaturate,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlendOp
{
	Add,
	Subtract,
	RevSubtract
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonFunc
{
	Never,
	Less,
	Equal,
	Lequal,
	Greater,
	Notequal,
	Gequal,
	Always
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum CullMode
{
	None,
	Front,
	Back,
	FrontBack,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrontFace
{
	CW,
	CCW,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolygonMode
{
	Point,
	Wireframe,
	Solid,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct VertexAttrib
{
	pub index:u8,
	pub count:u8,
	pub size:u8,
	pub stride:u8,
	pub offset:u16,
	pub format:Format,
}

impl VertexAttrib
{
	pub fn new(index:u8, format:Format, stride:u8, offset:u16) -> Self
	{
		Self
		{
			index:index,
			count:format.count() as u8,
			size:format.type_size() as u8,
			stride:stride,
			offset:offset,
			format:format,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenderState
{
	pub viewport:Option<(f32,f32,f32,f32)>,
	pub clear_depth:Option<f32>,
	pub clear_stencil:Option<u8>,
	pub clear_color:Option<(f32,f32,f32,f32)>,
	pub blend_enable:bool,
	pub blend_op:BlendOp,
	pub blend_src:BlendFactor,
	pub blend_dest:BlendFactor,
	pub blend_alpha_op:BlendOp,
	pub blend_alpha_src:BlendFactor,
	pub blend_alpha_dest:BlendFactor,
	pub color_write_mask:u32,
	pub polygon_mode:PolygonMode,
	pub depth_enable:bool,
	pub depth_write_enable:bool,
	pub depth_func:ComparisonFunc,
	pub cull_mode:CullMode,
	pub line_width:f32,
	pub front_face:FrontFace,
}

impl RenderState
{
	pub fn new() -> Self
	{
		Self
		{
			viewport:None,
			clear_depth:None,
			clear_stencil:None,
			clear_color:None,
			blend_enable:false,
			blend_op:BlendOp::Add,
			blend_src:BlendFactor::SrcAlpha,
			blend_dest:BlendFactor::OneMinusConstantAlpha,
			blend_alpha_op:BlendOp::Add,
			blend_alpha_src:BlendFactor::SrcAlpha,
			blend_alpha_dest:BlendFactor::OneMinusConstantAlpha,
			color_write_mask:0xFFFFFFFF,
			polygon_mode:PolygonMode::Solid,
			depth_enable:true,
			depth_write_enable:true,
			depth_func:ComparisonFunc::Lequal,
			cull_mode:CullMode::Back,
			line_width:1.0,
			front_face:FrontFace::CW,
		}
	}
}

pub trait Material : 
	  Debug 
	+ AsRef<RenderState> 
	+ AsMut<RenderState> 
	+ Resource
{
	fn input_layout(&self) -> &[VertexAttrib];
	fn uniforms(&self) -> &HashMap<String, Variant>;

	fn set_uniform(&mut self, name:&str, value:Variant);

	fn num_uniform(&self) -> usize
	{
		self.uniforms().len()
	}

	fn num_texture(&self) -> u32
	{
		let mut unit = 0;

		for (_key, value) in self.uniforms()
		{
			match value
			{
				Variant::Texture(_) => 
				{
					unit += 1;
				},
				_ => {},
			}
		}

		return unit;
	}

	fn vs(&self) -> &str { "" }
	fn fs(&self) -> &str { "" }

	fn viewport(&self) -> Option<&(f32,f32,f32,f32)> { self.as_ref().viewport.as_ref() }

	fn clear_depth(&self) -> &Option<f32> { &self.as_ref().clear_depth }
	fn clear_stencil(&self) -> &Option<u8> { &self.as_ref().clear_stencil }
	fn clear_color(&self) -> &Option<(f32,f32,f32,f32)> { &self.as_ref().clear_color }

	fn blend_enable(&self) -> bool { self.as_ref().blend_enable }
	fn blend_op(&self) -> BlendOp { self.as_ref().blend_op }
	fn blend_src(&self) -> BlendFactor { self.as_ref().blend_src }
	fn blend_dest(&self) -> BlendFactor { self.as_ref().blend_dest }
	fn blend_alpha_op(&self) -> BlendOp { self.as_ref().blend_alpha_op }
	fn blend_alpha_src(&self) -> BlendFactor { self.as_ref().blend_alpha_src }
	fn blend_alpha_dest(&self) -> BlendFactor { self.as_ref().blend_alpha_dest }

	fn color_write_mask(&self) -> u32 { self.as_ref().color_write_mask }

	fn polygon_mode(&self) -> PolygonMode { self.as_ref().polygon_mode }

	fn depth_enable(&self) -> bool { self.as_ref().depth_enable }
	fn depth_write_enable(&self) -> bool { self.as_ref().depth_write_enable }
	fn depth_func(&self) -> ComparisonFunc { self.as_ref().depth_func }

	fn cull_mode(&self) -> CullMode { self.as_ref().cull_mode }

	fn line_width(&self) -> f32 { self.as_ref().line_width }

	fn front_face(&self) -> FrontFace { self.as_ref().front_face }
}

impl Serialize for Material
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
	{
		let mut s = serializer.serialize_struct("material", 4)?;
		s.serialize_field("attrib", &self.input_layout())?;
		s.serialize_field("parameters", &self.uniforms())?;
		s.serialize_field("state", &self.as_ref())?;
		s.end()
	}
}