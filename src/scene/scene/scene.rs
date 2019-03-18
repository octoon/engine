use std::collections::HashMap;
use std::sync::Arc;
use super::{SceneNode, SceneSubData};
use super::super::core::{Canvas, CameraData, Geometry, Material};
use serde::ser::{Serialize, Serializer, SerializeStruct, SerializeSeq};

pub struct Scene
{
	pub cameras:Vec<SceneNode>,
	pub shapes:Vec<SceneNode>,
	pub lights:Vec<SceneNode>,
	pub models:Vec<SceneNode>,
}

impl Scene 
{
	pub fn new() -> Self 
	{
		Self
		{
			cameras:Vec::new(),
			lights:Vec::new(),
			shapes:Vec::new(),
			models:Vec::new(),
		}
	}

	pub fn add<T:AsRef<SceneNode>>(&mut self, node:T) -> &mut Self
	{
		match node.as_ref().kind()
		{
			SceneSubData::Light =>
			{
				self.lights.push(node.as_ref().clone());
			},
			SceneSubData::Shape =>
			{
				self.shapes.push(node.as_ref().clone());
			},
			SceneSubData::Camera => 
			{
				self.cameras.push(node.as_ref().clone());
				self.cameras.sort_by(
					|a, b|
					{
						let a_kind = a.user_data::<CameraData>().unwrap().kind;
						let b_kind = b.user_data::<CameraData>().unwrap().kind;
						a_kind.cmp(&b_kind)
					} 
				);
			},
			SceneSubData::Group =>
			{
				for actor in node.as_ref().data.borrow().children.iter()
				{
					self.add(actor);
				}

				self.models.push(node.as_ref().clone());
			}
		}

		self
	}

	pub fn cameras(&self) -> &[SceneNode]
	{
		&self.cameras[..]
	}

	pub fn lights(&self) -> &[SceneNode]
	{
		&self.lights[..]
	}

	pub fn shapes(&self) -> &[SceneNode]
	{
		&self.shapes[..]
	}

	pub fn num_camera(&self) -> usize
	{
		self.cameras.len()
	}

	pub fn num_light(&self) -> usize
	{
		self.lights.len()
	}

	pub fn num_shape(&self) -> usize
	{
		self.shapes.len()
	}

	pub fn update(&mut self, canvas:&Canvas)
	{
		for camera in self.cameras.iter_mut()
		{
			camera.update(canvas);
		}

		for light in self.lights.iter_mut()
		{
			light.update(canvas);
		}

		for shape in self.shapes.iter_mut()
		{
			shape.update(canvas);
		}
	}
}

struct ShapeSerialize
{
	geometries:HashMap<uuid::Uuid, Arc<Geometry>>
}

struct MaterialSerialize
{
	materials:HashMap<uuid::Uuid, Arc<Material>>
}

impl ShapeSerialize
{
	fn new(shapes:&Vec<SceneNode>) -> Self
	{
		let mut geometries = HashMap::new();

		for shape in shapes
		{
			match shape.geometry()
			{
				Some(ref data) => 
				{
					let uuid = data.uuid().clone();
					if !geometries.contains_key(&uuid)
					{
						geometries.insert(uuid, data.clone());
					}
				},
				None => {},
			}
		}

		Self
		{
			geometries
		}
	}
}

impl MaterialSerialize
{
	fn new(shapes:&Vec<SceneNode>) -> Self
	{
		let mut materials = HashMap::new();

		for shape in shapes
		{
			match shape.material()
			{
				Some(ref data) => 
				{
					let uuid = data.uuid().clone();
					if !materials.contains_key(&uuid)
					{
						materials.insert(uuid, data.clone());
					}
				},
				None => {},
			}
		}

		Self
		{
			materials
		}
	}
}

impl Serialize for ShapeSerialize
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut s = serializer.serialize_seq(Some(self.geometries.len()))?;
		for geometry in self.geometries.values()
		{
			s.serialize_element(&geometry)?;
		}
		s.end()
	}
}

impl Serialize for MaterialSerialize
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut s = serializer.serialize_seq(Some(self.materials.len()))?;
		for material in self.materials.iter()
		{
			s.serialize_element(&material)?;
		}
		s.end()
	}
}

impl Serialize for Scene
{
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		let mut s = serializer.serialize_struct("scene", 5)?;
		s.serialize_field("model", &ShapeSerialize::new(&self.shapes))?;
		s.serialize_field("materials", &MaterialSerialize::new(&self.shapes))?;
		s.serialize_field("shape", &self.shapes)?;
		s.serialize_field("camera", &self.cameras)?;
		s.serialize_field("light", &self.lights)?;
		s.end()
	}
}