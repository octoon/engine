use super::rgb::*;
use super::kelvin::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Celsius(pub f32);

impl From<Celsius> for Kelvin
{
	fn from(c:Celsius) -> Self
	{
		Kelvin(c.0 + 273.15)
	}
}

impl From<Kelvin> for Celsius
{
	fn from(k:Kelvin) -> Self
	{
		Celsius(k.0 - 273.15)
	}
}

impl From<Celsius> for RGBSpectrum
{
	fn from(c:Celsius) -> Self
	{
		let k : Kelvin = c.into();
		return k.into();
	}
}