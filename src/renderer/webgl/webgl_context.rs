use gl;
use std::ptr;
use std::ffi::CString;
use std::ops::Deref;
use std::os::raw::c_void;
use super::webgl_common::*;

const NAME_SIZE: usize = 64;

pub fn check_gl_error(msg: &str) -> Option<String>
{
    unsafe 
    {
        let err = gl::GetError();
        if err != 0
        {
            let error = format!("GLError: {} {}", msg, err);
            println!("{:?}", error);
            Some(error)
        }
        else
        {
            None
        }
    }
}

impl GLContext
{
    pub fn new<F>(loadfn: F) -> GLContext where F: FnMut(&str) -> *const c_void
    {
        gl::load_with(loadfn);

        GLContext
        {
            reference: 0
        }
    }

    pub fn active_texture(&self, texture: GLenum)
    {
        unsafe 
        {
            gl::ActiveTexture(texture as _);
        }

        check_gl_error("active_texture");
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader)
    {
        unsafe 
        {
            gl::AttachShader(program.0, shader.0);
        }

        check_gl_error("attach_shader");
    }

    pub fn bind_attrib_location(program:&WebGLProgram, index:GLuint, name:&str)
    {
        let src = CString::new(name).unwrap();
        unsafe 
        {
            gl::BindAttribLocation(program.0, index, src.as_ptr());
        }

        check_gl_error("attach_shader");
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: &WebGLBuffer)
    {
        unsafe 
        {
            gl::BindBuffer(target as _, buffer.0);
        }

        check_gl_error("bind_buffer");
    }

    pub fn bind_vertex_array(&self, vao: Option<&WebGLVertexArray>)
    {
        unsafe
        {
            gl::BindVertexArray(vao.unwrap().0);
        }

        check_gl_error("bind_vertex_array");
    }

    pub fn bind_framebuffer(target:GLenum, framebuffer:GLuint)
    {
        unsafe
        {
            gl::BindFramebuffer(target as _, framebuffer);
        }

        check_gl_error("bind_framebuffer");
    }

    pub fn bind_texture(&self, target:GLenum, texture: &WebGLTexture)
    {
        unsafe
        {
            gl::BindTexture(target as _, texture.0);
        }

        check_gl_error("bind_texture");
    }

    pub fn blend_color(&self, red:GLfloat, green:GLfloat, blue:GLfloat, alpha:GLfloat)
    {
        unsafe
        {
            gl::BlendColor(red, green, blue, alpha);
        }

        check_gl_error("blend_color");
    }

    pub fn blend_equation(&self, mode:GLenum)
    {
        unsafe
        {
            gl::BlendEquation(mode);
        }

        check_gl_error("blend_equation");
    }

    pub fn blend_equation_separate(&self, mode_rgb:GLenum, mode_alpha:GLenum)
    {
        unsafe
        {
            gl::BlendEquationSeparate(mode_rgb, mode_alpha);
        }

        check_gl_error("blend_equation_separate");
    }

    pub fn blend_func(&self, sfactor:GLenum, dfactor:GLenum)
    {
        unsafe
        {
            gl::BlendFunc(sfactor, dfactor);
        }

        check_gl_error("blend_func");
    }

    pub fn blend_func_separate(&self, sfactor_rgb:GLenum, dfactor_rgb:GLenum, sfactor_alpha:GLenum, dfactor_alpha:GLenum)
    {
        unsafe
        {
            gl::BlendFuncSeparate(sfactor_rgb, dfactor_rgb, sfactor_alpha, dfactor_alpha);
        }

        check_gl_error("blend_func_separate");
    }

    pub fn buffer_data<T>(&self, target: GLenum, data: &[T], usage: GLenum)
    {
        unsafe
        {
            gl::BufferData(target as _, (std::mem::size_of::<T>() * data.len()) as _, data.as_ptr() as _, usage as _);
        }

        check_gl_error("buffer_data");
    }

    pub fn buffer_sub_data<T>(&self, target: GLenum, offset: GLuint, data: &[T])
    {
        unsafe
        {
            gl::BufferSubData(target as _, offset as _, data.len() as _, data.as_ptr() as _);
        }

        check_gl_error("buffer_data");
    }

    pub fn clear(&self, bit: GLbitfield)
    {
        unsafe
        {
            gl::Clear(bit as _);
        }

        check_gl_error("clear");
    }

