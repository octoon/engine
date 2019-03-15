use std::ops::Deref;

pub type Reference = u32;
pub type GLbitfield = u32;
pub type GLboolean = bool;
pub type GLbyte = i8;
pub type GLclampf = f32;
pub type GLenum = u32;
pub type GLfloat = f32;
pub type GLint = i32;
pub type GLint64 = i64;
pub type GLintptr = i64;
pub type GLshort = i16;
pub type GLsizei = i32;
pub type GLsizeiptr = i64;
pub type GLubyte = u8;
pub type GLuint = u32;
pub type GLuint64 = u64;
pub type GLushort = u16;
pub type Int32List = Vec<i32>;
pub type Uint32List = Vec<u32>;
pub type Float32List = Vec<f32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GLContext{ pub reference: Reference }
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLRenderingContext{ pub common: GLContext }
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGL2RenderingContext { pub common: GLContext }
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLBuffer(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLSampler(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLShader(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLProgram(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLFramebuffer(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLRenderbuffer(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLTexture(pub Reference);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WebGLVertexArray(pub Reference);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebGLActiveInfo
{
    reference: Reference,
    name: String,
    size: u32,
    kind: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebGLUniformLocation
{
    pub reference: Reference,
    pub name: String,
}

impl WebGLActiveInfo
{
    pub fn new<T: Into<String>>(name: T, size: u32, kind: u32, reference: Reference) -> WebGLActiveInfo
    {
        let nam = name.into();
        WebGLActiveInfo
        {
            reference,
            name: nam,
            size,
            kind,
        }
    }
}

impl WebGLUniformLocation
{
    pub fn new(name: String, reference: Reference) -> WebGLUniformLocation
    {
        WebGLUniformLocation
        {
            name,
            reference 
        }
    }
}

pub trait AsReference
{
    fn as_reference(&self) -> &Reference;
}

impl AsReference for GLContext
{
    fn as_reference(&self) -> &Reference
    {
        &self.reference
    }
}

impl AsReference for WebGLRenderingContext
{
    fn as_reference(&self) -> &Reference
    {
        &self.common.as_reference()
    }
}

impl AsReference for WebGL2RenderingContext
{
    fn as_reference(&self) -> &Reference
    {
        &self.common.as_reference()
    }
}

impl Deref for WebGLRenderingContext
{
    type Target = GLContext;
    fn deref(&self) -> &GLContext
    {
        &self.common
    }
}

impl Deref for WebGL2RenderingContext
{
    type Target = GLContext;
    fn deref(&self) -> &GLContext
    {
        &self.common
    }
}


impl Deref for WebGLBuffer
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLSampler
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLShader
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLProgram
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLFramebuffer
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLRenderbuffer
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLTexture
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLVertexArray
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.0
    }
}

impl Deref for WebGLUniformLocation
{
    type Target = Reference;
    fn deref(&self) -> &Self::Target
    {
        &self.reference
    }
}