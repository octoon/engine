use std::sync::Arc;
use std::boxed::Box;
use std::any::Any;

use crate::math::{float3, float4x4};

use super::SceneNode;
use super::super::core::{Transform, Geometry, Material, Canvas};
use super::super::util::uuid::OsRandNewV4;

#[derive(Debug, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub enum SceneSubData
{
	Camera,
	Light,
	Shape,
	Group
}

pub struct SceneData
{
	pub visible:bool,
	pub name:String,
	pub uuid:uuid::Uuid,
	pub transform:Transform,
	pub sub_data:SceneSubData,
	pub user_data:Box<Any + 'static>,
	pub geometry:Option<Arc<Geometry + 'static>>,
	pub material:Option<Arc<Material + 'static>>,
	pub children:Vec<SceneNode>,
	pub dispatch:Option<Box<FnMut(&Canvas) + 'static>>,
}

impl SceneData
{
	#[inline]
	pub fn new(sub_data:SceneSubData) -> Self
	{
		let user_data = ();

		Self
		{
			sub_data:sub_data,
			visible:true,
			name:String::new(),
			uuid:uuid::Uuid::new_v4_osrng(),
			transform:Transform::new(),
			user_data:Box::new(user_data),
			children:Vec::new(),
			geometry:None,
			material:None,
			dispatch:None
		}
	}

	#[inline(always)]
	pub fn kind(&self) -> SceneSubData
	{
		self.sub_data
	}

	#[inline(always)]
	pub fn uuid(&self) -> &uuid::Uuid
	{
		&self.uuid
	}

	#[inline(always)]
	pub fn is_visible(&self) -> bool
	{
		self.visible
	}

	#[inline(always)]
	pub fn set_visible(&mut self, visible:bool) -> &mut Self
	{
		self.visible = visible;
		self
	}

	#[inline(always)]
	pub fn name(&self) -> &str
	{
		&self.name
	}

	#[inline(always)]
	pub fn set_name(&mut self, name:&str) -> &mut Self
	{
		self.name = name.to_string();
		self
	}

	#[inline(always)]
	pub fn add_child(&mut self, child:SceneNode) -> &mut Self
	{
		self.children.push(child);
		self
	}

	#[inline(always)]
	pub fn remove_child(&mut self, child:&SceneNode) -> &mut Self
	{
		let mut remove_item = self.children.len();

		for (i, item) in self.children.iter().enumerate()
		{
			if item.uuid == child.uuid
			{
				remove_item = i;
				break;
			}
		}

		if remove_item < self.children.len()
		{
			self.children.remove(remove_item);
		}

		self
	}

	#[inline(always)]
	pub fn num_children(&self) -> usize
	{
		self.children.len()
	}

	#[inline(always)]
	pub fn translate(&self) -> float3
	{
		self.transform.translate()
	}

	#[inline(always)]
	pub fn set_translate(&mut self, pos:float3) -> &mut Self
	{
		self.transform.set_translate(pos);
		self
	}

	#[inline(always)]
	pub fn scale(&self) -> float3
	{
		self.transform.scale()
	}

	#[inline(always)]
	pub fn set_scale(&mut self, sz:float3) -> &mut Self
	{
		self.transform.set_scale(sz);
		self
	}

	#[inline(always)]
	pub fn rotation(&self) -> float3
	{
		self.transform.rotation()
	}

	#[inline(always)]
	pub fn set_rotation(&mut self, rot:float3) -> &mut Self
	{
		self.transform.set_rotation(rot);
		self
	}

	#[inline(always)]
	pub fn transform(&self) -> float4x4
	{
		self.transform.transform()
	}

	#[inline(always)]
	pub fn transform_inverse(&self) -> float4x4
	{
		self.transform.transform_inverse()
	}

	#[inline(always)]
	pub fn geometry(&self) -> Option<Arc<Geometry + 'static>>
	{
		self.geometry.clone()
	}

	#[inline(always)]
	pub fn set_geometry(&mut self, geometry:Option<Arc<Geometry + 'static>>) -> &mut Self
	{
		self.geometry = geometry;
		self
	}

	#[inline(always)]
	pub fn material(&self) -> Option<Arc<Material + 'static>>
	{
		self.material.clone()
	}

	#[inline(always)]
	pub fn set_material(&mut self, material:Option<Arc<Material + 'static>>) -> &mut Self
	{
		self.material = material;
		self
	}

	#[inline(always)]
	pub fn user_data(&self) -> &Box<Any + 'static>
	{
		&self.user_data
	}

	#[inline(always)]
	pub fn set_user_data(&mut self, user_data:Box<Any + 'static>) -> &mut Self
	{
		self.user_data = user_data;
		self
	}

	#[inline(always)]
	pub fn with<T:FnMut(&Canvas) + 'static>(&mut self, method:T) -> &mut Self
	{
		self.dispatch = Some(Box::new(method));
		self
	}

	#[inline(always)]
	pub fn update(&mut self, canvas:&Canvas) -> &mut Self
	{
		match self.dispatch
		{
			Some(ref mut callback) => { callback(canvas) }
			_ => {}
		}

		self
	}
}

impl std::fmt::Debug for SceneData
{
	fn fmt(&self, f:&mut std::fmt::Formatter) -> std::fmt::Result
	{
		write!(f, "{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", 
			self.visible,
			self.name,
			self.uuid,
			self.transform,
			self.sub_data,
			self.geometry,
			self.material,
			self.children,
		)
	}
}