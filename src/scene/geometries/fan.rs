use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct FanGeometry
{
	uuid: uuid::Uuid,
	radius_top:f32,
	radius_bottom:f32,
	segments:u32,
	top_length:f32,
	bottom_length:f32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl FanGeometry 
{
	pub fn new(top_radius:f32, bottom_radius:f32, segments:u32, top_length:f32, bottom_length:f32) -> Self 
	{
		let mut _vertices = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _normals = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _texcoords = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _indices = Vec::with_capacity(segments as usize * 6);

		let top_start = -(f32::pi() + bottom_length) * 0.5;
		let top_segment = top_length / segments as f32;

		let bottom_start = -(f32::pi() + top_length) * 0.5;
		let bottom_segment = bottom_length / segments as f32;

		for i in 0..segments + 1
		{
			let sin = f32::sin(bottom_start + i as f32 * bottom_segment);
			let cos = f32::cos(bottom_start + i as f32 * bottom_segment);

			let v = float!(bottom_radius * cos, -bottom_radius * sin, 0.0);

			_vertices.push(v);
			_normals.push(float!(0.0,0.0,1.0));
			_texcoords.push(float!(i as f32 / segments as f32, 1.0));
		}

		for i in 0..segments + 1
		{
			let sin = f32::sin(top_start + i as f32 * top_segment);
			let cos = f32::cos(top_start + i as f32 * top_segment);

			let v = float!(top_radius * cos, -top_radius * sin, 0.0);

			_vertices.push(v);
			_normals.push(float!(0.0,0.0,1.0));
			_texcoords.push(float!(i as f32 / segments as f32, 0.0));
		}

		for i in 0..segments
		{
			let v1 = i;
			let v3 = i + 1 + segments;
			let v4 = i + 2 + segments;

			_indices.push(v1 as u16);
			_indices.push(v3 as u16);
			_indices.push(v4 as u16);
		}

		for i in 0..segments
		{
			let v1 = i;
			let v2 = i + 1;
			let v4 = i + 2 + segments;

			_indices.push(v1 as u16);
			_indices.push(v4 as u16);
			_indices.push(v2 as u16);
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			radius_top:top_radius,
			radius_bottom:bottom_radius,
			segments:segments,
			top_length:top_length,
			bottom_length:bottom_length,
			vertices:_vertices,
			normals:_normals,
			texcoords:_texcoords,
			indices:_indices,
			weights:Vec::new()
		}
	}

	pub fn builder() -> FanGeometryBuilder
	{
		FanGeometryBuilder::new()
	}

	pub fn top_radius(&self) -> f32
	{
		self.radius_top
	}

	pub fn top_length(&self) -> f32
	{
		self.top_length
	}

	pub fn bottom_radius(&self) -> f32
	{
		self.radius_top
	}

	pub fn bottom_length(&self) -> f32
	{
		self.bottom_length
	}

	pub fn segments(&self) -> u32
	{
		self.segments
	}
}

impl Geometry for FanGeometry
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

impl Resource for FanGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<FanGeometry> for Rc<Geometry + 'static>
{
	fn from(shape:FanGeometry) -> Self
	{
		Rc::new(shape)
	}
}

impl From<FanGeometry> for Arc<Geometry + 'static>
{
	fn from(shape:FanGeometry) -> Self
	{
		Arc::new(shape)
	}
}

impl From<FanGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(shape:FanGeometry) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<FanGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(shape:FanGeometry) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct FanGeometryBuilder
{
	top_radius:f32,
	bottom_radius:f32,
	segments:u32,
	top_length:f32,
	bottom_length:f32,
}

impl FanGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			top_radius:1.0,
			bottom_radius:1.0,
			segments:32,
			top_length:0.0,
			bottom_length:f32::pi(),
		}
	}

	#[inline]
	pub fn build(self) -> FanGeometry
	{
		FanGeometry::new(self.top_radius, self.bottom_radius, self.segments, self.top_length, self.bottom_length)
	}

	#[inline]
	pub fn set_top_radius(mut self, top_radius:f32) -> Self
	{
		self.top_radius = top_radius;
		self
	}

	#[inline]
	pub fn set_bottom_radius(mut self, bottom_radius:f32) -> Self
	{
		self.bottom_radius = bottom_radius;
		self
	}

	#[inline]
	pub fn set_segments(mut self, segments:u32) -> Self
	{
		self.segments = segments;
		self
	}

	#[inline]
	pub fn set_top_length(mut self, top_length:f32) -> Self
	{
		self.top_length = top_length;
		self
	}

	#[inline]
	pub fn set_bottom_length(mut self, bottom_length:f32) -> Self
	{
		self.bottom_length = bottom_length;
		self
	}
}