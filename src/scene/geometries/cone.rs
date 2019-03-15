use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConeGeometry
{
	uuid: uuid::Uuid,
	radius:f32,
	height:f32,
	segments:u32,
	theta_start:f32,
	theta_length:f32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl ConeGeometry 
{
	pub fn new(radius:f32, height:f32, segments:u32, theta_start:f32, theta_length:f32) -> Self 
	{
		let mut _vertices = Vec::with_capacity(segments as usize + 2);
		let mut _normals = Vec::with_capacity(segments as usize + 2);
		let mut _texcoords = Vec::with_capacity(segments as usize + 2);
		let mut _indices = Vec::with_capacity(segments as usize * 6);

		_vertices.push(float!(0.0, 0.0, 0.0));
		_vertices.push(float!(0.0, height as f32, 0.0));

		_normals.push(float!(0.0, 0.0, 0.0));
		_normals.push(float!(0.0, 1.0, 0.0));

		_texcoords.push(float!(0.0, 0.0));
		_texcoords.push(float!(1.0, 1.0));

		let segment = theta_length / segments as f32;

		for i in 0..segments + 2
		{
			let sin = f32::sin(theta_start + i as f32 * segment);
			let cos = f32::cos(theta_start + i as f32 * segment);

			let v = float3
			{
				x:radius * cos,
				y:0.0,
				z:-radius * sin,
			};

			_vertices.push(v);
			_normals.push(float!(cos, 0.0, sin));
			_texcoords.push(float!(v.x / radius + 1.0, (v.y / radius + 1.0) * 0.5));
		}

		for i in 2..segments + 2
		{
			let v1 = i;
			let v2 = 0;
			let v3 = i + 1;

			_indices.push(v1 as u16);
			_indices.push(v2 as u16);
			_indices.push(v3 as u16);
		}

		for i in 2..segments + 2
		{
			let v1 = i;
			let v2 = 1;
			let v3 = i + 1;

			_indices.push(v3 as u16);
			_indices.push(v2 as u16);
			_indices.push(v1 as u16);
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			radius:radius,
			height:height,
			segments:segments,
			theta_start:theta_start,
			theta_length:theta_length,
			vertices:_vertices,
			normals:_normals,
			texcoords:_texcoords,
			indices:_indices,
			weights:Vec::new()
		}
	}

	pub fn builder() -> ConeGeometryBuilder
	{
		ConeGeometryBuilder::new()
	}

	pub fn radius(&self) -> f32
	{
		self.radius
	}

	pub fn height(&self) -> f32
	{
		self.height
	}

	pub fn segments(&self) -> u32
	{
		self.segments
	}

	pub fn theta_start(&self) -> f32
	{
		self.theta_start
	}

	pub fn theta_length(&self) -> f32
	{
		self.theta_length
	}
}

impl Geometry for ConeGeometry
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

impl Resource for ConeGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<ConeGeometry> for Rc<Geometry + 'static>
{
	fn from(shape:ConeGeometry) -> Self
	{
		Rc::new(shape)
	}
}

impl From<ConeGeometry> for Arc<Geometry + 'static>
{
	fn from(shape:ConeGeometry) -> Self
	{
		Arc::new(shape)
	}
}

impl From<ConeGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(shape:ConeGeometry) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<ConeGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(shape:ConeGeometry) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct ConeGeometryBuilder
{
	radius:f32,
	height:f32,
	segments:u32,
	theta_start:f32,
	theta_length:f32,
}

impl ConeGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			radius:1.0,
			height:1.0,
			segments:32,
			theta_start:0.0,
			theta_length:f32::pi(),
		}
	}

	#[inline]
	pub fn build(self) -> ConeGeometry
	{
		ConeGeometry::new(self.radius, self.height, self.segments, self.theta_start, self.theta_length)
	}

	#[inline]
	pub fn set_radius(mut self, radius:f32) -> Self
	{
		self.radius = radius;
		self
	}

	#[inline]
	pub fn set_height(mut self, height:f32) -> Self
	{
		self.height = height;
		self
	}

	#[inline]
	pub fn set_segments(mut self, segments:u32) -> Self
	{
		self.segments = segments;
		self
	}

	#[inline]
	pub fn set_theta_start(mut self, theta_start:f32) -> Self
	{
		self.theta_start = theta_start;
		self
	}

	#[inline]
	pub fn set_theta_length(mut self, theta_length:f32) -> Self
	{
		self.theta_length = theta_length;
		self
	}
}