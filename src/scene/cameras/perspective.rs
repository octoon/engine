use std::f32;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::any::Any;

use crate::math::*;

use super::super::scene::{ SceneData, SceneNode, SceneSubData };
use super::super::core::{Object, Downcast, Resource, Camera, CameraType, CameraData, Canvas, Dimensions};
use super::super::materials::{ CustomMaterial };

#[derive(Debug)]
pub struct PerspectiveCamera 
{
	pub node:SceneNode,
	kind:CameraType,
	viewport:(f32,f32,f32,f32),
	color:(f32,f32,f32,f32),
	width:Dimensions,
	height:Dimensions,
	width_rel:f32,
	height_rel:f32,
	aperture:f32,
	shutter_speed:f32,
	film_size:f32,
	focal_length:f32,
	iso:f32,
	projection:float4x4,
	projection_inverse:float4x4,
	view_projection:float4x4,
	view_projection_inverse:float4x4,
	need_update:bool
}

impl PerspectiveCamera 
{
	pub fn new() -> Self 
	{
		let mut material = CustomMaterial::new();
		material.state.clear_depth = Some(1.0);
		material.state.clear_stencil = Some(0);
		material.state.clear_color = Some((0.0, 0.0, 0.0, 0.0));
		material.state.viewport = Some((0.0, 0.0, 1.0, 1.0));

	    fn update(node:&mut SceneData, canvas:&Canvas)
	    {
			let fov = 60.0;
			let ratio = canvas.width() as f32 / canvas.height() as f32;
			let znear = 0.001;
			let zfar = 65535.0;

			let projection = float4x4::perspective_fov_lh(fov, ratio, znear, zfar);
			let projection_inverse = projection.inverse();
			let view_projection = projection * node.transform_inverse();
			let view_projection_inverse = view_projection.inverse();

			let userdata = CameraData
			{
				kind:CameraType::Main,
				view:node.transform_inverse(),
				view_inverse:node.transform(),
				projection:projection,
				projection_inverse:projection_inverse,
				view_projection:view_projection,
				view_projection_inverse:view_projection_inverse,
			};

			node.set_user_data(Box::new(userdata));
	    }

		let mut node = SceneNode::new(SceneSubData::Camera);
		node.set_material(Some(material.into()));
		node.set_user_data(Box::new(CameraData::new()));
		node.with(update);

		Self
		{
			node:node,
			kind:CameraType::Main,
			color:(0.0, 0.0, 0.0, 0.0),
			viewport:(0.0,0.0,1.0,1.0),
			width:Dimensions::Automatic,
			height:Dimensions::Automatic,
			width_rel:0.0,
			height_rel:0.0,
			film_size:36.0,
			focal_length:50.0,
			aperture:16.0,
			shutter_speed:1.0/125.0,
			iso:100.0,
			projection:float4x4::one(),
			projection_inverse:float4x4::one(),
			view_projection:float4x4::one(),
			view_projection_inverse:float4x4::one(),
			need_update:true
		}
	}

	#[inline]
	pub fn builder() -> PerspectiveCameraBuilder
	{
		PerspectiveCameraBuilder::new()
	}

	#[inline]
	pub fn aperture(&self) -> f32
	{
		self.aperture
	}

	#[inline]
	pub fn shutter_speed(&self) -> f32
	{
		self.shutter_speed
	}

	#[inline]
	pub fn iso(&self) -> f32
	{
		self.iso
	}

	#[inline]
	pub fn is_main(&mut self) -> bool
	{
		self.kind == CameraType::Main
	}

	#[inline]
	pub fn film_size(&self) -> f32
	{
		self.film_size
	}

	#[inline]
	pub fn set_width(&mut self, width:Dimensions) -> &mut Self
	{
		self.width = width;
		self.need_update = true;
		self
	}

	#[inline]
	pub fn set_height(&mut self, height:Dimensions) -> &mut Self
	{
		self.height = height;
		self.need_update = true;
		self
	}

	#[inline]
	pub fn set_film_size(&mut self, film_size:f32) -> &mut Self
	{
		self.film_size = film_size;
		self.need_update = true;
		self
	}

	#[inline]
	pub fn set_focal_length(&mut self, focal_length:f32) -> &mut Self
	{
		self.focal_length = focal_length;
		self.need_update = true;
		self
	}

	#[inline]
	pub fn set_aperture(&mut self, aperture:f32) -> &mut Self
	{
		self.aperture = aperture;
		self
	}

