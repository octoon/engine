#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub mod web;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
pub mod native;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
pub use self::web::Window;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
pub use self::native::Window;