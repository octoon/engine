use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::any::Any;

use crate::math::{float3, float4x4};

use super::{SceneData, SceneSubData};
use super::super::core::{Geometry, Material, Canvas};

#[derive(Clone)]
pub struct SceneNode
{
	pub uuid:uuid::Uuid,
	pub data:Rc<RefCell<SceneData>>,
}

impl SceneNode 
{
	#[inline(always)]
	pub fn new(sub_data:SceneSubData) -> Self
	{
		let data = SceneData::new(sub_data);
		Self
		{
			uuid: *data.uuid(),
			data: Rc::new(RefCell::new(data)),
		}
	}

	#[inline(always)]
	pub fn kind(&self) -> SceneSubData
	{
		self.data.borrow().kind()
	}

	#[inline(always)]
	pub fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}

	#[inline(always)]
	pub fn is_visible(&self) -> bool
	{
		self.data.borrow().is_visible()
	}

	#[inline(always)]
	pub fn set_visible(&mut self, visible: bool)
	{
		self.data.borrow_mut().set_visible(visible);
	}

	#[inline(always)]
	pub fn transform(&self) -> float4x4
	{
		self.data.borrow().transform()
	}

	#[inline(always)]
	pub fn transform_inverse(&self) -> float4x4
	{
		self.data.borrow().transform_inverse()
	}

	#[inline(always)]
	pub fn name(&self) -> String
	{
		self.data.borrow().name().to_string()
	}

	#[inline(always)]
	pub fn set_name(&mut self, name: &str)
	{
		self.data.borrow_mut().set_name(name);
	}

	#[inline(always)]
	pub fn translate(&self) -> float3
	{
		self.data.borrow().translate()
	}

	#[inline(always)]
	pub fn set_translate(&mut self, pos:float3) -> &mut SceneNode
	{
		self.data.borrow_mut().set_translate(pos);
		self
	}

	#[inline(always)]
	pub fn scale(&self) -> float3
	{
		self.data.borrow().scale()
	}

	#[inline(always)]
	pub fn set_scale(&mut self, sz:float3) -> &mut SceneNode
	{
		self.data.borrow_mut().set_scale(sz);
		self
	}

	#[inline(always)]
	pub fn rotation(&self) -> float3
	{
		self.data.borrow().rotation()
	}

	#[inline(always)]
	pub fn set_rotation(&mut self, rot:float3) -> &mut SceneNode
	{
		self.data.borrow_mut().set_rotation(rot);
		self
	}

	#[inline(always)]
	pub fn geometry(&self) -> Option<Arc<Geometry + 'static>>
	{
		self.data.borrow().geometry()
	}

	#[inline(always)]
	pub fn set_geometry(&mut self, geometry: Option<Arc<Geometry + 'static>>) -> &mut Self
	{
		self.data.borrow_mut().set_geometry(geometry);
		self
	}

	#[inline(always)]
	pub fn material(&self) -> Option<Arc<Material + 'static>>
	{
		self.data.borrow().material()
	}

	#[inline(always)]
	pub fn set_material(&mut self, material: Option<Arc<Material + 'static>>) -> &mut Self
	{
		self.data.borrow_mut().set_material(material);
		self
	}

	#[inline(always)]
	pub fn set_user_data(&mut self, user_data: Box<Any + 'static>)
	{
		self.data.borrow_mut().set_user_data(user_data);
	}

	#[inline(always)]
	pub fn user_data<T:Clone + 'static>(&self) -> Result<T, &str>
	{
		match self.data.borrow().user_data.downcast_ref::<T>()
		{
			Some(v) => { Ok(v.clone()) },
			None => { Err("Invalid downcast type for user data.") }
		}
	}

	#[inline(always)]
	pub fn add_child(&mut self, child:SceneNode)
	{
		self.data.borrow_mut().add_child(child);
	}

	#[inline(always)]
	pub fn remove_child(&mut self, child:&SceneNode)
	{
		self.data.borrow_mut().remove_child(child);
	}

	#[inline(always)]
	pub fn with<T:FnMut(&Canvas) + 'static>(&mut self, method:T) -> &mut Self
	{
		self.data.borrow_mut().with(method);
		self
	}

	#[inline(always)]
	pub fn update(&mut self, canvas:&Canvas) -> &mut Self
	{
		self.data.borrow_mut().update(canvas);
		self
	}
}

impl std::fmt::Debug for SceneNode
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{:?}", self.data)
	}
}

impl AsRef<SceneNode> for SceneNode
{
	fn as_ref(&self) -> &SceneNode
	{
		self
	}
}

impl AsMut<SceneNode> for SceneNode
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		self
	}
}