	#[inline]
	pub fn set_shutter_speed(&mut self, shutter_speed:f32) -> &mut Self
	{
		self.shutter_speed = shutter_speed;
		self
	}

	#[inline]
	pub fn set_iso(&mut self, iso:f32) -> &mut Self
	{
		self.iso = iso;
		self
	}

	#[inline]
	pub fn set_fov(&mut self, fov:f32) -> &mut Self
	{
		let ratio = f32::tan(fov.to_radians());
		let length = self.film_size / ratio;
		self.focal_length = length;
		self.need_update = true;
		self
	}

	#[inline]
	pub fn fov(&self) -> f32
	{
		// fov = width / focal_distance
		// fov = width / (focal_length / film_size * width)
		// fov = 1.0 / (focal_length / film_size)
		// fov = film_size / focal_length
		let fov = self.film_size() / self.focal_length();
		fov.atan().to_degrees()
	}

	#[inline]
	pub fn upload(&mut self, canvas:&Canvas)
	{
		if 	self.width_rel as u32 != canvas.width() || 
			self.height_rel as u32 != canvas.height()
		{
			self.need_update = true;
		}

		if self.need_update
		{
			match self.width
			{
				Dimensions::Sized(sz) => self.width_rel = sz,
				Dimensions::Automatic => self.width_rel = canvas.width() as f32,
			}

			match self.height
			{
				Dimensions::Sized(sz) => self.height_rel = sz,
				Dimensions::Automatic => self.height_rel = canvas.height() as f32,
			}

			let fov = self.fov();
			let ratio = self.ratio();
			let znear = 0.001;
			let zfar = 65535.0;

			self.node.update(canvas);
			self.projection = float4x4::perspective_fov_lh(fov, ratio, znear, zfar);
			self.projection_inverse = self.projection.inverse();
			self.view_projection = self.projection * self.node.transform_inverse();
			self.view_projection_inverse = self.view_projection.inverse();

			let userdata = CameraData
			{
				kind:self.kind(),
				view:self.transform_inverse(),
				view_inverse:self.transform(),
				projection:self.projection,
				projection_inverse:self.projection_inverse,
				view_projection:self.view_projection,
				view_projection_inverse:self.view_projection_inverse,
			};

			self.node.set_user_data(Box::new(userdata));
			
			self.need_update = false;
		}
	}
}

impl Camera for PerspectiveCamera
{
	#[inline]
	fn kind(&self) -> CameraType
	{
		self.kind
	}

	#[inline]
	fn focal_length(&self) -> f32
	{
		self.focal_length
	}

	#[inline]
	fn focal_distance(&self) -> f32
	{
		self.focal_length() / self.film_size() * self.width() 
	}

	#[inline]
	fn width(&self) -> f32
	{
		self.width_rel
	}

	#[inline]
	fn height(&self) -> f32
	{
		self.height_rel
	}

	#[inline]
	fn clear_color(&self) -> (f32,f32,f32,f32)
	{
		self.color
	}
	
	#[inline]
	fn viewport(&self) -> (f32,f32,f32,f32)
	{
		self.viewport
	}
	
	#[inline]
	fn set_clear_color(&mut self, r:f32, g:f32, b:f32, a:f32)
	{
		self.color = (r, g, b, a);
	}

	#[inline]
	fn set_viewport(&mut self, x:f32, y:f32, z:f32, w:f32)
	{
		self.viewport = (x, y, z, w);
	}

	#[inline]
	fn view(&self) -> float4x4
	{
		self.node.transform_inverse()
	}

	#[inline]
	fn view_inverse(&self) -> float4x4
	{
		self.node.transform()
	}

	#[inline]
	fn projection(&self) -> float4x4
	{
		self.projection
	}

	#[inline]
	fn projection_inverse(&self) -> float4x4
	{
		self.projection_inverse
	}

	#[inline]
	fn view_projection(&self) -> float4x4
	{
		self.view_projection
	}

	#[inline]
	fn view_projection_inverse(&self) -> float4x4
	{
		self.view_projection_inverse
	}

	#[inline]
	fn exposure(&self) -> f32
	{
		let aperture = self.aperture();
		let ev100 = f32::log2((aperture * aperture) * 100.0) / (self.shutter_speed() * self.iso());
		return 1.0 / (1.2 * f32::powf(2.0, ev100));
	}
}

