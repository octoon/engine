use std::fmt::Debug;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use base64;
use crate::math::{float2, float3};
use crate::models::{VertexWeight};
use super::{Resource};

pub trait Geometry : Debug + Resource
{
	fn vertices(&self) -> &[float3];
	fn normals(&self) -> &[float3];
	fn texcoords(&self) -> &[float2];
	fn indices(&self) -> &[u16];
	fn weights(&self) -> &[VertexWeight];

	#[inline(always)]
	fn num_vertices(&self) -> usize { self.vertices().len() }

	#[inline(always)]
	fn num_normals(&self) -> usize { self.normals().len() }

	#[inline(always)]
	fn num_texcoords(&self) -> usize { self.texcoords().len() }

	#[inline(always)]
	fn num_indices(&self) -> usize { self.indices().len() }

	#[inline(always)]
	fn num_weights(&self) -> usize { self.weights().len() }
}

impl Serialize for Geometry
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer
	{
		let mut s = serializer.serialize_struct("mesh", 4)?;
		s.serialize_field("uuid", &self.uuid())?;

		unsafe
		{
			let v = std::slice::from_raw_parts(self.vertices().as_ptr() as *const u8, self.vertices().len() * std::mem::size_of::<float3>());
			let n = std::slice::from_raw_parts(self.normals().as_ptr() as *const u8, self.normals().len() * std::mem::size_of::<float3>());
			let uv = std::slice::from_raw_parts(self.texcoords().as_ptr() as *const u8, self.texcoords().len() * std::mem::size_of::<float2>());
			let i = std::slice::from_raw_parts(self.indices().as_ptr() as *const u8, self.indices().len() * std::mem::size_of::<u16>());

			s.serialize_field("v", &base64::encode(v))?;
			s.serialize_field("n", &base64::encode(n))?;
			s.serialize_field("uv", &base64::encode(uv))?;
			s.serialize_field("i", &base64::encode(i))?;
		}

		s.end()
	}
}