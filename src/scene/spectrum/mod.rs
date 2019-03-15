pub mod rgb;
pub mod srgb;
pub mod xyz;
pub mod kelvin;
pub mod celsius;
pub mod unit;

pub use self::rgb::*;
pub use self::srgb::*;
pub use self::xyz::*;
pub use self::kelvin::*;
pub use self::celsius::*;
pub use self::unit::*;

pub type Spectrum = rgb::RGBSpectrum;