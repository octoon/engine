use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereGeometry
{
	uuid: uuid::Uuid,
	radius:f32,
	width_segments:u32,
	height_segments:u32,
	phi_start:f32,
	phi_length:f32,
	theta_start:f32,
	theta_length:f32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl SphereGeometry 
{
	pub fn new(radius:f32, width_segments:u32, height_segments:u32, phi_start:f32, phi_length:f32, theta_start:f32, theta_length:f32) -> Self 
	{
		let num_vertices = ((width_segments + 1) * (height_segments + 1)) as usize;
		let num_indices = (((width_segments - 1) * (height_segments - 1)) * 6 + ((width_segments - 1) + (height_segments - 1)) * 3)  as usize;

		let mut _vertices = Vec::with_capacity(num_vertices);
		let mut _normals = Vec::with_capacity(num_vertices);
		let mut _texcoords = Vec::with_capacity(num_vertices);
		let mut _indices = Vec::with_capacity(num_indices);
		let mut vertices = Vec::with_capacity(num_vertices);
		
		for y in 0.. height_segments + 1
		{
			for x in 0.. width_segments + 1
			{
				let u = (x as f32) / (width_segments as f32);
				let v = (y as f32) / (height_segments as f32);

				let vertex = float!(
					-radius * (theta_start + v * theta_length).sin() * (phi_start + u * phi_length).cos(),
					radius * (theta_start + v * theta_length).cos(),
					radius * (theta_start + v * theta_length).sin() * (phi_start + u * phi_length).sin()
				);

				let normal = vertex.normalize();

				_vertices.push(vertex);
				_normals.push(normal);
				_texcoords.push(float2::new(u, v));

				vertices.push((_vertices.len() - 1) as u16);
			}
		}

		for y in 0..height_segments
		{
			for x in 0..width_segments
			{
				let v1 = vertices[(y * (width_segments + 1) + x) as usize];
				let v2 = vertices[(y * (width_segments + 1) + x + 1) as usize];
				let v3 = vertices[((y + 1) * (width_segments + 1) + x) as usize];
				let v4 = vertices[((y + 1) * (width_segments + 1) + x + 1) as usize];

				if _vertices[v2 as usize].y.abs() == radius
				{
					_indices.push(v2);
					_indices.push(v3);
					_indices.push(v4);
				}
				else if _vertices[v3 as usize].y.abs() == radius
				{
					_indices.push(v2);
					_indices.push(v1);
					_indices.push(v3);
				}
				else
				{
					_indices.push(v2);
					_indices.push(v3);
					_indices.push(v4);

					_indices.push(v2);
					_indices.push(v1);
					_indices.push(v3);
				}
			}
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			radius:radius,
			width_segments:width_segments,
			height_segments:height_segments,
			phi_start:phi_start,
			phi_length:phi_length,
			theta_start:theta_start,
			theta_length:theta_length,
			vertices:_vertices,
			normals:_normals,
			texcoords:_texcoords,
			indices:_indices,
			weights:Vec::new()
		}
	}

	#[inline]
	pub fn builder() -> SphereGeometryBuilder
	{
		SphereGeometryBuilder::new()
	}

	pub fn radius(&self) -> f32
	{
		self.radius
	}

	pub fn width_segments(&self) -> u32
	{
		self.width_segments
	}

	pub fn height_segments(&self) -> u32
	{
		self.height_segments
	}

	pub fn phi_start(&self) -> f32
	{
		self.phi_start
	}

	pub fn phi_length(&self) -> f32
	{
		self.phi_length
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

impl Geometry for SphereGeometry
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

impl Resource for SphereGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<SphereGeometry> for Rc<Geometry + 'static>
{
	fn from(sphere:SphereGeometry) -> Self
	{
		Rc::new(sphere)
	}
}

impl From<SphereGeometry> for Arc<Geometry + 'static>
{
	fn from(sphere:SphereGeometry) -> Self
	{
		Arc::new(sphere)
	}
}

impl From<SphereGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(sphere:SphereGeometry) -> Self
	{
		Rc::new(RefCell::new(sphere))
	}
}

impl From<SphereGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(sphere:SphereGeometry) -> Self
	{
		Arc::new(RefCell::new(sphere))
	}
}

pub struct SphereGeometryBuilder
{
	radius:f32,
	width_segments:u32,
	height_segments:u32,
	phi_start:f32,
	phi_length:f32,
	theta_start:f32,
	theta_length:f32,
}

impl SphereGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			radius:1.0,
			width_segments:32,
			height_segments:32,
			phi_start:0.0,
			phi_length:f32::pi2(),
			theta_start:0.0,
			theta_length:f32::pi(),
		}
	}

	#[inline]
	pub fn build(self) -> SphereGeometry
	{
		SphereGeometry::new(
			self.radius, 
			self.width_segments, 
			self.height_segments, 
			self.phi_start, 
			self.phi_length, 
			self.theta_start, 
			self.theta_length
		)
	}

	#[inline]
	pub fn set_radius(mut self, radius:f32) -> Self
	{
		self.radius = radius;
		self
	}

	#[inline]
	pub fn set_width_segments(mut self, segments:u32) -> Self
	{
		self.width_segments = segments;
		self
	}

	#[inline]
	pub fn set_height_segments(mut self, segments:u32) -> Self
	{
		self.height_segments = segments;
		self
	}

	#[inline]
	pub fn set_phi_start(mut self, phi_start:f32) -> Self
	{
		self.phi_start = phi_start;
		self
	}

	#[inline]
	pub fn set_phi_length(mut self, phi_length:f32) -> Self
	{
		self.phi_length = phi_length;
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