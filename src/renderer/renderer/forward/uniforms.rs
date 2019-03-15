use super::super::super::webgl::{ WebGLTexture, WebGLBuffer, WebGLProgram, WebGLUniformLocation };

pub struct LightUniform
{
	pub color: Option<WebGLUniformLocation>,
	pub position: Option<WebGLUniformLocation>,
	pub direction: Option<WebGLUniformLocation>,
	pub radiance: Option<WebGLUniformLocation>,
	pub irradiance: Option<WebGLUniformLocation>,
	pub angle: Option<WebGLUniformLocation>,
}

pub struct MaterialUniform
{
	pub program: WebGLProgram,
	pub model: Option<WebGLUniformLocation>,
	pub view: Option<WebGLUniformLocation>,
	pub viewproject: Option<WebGLUniformLocation>,
	pub eye_position: Option<WebGLUniformLocation>,
	pub exposure:  Option<WebGLUniformLocation>,
	pub lights:Vec<LightUniform>,
	pub locations: Vec<Option<WebGLUniformLocation>>,
}

pub struct TextureUniform 
{
	pub texture: WebGLTexture,
}

pub struct GeometryUniform 
{
	pub vertex_buffer: WebGLBuffer,
	pub index_buffer: WebGLBuffer,
	pub count:i32,
}

impl TextureUniform
{
	pub fn new(texture:WebGLTexture) -> Self
	{
		Self
		{
			texture: texture,
		}
	}
}

impl GeometryUniform
{
	pub fn new(vertex_buffer:WebGLBuffer, index_buffer:WebGLBuffer, count:i32) -> Self
	{
		Self
		{
			vertex_buffer: vertex_buffer,
			index_buffer: index_buffer,
			count:count
		}
	}
}