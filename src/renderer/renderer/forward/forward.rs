use std::option::*;
use std::collections::HashMap;
use std::sync::Arc;

use crate::math::*;
use crate::math::type_size::*;

use crate::scene::core::*;
use crate::scene::scene::{Scene, SceneNode};

use super::uniforms::*;
use super::shaders::*;
use super::super::super::webgl::{ WebGLRenderingContext as gl, WebGLProgram, WebGLTexture, WebGLBuffer };

pub struct ForwardRenderer
{
	w:u32,
	h:u32,
	context:gl,
	texture:WebGLTexture,
	shapes:HashMap<uuid::Uuid, GeometryUniform>,
	materials:HashMap<uuid::Uuid, MaterialUniform>,
	textures:HashMap<uuid::Uuid, TextureUniform>,
	depth_enable:bool,
	depth_write_enable:bool,
	depth_func:ComparisonFunc,
	cull_mode:CullMode,
	front_face:FrontFace,
	polygon_mode:PolygonMode,
}

impl ForwardRenderer
{
	pub fn new(context:gl, width:u32, height:u32) -> Self 
	{
		let texture = context.create_texture().unwrap();
		context.bind_texture(gl::TEXTURE_2D, &texture);
		context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
		context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
		context.tex_image2d(gl::TEXTURE_2D, 0, gl::RGBA, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, &vec![255,255,0,255][..]);

		Self
		{
			w:width,
			h:height,
			context:context,
			texture:texture,
			materials:HashMap::new(),
			shapes:HashMap::new(),
			textures:HashMap::new(),
			depth_enable:false,
			depth_write_enable:false,
			depth_func:ComparisonFunc::Lequal,
			cull_mode:CullMode::None,
			front_face:FrontFace::CCW,
			polygon_mode:PolygonMode::Solid
		}
	}

	pub fn set_width(&mut self, width:u32)
	{
		self.w = width;
	}

	pub fn set_height(&mut self, height:u32)
	{
		self.h = height;
	}

