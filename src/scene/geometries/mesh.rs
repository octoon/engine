use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct MeshGeometry
{
	uuid: uuid::Uuid,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl MeshGeometry 
{
	pub fn new(vertices:float3s, normals:float3s, texcoords:float2s, weights:Vec<VertexWeight>, indices:Vec<u16>) -> Self 
	{
		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			vertices:vertices,
			normals:normals,
			texcoords:texcoords,
			indices:indices,
			weights:weights
		}
	}

	pub fn builder() -> MeshGeometryBuilder
	{
		MeshGeometryBuilder::new()
	}
}

impl Geometry for MeshGeometry
{
	fn vertices(&self) -> &[float3]
	{
		&self.vertices[..]
	}

	fn normals(&self) -> &[float3]
	{
		&self.normals[..]
	}

	fn texcoords(&self) -> &[float2]
	{
		&self.texcoords[..]
	}

	fn indices(&self) -> &[u16]
	{
		&self.indices[..]
	}

	fn weights(&self) -> &[VertexWeight]
	{
		&self.weights[..]
	}
}

impl Resource for MeshGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<MeshGeometry> for Rc<Geometry + 'static>
{
	fn from(shape:MeshGeometry) -> Self
	{
		Rc::new(shape)
	}
}

impl From<MeshGeometry> for Arc<Geometry + 'static>
{
	fn from(shape:MeshGeometry) -> Self
	{
		Arc::new(shape)
	}
}

impl From<MeshGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(shape:MeshGeometry) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<MeshGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(shape:MeshGeometry) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct MeshGeometryBuilder
{
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl MeshGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			vertices:Vec::new(),
			normals:Vec::new(),
			texcoords:Vec::new(),
			indices:Vec::new(),
			weights:Vec::new(),
		}
	}

	#[inline]
	pub fn build(self) -> MeshGeometry
	{
		assert_eq!(self.vertices.len(), self.normals.len());
		assert_eq!(self.vertices.len(), self.texcoords.len());

		MeshGeometry::new(self.vertices, self.normals, self.texcoords, self.weights, self.indices)
	}

	#[inline]
	pub fn vertices_capacity(mut self, num_vertices:usize) -> Self
	{
		self.vertices = Vec::with_capacity(num_vertices);
		self
	}

	#[inline]
	pub fn normals_capacity(mut self, num_normals:usize) -> Self
	{
		self.normals = Vec::with_capacity(num_normals);
		self
	}

	#[inline]
	pub fn texcoords_capacity(mut self, num_texcoords:usize) -> Self
	{
		self.texcoords = Vec::with_capacity(num_texcoords);
		self
	}

	#[inline]
	pub fn weights_capacity(mut self, num_weights:usize) -> Self
	{
		self.weights = Vec::with_capacity(num_weights);
		self
	}

	#[inline]
	pub fn set_vertices(mut self, vertices:float3s) -> Self
	{
		self.vertices = vertices;
		self
	}

	#[inline]
	pub fn set_normals(mut self, normals:float3s) -> Self
	{
		self.normals = normals;
		self
	}

	#[inline]
	pub fn set_texcoords(mut self, texcoords:float2s) -> Self
	{
		self.texcoords = texcoords;
		self
	}

	#[inline]
	pub fn set_indices(mut self, indices:Vec<u16>) -> Self
	{
		self.indices = indices;
		self
	}

	#[inline]
	pub fn set_weights(mut self, weights:Vec<VertexWeight>) -> Self
	{
		self.weights = weights;
		self
	}

	#[inline]
	pub fn add_vertex(mut self, v:float3) -> Self
	{
		self.vertices.push(v);
		self
	}

	#[inline]
	pub fn add_normal(mut self, n:float3) -> Self
	{
		self.normals.push(n);
		self
	}

	#[inline]
	pub fn add_texcoord(mut self, uv:float2) -> Self
	{
		self.texcoords.push(uv);
		self
	}

	#[inline]
	pub fn add_indice(mut self, index:u16) -> Self
	{
		self.indices.push(index);
		self
	}
}