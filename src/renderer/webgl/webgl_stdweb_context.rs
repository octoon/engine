extern crate stdweb;

use self::stdweb::{Reference, Value, JsSerialize, InstanceOf};
use self::stdweb::unstable::{TryInto};
use self::stdweb::web::{ArrayBuffer};
use self::stdweb::web::html_element::CanvasElement;

use super::webgl_stdweb_common::*;

impl GLContext 
{   
    pub fn active_texture(&self, texture: GLenum) {
        js!( @{self}.activeTexture(@{texture}); );
    }

    pub fn attach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.attachShader(@{program}, @{shader}); );
    }

    pub fn begin_query(&self, target: GLenum, query: &WebGLQuery) {
        js!( @{self}.beginQuery(@{target}, @{query}); );
    }

    pub fn begin_transform_feedback(&self, primitive_mode: GLenum) {
        js!( @{self}.beginTransformFeedback(@{primitive_mode}); );
    }

    pub fn bind_attrib_location(&self, program: &WebGLProgram, index: GLuint, name: &str) {
        js!( @{self}.bindAttribLocation(@{program}, @{index}, @{name}); );
    }

    pub fn bind_buffer(&self, target: GLenum, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBuffer(@{target}, @{buffer}); );
    }

    pub fn bind_buffer_base(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>) {
        js!( @{self}.bindBufferBase(@{target}, @{index}, @{buffer}); );
    }

    pub fn bind_buffer_range(&self, target: GLenum, index: GLuint, buffer: Option<&WebGLBuffer>, offset: GLintptr, size: GLsizeiptr) {
        js!( @{self}.bindBufferRange(@{target}, @{index}, @{buffer}, @{offset as f64}, @{size as f64}); );
    }

    pub fn bind_framebuffer(&self, target: GLenum, framebuffer: Option<&WebGLFramebuffer>) {
        js!( @{self}.bindFramebuffer(@{target}, @{framebuffer}); );
    }

    pub fn bind_renderbuffer(&self, target: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.bindRenderbuffer(@{target}, @{renderbuffer}); );
    }

    pub fn bind_sampler(&self, unit: GLuint, sampler: Option<&WebGLSampler>) {
        js!( @{self}.bindSampler(@{unit}, @{sampler}); );
    }

    pub fn bind_texture(&self, target: GLenum, texture: Option<&WebGLTexture>) {
        js!( @{self}.bindTexture(@{target}, @{texture}); );
    }

    pub fn bind_transform_feedback(&self, target: GLenum, tf: Option<&WebGLTransformFeedback>) {
        js!( @{self}.bindTransformFeedback(@{target}, @{tf}); );
    }

    pub fn bind_vertex_array(&self, array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.bindVertexArray(@{array}); );
    }

    pub fn blend_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.blendColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn blend_equation(&self, mode: GLenum) {
        js!( @{self}.blendEquation(@{mode}); );
    }

    pub fn blend_equation_separate(&self, mode_rgb: GLenum, mode_alpha: GLenum) {
        js!( @{self}.blendEquationSeparate(@{mode_rgb}, @{mode_alpha}); );
    }

    pub fn blend_func(&self, sfactor: GLenum, dfactor: GLenum) {
        js!( @{self}.blendFunc(@{sfactor}, @{dfactor}); );
    }

    pub fn blend_func_separate(&self, src_rgb: GLenum, dst_rgb: GLenum, src_alpha: GLenum, dst_alpha: GLenum) {
        js!( @{self}.blendFuncSeparate(@{src_rgb}, @{dst_rgb}, @{src_alpha}, @{dst_alpha}); );
    }

    pub fn blit_framebuffer(&self, src_x0: GLint, src_y0: GLint, src_x1: GLint, src_y1: GLint, dst_x0: GLint, dst_y0: GLint, dst_x1: GLint, dst_y1: GLint, mask: GLbitfield, filter: GLenum) {
        js!( @{self}.blitFramebuffer(@{src_x0}, @{src_y0}, @{src_x1}, @{src_y1}, @{dst_x0}, @{dst_y0}, @{dst_x1}, @{dst_y1}, @{mask}, @{filter}); );
    }

    pub fn buffer_data(&self, target: GLenum, size: GLsizeiptr, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{size as f64}, @{usage}); );
    }

    pub fn buffer_data_1(&self, target: GLenum, src_data: Option<&ArrayBuffer>, usage: GLenum) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}); );
    }

    pub fn buffer_data_2(&self, target: GLenum, src_data: &ArrayBuffer, usage: GLenum, src_offset: GLuint, length: GLuint) {
        js!( @{self}.bufferData(@{target}, @{src_data}, @{usage}, @{src_offset}, @{length}); );
    }

    pub fn buffer_sub_data(&self, target: GLenum, dst_byte_offset: GLintptr, src_data: &ArrayBuffer) {
        js!( @{self}.bufferSubData(@{target}, @{dst_byte_offset as f64}, @{src_data}); );
    }

    pub fn buffer_sub_data_1(&self, target: GLenum, dst_byte_offset: GLintptr, src_data: &ArrayBuffer, src_offset: GLuint, length: GLuint) {
        js!( @{self}.bufferSubData(@{target}, @{dst_byte_offset as f64}, @{src_data}, @{src_offset}, @{length}); );
    }

    pub fn canvas(&self) -> CanvasElement {
        (js! { return @{self}.canvas; } ).try_into().unwrap()
    }

    pub fn check_framebuffer_status(&self, target: GLenum) -> GLenum {
        (js! { return @{self}.checkFramebufferStatus(@{target}); } ).try_into().unwrap()
    }

    pub fn clear(&self, mask: GLbitfield) {
        js!( @{self}.clear(@{mask}); );
    }

    pub fn clear_bufferfi(&self, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint) {
        js!( @{self}.clearBufferfi(@{buffer}, @{drawbuffer}, @{depth}, @{stencil}); );
    }

    pub fn clear_bufferfv<'a0, T0>(&self, buffer: GLenum, drawbuffer: GLint, values: T0, src_offset: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.clearBufferfv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferiv<'a0, T0>(&self, buffer: GLenum, drawbuffer: GLint, values: T0, src_offset: GLuint) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.clearBufferiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_bufferuiv<'a0, T0>(&self, buffer: GLenum, drawbuffer: GLint, values: T0, src_offset: GLuint) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.clearBufferuiv(@{buffer}, @{drawbuffer}, @{unsafe { values.as_typed_array() }}, @{src_offset}); );
    }

    pub fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        js!( @{self}.clearColor(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn clear_depth(&self, depth: GLclampf) {
        js!( @{self}.clearDepth(@{depth}); );
    }

    pub fn clear_stencil(&self, s: GLint) {
        js!( @{self}.clearStencil(@{s}); );
    }

    pub fn client_wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLuint64) -> GLenum {
        (js! { return @{self}.clientWaitSync(@{sync}, @{flags}, @{timeout as f64}); } ).try_into().unwrap()
    }

    pub fn color_mask(&self, red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) {
        js!( @{self}.colorMask(@{red}, @{green}, @{blue}, @{alpha}); );
    }

    pub fn compile_shader(&self, shader: &WebGLShader) {
        js!( @{self}.compileShader(@{shader}); );
    }

    pub fn compressed_tex_image2_d(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, image_size: GLsizei, offset: GLintptr) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{image_size}, @{offset as f64}); );
    }

    pub fn compressed_tex_image2_d_1(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, src_data: &ArrayBuffer, src_offset: GLuint, src_length_override: GLuint) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_image2_d_2(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, border: GLint, data: &ArrayBuffer) {
        js!( @{self}.compressedTexImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{data}); );
    }

    pub fn compressed_tex_image3_d(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, image_size: GLsizei, offset: GLintptr) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{image_size}, @{offset as f64}); );
    }

    pub fn compressed_tex_image3_d_1(&self, target: GLenum, level: GLint, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, src_data: &ArrayBuffer, src_offset: GLuint, src_length_override: GLuint) {
        js!( @{self}.compressedTexImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, image_size: GLsizei, offset: GLintptr) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{image_size}, @{offset as f64}); );
    }

    pub fn compressed_tex_sub_image2_d_1(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, src_data: &ArrayBuffer, src_offset: GLuint, src_length_override: GLuint) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn compressed_tex_sub_image2_d_2(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, data: &ArrayBuffer) {
        js!( @{self}.compressedTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{data}); );
    }

    pub fn compressed_tex_sub_image3_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, image_size: GLsizei, offset: GLintptr) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{image_size}, @{offset as f64}); );
    }

    pub fn compressed_tex_sub_image3_d_1(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, src_data: &ArrayBuffer, src_offset: GLuint, src_length_override: GLuint) {
        js!( @{self}.compressedTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{src_data}, @{src_offset}, @{src_length_override}); );
    }

    pub fn copy_buffer_sub_data(&self, read_target: GLenum, write_target: GLenum, read_offset: GLintptr, write_offset: GLintptr, size: GLsizeiptr) {
        js!( @{self}.copyBufferSubData(@{read_target}, @{write_target}, @{read_offset as f64}, @{write_offset as f64}, @{size as f64}); );
    }

    pub fn copy_tex_image2_d(&self, target: GLenum, level: GLint, internalformat: GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint) {
        js!( @{self}.copyTexImage2D(@{target}, @{level}, @{internalformat}, @{x}, @{y}, @{width}, @{height}, @{border}); );
    }

    pub fn copy_tex_sub_image2_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.copyTexSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn copy_tex_sub_image3_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.copyTexSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn create_buffer(&self, ) -> Option<WebGLBuffer> {
        (js! { return @{self}.createBuffer(); } ).try_into().ok()
    }

    pub fn create_framebuffer(&self, ) -> Option<WebGLFramebuffer> {
        (js! { return @{self}.createFramebuffer(); } ).try_into().ok()
    }

    pub fn create_program(&self, ) -> Option<WebGLProgram> {
        (js! { return @{self}.createProgram(); } ).try_into().ok()
    }

    pub fn create_query(&self, ) -> Option<WebGLQuery> {
        (js! { return @{self}.createQuery(); } ).try_into().ok()
    }

    pub fn create_renderbuffer(&self, ) -> Option<WebGLRenderbuffer> {
        (js! { return @{self}.createRenderbuffer(); } ).try_into().ok()
    }

    pub fn create_sampler(&self, ) -> Option<WebGLSampler> {
        (js! { return @{self}.createSampler(); } ).try_into().ok()
    }

    pub fn create_shader(&self, type_: GLenum) -> Option<WebGLShader> {
        (js! { return @{self}.createShader(@{type_}); } ).try_into().ok()
    }

    pub fn create_texture(&self, ) -> Option<WebGLTexture> {
        (js! { return @{self}.createTexture(); } ).try_into().ok()
    }

    pub fn create_transform_feedback(&self, ) -> Option<WebGLTransformFeedback> {
        (js! { return @{self}.createTransformFeedback(); } ).try_into().ok()
    }

    pub fn create_vertex_array(&self, ) -> Option<WebGLVertexArrayObject> {
        (js! { return @{self}.createVertexArray(); } ).try_into().ok()
    }

    pub fn cull_face(&self, mode: GLenum) {
        js!( @{self}.cullFace(@{mode}); );
    }

    pub fn delete_buffer(&self, buffer: &WebGLBuffer) {
        js!( @{self}.deleteBuffer(@{buffer}); );
    }

    pub fn delete_framebuffer(&self, framebuffer: &WebGLFramebuffer) {
        js!( @{self}.deleteFramebuffer(@{framebuffer}); );
    }

    pub fn delete_program(&self, program: &WebGLProgram) {
        js!( @{self}.deleteProgram(@{program}); );
    }

    pub fn delete_query(&self, query: &WebGLQuery) {
        js!( @{self}.deleteQuery(@{query}); );
    }

    pub fn delete_renderbuffer(&self, renderbuffer: &WebGLRenderbuffer) {
        js!( @{self}.deleteRenderbuffer(@{renderbuffer}); );
    }

    pub fn delete_sampler(&self, sampler: &WebGLSampler) {
        js!( @{self}.deleteSampler(@{sampler}); );
    }

    pub fn delete_shader(&self, shader: &WebGLShader) {
        js!( @{self}.deleteShader(@{shader}); );
    }

    pub fn delete_sync(&self, sync: &WebGLSync) {
        js!( @{self}.deleteSync(@{sync}); );
    }

    pub fn delete_texture(&self, texture: &WebGLTexture) {
        js!( @{self}.deleteTexture(@{texture}); );
    }

    pub fn delete_transform_feedback(&self, tf: &WebGLTransformFeedback) {
        js!( @{self}.deleteTransformFeedback(@{tf}); );
    }

    pub fn delete_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) {
        js!( @{self}.deleteVertexArray(@{vertex_array}); );
    }

    pub fn depth_func(&self, func: GLenum) {
        js!( @{self}.depthFunc(@{func}); );
    }

    pub fn depth_mask(&self, flag: GLboolean) {
        js!( @{self}.depthMask(@{flag}); );
    }

    pub fn depth_range(&self, z_near: GLclampf, z_far: GLclampf) {
        js!( @{self}.depthRange(@{z_near}, @{z_far}); );
    }

    pub fn detach_shader(&self, program: &WebGLProgram, shader: &WebGLShader) {
        js!( @{self}.detachShader(@{program}, @{shader}); );
    }

    pub fn disable(&self, cap: GLenum) {
        js!( @{self}.disable(@{cap}); );
    }

    pub fn disable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.disableVertexAttribArray(@{index}); );
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        js!( @{self}.drawArrays(@{mode}, @{first}, @{count}); );
    }

    pub fn draw_arrays_instanced(&self, mode: GLenum, first: GLint, count: GLsizei, instance_count: GLsizei) {
        js!( @{self}.drawArraysInstanced(@{mode}, @{first}, @{count}, @{instance_count}); );
    }

    pub fn draw_buffers(&self, buffers: &[GLenum]) {
        js!( @{self}.drawBuffers(@{buffers}); );
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @{self}.drawElements(@{mode}, @{count}, @{type_}, @{offset as f64}); );
    }

    pub fn draw_elements_instanced(&self, mode: GLenum, count: GLsizei, type_: GLenum, offset: GLintptr, instance_count: GLsizei) {
        js!( @{self}.drawElementsInstanced(@{mode}, @{count}, @{type_}, @{offset as f64}, @{instance_count}); );
    }

    pub fn draw_range_elements(&self, mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, offset: GLintptr) {
        js!( @{self}.drawRangeElements(@{mode}, @{start}, @{end}, @{count}, @{type_}, @{offset as f64}); );
    }

    pub fn drawing_buffer_height(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferHeight; } ).try_into().unwrap()
    }

    pub fn drawing_buffer_width(&self) -> GLsizei {
        (js! { return @{self}.drawingBufferWidth; } ).try_into().unwrap()
    }

    pub fn enable(&self, cap: GLenum) {
        js!( @{self}.enable(@{cap}); );
    }

    pub fn enable_vertex_attrib_array(&self, index: GLuint) {
        js!( @{self}.enableVertexAttribArray(@{index}); );
    }

    pub fn end_query(&self, target: GLenum) {
        js!( @{self}.endQuery(@{target}); );
    }

    pub fn end_transform_feedback(&self, ) {
        js!( @{self}.endTransformFeedback(); );
    }

    pub fn fence_sync(&self, condition: GLenum, flags: GLbitfield) -> Option<WebGLSync> {
        (js! { return @{self}.fenceSync(@{condition}, @{flags}); } ).try_into().ok()
    }

    pub fn finish(&self, ) {
        js!( @{self}.finish(); );
    }

    pub fn flush(&self, ) {
        js!( @{self}.flush(); );
    }

    pub fn framebuffer_renderbuffer(&self, target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: Option<&WebGLRenderbuffer>) {
        js!( @{self}.framebufferRenderbuffer(@{target}, @{attachment}, @{renderbuffertarget}, @{renderbuffer}); );
    }

    pub fn framebuffer_texture2_d(&self, target: GLenum, attachment: GLenum, textarget: GLenum, texture: Option<&WebGLTexture>, level: GLint) {
        js!( @{self}.framebufferTexture2D(@{target}, @{attachment}, @{textarget}, @{texture}, @{level}); );
    }

    pub fn framebuffer_texture_layer(&self, target: GLenum, attachment: GLenum, texture: Option<&WebGLTexture>, level: GLint, layer: GLint) {
        js!( @{self}.framebufferTextureLayer(@{target}, @{attachment}, @{texture}, @{level}, @{layer}); );
    }

    pub fn front_face(&self, mode: GLenum) {
        js!( @{self}.frontFace(@{mode}); );
    }

    pub fn generate_mipmap(&self, target: GLenum) {
        js!( @{self}.generateMipmap(@{target}); );
    }

    pub fn get_active_attrib(&self, program: &WebGLProgram, index: GLuint) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveAttrib(@{program}, @{index}); } ).try_into().ok()
    }

    pub fn get_active_uniform(&self, program: &WebGLProgram, index: GLuint) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getActiveUniform(@{program}, @{index}); } ).try_into().ok()
    }

    pub fn get_active_uniform_block_name(&self, program: &WebGLProgram, uniform_block_index: GLuint) -> Option<String> {
        (js! { return @{self}.getActiveUniformBlockName(@{program}, @{uniform_block_index}); } ).try_into().ok()
    }

    pub fn get_active_uniform_block_parameter(&self, program: &WebGLProgram, uniform_block_index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getActiveUniformBlockParameter(@{program}, @{uniform_block_index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_active_uniforms(&self, program: &WebGLProgram, uniform_indices: &[GLuint], pname: GLenum) -> Value {
        (js! { return @{self}.getActiveUniforms(@{program}, @{uniform_indices}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_attached_shaders(&self, program: &WebGLProgram) -> Option<Vec<WebGLShader>> {
        (js! { return @{self}.getAttachedShaders(@{program}); } ).try_into().ok()
    }

    pub fn get_attrib_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getAttribLocation(@{program}, @{name}); } ).try_into().unwrap()
    }

    pub fn get_buffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getBufferParameter(@{target}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_buffer_sub_data(&self, target: GLenum, src_byte_offset: GLintptr, dst_buffer: &ArrayBuffer, dst_offset: GLuint, length: GLuint) {
        js!( @{self}.getBufferSubData(@{target}, @{src_byte_offset as f64}, @{dst_buffer}, @{dst_offset}, @{length}); );
    }

    pub fn get_context_attributes(&self, ) -> Option<WebGLContextAttributes> {
        (js! { return @{self}.getContextAttributes(); } ).try_into().ok()
    }

    pub fn get_error(&self, ) -> GLenum {
        (js! { return @{self}.getError(); } ).try_into().unwrap()
    }

    pub fn get_extension<E: Extension>(&self) -> Option<E> {
        (js! { return @{self}.getExtension({E::NAME}); } ).try_into().ok()
    }

    pub fn get_frag_data_location(&self, program: &WebGLProgram, name: &str) -> GLint {
        (js! { return @{self}.getFragDataLocation(@{program}, @{name}); } ).try_into().unwrap()
    }

    pub fn get_framebuffer_attachment_parameter(&self, target: GLenum, attachment: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getFramebufferAttachmentParameter(@{target}, @{attachment}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_indexed_parameter(&self, target: GLenum, index: GLuint) -> Value {
        (js! { return @{self}.getIndexedParameter(@{target}, @{index}); } ).try_into().unwrap()
    }

    pub fn get_internalformat_parameter(&self, target: GLenum, internalformat: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getInternalformatParameter(@{target}, @{internalformat}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_parameter(&self, pname: GLenum) -> Value {
        (js! { return @{self}.getParameter(@{pname}); } ).try_into().unwrap()
    }

    pub fn get_program_info_log(&self, program: &WebGLProgram) -> Option<String> {
        (js! { return @{self}.getProgramInfoLog(@{program}); } ).try_into().ok()
    }

    pub fn get_program_parameter(&self, program: &WebGLProgram, pname: GLenum) -> Value {
        (js! { return @{self}.getProgramParameter(@{program}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_query(&self, target: GLenum, pname: GLenum) -> Option<WebGLQuery> {
        (js! { return @{self}.getQuery(@{target}, @{pname}); } ).try_into().ok()
    }

    pub fn get_query_parameter(&self, query: &WebGLQuery, pname: GLenum) -> Value {
        (js! { return @{self}.getQueryParameter(@{query}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_renderbuffer_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getRenderbufferParameter(@{target}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_sampler_parameter(&self, sampler: &WebGLSampler, pname: GLenum) -> Value {
        (js! { return @{self}.getSamplerParameter(@{sampler}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_shader_info_log(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderInfoLog(@{shader}); } ).try_into().ok()
    }

    pub fn get_shader_parameter(&self, shader: &WebGLShader, pname: GLenum) -> Value {
        (js! { return @{self}.getShaderParameter(@{shader}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_shader_precision_format(&self, shadertype: GLenum, precisiontype: GLenum) -> Option<WebGLShaderPrecisionFormat> {
        (js! { return @{self}.getShaderPrecisionFormat(@{shadertype}, @{precisiontype}); } ).try_into().ok()
    }

    pub fn get_shader_source(&self, shader: &WebGLShader) -> Option<String> {
        (js! { return @{self}.getShaderSource(@{shader}); } ).try_into().ok()
    }

    pub fn get_supported_extensions(&self, ) -> Option<Vec<String>> {
        (js! { return @{self}.getSupportedExtensions(); } ).try_into().ok()
    }

    pub fn get_sync_parameter(&self, sync: &WebGLSync, pname: GLenum) -> Value {
        (js! { return @{self}.getSyncParameter(@{sync}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_tex_parameter(&self, target: GLenum, pname: GLenum) -> Value {
        (js! { return @{self}.getTexParameter(@{target}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_transform_feedback_varying(&self, program: &WebGLProgram, index: GLuint) -> Option<WebGLActiveInfo> {
        (js! { return @{self}.getTransformFeedbackVarying(@{program}, @{index}); } ).try_into().ok()
    }

    pub fn get_uniform(&self, program: &WebGLProgram, location: &WebGLUniformLocation) -> Value {
        (js! { return @{self}.getUniform(@{program}, @{location}); } ).try_into().unwrap()
    }

    pub fn get_uniform_block_index(&self, program: &WebGLProgram, uniform_block_name: &str) -> GLuint {
        (js! { return @{self}.getUniformBlockIndex(@{program}, @{uniform_block_name}); } ).try_into().unwrap()
    }

    pub fn get_uniform_indices(&self, program: &WebGLProgram, uniform_names: &[&str]) -> Option<Vec<GLuint>> {
        (js! { return @{self}.getUniformIndices(@{program}, @{uniform_names}); } ).try_into().ok()
    }

    pub fn get_uniform_location(&self, program: &WebGLProgram, name: &str) -> Option<WebGLUniformLocation> {
        (js! { return @{self}.getUniformLocation(@{program}, @{name}); } ).try_into().ok()
    }

    pub fn get_vertex_attrib(&self, index: GLuint, pname: GLenum) -> Value {
        (js! { return @{self}.getVertexAttrib(@{index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn get_vertex_attrib_offset(&self, index: GLuint, pname: GLenum) -> GLintptr {
        (js! { return @{self}.getVertexAttribOffset(@{index}, @{pname}); } ).try_into().unwrap()
    }

    pub fn hint(&self, target: GLenum, mode: GLenum) {
        js!( @{self}.hint(@{target}, @{mode}); );
    }

    pub fn invalidate_framebuffer(&self, target: GLenum, attachments: &[GLenum]) {
        js!( @{self}.invalidateFramebuffer(@{target}, @{attachments}); );
    }

    pub fn invalidate_sub_framebuffer(&self, target: GLenum, attachments: &[GLenum], x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.invalidateSubFramebuffer(@{target}, @{attachments}, @{x}, @{y}, @{width}, @{height}); );
    }

    pub fn is_buffer(&self, buffer: Option<&WebGLBuffer>) -> GLboolean {
        (js! { return @{self}.isBuffer(@{buffer}); } ).try_into().unwrap()
    }

    pub fn is_context_lost(&self, ) -> bool {
        (js! { return @{self}.isContextLost(); } ).try_into().unwrap()
    }

    pub fn is_enabled(&self, cap: GLenum) -> GLboolean {
        (js! { return @{self}.isEnabled(@{cap}); } ).try_into().unwrap()
    }

    pub fn is_framebuffer(&self, framebuffer: Option<&WebGLFramebuffer>) -> GLboolean {
        (js! { return @{self}.isFramebuffer(@{framebuffer}); } ).try_into().unwrap()
    }

    pub fn is_program(&self, program: Option<&WebGLProgram>) -> GLboolean {
        (js! { return @{self}.isProgram(@{program}); } ).try_into().unwrap()
    }

    pub fn is_query(&self, query: Option<&WebGLQuery>) -> GLboolean {
        (js! { return @{self}.isQuery(@{query}); } ).try_into().unwrap()
    }

    pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGLRenderbuffer>) -> GLboolean {
        (js! { return @{self}.isRenderbuffer(@{renderbuffer}); } ).try_into().unwrap()
    }

    pub fn is_sampler(&self, sampler: Option<&WebGLSampler>) -> GLboolean {
        (js! { return @{self}.isSampler(@{sampler}); } ).try_into().unwrap()
    }

    pub fn is_shader(&self, shader: Option<&WebGLShader>) -> GLboolean {
        (js! { return @{self}.isShader(@{shader}); } ).try_into().unwrap()
    }

    pub fn is_sync(&self, sync: Option<&WebGLSync>) -> GLboolean {
        (js! { return @{self}.isSync(@{sync}); } ).try_into().unwrap()
    }

    pub fn is_texture(&self, texture: Option<&WebGLTexture>) -> GLboolean {
        (js! { return @{self}.isTexture(@{texture}); } ).try_into().unwrap()
    }

    pub fn is_transform_feedback(&self, tf: Option<&WebGLTransformFeedback>) -> GLboolean {
        (js! { return @{self}.isTransformFeedback(@{tf}); } ).try_into().unwrap()
    }

    pub fn is_vertex_array(&self, vertex_array: Option<&WebGLVertexArrayObject>) -> GLboolean {
        (js! { return @{self}.isVertexArray(@{vertex_array}); } ).try_into().unwrap()
    }

    pub fn line_width(&self, width: GLfloat) {
        js!( @{self}.lineWidth(@{width}); );
    }

    pub fn link_program(&self, program: &WebGLProgram) {
        js!( @{self}.linkProgram(@{program}); );
    }

    pub fn pause_transform_feedback(&self, ) {
        js!( @{self}.pauseTransformFeedback(); );
    }

    pub fn pixel_storei(&self, pname: GLenum, param: GLint) {
        js!( @{self}.pixelStorei(@{pname}, @{param}); );
    }

    pub fn polygon_offset(&self, factor: GLfloat, units: GLfloat) {
        js!( @{self}.polygonOffset(@{factor}, @{units}); );
    }

    pub fn read_buffer(&self, src: GLenum) {
        js!( @{self}.readBuffer(@{src}); );
    }

    pub fn read_pixels(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, dst_data: Option<&ArrayBuffer>) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}); );
    }

    pub fn read_pixels_1(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, offset: GLintptr) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{offset as f64}); );
    }

    pub fn read_pixels_2(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, dst_data: &ArrayBuffer, dst_offset: GLuint) {
        js!( @{self}.readPixels(@{x}, @{y}, @{width}, @{height}, @{format}, @{type_}, @{dst_data}, @{dst_offset}); );
    }

    pub fn renderbuffer_storage(&self, target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        js!( @{self}.renderbufferStorage(@{target}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn renderbuffer_storage_multisample(&self, target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        js!( @{self}.renderbufferStorageMultisample(@{target}, @{samples}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn resume_transform_feedback(&self, ) {
        js!( @{self}.resumeTransformFeedback(); );
    }

    pub fn sample_coverage(&self, value: GLclampf, invert: GLboolean) {
        js!( @{self}.sampleCoverage(@{value}, @{invert}); );
    }

    pub fn sampler_parameterf(&self, sampler: &WebGLSampler, pname: GLenum, param: GLfloat) {
        js!( @{self}.samplerParameterf(@{sampler}, @{pname}, @{param}); );
    }

    pub fn sampler_parameteri(&self, sampler: &WebGLSampler, pname: GLenum, param: GLint) {
        js!( @{self}.samplerParameteri(@{sampler}, @{pname}, @{param}); );
    }

    pub fn scissor(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.scissor(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn shader_source(&self, shader: &WebGLShader, source: &str) {
        js!( @{self}.shaderSource(@{shader}, @{source}); );
    }

    pub fn stencil_func(&self, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFunc(@{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_func_separate(&self, face: GLenum, func: GLenum, ref_: GLint, mask: GLuint) {
        js!( @{self}.stencilFuncSeparate(@{face}, @{func}, @{ref_}, @{mask}); );
    }

    pub fn stencil_mask(&self, mask: GLuint) {
        js!( @{self}.stencilMask(@{mask}); );
    }

    pub fn stencil_mask_separate(&self, face: GLenum, mask: GLuint) {
        js!( @{self}.stencilMaskSeparate(@{face}, @{mask}); );
    }

    pub fn stencil_op(&self, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOp(@{fail}, @{zfail}, @{zpass}); );
    }

    pub fn stencil_op_separate(&self, face: GLenum, fail: GLenum, zfail: GLenum, zpass: GLenum) {
        js!( @{self}.stencilOpSeparate(@{face}, @{fail}, @{zfail}, @{zpass}); );
    }

    pub fn tex_image2_d(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: Option<&ArrayBuffer>) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_image2_d_1<T0>(&self, target: GLenum, level: GLint, internalformat: GLint, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_2(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pbo_offset: GLintptr) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{pbo_offset as f64}); );
    }

    pub fn tex_image2_d_3<T0>(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image2_d_4(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, src_data: &ArrayBuffer, src_offset: GLuint) {
        js!( @{self}.texImage2D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_image3_d(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, pbo_offset: GLintptr) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{pbo_offset as f64}); );
    }

    pub fn tex_image3_d_1<T0>(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_image3_d_2(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, src_data: Option<&ArrayBuffer>) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}); );
    }

    pub fn tex_image3_d_3(&self, target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: GLenum, type_: GLenum, src_data: &ArrayBuffer, src_offset: GLuint) {
        js!( @{self}.texImage3D(@{target}, @{level}, @{internalformat}, @{width}, @{height}, @{depth}, @{border}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_parameterf(&self, target: GLenum, pname: GLenum, param: GLfloat) {
        js!( @{self}.texParameterf(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_parameteri(&self, target: GLenum, pname: GLenum, param: GLint) {
        js!( @{self}.texParameteri(@{target}, @{pname}, @{param}); );
    }

    pub fn tex_storage2_d(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei) {
        js!( @{self}.texStorage2D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}); );
    }

    pub fn tex_storage3_d(&self, target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei) {
        js!( @{self}.texStorage3D(@{target}, @{levels}, @{internalformat}, @{width}, @{height}, @{depth}); );
    }

    pub fn tex_sub_image2_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: Option<&ArrayBuffer>) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pixels}); );
    }

    pub fn tex_sub_image2_d_1<T0>(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_2(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pbo_offset: GLintptr) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{pbo_offset as f64}); );
    }

    pub fn tex_sub_image2_d_3<T0>(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image2_d_4(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, src_data: &ArrayBuffer, src_offset: GLuint) {
        js!( @{self}.texSubImage2D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{width}, @{height}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn tex_sub_image3_d(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pbo_offset: GLintptr) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{pbo_offset as f64}); );
    }

    pub fn tex_sub_image3_d_1<T0>(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, source: T0) where T0: JsSerialize {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{source}); );
    }

    pub fn tex_sub_image3_d_2(&self, target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, src_data: Option<&ArrayBuffer>, src_offset: GLuint) {
        js!( @{self}.texSubImage3D(@{target}, @{level}, @{xoffset}, @{yoffset}, @{zoffset}, @{width}, @{height}, @{depth}, @{format}, @{type_}, @{src_data}, @{src_offset}); );
    }

    pub fn transform_feedback_varyings(&self, program: &WebGLProgram, varyings: &[&str], buffer_mode: GLenum) {
        js!( @{self}.transformFeedbackVaryings(@{program}, @{varyings}, @{buffer_mode}); );
    }

    pub fn uniform1f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat) {
        js!( @{self}.uniform1f(@{location}, @{x}); );
    }

    pub fn uniform1fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform1fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1i(&self, location: Option<&WebGLUniformLocation>, x: GLint) {
        js!( @{self}.uniform1i(@{location}, @{x}); );
    }

    pub fn uniform1iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform1iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform1iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform1ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint) {
        js!( @{self}.uniform1ui(@{location}, @{v0}); );
    }

    pub fn uniform1uiv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.uniform1uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat) {
        js!( @{self}.uniform2f(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform2fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint) {
        js!( @{self}.uniform2i(@{location}, @{x}, @{y}); );
    }

    pub fn uniform2iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform2iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform2iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform2ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint) {
        js!( @{self}.uniform2ui(@{location}, @{v0}, @{v1}); );
    }

    pub fn uniform2uiv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.uniform2uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @{self}.uniform3f(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform3fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint) {
        js!( @{self}.uniform3i(@{location}, @{x}, @{y}, @{z}); );
    }

    pub fn uniform3iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform3iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform3iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform3ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint, v2: GLuint) {
        js!( @{self}.uniform3ui(@{location}, @{v0}, @{v1}, @{v2}); );
    }

    pub fn uniform3uiv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.uniform3uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4f(&self, location: Option<&WebGLUniformLocation>, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @{self}.uniform4f(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniform4fv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4i(&self, location: Option<&WebGLUniformLocation>, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @{self}.uniform4i(@{location}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn uniform4iv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform4iv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, v: T0) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.uniform4iv(@{location}, @{unsafe { v.as_typed_array() }}); );
    }

    pub fn uniform4ui(&self, location: Option<&WebGLUniformLocation>, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
        js!( @{self}.uniform4ui(@{location}, @{v0}, @{v1}, @{v2}, @{v3}); );
    }

    pub fn uniform4uiv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.uniform4uiv(@{location}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_block_binding(&self, program: &WebGLProgram, uniform_block_index: GLuint, uniform_block_binding: GLuint) {
        js!( @{self}.uniformBlockBinding(@{program}, @{uniform_block_index}, @{uniform_block_binding}); );
    }

    pub fn uniform_matrix2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, value: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix2fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix2x3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix2x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix2x4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix2x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, value: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix3fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix3x2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix3x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix3x4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix3x4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4fv_1<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, value: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix4fv(@{location}, @{transpose}, @{unsafe { value.as_typed_array() }}); );
    }

    pub fn uniform_matrix4x2fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix4x2fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn uniform_matrix4x3fv<'a0, T0>(&self, location: Option<&WebGLUniformLocation>, transpose: GLboolean, data: T0, src_offset: GLuint, src_length: GLuint) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.uniformMatrix4x3fv(@{location}, @{transpose}, @{unsafe { data.as_typed_array() }}, @{src_offset}, @{src_length}); );
    }

    pub fn use_program(&self, program: Option<&WebGLProgram>) {
        js!( @{self}.useProgram(@{program}); );
    }

    pub fn validate_program(&self, program: &WebGLProgram) {
        js!( @{self}.validateProgram(@{program}); );
    }

    pub fn vertex_attrib1f(&self, index: GLuint, x: GLfloat) {
        js!( @{self}.vertexAttrib1f(@{index}, @{x}); );
    }

    pub fn vertex_attrib1fv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.vertexAttrib1fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib2f(&self, index: GLuint, x: GLfloat, y: GLfloat) {
        js!( @{self}.vertexAttrib2f(@{index}, @{x}, @{y}); );
    }

    pub fn vertex_attrib2fv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.vertexAttrib2fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib3f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
        js!( @{self}.vertexAttrib3f(@{index}, @{x}, @{y}, @{z}); );
    }

    pub fn vertex_attrib3fv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.vertexAttrib3fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib4f(&self, index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
        js!( @{self}.vertexAttrib4f(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib4fv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, f32> {
        js!( @{self}.vertexAttrib4fv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_divisor(&self, index: GLuint, divisor: GLuint) {
        js!( @{self}.vertexAttribDivisor(@{index}, @{divisor}); );
    }

    pub fn vertex_attrib_i4i(&self, index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint) {
        js!( @{self}.vertexAttribI4i(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4iv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, i32> {
        js!( @{self}.vertexAttribI4iv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i4ui(&self, index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint) {
        js!( @{self}.vertexAttribI4ui(@{index}, @{x}, @{y}, @{z}, @{w}); );
    }

    pub fn vertex_attrib_i4uiv<'a0, T0>(&self, index: GLuint, values: T0) where T0: AsTypedArray<'a0, u32> {
        js!( @{self}.vertexAttribI4uiv(@{index}, @{unsafe { values.as_typed_array() }}); );
    }

    pub fn vertex_attrib_i_pointer(&self, index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, offset: GLintptr) {
        js!( @{self}.vertexAttribIPointer(@{index}, @{size}, @{type_}, @{stride}, @{offset as f64}); );
    }

    pub fn vertex_attrib_pointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, offset: GLintptr) {
        js!( @{self}.vertexAttribPointer(@{index}, @{size}, @{type_}, @{normalized}, @{stride}, @{offset as f64}); );
    }

    pub fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        js!( @{self}.viewport(@{x}, @{y}, @{width}, @{height}); );
    }

    pub fn wait_sync(&self, sync: &WebGLSync, flags: GLbitfield, timeout: GLint64) {
        js!( @{self}.waitSync(@{sync}, @{flags}, @{timeout as f64}); );
    }
}

impl InstanceOf for GLContext {
    #[inline]
    fn instance_of( reference: &Reference ) -> bool {
        js!(
            return [WebGLRenderingContext, WebGL2RenderingContext].includes( @{{reference}}.constructor );
        ).try_into().unwrap()
    }
}