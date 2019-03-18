use std::sync::Arc;
use crate::math::*;
use super::texture::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Variant
{
	Boolean(bool),
	Int1(i32_1),
	Int2(i32_2),
	Int3(i32_3),
	Int4(i32_4),
	Uint1(u32_1),
	Uint2(u32_2),
	Uint3(u32_3),
	Uint4(u32_4),
	Float1(float1),
	Float2(float2),
	Float3(float3),
	Float4(float4),
	Float2x2(float2x2),
	Float3x3(float3x3),
	Float4x4(float4x4),
	Double2x2(double2x2),
	Double3x3(double3x3),
	Double4x4(double4x4),
	Float2x2s(Vec<float2x2>),
	Float3x3s(Vec<float3x3>),
	Float4x4s(Vec<float4x4>),
	Double2x2s(Vec<double2x2>),
	Double3x3s(Vec<double3x3>),
	Double4x4s(Vec<double4x4>),
	Texture(Option<Arc<Texture>>)
}

impl From<bool> for Variant
{
	#[inline(always)]
	fn from(v:bool) -> Self
	{
		Variant::Boolean(v)
	}
}

impl From<i32_1> for Variant
{
	#[inline(always)]
	fn from(v:i32_1) -> Self
	{
		Variant::Int1(v)
	}
}

impl From<i32_2> for Variant
{
	#[inline(always)]
	fn from(v:i32_2) -> Self
	{
		Variant::Int2(v)
	}
}

impl From<i32_3> for Variant
{
	#[inline(always)]
	fn from(v:i32_3) -> Self
	{
		Variant::Int3(v)
	}
}

impl From<i32_4> for Variant
{
	#[inline(always)]
	fn from(v:i32_4) -> Self
	{
		Variant::Int4(v)
	}
}

impl From<u32_1> for Variant
{
	#[inline(always)]
	fn from(v:u32_1) -> Self
	{
		Variant::Uint1(v)
	}
}

impl From<u32_2> for Variant
{
	#[inline(always)]
	fn from(v:u32_2) -> Self
	{
		Variant::Uint2(v)
	}
}

impl From<u32_3> for Variant
{
	#[inline(always)]
	fn from(v:u32_3) -> Self
	{
		Variant::Uint3(v)
	}
}

impl From<u32_4> for Variant
{
	#[inline(always)]
	fn from(v:u32_4) -> Self
	{
		Variant::Uint4(v)
	}
}

impl From<float1> for Variant
{
	#[inline(always)]
	fn from(v:float1) -> Self
	{
		Variant::Float1(v)
	}
}

impl From<float2> for Variant
{
	#[inline(always)]
	fn from(v:float2) -> Self
	{
		Variant::Float2(v)
	}
}

impl From<float3> for Variant
{
	#[inline(always)]
	fn from(v:float3) -> Self
	{
		Variant::Float3(v)
	}
}

impl From<float4> for Variant
{
	#[inline(always)]
	fn from(v:float4) -> Self
	{
		Variant::Float4(v)
	}
}

impl From<float2x2> for Variant
{
	#[inline(always)]
	fn from(v:float2x2) -> Self
	{
		Variant::Float2x2(v)
	}
}

impl From<float3x3> for Variant
{
	#[inline(always)]
	fn from(v:float3x3) -> Self
	{
		Variant::Float3x3(v)
	}
}

impl From<float4x4> for Variant
{
	#[inline(always)]
	fn from(v:float4x4) -> Self
	{
		Variant::Float4x4(v)
	}
}

impl From<double2x2> for Variant
{
	#[inline(always)]
	fn from(v:double2x2) -> Self
	{
		Variant::Double2x2(v)
	}
}

impl From<double3x3> for Variant
{
	#[inline(always)]
	fn from(v:double3x3) -> Self
	{
		Variant::Double3x3(v)
	}
}

impl From<double4x4> for Variant
{
	#[inline(always)]
	fn from(v:double4x4) -> Self
	{
		Variant::Double4x4(v)
	}
}

impl From<Texture> for Variant
{
	#[inline(always)]
	fn from(texture:Texture) -> Self
	{
		Variant::Texture(Some(Arc::new(texture)))
	}
}

impl From<Option<Arc<Texture>>> for Variant
{
	#[inline(always)]
	fn from(texture:Option<Arc<Texture>>) -> Self
	{
		Variant::Texture(texture)
	}
}