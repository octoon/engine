use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::core::*;
use super::super::scene::{ SceneNode, SceneSubData };

#[derive(Debug)]
pub struct MeshShape
{
	pub node:SceneNode
}

impl MeshShape
{
	pub fn new(geometry:Arc<Geometry + 'static>, material: Arc<Material + 'static>) -> Self 
	{
		let mut node = SceneNode::new(SceneSubData::Shape);
		node.set_geometry(Some(geometry));
		node.set_material(Some(material));

		Self
		{
			node:node,
		}
	}

	#[inline]
	pub fn builder() -> MeshShapeBuilder
	{
		MeshShapeBuilder::new()
	}
}

impl Shape for MeshShape
{
	#[inline]
	fn geometry(&self) -> Arc<Geometry + 'static>
	{
		self.node.geometry().unwrap()
	}

	#[inline]
	fn set_geometry(&mut self, geometry: Arc<Geometry + 'static>) 
	{
		self.node.set_geometry(Some(geometry));
	}

	#[inline]
	fn material(&self) -> Arc<Material + 'static>
	{
		self.node.material().unwrap()
	}

	#[inline]
	fn set_material(&mut self, material: Arc<Material + 'static>) 
	{
		self.node.set_material(Some(material));
	}
}

impl Object for MeshShape
{
}

impl Resource for MeshShape
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Downcast for MeshShape
{
	fn as_any(&self) -> &Any { self }
	fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for MeshShape
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for MeshShape
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<MeshShape> for Box<Shape + 'static>
{
	fn from(shape:MeshShape) -> Self
	{
		Box::new(shape)
	}
}

impl From<MeshShape> for Rc<Shape + 'static>
{
	fn from(shape:MeshShape) -> Self
	{
		Rc::new(shape)
	}
}

impl From<MeshShape> for Arc<Shape + 'static>
{
	fn from(shape:MeshShape) -> Self
	{
		Arc::new(shape)
	}
}

impl From<MeshShape> for Rc<RefCell<Shape + 'static>>
{
	fn from(shape:MeshShape) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<MeshShape> for Arc<RefCell<Shape + 'static>>
{
	fn from(shape:MeshShape) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct MeshShapeBuilder
{
	pub position:float3,
	pub scale:float3,
	pub rotation:float3,
	pub geometry:Option<Arc<Geometry + 'static>>,
	pub material:Option<Arc<Material + 'static>>
}

impl MeshShapeBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			position:float3::zero(),
			scale:float3::one(),
			rotation:float3::zero(),
			geometry:None,
			material:None,
		}
	}

	#[inline]
	pub fn build(self) -> MeshShape
	{
		assert_eq!(self.geometry.is_some(), true);
		assert_eq!(self.material.is_some(), true);

		let mut mesh = MeshShape::new(self.geometry.unwrap(), self.material.unwrap());
		mesh.set_translate(self.position);
		mesh.set_scale(self.scale);
		mesh.set_rotation(self.rotation);
		mesh
	}

	#[inline]
	pub fn set_geometry(mut self, geometry:Arc<Geometry + 'static>) -> Self
	{
		self.geometry = Some(geometry);
		self
	}

	#[inline]
	pub fn set_material(mut self, material:Arc<Material + 'static>) -> Self
	{
		self.material = Some(material);
		self
	}

	#[inline]
	pub fn set_translate(mut self, pos:float3) -> Self
	{
		self.position = pos;
		self
	}

	#[inline]
	pub fn set_scale(mut self, sz:float3) -> Self
	{
		self.scale = sz;
		self
	}

	#[inline]
	pub fn set_rotation(mut self, rot:float3) -> Self
	{
		self.rotation = rot;
		self
	}
}