use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::core::*;
use super::super::scene::{ SceneNode, SceneSubData };
use super::super::shapes::{ MeshShape };

#[derive(Debug)]
pub struct ModelShape
{
	pub node:SceneNode,
	pub shapes:Vec<MeshShape>
}

impl ModelShape
{
	pub fn new(shapes:Vec<MeshShape>) -> Self 
	{
		let mut node = SceneNode::new(SceneSubData::Group);

		for shape in shapes.iter()
		{
			let mut shape_node = SceneNode::new(SceneSubData::Shape);
			shape_node.set_geometry(Some(shape.geometry()));
			shape_node.set_material(Some(shape.material()));

			node.add_child(shape_node);
		}	

		Self
		{
			node:node,
			shapes:shapes
		}
	}

	#[inline]
	pub fn builder() -> ModelShapeBuilder
	{
		ModelShapeBuilder::new()
	}
}

impl Object for ModelShape
{
	#[inline(always)]
	fn set_translate(&mut self, pos:float3) -> &mut SceneNode
	{
		self.as_mut().set_translate(pos);

		for child in self.node.data.borrow_mut().children.iter_mut()
		{
			child.set_translate(pos);
		}

		self.as_mut()
	}

	#[inline(always)]
	fn set_scale(&mut self, sz:float3) -> &mut SceneNode
	{
		self.as_mut().set_scale(sz);

		for child in self.node.data.borrow_mut().children.iter_mut()
		{
			child.set_scale(sz);
		}

		self.as_mut()
	}
	
	#[inline(always)]
	fn set_rotation(&mut self, rot:float3) -> &mut SceneNode
	{
		self.as_mut().set_rotation(rot);

		for child in self.node.data.borrow_mut().children.iter_mut()
		{
			child.set_rotation(rot);
		}

		self.as_mut()
	}
}

impl Resource for ModelShape
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Downcast for ModelShape
{
	fn as_any(&self) -> &Any { self }
	fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for ModelShape
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for ModelShape
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<ModelShape> for Box<Object + 'static>
{
	fn from(shape:ModelShape) -> Self
	{
		Box::new(shape)
	}
}

impl From<ModelShape> for Rc<Object + 'static>
{
	fn from(shape:ModelShape) -> Self
	{
		Rc::new(shape)
	}
}

impl From<ModelShape> for Arc<Object + 'static>
{
	fn from(shape:ModelShape) -> Self
	{
		Arc::new(shape)
	}
}

impl From<ModelShape> for Rc<RefCell<Object + 'static>>
{
	fn from(shape:ModelShape) -> Self
	{
		Rc::new(RefCell::new(shape))
	}
}

impl From<ModelShape> for Arc<RefCell<Object + 'static>>
{
	fn from(shape:ModelShape) -> Self
	{
		Arc::new(RefCell::new(shape))
	}
}

pub struct ModelShapeBuilder
{
	pub position:float3,
	pub scale:float3,
	pub rotation:float3,
	pub shapes:Vec<MeshShape>
}

impl ModelShapeBuilder
{
	#[inline]
	pub fn new() -> Self
	{
		Self
		{
			position:float3::zero(),
			scale:float3::one(),
			rotation:float3::zero(),
			shapes:Vec::new(),
		}
	}

	#[inline]
	pub fn build(self) -> ModelShape
	{
		let mut mesh = ModelShape::new(self.shapes);
		mesh.set_translate(self.position);
		mesh.set_scale(self.scale);
		mesh.set_rotation(self.rotation);
		mesh
	}

	#[inline]
	pub fn add_shape(mut self, shape:MeshShape) -> Self
	{
		self.shapes.push(shape);
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