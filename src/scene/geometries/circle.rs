use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct CircleGeometry
{
	uuid: uuid::Uuid,
	radius:f32,
	segments:u32,
	theta_start:f32,
	theta_length:f32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl CircleGeometry 
{
	pub fn new(radius:f32, segments:u32, theta_start:f32, theta_length:f32) -> Self 
	{
		let mut _vertices = Vec::with_capacity(segments as usize);
		let mut _normals = Vec::with_capacity(segments as usize);
		let mut _texcoords = Vec::with_capacity(segments as usize);
		let mut _indices = Vec::with_capacity((segments - 1) as usize * 3);

		let normal = float!(0.0,0.0,1.0);

		for i in 0..segments
		{
			let segment = theta_start + i as f32 / segments as f32 * theta_length;

			let v = float3
			{
				x:radius * f32::cos(segment),
				y:radius * f32::sin(segment),
				z:0.0,
			};

			_vertices.push(v);
			_normals.push(normal);
			_texcoords.push(float!(v.x / radius + 1.0, (v.y / radius + 1.0) * 0.5));
		}

		for i in 1..segments
		{
			let v1 = i;
			let v2 = i + 1;
			let v3 = 0;

			_indices.push(v1 as u16);
			_indices.push(v2 as u16);
			_indices.push(v3 as u16);
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			radius:radius,
			segments:segments,
			theta_start:theta_start,
			theta_length:theta_length,
			vertices:_vertices,
			normals:_normals,
			texcoords:_texcoords,
			indices:_indices,
			weights:Vec::new(),
		}
	}

	pub fn builder() -> CircleGeometryBuilder
	{
		CircleGeometryBuilder::new()
	}

	pub fn radius(&self) -> f32
	{
		self.radius
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

impl Geometry for CircleGeometry
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

impl Resource for CircleGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<CircleGeometry> for Rc<Geometry + 'static>
{
	fn from(shape:CircleGeometry) -> Self
	{
		Rc::new(shape)
	}
}

impl From<CircleGeometry> for Arc<Geometry + 'static>
{
	fn from(shape:CircleGeometry) -> Self
	{
		Arc::new(shape)
	}
}

impl From<CircleGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(shape:CircleGeometry) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<CircleGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(shape:CircleGeometry) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct CircleGeometryBuilder
{
	radius:f32,
	segments:u32,
	theta_start:f32,
	theta_length:f32,
}

impl CircleGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			radius:1.0,
			segments:32,
			theta_start:0.0,
			theta_length:f32::pi(),
		}
	}

	#[inline]
	pub fn build(self) -> CircleGeometry
	{
		CircleGeometry::new(self.radius, self.segments, self.theta_start, self.theta_length)
	}

	#[inline]
	pub fn set_radius(mut self, radius:f32) -> Self
	{
		self.radius = radius;
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