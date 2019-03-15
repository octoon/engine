#![recursion_limit = "128"]
#![feature(custom_attribute)]
#![allow(non_upper_case_globals)]

extern crate byteorder;
extern crate encoding;
extern crate uuid;
extern crate rand;
extern crate image;

#[macro_use]
extern crate log;

#[macro_use]
#[cfg(any(feature = "serialize", feature = "webgl", feature = "default"))]
extern crate serde_derive;
#[cfg(any(feature = "serialize", feature = "webgl", feature = "default"))]
extern crate serde;

#[cfg(feature = "default")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
extern crate gl;

#[cfg(feature = "default")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
extern crate glfw;

#[macro_use]
#[cfg(feature = "webgl")]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb;

#[macro_use]
#[cfg(feature = "webgl")]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb_derive;

#[macro_use]
pub mod math;
pub mod animation;
pub mod models;
pub mod scene;
pub mod renderer;

pub use self::animation::*;
pub use self::math::*;
pub use self::models::*;
pub use self::renderer::*;
pub use self::scene::*;