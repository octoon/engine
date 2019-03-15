use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneGeometry
{
	uuid: uuid::Uuid,
	width:f32,
	height:f32,
	width_segments:u32,
	height_segments:u32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl PlaneGeometry
{
	pub fn new(width:f32, height:f32, width_segments:u32, height_segments:u32) -> Self 
	{
		let capacity = (width_segments * height_segments) as usize;
		let capacity_inc = ((width_segments + 1) * (height_segments + 1)) as usize;

		let mut _vertices = Vec::with_capacity(capacity_inc);
		let mut _normals = Vec::with_capacity(capacity_inc);
		let mut _texcoords = Vec::with_capacity(capacity * 4);
		let mut _indices = Vec::with_capacity(capacity * 6);

		let grid = (width_segments, height_segments);
		let grid1 = (grid.0 + 1, grid.1 + 1);

		let width_half = width * 0.5;
		let height_half = height * 0.5;
		let segment_width = width / grid.0 as f32;
		let segment_height = height / grid.1 as f32;

		let normal = float!(0.0,1.0,0.0);

		for iz in 0..grid1.1
		{
			for ix in 0..grid1.0
			{
				let x = ix as f32 * segment_width - width_half;
				let z = iz as f32 * segment_height - height_half;

				_vertices.push(float!(x, 0.0, z));
				_normals.push(normal);
			}
		}

		for iy in 0..grid.1
		{
			for ix in 0..grid.0
			{
				let fx = ix as f32;
				let fy = iy as f32;

				let a = ix + grid1.0 * iy;
				let b = ix + grid1.0 * (iy + 1);
				let c = ix + grid1.0 * (iy + 1) + 1;
				let d = ix + grid1.0 * iy + 1;

				_texcoords.push(float!((fx      ) / grid.0 as f32, (fy + 1.0) / grid.1 as f32));
				_texcoords.push(float!((fx + 1.0) / grid.0 as f32, (fy + 1.0) / grid.1 as f32));
				_texcoords.push(float!((fx      ) / grid.0 as f32, (fy      ) / grid.1 as f32));
				_texcoords.push(float!((fx + 1.0) / grid.0 as f32, (fy      ) / grid.1 as f32));

				_indices.push(a as u16);
				_indices.push(b as u16);
				_indices.push(c as u16);

				_indices.push(c as u16);
				_indices.push(d as u16);
				_indices.push(a as u16);
			}
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			width:width,
			height:height,
			width_segments:width_segments,
			height_segments:height_segments,
			vertices:_vertices,
			normals:_normals,
			texcoords:_texcoords,
			indices:_indices,
			weights:Vec::new()
		}
	}

	pub fn builder() -> PlaneGeometryBuilder
	{
		PlaneGeometryBuilder::new()
	}

	pub fn width(&self) -> f32
	{
		return self.width;
	}

	pub fn height(&self) -> f32
	{
		return self.height;
	}

	pub fn width_segments(&self) -> u32
	{
		return self.width_segments;
	}

	pub fn height_segments(&self) -> u32
	{
		return self.height_segments;
	}
}

impl Geometry for PlaneGeometry
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

impl Resource for PlaneGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<PlaneGeometry> for Rc<Geometry + 'static>
{
	fn from(geoemtry:PlaneGeometry) -> Self
	{
		Rc::new(geoemtry)
	}
}

impl From<PlaneGeometry> for Arc<Geometry + 'static>
{
	fn from(geoemtry:PlaneGeometry) -> Self
	{
		Arc::new(geoemtry)
	}
}

impl From<PlaneGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(geoemtry:PlaneGeometry) -> Self
	{
		Rc::new(RefCell::new(geoemtry))
	}
}

impl From<PlaneGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(geoemtry:PlaneGeometry) -> Self
	{
		Arc::new(RefCell::new(geoemtry))
	}
}

pub struct PlaneGeometryBuilder
{
	width:f32,
	height:f32,
	width_segments:u32,
	height_segments:u32,
}

impl PlaneGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			width:1.0,
			height:1.0,
			width_segments:1,
			height_segments:1,
		}
	}

	#[inline]
	pub fn build(self) -> PlaneGeometry
	{
		PlaneGeometry::new(self.width, self.height, self.width_segments, self.height_segments)
	}

	#[inline]
	pub fn set_width(mut self, width:f32) -> Self
	{
		self.width = width;
		self
	}

	#[inline]
	pub fn set_height(mut self, height:f32) -> Self
	{
		self.height = height;
		self
	}

	#[inline]
	pub fn set_width_segments(mut self, width_segments:u32) -> Self
	{
		self.width_segments = width_segments;
		self
	}

	#[inline]
	pub fn set_height_segments(mut self, height_segments:u32) -> Self
	{
		self.height_segments = height_segments;
		self
	}
}