	fn init_geometry(&mut self, geometry:&Arc<Geometry>)
	{
		if !self.shapes.contains_key(geometry.uuid())
		{
			let count = geometry.num_vertices();
			let stride = float3::type_size() + float3::type_size() + float2::type_size();
			let mut vertices:Vec<f32> = Vec::with_capacity(count * stride);

			for i in 0..count
			{
				let v = &geometry.vertices()[i];
				let n = &geometry.normals()[i];
				let uv  = &geometry.texcoords()[i];

				vertices.push(v.x);
				vertices.push(v.y);
				vertices.push(v.z);

				vertices.push(n.x);
				vertices.push(n.y);
				vertices.push(n.z);

				vertices.push(uv.x);
				vertices.push(uv.y);
			}

			// Create and store data into vertex buffer
			let vertex_buffer = self.context.create_buffer().unwrap();
			let index_buffer = self.context.create_buffer().unwrap();

			self.context.bind_buffer(gl::ARRAY_BUFFER, &vertex_buffer);
			self.context.buffer_data(gl::ARRAY_BUFFER, &vertices[..], gl::STATIC_DRAW);

			// Create and store data into index buffer
			self.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, &index_buffer);
			self.context.buffer_data(gl::ELEMENT_ARRAY_BUFFER, geometry.indices(), gl::STATIC_DRAW);

			self.shapes.insert(geometry.uuid().clone(), GeometryUniform::new(vertex_buffer, index_buffer, geometry.num_indices() as i32));
		}
	}

	fn init_texture(&mut self, image:&Arc<Texture>)
	{
		if !self.textures.contains_key(image.uuid())
		{
			let texture = self.context.create_texture().unwrap();
			self.context.bind_texture(gl::TEXTURE_2D, &texture);
			self.context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
			self.context.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);

			match image.color_type()
			{
				ColorType::RGBA(_) => 
				{
					self.context.tex_image2d(gl::TEXTURE_2D, 0, gl::RGBA, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, image.raw_pixels());
				},
				ColorType::RGB(_) => 
				{
					self.context.tex_image2d(gl::TEXTURE_2D, 0, gl::RGB, image.width() as i32, image.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, image.raw_pixels());
				},
				_ => {},
			}

			self.textures.insert(image.uuid().clone(), TextureUniform::new(texture));
		}
	}

	fn init_vs(&self, material:&Arc<Material + 'static>) -> String
	{
		let mut vs = VERT_CODE_HEADER.to_string();
		vs += SHADER_CODE_BEGIN;
		vs += SHADER_ATTRIB_POSITION;
		vs += SHADER_ATTRIB_NORMAL;
		vs += SHADER_ATTRIB_TEXCOORD0;
		vs += SHADER_UNIORMS_MODEL;
		vs += SHADER_UNIORMS_VIEW;
		vs += SHADER_UNIORMS_VIEWPROJECT;
		vs += SHADER_VARYING_POSITION;
		vs += SHADER_VARYING_NORMAL;
		vs += SHADER_VARYING_TEXCOORD0;
		vs += SHADER_CODE_END;
		vs += VERT_CODE_BODY_HELPER;
		vs += material.vs();
		vs += VERT_CODE_BODY;

		return vs;
	}

	fn init_fs(&self, material:&Arc<Material + 'static>, lights:&[SceneNode]) -> String
	{
		let mut fs : String = FRAG_CODE_HEANDER.to_string();
		fs += SHADER_CODE_BEGIN;
		fs += SHADER_VARYING_POSITION;
		fs += SHADER_VARYING_NORMAL;
		fs += SHADER_VARYING_TEXCOORD0;
		fs += SHADER_UNIORMS_CAMERAPOSITION;
		fs += SHADER_UNIORMS_EXPOSURE;
		fs += SHADER_CODE_END;

		for i in 0..lights.len()
		{
			let kind = lights[i].user_data::<LightData>().unwrap().kind;
			match kind
			{
				LightType::Sky => { fs.push_str(&sky_light_uniforms(i)); },
				LightType::Point => { fs.push_str(&point_light_uniforms(i)); },
				LightType::Spot => { fs.push_str(&spot_light_uniforms(i)); },
				LightType::Directional => { fs.push_str(&directional_light_uniforms(i)); }
			}
		}

		fs += FRAG_CODE_HELPER;
		fs += material.fs();
		fs += FRAG_CODE_BODY_BEGIN;

		for i in 0..lights.len()
		{
			let kind = lights[i].user_data::<LightData>().unwrap().kind;
			match kind
			{
				LightType::Sky => { fs.push_str(sky_light_shading(i).as_str()); },
				LightType::Point => { fs.push_str(point_light_shading(i).as_str()); },
				LightType::Spot => { fs.push_str(spot_light_shading(i).as_str()); },
				LightType::Directional => { fs.push_str(directional_light_shading(i).as_str()); }
			}
		}

		fs += FRAG_CODE_BODY_END;

		return fs;
	}

	fn init_program(&self, vs:String, fs:String) -> Option<WebGLProgram>
	{
		let program = self.context.create_program().unwrap();
		let vert_shader = self.context.create_shader(gl::VERTEX_SHADER).unwrap();
		let frag_shader = self.context.create_shader(gl::FRAGMENT_SHADER).unwrap();
		
		self.context.shader_source(&vert_shader, &vs);
		self.context.compile_shader(&vert_shader);
		self.context.get_shader_info_log(&vert_shader).unwrap();

		self.context.shader_source(&frag_shader, &fs);
		self.context.compile_shader(&frag_shader);
		self.context.get_shader_info_log(&frag_shader).unwrap();

		self.context.attach_shader(&program, &vert_shader);
		self.context.attach_shader(&program, &frag_shader);
		self.context.link_program(&program);
		self.context.validate_program(&program);
		self.context.get_program_info_log(&program).unwrap();

		self.context.delete_shader(&vert_shader);
		self.context.delete_shader(&frag_shader);

		return Some(program);
	}

	fn init_uniforms(&mut self, lights:&[SceneNode], material:&Arc<Material>, program:WebGLProgram) -> MaterialUniform
	{
		self.context.use_program(&program);

		let model = self.context.get_uniform_location(&program, "matModel");
		let view = self.context.get_uniform_location(&program, "matView");
		let viewproject = self.context.get_uniform_location(&program, "matViewProject");
		let eye_position = self.context.get_uniform_location(&program, "CameraPosition");
		let exposure = self.context.get_uniform_location(&program, "Exposure");

		let mut lights_vec = Vec::with_capacity(lights.len());
		for i in 0..lights.len()
		{
			let locations = LightUniform
			{
				color: self.context.get_uniform_location(&program, &format!("LightColor{}", i)),
				position: self.context.get_uniform_location(&program, &format!("LightPosition{}", i)),
				direction: self.context.get_uniform_location(&program, &format!("LightDirection{}", i)),
				radiance: self.context.get_uniform_location(&program, &format!("LightRadiance{}", i)),
				irradiance: self.context.get_uniform_location(&program, &format!("LightIrradiance{}", i)),
				angle: self.context.get_uniform_location(&program, &format!("LightAngle{}", i)),
			};

			lights_vec.push(locations);
		}

		let mut unit = 0;
		let mut locations = Vec::with_capacity(material.num_uniform());

		for (key, value) in material.uniforms()
		{
			let location = self.context.get_uniform_location(&program, key);
			if !location.is_some()
			{
				locations.push(location);
				continue;
			}

			match value
			{
				Variant::Boolean(v) => { self.context.uniform1i(location.as_ref(), *v as _); },
				Variant::Int1(v) => { self.context.uniform1i(location.as_ref(), *v as _); },
				Variant::Int2(v) => { self.context.uniform2iv(location.as_ref(), v.to_tuple()); },
				Variant::Int3(v) => { self.context.uniform3iv(location.as_ref(), v.to_tuple()); },
				Variant::Int4(v) => { self.context.uniform4iv(location.as_ref(), v.to_tuple()); },
				Variant::Float1(v) => { self.context.uniform1f(location.as_ref(), *v); },
				Variant::Float2(v) => { self.context.uniform2fv(location.as_ref(), v.to_tuple()); },
				Variant::Float3(v) => { self.context.uniform3fv(location.as_ref(), v.to_tuple()); },
				Variant::Float4(v) => { self.context.uniform4fv(location.as_ref(), v.to_tuple()); },
				Variant::Float2x2(m) => { self.context.uniform_matrix2fv(location.as_ref(), false, &m.to_array()[..]); },
				Variant::Float3x3(m) => { self.context.uniform_matrix3fv(location.as_ref(), false, &m.to_array()[..]); },
				Variant::Float4x4(m) => { self.context.uniform_matrix4fv(location.as_ref(), false, &m.to_array()[..]); },
				Variant::Texture(texture) => 
				{
					if texture.is_some()
					{
						self.init_texture(&texture.clone().unwrap());
					}

					self.context.uniform1i(location.as_ref(), unit);
					unit += 1;
				},
				_=>{}
			}

			locations.push(location);
		}

		let m = MaterialUniform
		{
			program:program,
			model:model,
			view:view,
			viewproject:viewproject,
			eye_position:eye_position,
			exposure:exposure,
			locations:locations,
			lights:lights_vec 
		};

		return m;
	}

	fn init_material(&mut self, material:&Arc<Material>, lights:&[SceneNode])
	{
		if !self.materials.contains_key(material.uuid())
		{
			let vs = self.init_vs(&material);
			let fs = self.init_fs(&material, lights);
			let program = self.init_program(vs, fs);
			let uniform = self.init_uniforms(&lights, &material, program.unwrap());

			self.materials.insert(material.uuid().clone(), uniform);
		}
	}

	fn init_shapes(&mut self, shapes:&[SceneNode], lights:&[SceneNode])
	{
		for shape in shapes
		{
			self.init_geometry(&shape.geometry().unwrap());
			self.init_material(&shape.material().unwrap(), lights);
		}
	}

	fn init_lights(&mut self, lights:&[SceneNode])
	{
		for light in lights
		{
			let kind = light.user_data::<LightData>().unwrap().kind;
			match kind
			{
				LightType::Sky => 
				{
					let user_data = light.user_data::<LightData>();
					if user_data.is_ok()
					{
						let sky = user_data.unwrap();
						self.init_texture(&sky.radiance.unwrap()); 
						self.init_texture(&sky.irradiance.unwrap()); 
					}
				},
				_ => {}
			}
		}
	}

	fn set_input_layout(&self, layouts:&[VertexAttrib])
	{
		for layout in layouts
		{
			self.context.enable_vertex_attrib_array(layout.index as _);
			self.context.vertex_attrib_pointer(
				layout.index as _,
				layout.count as _,
				gl::FLOAT as _,
				false,
				layout.stride as _,
				layout.offset as _
				);	
		}
	}

	fn set_draw_buffer(&self, vbo:&WebGLBuffer, ibo:&WebGLBuffer)
	{
		self.context.bind_buffer(gl::ARRAY_BUFFER, vbo);
		self.context.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
	}

	fn set_camera_uniforms(&self, camera:&SceneNode, uniform:&MaterialUniform)
	{
		let user_data = camera.user_data::<CameraData>().unwrap();

		if uniform.view.is_some()
		{
			let view_matrix = user_data.view.to_array();
			self.context.uniform_matrix4fv(uniform.view.as_ref(), false, &view_matrix[..]);
		}

		if uniform.viewproject.is_some()
		{
			let viewproj_matrix = user_data.view_projection.to_array();
			self.context.uniform_matrix4fv(uniform.viewproject.as_ref(), false, &viewproj_matrix[..]);
		}

		if uniform.eye_position.is_some()
		{
			self.context.uniform3fv(uniform.eye_position.as_ref(), camera.translate().to_tuple());
		}

		if uniform.exposure.is_some()
		{
			self.context.uniform1f(uniform.exposure.as_ref(), 1.0);
		}
	}

	fn set_geometry_uniforms(&self, shape:&SceneNode, uniform:&MaterialUniform)
	{
		if uniform.model.is_some()
		{
			let mov_matrix = shape.transform().to_array();
			self.context.uniform_matrix4fv(uniform.model.as_ref(), false, &mov_matrix[..]);
		}
	}

	fn set_material_uniforms(&self, material:&Arc<Material>, uniforms:&MaterialUniform)
	{
		let mut unit = 0;
		let mut index = 0;

		for (_key, value) in material.uniforms()
		{
			let location = uniforms.locations[index].as_ref();
			index += 1;
			if !location.is_some() {
				continue;
			}

			match value
			{
				Variant::Boolean(v) => { self.context.uniform1i(location, *v as _); },
				Variant::Int1(v) => { self.context.uniform1i(location, *v as _); },
				Variant::Int2(v) => { self.context.uniform2iv(location, v.to_tuple()); },
				Variant::Int3(v) => { self.context.uniform3iv(location, v.to_tuple()); },
				Variant::Int4(v) => { self.context.uniform4iv(location, v.to_tuple()); },
				Variant::Float1(v) => { self.context.uniform1f(location, *v); },
				Variant::Float2(v) => { self.context.uniform2fv(location, v.to_tuple()); },
				Variant::Float3(v) => { self.context.uniform3fv(location, v.to_tuple()); },
				Variant::Float4(v) => { self.context.uniform4fv(location, v.to_tuple()); },
				Variant::Float2x2(m) => { self.context.uniform_matrix2fv(location, false, &m.to_array()[..]); },
				Variant::Float3x3(m) => { self.context.uniform_matrix3fv(location, false, &m.to_array()[..]); },
				Variant::Float4x4(m) => { self.context.uniform_matrix4fv(location, false, &m.to_array()[..]); },
				Variant::Texture(texture) => 
				{
					if texture.is_some()
					{
						let texture_id = texture.as_ref().unwrap();
						let texture_uniform = self.textures.get(texture_id.uuid()).unwrap();

						self.context.active_texture(gl::TEXTURE0 + unit);
						self.context.bind_texture(gl::TEXTURE_2D, &texture_uniform.texture);
					}
					else
					{
						self.context.active_texture(gl::TEXTURE0 + unit);
						self.context.bind_texture(gl::TEXTURE_2D, &self.texture);
					}

					unit += 1;
				},
				_=>{}
			}
		}
	}

	fn set_sky_light_uniforms(&self, value:&SceneNode, uniform:&LightUniform, unit:&mut u32)
	{
		let user_data = value.user_data::<LightData>();
		if user_data.is_ok()
		{
			let light = user_data.unwrap();

			if uniform.color.is_some()
			{
				let (r, g, b) = light.color.to_rgb();
				let intensity = light.intensity();
				self.context.uniform3f(uniform.color.as_ref(), r * intensity, g * intensity, b * intensity);	
			}

			if uniform.irradiance.is_some()
			{
				let texture_uniform = self.textures.get(light.irradiance.unwrap().uuid()).unwrap();

				self.context.uniform1i(uniform.irradiance.as_ref(), *unit as i32);
				self.context.active_texture(gl::TEXTURE0 + *unit);
				self.context.bind_texture(gl::TEXTURE_2D, &texture_uniform.texture);

				*unit += 1;
			}

			if uniform.radiance.is_some()
			{
				let texture_uniform = self.textures.get(light.radiance.unwrap().uuid()).unwrap();

				self.context.uniform1i(uniform.radiance.as_ref(), *unit as i32);
				self.context.active_texture(gl::TEXTURE0 + *unit);
				self.context.bind_texture(gl::TEXTURE_2D, &texture_uniform.texture);

				*unit += 1;
			}
		}
	}

	fn set_point_light_uniforms(&self, node:&SceneNode, uniform:&LightUniform)
	{
		let user_data = node.user_data::<LightData>();
		if user_data.is_ok()
		{
			let light = user_data.unwrap();

			if uniform.color.is_some()
			{
				let (r, g, b) = light.color.to_rgb();
				let intensity = light.intensity();
				self.context.uniform3f(uniform.color.as_ref(), r * intensity, g * intensity, b * intensity);	
			}

			if uniform.position.is_some()
			{
				self.context.uniform3fv(uniform.position.as_ref(), node.translate().to_tuple());
			}
		}
	}

	fn set_spot_light_uniforms(&self, node:&SceneNode, uniform:&LightUniform)
	{
		let user_data = node.user_data::<LightData>();
		if user_data.is_ok()
		{
			let light = user_data.unwrap();

			if uniform.color.is_some()
			{
				let (r, g, b) = light.color.to_rgb();
				let intensity = light.intensity();
				self.context.uniform3f(uniform.color.as_ref(), r * intensity, g * intensity, b * intensity);
			}

			if uniform.position.is_some()
			{
				self.context.uniform3fv(uniform.position.as_ref(), node.translate().to_tuple());
			}

			if uniform.direction.is_some()
			{
				self.context.uniform3fv(uniform.direction.as_ref(), light.direction().to_tuple());
			}

			if uniform.angle.is_some()
			{
				self.context.uniform1f(uniform.angle.as_ref(), light.cos_angle());
			}
		}
	}

	fn set_directional_light_uniforms(&self, node:&SceneNode, uniform:&LightUniform)
	{
		let user_data = node.user_data::<LightData>();
		if user_data.is_ok()
		{
			let light = user_data.unwrap();

			if uniform.color.is_some()
			{
				let (r, g, b) = light.color.to_rgb();
				let intensity = light.intensity();
				self.context.uniform3f(uniform.color.as_ref(), r * intensity, g * intensity, b * intensity);	
			}

			if uniform.position.is_some()
			{
				self.context.uniform3fv(uniform.position.as_ref(), node.translate().to_tuple());
			}

			if uniform.direction.is_some()
			{
				self.context.uniform3fv(uniform.direction.as_ref(), light.direction().to_tuple());
			}
		}
	}

	fn set_light_uniforms(&self, light:&SceneNode, uniform:&LightUniform, unit:&mut u32)
	{
		let kind = light.user_data::<LightData>().unwrap().kind;
		match kind
		{
			LightType::Sky => { self.set_sky_light_uniforms(light, uniform, unit); },
			LightType::Point => { self.set_point_light_uniforms(light, uniform); },
			LightType::Spot => { self.set_spot_light_uniforms(light, uniform); },
			LightType::Directional => { self.set_directional_light_uniforms(light, uniform); }
		}
	}

	fn set_lights_uniforms(&self, lights:&[SceneNode], material:&Arc<Material>, uniform:&MaterialUniform)
	{
		let mut unit = material.num_texture();

		for i in 0..lights.len()
		{
			self.set_light_uniforms(&lights[i], &uniform.lights[i], &mut unit);
		}
	}

	fn set_render_uniforms(&self, lights:&[SceneNode], camera:&SceneNode, shape:&SceneNode, material:&Arc<Material>)
	{
		let uniform = self.materials.get(material.uuid()).unwrap();

		self.context.use_program(&uniform.program);
		self.set_camera_uniforms(camera, uniform);
		self.set_geometry_uniforms(shape, uniform);
		self.set_material_uniforms(material, uniform);
		self.set_lights_uniforms(lights, material, uniform);
	}

	fn set_viewport(&self, v:&(f32,f32,f32,f32))
	{
		let (x, y) = (self.width() as f32 * v.0, self.height() as f32 * v.1);
		let (w, h) = (self.width() as f32 * v.2, self.height() as f32 * v.3);
		self.context.viewport(x as i32, y as i32, w as i32, h as i32);
	}

	fn clear(&self, clear_color:&Option<(f32,f32,f32,f32)>, clear_depth:&Option<f32>, clear_stencil:&Option<u8>)
	{
		let mut flags = 0;

		if clear_depth.is_some()
		{
			self.context.clear_depth(clear_depth.unwrap());
			flags |= gl::DEPTH_BUFFER_BIT;
		}

		if clear_stencil.is_some()
		{
			self.context.clear_stencil(clear_stencil.unwrap() as _);
			flags |= gl::STENCIL_BUFFER_BIT;
		}

		if clear_color.is_some()
		{
			let (r,g,b,a) = clear_color.unwrap();
			self.context.clear_color(r,g,b,a);
			flags |= gl::COLOR_BUFFER_BIT;
		}

		if flags > 0
		{
			self.context.clear(flags);
		}
	}

	fn set_render_state(&mut self, material:&Arc<Material>)
	{
		let viewport = material.viewport();
		if viewport.is_some()
		{
			self.set_viewport(viewport.unwrap());
		}

		let clear_color = material.clear_color();
		let clear_depth = material.clear_depth();
		let clear_stencil = material.clear_stencil();
		self.clear(clear_color, clear_depth, clear_stencil);

		if material.depth_enable()
		{
			if !self.depth_enable
			{
				self.context.enable(gl::DEPTH_TEST);
				self.depth_enable = true;
			}

			if self.depth_func != material.depth_func()
			{
				match material.depth_func()
				{
					ComparisonFunc::Never => { self.context.depth_func(gl::NEVER); },
					ComparisonFunc::Less => { self.context.depth_func(gl::LESS); },
					ComparisonFunc::Equal => { self.context.depth_func(gl::EQUAL); },
					ComparisonFunc::Lequal => { self.context.depth_func(gl::LEQUAL); },
					ComparisonFunc::Greater => { self.context.depth_func(gl::GREATER); },
					ComparisonFunc::Notequal => {self.context.depth_func(gl::NOTEQUAL);},
					ComparisonFunc::Gequal => {self.context.depth_func(gl::GEQUAL);},
					ComparisonFunc::Always=> {self.context.depth_func(gl::ALWAYS);},
				}

				self.depth_func = material.depth_func();
			}
		}
		else
		{
			if self.depth_enable
			{
				self.context.disable(gl::DEPTH_TEST);
				self.depth_enable = false;
			}			
		}

		let depth_write_enable = material.depth_write_enable();
		if self.depth_write_enable != depth_write_enable
		{
			self.context.depth_mask(depth_write_enable);
			self.depth_write_enable = depth_write_enable;
		}

		if material.blend_enable()
		{
			self.context.enable(gl::BLEND);

			let blend_src;
			match material.blend_src()
			{
				BlendFactor::Zero => { blend_src = gl::ZERO },
				BlendFactor::One => { blend_src = gl::ONE },
				BlendFactor::DstCol => { blend_src = gl::DST_COLOR },
				BlendFactor::SrcColor => { blend_src = gl::SRC_COLOR },
				BlendFactor::SrcAlpha => { blend_src = gl::SRC_ALPHA },
				BlendFactor::DstAlpha => { blend_src = gl::DST_ALPHA },
				BlendFactor::OneMinusSrcCol => { blend_src = gl::ONE_MINUS_SRC_COLOR },
				BlendFactor::OneMinusDstCol => { blend_src = gl::ONE_MINUS_DST_COLOR },
				BlendFactor::OneMinusSrcAlpha => { blend_src = gl::ONE_MINUS_SRC_ALPHA },
				BlendFactor::OneMinusDstAlpha => { blend_src = gl::ONE_MINUS_DST_ALPHA },
				BlendFactor::ConstantColor => { blend_src = gl::CONSTANT_COLOR },
				BlendFactor::ConstantAlpha => { blend_src = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantColor => { blend_src = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantAlpha => { blend_src = gl::CONSTANT_ALPHA },
				BlendFactor::SrcAlphaSaturate => { blend_src = gl::SRC_ALPHA_SATURATE },
			}

			let blend_alpha_src;
			match material.blend_alpha_src()
			{
				BlendFactor::Zero => { blend_alpha_src = gl::ZERO },
				BlendFactor::One => { blend_alpha_src = gl::ONE },
				BlendFactor::DstCol => { blend_alpha_src = gl::DST_COLOR },
				BlendFactor::SrcColor => { blend_alpha_src = gl::SRC_COLOR },
				BlendFactor::SrcAlpha => { blend_alpha_src = gl::SRC_ALPHA },
				BlendFactor::DstAlpha => { blend_alpha_src = gl::DST_ALPHA },
				BlendFactor::OneMinusSrcCol => { blend_alpha_src = gl::ONE_MINUS_SRC_COLOR },
				BlendFactor::OneMinusDstCol => { blend_alpha_src = gl::ONE_MINUS_DST_COLOR },
				BlendFactor::OneMinusSrcAlpha => { blend_alpha_src = gl::ONE_MINUS_SRC_ALPHA },
				BlendFactor::OneMinusDstAlpha => { blend_alpha_src = gl::ONE_MINUS_DST_ALPHA },
				BlendFactor::ConstantColor => { blend_alpha_src = gl::CONSTANT_COLOR },
				BlendFactor::ConstantAlpha => { blend_alpha_src = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantColor => { blend_alpha_src = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantAlpha => { blend_alpha_src = gl::CONSTANT_ALPHA },
				BlendFactor::SrcAlphaSaturate => { blend_alpha_src = gl::SRC_ALPHA_SATURATE },
			}

			let blend_dest;
			match material.blend_dest()
			{
				BlendFactor::Zero => { blend_dest = gl::ZERO },
				BlendFactor::One => { blend_dest = gl::ONE },
				BlendFactor::DstCol => { blend_dest = gl::DST_COLOR },
				BlendFactor::SrcColor => { blend_dest = gl::SRC_COLOR },
				BlendFactor::SrcAlpha => { blend_dest = gl::SRC_ALPHA },
				BlendFactor::DstAlpha => { blend_dest = gl::DST_ALPHA },
				BlendFactor::OneMinusSrcCol => { blend_dest = gl::ONE_MINUS_SRC_COLOR },
				BlendFactor::OneMinusDstCol => { blend_dest = gl::ONE_MINUS_DST_COLOR },
				BlendFactor::OneMinusSrcAlpha => { blend_dest = gl::ONE_MINUS_SRC_ALPHA },
				BlendFactor::OneMinusDstAlpha => { blend_dest = gl::ONE_MINUS_DST_ALPHA },
				BlendFactor::ConstantColor => { blend_dest = gl::CONSTANT_COLOR },
				BlendFactor::ConstantAlpha => { blend_dest = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantColor => { blend_dest = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantAlpha => { blend_dest = gl::CONSTANT_ALPHA },
				BlendFactor::SrcAlphaSaturate => { blend_dest = gl::SRC_ALPHA_SATURATE },
			}

			let blend_alpha_dest;
			match material.blend_alpha_dest()
			{
				BlendFactor::Zero => { blend_alpha_dest = gl::ZERO },
				BlendFactor::One => { blend_alpha_dest = gl::ONE },
				BlendFactor::DstCol => { blend_alpha_dest = gl::DST_COLOR },
				BlendFactor::SrcColor => { blend_alpha_dest = gl::SRC_COLOR },
				BlendFactor::SrcAlpha => { blend_alpha_dest = gl::SRC_ALPHA },
				BlendFactor::DstAlpha => { blend_alpha_dest = gl::DST_ALPHA },
				BlendFactor::OneMinusSrcCol => { blend_alpha_dest = gl::ONE_MINUS_SRC_COLOR },
				BlendFactor::OneMinusDstCol => { blend_alpha_dest = gl::ONE_MINUS_DST_COLOR },
				BlendFactor::OneMinusSrcAlpha => { blend_alpha_dest = gl::ONE_MINUS_SRC_ALPHA },
				BlendFactor::OneMinusDstAlpha => { blend_alpha_dest = gl::ONE_MINUS_DST_ALPHA },
				BlendFactor::ConstantColor => { blend_alpha_dest = gl::CONSTANT_COLOR },
				BlendFactor::ConstantAlpha => { blend_alpha_dest = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantColor => { blend_alpha_dest = gl::CONSTANT_ALPHA },
				BlendFactor::OneMinusConstantAlpha => { blend_alpha_dest = gl::CONSTANT_ALPHA },
				BlendFactor::SrcAlphaSaturate => { blend_alpha_dest = gl::SRC_ALPHA_SATURATE },
			}

			let blend_op;
			match material.blend_op()
			{
				BlendOp::Add => { blend_op = gl::FUNC_ADD; },
				BlendOp::Subtract => { blend_op = gl::FUNC_SUBTRACT; },
				BlendOp::RevSubtract => { blend_op = gl::FUNC_REVERSE_SUBTRACT; }
			}

			let blend_alpha_op;
			match material.blend_alpha_op()
			{
				BlendOp::Add => { blend_alpha_op = gl::FUNC_ADD; },
				BlendOp::Subtract => { blend_alpha_op = gl::FUNC_SUBTRACT; },
				BlendOp::RevSubtract => { blend_alpha_op = gl::FUNC_REVERSE_SUBTRACT; }
			}

			self.context.blend_equation_separate(blend_op, blend_alpha_op);
			self.context.blend_func_separate(blend_src, blend_dest, blend_alpha_src, blend_alpha_dest);
		}
		else
		{
			self.context.disable(gl::BLEND);
		}

		let cull_mode = material.cull_mode();
		if self.cull_mode != cull_mode
		{
			match material.cull_mode()
			{
				CullMode::None =>
				{ 
					self.context.disable(gl::CULL_FACE);
				},
				CullMode::Front =>
				{ 
					self.context.enable(gl::CULL_FACE);
					self.context.cull_face(gl::FRONT);
				},
				CullMode::Back =>
				{ 
					self.context.enable(gl::CULL_FACE);
					self.context.cull_face(gl::BACK);
				},
				CullMode::FrontBack =>
				{ 
					self.context.enable(gl::CULL_FACE);
					self.context.cull_face(gl::FRONT_AND_BACK);
				},
			}

			self.cull_mode = cull_mode;
		}

		let front_face = material.front_face();
		if self.front_face != front_face
		{
			match material.front_face()
			{
				FrontFace::CW => 
				{
					self.context.front_face(gl::CW);
				},
				FrontFace::CCW => 
				{
					self.context.front_face(gl::CCW);
				},
			}

			self.front_face = front_face;
		}

		let polygon_mode = material.polygon_mode();
		if self.polygon_mode != polygon_mode
		{
			match material.polygon_mode()
			{
				PolygonMode::Point => 
				{
				},
				PolygonMode::Wireframe => 
				{
					self.context.line_width(material.line_width());
				},
				PolygonMode::Solid => 
				{
				}
			}

			self.polygon_mode = polygon_mode;
		}
	}
}

impl Canvas for ForwardRenderer
{
	fn width(&self) -> u32
	{
		return self.w;
	}

	fn height(&self) -> u32
	{
		return self.h;
	}

	fn render(&mut self, scene:&Scene)
	{
		self.init_shapes(scene.shapes(), scene.lights());
		self.init_lights(scene.lights());

		for camera in scene.cameras() 
		{
			self.set_render_state(&camera.material().unwrap());

			for shape in scene.shapes()
			{
				let geometry = shape.geometry().unwrap();
				let material = shape.material().unwrap();

				self.set_render_uniforms(scene.lights(), &camera, &shape, &material);
				self.set_render_state(&material);

				let buffer = self.shapes.get(geometry.uuid()).unwrap();
				self.set_draw_buffer(&buffer.vertex_buffer, &buffer.index_buffer);
				self.set_input_layout(material.input_layout());

				self.context.draw_elements(gl::TRIANGLES, buffer.count, gl::UNSIGNED_SHORT, 0);
			}
		}
	}
}

impl Drop for ForwardRenderer
{
	fn drop(&mut self)
	{
		/*self.context.delete_texture(&self.texture);

		for (_, material) in self.materials.iter()
		{
			self.context.delete_program(&material.program);
		}

		for (_, shape) in self.shapes.iter()
		{
			self.context.delete_buffer(&shape.vertex_buffer);
			self.context.delete_buffer(&shape.index_buffer);
		}

		for (_, texture) in self.textures.iter()
		{
			self.context.delete_texture(&texture.texture);
		}*/
	}
}