impl Object for PerspectiveCamera
{
}

impl Resource for PerspectiveCamera
{
	#[inline]
	fn uuid(&self) -> &uuid::Uuid
	{
		self.node.uuid()
	}
}

impl Downcast for PerspectiveCamera
{
	fn as_any(&self) -> &Any { self }
	fn as_any_mut(&mut self) -> &mut Any { self }
}

impl AsRef<SceneNode> for PerspectiveCamera
{
	fn as_ref(&self) -> &SceneNode
	{
		&self.node
	}
}

impl AsMut<SceneNode> for PerspectiveCamera
{
	fn as_mut(&mut self) -> &mut SceneNode
	{
		&mut self.node
	}
}

impl From<PerspectiveCamera> for Box<Camera + 'static>
{
	fn from(camera:PerspectiveCamera) -> Self
	{
		Box::new(camera)
	}
}

impl From<PerspectiveCamera> for Rc<Camera + 'static>
{
	fn from(camera:PerspectiveCamera) -> Self
	{
		Rc::new(camera)
	}
}

impl From<PerspectiveCamera> for Arc<Camera + 'static>
{
	fn from(camera:PerspectiveCamera) -> Self
	{
		Arc::new(camera)
	}
}

impl From<PerspectiveCamera> for Rc<RefCell<Camera + 'static>>
{
	fn from(camera:PerspectiveCamera) -> Self
	{
		Rc::new(RefCell::new(camera))
	}
}

impl From<PerspectiveCamera> for Arc<RefCell<Camera + 'static>>
{
	fn from(camera:PerspectiveCamera) -> Self
	{
		Arc::new(RefCell::new(camera))
	}
}

pub struct PerspectiveCameraBuilder
{
	camera:PerspectiveCamera
}

impl PerspectiveCameraBuilder
{
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			camera:PerspectiveCamera::new()
		}
	}

	#[inline(always)]
	pub fn build(self) -> PerspectiveCamera
	{
		self.camera
	}

	#[inline(always)]
	pub fn main(mut self, main:bool) -> Self
	{
		self.camera.kind = if main { CameraType::Main } else { CameraType::Custom };
		self
	}

	#[inline(always)]
	pub fn set_type(mut self, kind:CameraType) -> Self
	{
		self.camera.kind = kind;
		self
	}

	#[inline(always)]
	pub fn set_film_size(mut self, film_size:f32) -> Self
	{
		self.camera.set_film_size(film_size);
		self
	}

	#[inline(always)]
	pub fn set_focal_length(mut self, focal_length:f32) -> Self
	{
		self.camera.set_focal_length(focal_length);
		self
	}

	#[inline(always)]
	pub fn set_aperture(mut self, aperture:f32) -> Self
	{
		self.camera.set_aperture(aperture);
		self
	}

	#[inline(always)]
	pub fn set_shutter_speed(mut self, shutter_speed:f32) -> Self
	{
		self.camera.set_shutter_speed(shutter_speed);
		self
	}

	#[inline(always)]
	pub fn set_iso(mut self, iso:f32) -> Self
	{
		self.camera.set_iso(iso);
		self
	}

	#[inline(always)]
	pub fn set_fov(mut self, fov:f32) -> Self
	{
		self.camera.set_fov(fov);
		self
	}

	#[inline(always)]
	pub fn set_width(mut self, width:Dimensions) -> Self
	{
		self.camera.set_width(width);
		self
	}

	#[inline(always)]
	pub fn set_height(mut self, height:Dimensions) -> Self
	{
		self.camera.set_height(height);
		self
	}

	#[inline(always)]
	pub fn set_clear_color(mut self, r:f32, g:f32, b:f32, a:f32) -> Self
	{
		self.camera.set_clear_color(r, g, b, a);
		self
	}

	#[inline(always)]
	pub fn set_viewport(mut self, x:f32, y:f32, z:f32, w:f32) -> Self
	{
		self.camera.set_viewport(x, y, z, w);
		self
	}

	#[inline(always)]
	pub fn set_translate(mut self, pos:float3) -> Self
	{
		self.camera.set_translate(pos);
		self
	}

	#[inline(always)]
	pub fn set_scale(mut self, sz:float3) -> Self
	{
		self.camera.set_scale(sz);
		self
	}

	#[inline(always)]
	pub fn set_rotation(mut self, rot:float3) -> Self
	{
		self.camera.set_rotation(rot);
		self
	}
}