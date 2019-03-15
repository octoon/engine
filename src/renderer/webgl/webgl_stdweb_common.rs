use stdweb::{Reference, Value, UnsafeTypedArray, JsSerialize};
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::web::{RenderingContext, TypedArray};
use stdweb::web::html_element::CanvasElement;

pub trait AsTypedArray<'a, T>
{
    type Result: JsSerialize;

    unsafe fn as_typed_array(self) -> Self::Result;
}

pub trait Extension: TryFrom<Value>
{
    const NAME: &'static str;
}

macro_rules! define_array
{
    ($elem:ty) =>
    {
        impl<'a> AsTypedArray<'a, $elem> for &'a TypedArray<$elem> {
            type Result = Self;

            unsafe fn as_typed_array(self) -> Self::Result { self }
        }

        impl<'a> AsTypedArray<'a, $elem> for &'a [$elem] {
            type Result = UnsafeTypedArray<'a, $elem>;

            unsafe fn as_typed_array(self) -> Self::Result { UnsafeTypedArray::new(self) }
        }
    }
}

define_array!(i8);
define_array!(u8);
define_array!(i16);
define_array!(u16);
define_array!(i32);
define_array!(u32);
define_array!(f32);
define_array!(f64);
    
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
pub type Int32List = TypedArray<i32>;
pub type TexImageSource = Value;
pub type Uint32List = TypedArray<u32>;
pub type Float32List = TypedArray<f32>;

type ConversionError = <Reference as TryFrom<Value>>::Error;

#[derive(Debug, Clone, PartialEq, ReferenceType)]
pub struct GLContext(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLRenderingContext")]
pub struct WebGLRenderingContext(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGL2RenderingContext")]
pub struct WebGL2RenderingContext(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLActiveInfo")]
pub struct WebGLActiveInfo(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLBuffer")]
pub struct WebGLBuffer(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLContextEvent")]
pub struct WebGLContextEvent(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLFramebuffer")]
pub struct WebGLFramebuffer(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLProgram")]
pub struct WebGLProgram(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLQuery")]
pub struct WebGLQuery(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLRenderbuffer")]
pub struct WebGLRenderbuffer(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLSampler")]
pub struct WebGLSampler(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLShader")]
pub struct WebGLShader(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLShaderPrecisionFormat")]
pub struct WebGLShaderPrecisionFormat(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLSync")]
pub struct WebGLSync(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLTexture")]
pub struct WebGLTexture(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLTransformFeedback")]
pub struct WebGLTransformFeedback(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLUniformLocation")]
pub struct WebGLUniformLocation(Reference);

#[derive(Debug, Clone, PartialEq, ReferenceType)]
#[reference(instance_of = "WebGLVertexArrayObject")]
pub struct WebGLVertexArrayObject(Reference);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WebGLPowerPreference
{
    Default,
    HighPerformance,
    LowPower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebGLContextAttributes
{    
    alpha: GLboolean,
    antialias: GLboolean,
    depth: GLboolean,
    fail_if_major_performance_caveat: GLboolean,
    power_preference: WebGLPowerPreference,
    premultiplied_alpha: GLboolean,
    preserve_drawing_buffer: GLboolean,
    stencil: GLboolean,
}

impl RenderingContext for WebGLRenderingContext
{
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!( return @{canvas}.getContext("webgl"); ).try_into()
    }
}

impl RenderingContext for WebGL2RenderingContext
{
    type Error = ConversionError;
    fn from_canvas(canvas: &CanvasElement) -> Result<Self, ConversionError> {
        js!( return @{canvas}.getContext("webgl2"); ).try_into()
    }
}

impl WebGLContextEvent 
{
    pub fn status_message(&self) -> String {
        (js! { return @{self}.statusMessage; } ).try_into().unwrap()
    }
}

impl WebGLActiveInfo 
{    
    pub fn name(&self) -> String {
        (js! { return @{self}.name; } ).try_into().unwrap()
    }

    pub fn size(&self) -> GLint {
        (js! { return @{self}.size; } ).try_into().unwrap()
    }

    pub fn type_(&self) -> GLenum {
        (js! { return @{self}.type; } ).try_into().unwrap()
    }
}

impl WebGLShaderPrecisionFormat
{
    pub fn precision(&self) -> GLint {
        (js! { return @{self}.precision; } ).try_into().unwrap()
    }

    pub fn range_max(&self) -> GLint {
        (js! { return @{self}.rangeMax; } ).try_into().unwrap()
    }

    pub fn range_min(&self) -> GLint {
        (js! { return @{self}.rangeMin; } ).try_into().unwrap()
    }
}

js_serializable!(WebGLPowerPreference);
js_serializable!(WebGLContextAttributes);

js_deserializable!(WebGLPowerPreference);
js_deserializable!(WebGLContextAttributes);