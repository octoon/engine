use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderGeometry
{
	uuid: uuid::Uuid,
	radius_top:f32,
	radius_bottom:f32,
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

impl CylinderGeometry 
{
	pub fn new(radius_top:f32, radius_bottom:f32, height:f32, segments:u32, theta_start:f32, theta_length:f32) -> Self 
	{
		let mut _vertices = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _normals = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _texcoords = Vec::with_capacity((segments + 1) as usize * 2);
		let mut _indices = Vec::with_capacity(segments as usize * 6);

		let segment = theta_length / segments as f32;

		for i in 0..segments + 1
		{
			let sin = f32::sin(theta_start + i as f32 * segment);
			let cos = f32::cos(theta_start + i as f32 * segment);

			let v = float!(radius_bottom * cos, 0.0, -radius_bottom * sin);

			_vertices.push(v);
			_normals.push(float!(cos, 0.0, -sin));
			_texcoords.push(float!(i as f32 / segments as f32, 1.0));
		}

		for i in 0..segments + 1
		{
			let sin = f32::sin(theta_start + i as f32 * segment);
			let cos = f32::cos(theta_start + i as f32 * segment);

			let v = float!(radius_top * cos, height, -radius_top * sin);

			_vertices.push(v);
			_normals.push(float!(cos, 0.0, -sin));
			_texcoords.push(float!(i as f32 / segments as f32, 0.0));
		}

		for i in 0..segments
		{
			let v1 = i;
			let v2 = i + 1;
			let v3 = i + 1 + segments;
			let v4 = i + 2 + segments;

			_indices.push(v1 as u16);
			_indices.push(v4 as u16);
			_indices.push(v3 as u16);

			_indices.push(v1 as u16);
			_indices.push(v2 as u16);
			_indices.push(v4 as u16);
		}

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			radius_top:radius_top,
			radius_bottom:radius_bottom,
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

	pub fn builder() -> CylinderGeometryBuilder
	{
		CylinderGeometryBuilder::new()
	}

	pub fn top_radius(&self) -> f32
	{
		self.radius_top
	}

	pub fn top_diameter(&self) -> f32
	{
		self.radius_top * 2.0
	}

	pub fn bottom_radius(&self) -> f32
	{
		self.radius_bottom
	}

	pub fn bottom_diameter(&self) -> f32
	{
		self.radius_bottom * 2.0
	}

	pub fn height(&self) -> f32
	{
		self.height
	}

	pub fn height_in_cone(&self) -> f32
	{
		let r1 = self.radius_top * 2.0;
		let r2 = self.radius_bottom * 2.0;
		let h = if r2 == 0.0 { self.height } else { r2 * self.height / (r1 - r2) };
		return h;
	}

	pub fn top_radius_in_fan(&self) -> f32
	{
		let r1 = f32::max(self.radius_top * 2.0, self.radius_bottom * 2.0);
		let h = self.height_in_cone() + self.height;
		let r = r1 * 0.5;
		return (h * h + r * r).sqrt();
	}

	pub fn top_length_in_fan(&self) -> f32
	{
		self.top_diameter() / self.top_radius_in_fan() * f32::pi()
	}

	pub fn bottom_radius_in_fan(&self) -> f32
	{
		let r2 = f32::min(self.radius_top * 2.0, self.radius_bottom * 2.0);
		let h = self.height_in_cone();
		let r = r2 * 0.5;
		return (h * h + r * r).sqrt();
	}

	pub fn bottom_length_in_fan(&self) -> f32
	{
		self.bottom_diameter() / self.bottom_radius_in_fan() * f32::pi()
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

impl Geometry for CylinderGeometry
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

impl Resource for CylinderGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<CylinderGeometry> for Rc<Geometry + 'static>
{
	fn from(shape:CylinderGeometry) -> Self
	{
		Rc::new(shape)
	}
}

impl From<CylinderGeometry> for Arc<Geometry + 'static>
{
	fn from(shape:CylinderGeometry) -> Self
	{
		Arc::new(shape)
	}
}

impl From<CylinderGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(shape:CylinderGeometry) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<CylinderGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(shape:CylinderGeometry) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct CylinderGeometryBuilder
{
	top_radius:f32,
	bottom_radius:f32,
	height:f32,
	segments:u32,
	theta_start:f32,
	theta_length:f32,
}

impl CylinderGeometryBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			top_radius:1.0,
			bottom_radius:1.0,
			height:1.0,
			segments:32,
			theta_start:0.0,
			theta_length:f32::pi(),
		}
	}

	#[inline]
	pub fn build(self) -> CylinderGeometry
	{
		CylinderGeometry::new(self.top_radius, self.bottom_radius, self.height, self.segments, self.theta_start, self.theta_length)
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