    pub fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat)
    {
        unsafe
        {
            gl::ClearColor(r, g, b, a);
        }

        check_gl_error("clear_color");
    }

    pub fn clear_depth(&self, value: GLfloat)
    {
        unsafe
        {
            gl::ClearDepth(value as _);
        }

        check_gl_error("clear_depth");
    }

    pub fn clear_stencil(&self, value: GLint)
    {
        unsafe
        {
            gl::ClearStencil(value as _);
        }

        check_gl_error("clear_stencil");
    }

    pub fn color_mask(red:GLboolean, green:GLboolean, blue:GLboolean, alpha:GLboolean)
    {
        unsafe
        {
            gl::ColorMask(red as _, green as _, blue as _, alpha as _);
        }

        check_gl_error("color_mask");
    }

    pub fn compile_shader(&self, shader: &WebGLShader)
    {
        unsafe
        {
            gl::CompileShader(shader.0);
        }
        check_gl_error("compile_shader");
    }

    pub fn create_buffer(&self) -> Result<WebGLBuffer, &str>
    {
        let mut buffer:GLuint = 0;
        unsafe
        {
            gl::GenBuffers(1, &mut buffer);
        }

        check_gl_error("create_buffer");

        match buffer 
        {
            0 => Err("create_buffer() fail"),
            _ => Ok(WebGLBuffer(buffer))
        }
    }

    pub fn create_shader(&self, target: GLenum) -> Result<WebGLShader, &str>
    {
        let shader:GLuint;
        unsafe
        {
            shader = gl::CreateShader(target as _);
        }

        check_gl_error("create_shader");

        match shader 
        {
            0 => Err("create_shader() fail"),
            _ => Ok(WebGLShader(shader))
        }
    }

    pub fn create_program(&self) -> Result<WebGLProgram, &str>
    {
        let program:GLuint;
        unsafe
        {
            program = gl::CreateProgram();
        }

        check_gl_error("create_program");

        match program 
        {
            0 => Err("create_program() fail"),
            _ => Ok(WebGLProgram(program))
        }
    }

    pub fn create_sampler(&self, ) -> Result<WebGLSampler, &str>
    {
        let mut sampler:GLuint = 0;
        unsafe
        {
            gl::CreateSamplers(1, &mut sampler);
        }

        check_gl_error("create_sampler");

        match sampler 
        {
            0 => Err("create_sampler() fail"),
            _ => Ok(WebGLSampler(sampler))
        }
    }

    pub fn create_texture(&self) -> Result<WebGLTexture, &str>
    {
        let mut texture:GLuint = 0;
        unsafe
        {
            gl::GenTextures(1, &mut texture);
        }

        check_gl_error("create_texture");

        match texture 
        {
            0 => Err("create_texture() fail"),
            _ => Ok(WebGLTexture(texture))
        }
    }

    pub fn create_vertex_array(&self) -> Result<WebGLVertexArray, &str>
    {
        let mut vao:GLuint = 0;
        unsafe
        {
            gl::GenVertexArrays(1, &mut vao);
        }

        check_gl_error("create_vertex_array");

        match vao 
        {
            0 => Err("create_vao() fail"),
            _ => Ok(WebGLVertexArray(vao))
        }
    }

    pub fn create_framebuffer(&self, ) -> Result<WebGLFramebuffer, &str>
    {
        let mut framebuffer:GLuint = 0;
        unsafe
        {
            gl::CreateFramebuffers(1, &mut framebuffer);
        }
        check_gl_error("create_framebuffer");

        match framebuffer 
        {
            0 => Err("create_framebuffer() fail"),
            _ => Ok(WebGLFramebuffer(framebuffer))
        }
    }

    pub fn create_renderbuffer(&self, ) -> Result<WebGLRenderbuffer, &str>
    {
        let mut renderbuffer:GLuint = 0;
        unsafe
        {
            gl::CreateRenderbuffers(1, &mut renderbuffer);
        }
        check_gl_error("create_renderbuffer");

        match renderbuffer 
        {
            0 => Err("create_renderbuffer() fail"),
            _ => Ok(WebGLRenderbuffer(renderbuffer))
        }
    }

    pub fn cull_face(&self, mode:GLenum)
    {
        unsafe
        {
            gl::CullFace(mode);
        }

        check_gl_error("cull_face");
    }

    pub fn compressed_tex_image2d(&self, target: GLenum, level: u8, compression: GLenum, width: u16, height: u16, data: &[u8])
    {
        unsafe {
            gl::CompressedTexImage2D(
                target as _,
                level as _,
                compression as _,
                width as _,
                height as _,
                0,
                (data.len() - 128) as _, //gl::UNSIGNED_BYTE as _,
                &data[128] as *const u8 as _,
            );
        }
        check_gl_error("compressed_tex_image2d");
    }

    pub fn delete_buffer(&self, buffer: &WebGLBuffer)
    {
        unsafe
        {
            gl::DeleteBuffers(1, buffer.0 as _);
        }
        check_gl_error("delete_buffer");
    }

    pub fn delete_framebuffer(&self, fbo: GLuint)
    {
        unsafe
        {
            gl::DeleteFramebuffers(1, fbo as _);
        }
        check_gl_error("delete_framebuffer");
    }

    pub fn delete_program(&self, program: &WebGLProgram)
    {
        unsafe
        {
            gl::DeleteProgram(program.0 as _);
        }
        check_gl_error("delete_program");
    }

    pub fn delete_shader(&self, shader: &WebGLShader)
    {
        unsafe
        {
            gl::DeleteShader(shader.0 as _);
        }
        check_gl_error("delete_shader");
    }

    pub fn delete_texture(&self, texture: &WebGLTexture)
    {
        unsafe
        {
            gl::DeleteTextures(1, texture.0 as _);
        }
        check_gl_error("delete_texture");
    }

    pub fn depth_func(&self, func:GLenum)
    {
        unsafe
        {
            gl::DepthFunc(func as _);
        }

        check_gl_error("depth_func");
    }

    pub fn depth_mask(&self, flag:GLboolean)
    {
        unsafe
        {
            gl::DepthMask(flag as _);
        }

        check_gl_error("depth_mask");
    }

    pub fn depth_rangef(&self, n:GLfloat, f:GLfloat)
    {
        unsafe
        {
            gl::DepthRangef(n as _, f as _);
        }

        check_gl_error("depth_rangef");
    }

    pub fn detach_shader(&self, program:&WebGLProgram, shader:&WebGLShader)
    {
        unsafe
        {
            gl::DetachShader(program.0 as _ , shader.0 as _);
        }

        check_gl_error("detach_shader");
    }

    pub fn disable(&self, flag: GLenum)
    {
        unsafe
        {
            gl::Disable(flag as _);
        }

        check_gl_error("disable");
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, kind: GLenum, offset: GLuint)
    {
        unsafe
        {
            gl::DrawElements(mode as _, count as _, kind as _, offset as _);
        };

        check_gl_error("draw_elements");
    }

    pub fn draw_arrays(&self, mode: GLenum, count: GLsizei)
    {
        unsafe
        {
            gl::DrawArrays(mode as _, 0, count as _);
        };

        check_gl_error("draw_arrays");
    }

    pub fn framebuffer_texture2_d(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: Option<&WebGLTexture>, level: GLint)
    {
        unsafe
        {
            gl::FramebufferTexture2D(target, attachment, textarget, texture.unwrap().0, level);
        }
        check_gl_error("framebuffer_texture2_d");
    }

    pub fn front_face(&self, mode: GLenum)
    {
        unsafe
        {
            gl::FrontFace(mode as _);
        };

        check_gl_error("front_face");
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str)
    {
        let src = CString::new(source).unwrap();
        unsafe
        {
            gl::ShaderSource(shader.0, 1, &src.as_ptr(), ptr::null());
        }
        check_gl_error("shader_source");
    }

    pub fn line_width(&self, width: GLfloat)
    {
        unsafe 
        {
            gl::LineWidth(width);
        }
        check_gl_error("line_width");
    }

    pub fn link_program(&self, program: &WebGLProgram)
    {
        unsafe 
        {
            gl::LinkProgram(program.0);
        }
        check_gl_error("link_program");
    }

    pub fn validate_program(&self, program: &WebGLProgram)
    {
        unsafe 
        {
            gl::ValidateProgram(program.0)
        }
        check_gl_error("validate_program");
    }

    pub fn use_program(&self, program: &WebGLProgram)
    {
        unsafe
        {
            gl::UseProgram(program.0 as _);
        }

        check_gl_error("use_program");
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> GLint
    {
        let mut res = 0;
        unsafe
        {
            gl::GetProgramiv(program.0, pname as _, &mut res);
        }
        check_gl_error("get_program_parameter");
        res
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Result<(), String>
    {
        if !(self.get_program_parameter(program, gl::LINK_STATUS) > 0)
        {
            let mut length = self.get_program_parameter(program, gl::INFO_LOG_LENGTH);
            let mut name: Vec<u8> = Vec::with_capacity(length as usize);

            unsafe
            {
                gl::GetProgramInfoLog(
                    program.0,
                    length as _,
                    &mut length,
                    name.as_mut_ptr() as _
                );
            }
            check_gl_error("get_program_info_log");

            Err(String::from_utf8(name).unwrap())
        }
        else
        {
            Ok(())
        }
    }

    pub fn get_active_uniform(&self, program: Option<&WebGLProgram>, location: GLuint) -> WebGLActiveInfo 
    {
        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe
        {
            gl::GetActiveUniform(
                program.unwrap().0,
                location as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        };
        check_gl_error("get_active_uniform");

        WebGLActiveInfo::new(String::from_utf8(name).unwrap(), size as _, kind, 0)
    }

    pub fn get_active_attrib(&self, program: Option<&WebGLProgram>, location: GLuint) -> WebGLActiveInfo
    {
        let mut name: Vec<u8> = Vec::with_capacity(NAME_SIZE);
        let mut size = 0i32;
        let mut len = 0i32;
        let mut kind = 0u32;

        unsafe
        {
            gl::GetActiveAttrib(
                program.unwrap().0,
                location as _,
                NAME_SIZE as _,
                &mut len,
                &mut size,
                &mut kind,
                name.as_mut_ptr() as _,
            );
            name.set_len(len as _);
        }
        println!("name {:?}", name);
        check_gl_error("get_active_attrib");

        WebGLActiveInfo::new(String::from_utf8(name).expect("utf8 parse failed"), size as _, kind, 0)
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> i32
    {
        let c_name = CString::new(name).unwrap();
        unsafe
        {
            let location = gl::GetAttribLocation(program.0 as _, c_name.as_ptr());
            check_gl_error("get_attrib_location");
            if location == -1 
            {
                return 0;
            }

            return location;
        }
    }

    pub fn get_uniform_location(&self, program: &WebGLProgram, name: &str) -> Option<WebGLUniformLocation> 
    {
        let c_name = CString::new(name).unwrap();
        unsafe
        {
            let location = gl::GetUniformLocation(program.0 as _, c_name.as_ptr());
            check_gl_error("get_uniform_location");
            if location == -1 {
                return None;
            }
            return Some(WebGLUniformLocation {
                reference: location as _,
                name: name.into(),
            });
        }
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> GLint
    {
        let mut res = 0;
        unsafe
        {
            gl::GetShaderiv(shader.0, pname as _, &mut res);
        }
        check_gl_error("get_shader_parameter");
        res
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Result<(), String>
    {
        if !(self.get_shader_parameter(shader, gl::COMPILE_STATUS) > 0)
        {
            let mut length = self.get_shader_parameter(shader, gl::INFO_LOG_LENGTH);
            let mut name: Vec<u8> = Vec::with_capacity(length as usize);

            unsafe
            {
                gl::GetShaderInfoLog(
                    shader.0,
                    length as _,
                    &mut length,
                    name.as_mut_ptr() as _
                );
            }
            check_gl_error("get_shader_info_log");

            Err(String::from_utf8(name).unwrap())
        }
        else
        {
            Ok(())
        }
    }

    pub fn vertex_attrib_pointer(&self, location: GLuint, size: GLint, kind: GLenum, normalized: GLboolean, stride: GLsizei, offset: GLintptr)
    {
        unsafe
        {
            gl::VertexAttribPointer(location as _, size as _, kind as _, normalized as _, stride as _, offset as _);
        }

        check_gl_error("vertex_attrib_pointer");
    }

    pub fn enable(&self, flag: GLenum)
    {
        unsafe
        {
            gl::Enable(flag as _);
        }

        check_gl_error("enable");
    }

    pub fn enable_vertex_attrib_array(&self, location: GLuint)
    {
        unsafe
        {
            gl::EnableVertexAttribArray(location as _);
        }
        check_gl_error("enable_vertex_attrib_array");
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLint, height: GLint)
    {
        unsafe
        {
            gl::Viewport(x, y, width as _, height as _);
        };

        check_gl_error("viewport");
    }

    pub fn pixel_storei(&self, storage: GLenum, value: GLint)
    {
        unsafe
        {
            gl::PixelStorei(storage as _, value);
        }

        check_gl_error("pixel_storei");
    }

    pub fn polygon_mode(&self, face: GLenum, mode: GLenum)
    {
        unsafe
        {
            gl::PolygonMode(face as _, mode);
        }

        check_gl_error("polygon_mode");
    }

    pub fn tex_image2d(&self, target:GLenum, level:GLint, internalformat:GLenum, width:GLsizei, height:GLsizei, border:GLint, format:GLenum, kind:GLenum, pixels:&[u8])
    {
        unsafe
        {
            gl::TexImage2D(target, level, internalformat as _, width, height, border, format, kind, pixels.as_ptr() as _);
        }

        check_gl_error("tex_image2d");
    }

    pub fn tex_sub_image2d(&self, target:GLenum, level:GLint, xoffset:GLsizei, yoffset:GLsizei, width:GLsizei, height:GLsizei, format:GLenum, kind:GLenum, pixels:&[u8])
    {
        unsafe
        {
            gl::TexSubImage2D(target, level, xoffset, yoffset, width, height, format, kind, pixels.as_ptr() as _)
        }
        check_gl_error("tex_sub_image2d");
    }

    pub fn uniform_matrix4fv(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: &[f32])
    {
        unsafe
        {
            gl::UniformMatrix4fv(*location.unwrap().deref() as i32, 1, transpose as _, value.as_ptr());
        }
        check_gl_error("uniform_matrix_4fv");
    }

    pub fn uniform_matrix4fv_1(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: *const GLfloat)
    {
        unsafe
        {
            gl::UniformMatrix4fv(*location.unwrap().deref() as i32, 1, transpose as _, value as _);
        }
        check_gl_error("uniform_matrix_4fv");
    }

    pub fn uniform_matrix3fv(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: &[f32])
    {
        unsafe
        {
            gl::UniformMatrix3fv(*location.unwrap().deref() as i32, 1, transpose as _, value.as_ptr());
        }
        check_gl_error("uniform_matrix_3fv");
    }

    pub fn uniform_matrix3fv_1(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: *const GLfloat)
    {
        unsafe
        {
            gl::UniformMatrix3fv(*location.unwrap().deref() as i32, 1, transpose as _, value as _);
        }
        check_gl_error("uniform_matrix_4fv");
    }

    pub fn uniform_matrix2fv(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: &[f32])
    {
        unsafe
        {
            gl::UniformMatrix2fv(*location.unwrap().deref() as i32, 1, transpose as _, value.as_ptr());
        }
        check_gl_error("uniform_matrix_2fv");
    }

    pub fn uniform_matrix2fv_1(&self, location: Option<&WebGLUniformLocation>, transpose:GLboolean, value: *const GLfloat)
    {
        unsafe
        {
            gl::UniformMatrix2fv(*location.unwrap().deref() as i32, 1, transpose as _, value as _);
        }
        check_gl_error("uniform_matrix_2fv");
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, value: u32)
    {
        unsafe
        {
            gl::Uniform1ui(*location.unwrap().deref() as i32, value as _);
        }
        check_gl_error("uniform1ui");
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, x:u32, y:u32)
    {
        unsafe
        {
            gl::Uniform2ui(*location.unwrap().deref() as i32, x, y);
        }
        check_gl_error("uniform2ui");
    }

    pub fn uniform3ui(&self, location: Option<&WebGLUniformLocation>, x:u32, y:u32, z:u32)
    {
        unsafe
        {
            gl::Uniform3ui(*location.unwrap().deref() as i32, x, y, z);
        }
        check_gl_error("uniform3ui");
    }

    pub fn uniform4ui(&self, location: Option<&WebGLUniformLocation>, x:u32, y:u32, z:u32, w:u32)
    {
        unsafe
        {
            gl::Uniform4ui(*location.unwrap().deref() as i32, x, y, z, w);
        }
        check_gl_error("uniform4ui");
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x:i32)
    {
        unsafe
        {
            gl::Uniform1i(*location.unwrap().deref() as i32, x);
        }
        check_gl_error("uniform1i");
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x:i32, y:i32)
    {
        unsafe
        {
            gl::Uniform2i(*location.unwrap().deref() as i32, x, y);
        }
        check_gl_error("uniform2i");
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x:i32, y:i32, z:i32)
    {
        unsafe
        {
            gl::Uniform3i(*location.unwrap().deref() as i32, x, y, z);
        }
        check_gl_error("uniform3i");
    }

    pub fn uniform4i(&self, location: Option<&WebGLUniformLocation>, x:i32, y:i32, z:i32, w:i32)
    {
        unsafe
        {
            gl::Uniform4i(*location.unwrap().deref() as i32, x, y, z, w);
        }
        check_gl_error("uniform4i");
    }

    pub fn uniform2iv(&self, location: Option<&WebGLUniformLocation>, value: (i32, i32))
    {
        unsafe
        {
            gl::Uniform2i(*location.unwrap().deref() as i32, value.0, value.1);
        }
        check_gl_error("uniform2iv");
    }

    pub fn uniform3iv(&self, location: Option<&WebGLUniformLocation>, value: (i32, i32, i32))
    {
        unsafe
        {
            gl::Uniform3i(*location.unwrap().deref() as i32, value.0, value.1, value.2);
        }
        check_gl_error("uniform3iv");
    }

    pub fn uniform4iv(&self, location: Option<&WebGLUniformLocation>, value: (i32, i32, i32, i32))
    {
        unsafe
        {
            gl::Uniform4i(*location.unwrap().deref() as i32, value.0, value.1, value.2, value.3);
        }
        check_gl_error("uniform4iv");
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, value: f32)
    {
        unsafe
        {
            gl::Uniform1f(*location.unwrap().deref() as i32, value as _);
        }
        check_gl_error("uniform1f");
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: f32, y: f32)
    {
        unsafe
        {
            gl::Uniform2f(*location.unwrap().deref() as i32, x as _, y as _);
        }
        check_gl_error("uniform2f");
    }

    pub fn uniform2fv(&self, location: Option<&WebGLUniformLocation>, value: (f32, f32))
    {
        unsafe
        {
            gl::Uniform2f(*location.unwrap().deref() as i32, value.0, value.1);
        }
        check_gl_error("uniform3f");
    }

    pub fn uniform3f(&self, location: Option<&WebGLUniformLocation>, x: f32, y: f32, z: f32)
    {
        unsafe
        {
            gl::Uniform3f(*location.unwrap().deref() as i32, x as _, y as _, z as _);
        }
        check_gl_error("uniform3f");
    }

    pub fn uniform3fv(&self, location: Option<&WebGLUniformLocation>, value: (f32, f32, f32))
    {
        unsafe
        {
            gl::Uniform3f(*location.unwrap().deref() as i32, value.0, value.1, value.2);
        }
        check_gl_error("uniform3f");
    }

    pub fn uniform4f(&self, location: Option<&WebGLUniformLocation>, x: f32, y: f32, z: f32, w: f32)
    {
        unsafe
        {
            gl::Uniform4f(*location.unwrap().deref() as i32, x as _, y as _, z as _, w as _);
        }
        check_gl_error("uniform4fv");
    }

    pub fn uniform4fv(&self, location: Option<&WebGLUniformLocation>, value: (f32, f32, f32, f32))
    {
        unsafe
        {
            gl::Uniform4f(*location.unwrap().deref() as i32, value.0, value.1, value.2, value.3);
        }
        check_gl_error("uniform4f");
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: i32)
    {
        unsafe
        {
            gl::TexParameteri(target, pname as _, param);
        }
        check_gl_error("tex_parameteri");
    }

    pub fn tex_parameterfv(&self, target: GLenum, pname: GLenum, param: f32)
    {
        unsafe
        {
            gl::TexParameterfv(target, pname as _, &param);
        }
        check_gl_error("tex_parameterfv");
    }

    pub fn draw_buffers(&self, buffers: &[GLenum])
    {
        unsafe
        {
            gl::DrawBuffers(buffers.len() as _, buffers.as_ptr());
        }
        check_gl_error("draw_buffers");
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>)
    {
        unsafe
        {
            gl::BindRenderbuffer(target, renderbuffer.unwrap().0);
        }
        check_gl_error("bind_renderbuffer");
    }

    pub fn renderbuffer_storage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei)
    {
        unsafe
        {
            gl::RenderbufferStorage(target, internalformat, width, height);
        }
        check_gl_error("bind_renderbuffer");
    }

    pub fn framebuffer_renderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: Option<&WebGLRenderbuffer>)
    {
        unsafe
        {
            gl::FramebufferRenderbuffer(target, attachment, renderbuffertarget, renderbuffer.unwrap().0);
        }
        check_gl_error("framebuffer_renderbuffer");
    }
}