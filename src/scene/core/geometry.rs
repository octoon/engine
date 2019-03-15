use std::fmt::Debug;
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