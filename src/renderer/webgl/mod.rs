#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] mod webgl_native;
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] mod webgl_common;
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] mod webgl_context;

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] pub use self::webgl_common::*;
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] pub use self::webgl_native::*;
#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))] pub use self::webgl_context::*;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] mod webgl_stdweb_common;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] mod webgl_stdweb_native;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] mod webgl_stdweb_context;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] pub use self::webgl_stdweb_common::*;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] pub use self::webgl_stdweb_native::*;
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))] pub use self::webgl_stdweb_context::*;