use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

use crate::math::*;
use crate::models::VertexWeight;

use super::super::core::{Resource, Geometry};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Serialize, Deserialize)]
pub struct CubeGeometry
{
	uuid: uuid::Uuid,
	width:f32,
	height:f32,
	depth:f32,
	width_segments:u32,
	height_segments:u32,
	depth_segments:u32,
	vertices:float3s,
	normals:float3s,
	texcoords:float2s,
	indices:Vec<u16>,
	weights:Vec<VertexWeight>
}

impl CubeGeometry
{
	pub fn new(width:f32, height:f32, depth:f32, width_segments:u32, height_segments:u32, depth_segments:u32) -> Self 
	{
		let x = width * 0.5;
		let y = height * 0.5;
		let z = depth * 0.5;

		let vertices:Vec<float3> = vec![
			float!(-x,-y,-z), float!( x,-y,-z), float!( x, y,-z), float!(-x, y,-z), 
			float!(-x,-y, z), float!( x,-y, z), float!( x, y, z), float!(-x, y, z),
			float!(-x,-y,-z), float!(-x, y,-z), float!(-x, y, z), float!(-x,-y, z),
			float!( x,-y,-z), float!( x, y,-z), float!( x, y, z), float!( x,-y, z), 
			float!(-x,-y,-z), float!(-x,-y, z), float!( x,-y, z), float!( x,-y,-z),
			float!(-x, y,-z), float!(-x, y, z), float!( x, y, z), float!( x, y,-z), 
		];

		let normals:Vec<float3> = vec![
			float!(5.,3.,7.), float!(5.,3.,7.), float!(5.,3.,7.), float!(5.,3.,7.),
			float!(1.,1.,3.), float!(1.,1.,3.), float!(1.,1.,3.), float!(1.,1.,3.),
			float!(0.,0.,1.), float!(0.,0.,1.), float!(0.,0.,1.), float!(0.,0.,1.),
			float!(1.,0.,0.), float!(1.,0.,0.), float!(1.,0.,0.), float!(1.,0.,0.),
			float!(1.,1.,0.), float!(1.,1.,0.), float!(1.,1.,0.), float!(1.,1.,0.),
			float!(0.,1.,0.), float!(0.,1.,0.), float!(0.,1.,0.), float!(0.,1.,0.)
		];

		let texcoords:Vec<float2> = vec![
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
			float!(0.0, 0.0), float!(1.0, 0.0), float!(1.0, 1.0), float!(0.0, 1.0),
		];

		let indices:Vec<u16> = vec![
			0,1,2, 0,2,3, 4,5,6, 4,6,7,
			8,9,10, 8,10,11, 12,13,14, 12,14,15,
			16,17,18, 16,18,19, 20,21,22, 20,22,23 
		];

		Self
		{
			uuid:uuid::Uuid::new_v4_osrng(),
			width:width,
			height:height,
			depth:depth,
			width_segments:width_segments,
			height_segments:height_segments,
			depth_segments:depth_segments,
			vertices:vertices,
			normals:normals,
			texcoords:texcoords,
			indices:indices,
			weights:Vec::new()
		}
	}

	pub fn width(&self) -> f32
	{
		return self.width;
	}

	pub fn height(&self) -> f32
	{
		return self.height;
	}

	pub fn depth(&self) -> f32
	{
		return self.depth;
	}

	pub fn width_segments(&self) -> u32
	{
		return self.width_segments;
	}

	pub fn height_segments(&self) -> u32
	{
		return self.height_segments;
	}

	pub fn depth_segments(&self) -> u32
	{
		return self.depth_segments;
	}
}

impl Geometry for CubeGeometry
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

impl Resource for CubeGeometry
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}
}

impl From<CubeGeometry> for Rc<Geometry + 'static>
{
	fn from(geoemtry:CubeGeometry) -> Self
	{
		Rc::new(geoemtry)
	}
}

impl From<CubeGeometry> for Arc<Geometry + 'static>
{
	fn from(geoemtry:CubeGeometry) -> Self
	{
		Arc::new(geoemtry)
	}
}

impl From<CubeGeometry> for Rc<RefCell<Geometry + 'static>>
{
	fn from(geoemtry:CubeGeometry) -> Self
	{
		Rc::new(RefCell::new(geoemtry))
	}
}

impl From<CubeGeometry> for Arc<RefCell<Geometry + 'static>>
{
	fn from(geoemtry:CubeGeometry) -> Self
	{
		Arc::new(RefCell::new(geoemtry))
	}
}