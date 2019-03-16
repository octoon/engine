#![allow(non_upper_case_globals)]

#[macro_use]
extern crate serde_derive;

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