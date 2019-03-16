use super::{SceneNode, SceneSubData};
use super::super::core::{Canvas, CameraData};

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