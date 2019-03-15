use crate::math::Pi4;
use super::super::core::LightType;

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct Candela(pub f32);

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct Lumens(pub f32);

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct Incandescent(pub f32);

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct Halogen(pub f32);

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct LED(pub f32);

#[derive(Debug, Default, Copy, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fluorescent(pub f32);

impl Lumens
{
	pub fn from_cd(kind:LightType, intensity:f32) -> Self
	{
		let lumens;

		match kind
		{
			LightType::Sky => lumens = intensity,
			LightType::Point => lumens = intensity * f32::pi4(),
			LightType::Spot => lumens = intensity * f32::pi4(),
			LightType::Directional => lumens = intensity,
		}

		return Lumens(lumens);
	}

	pub fn to_cd(&self, kind:LightType) -> Candela
	{
		let intensity;

		match kind
		{
			LightType::Sky => intensity = self.0,
			LightType::Point => intensity = self.0 / f32::pi4(),
			LightType::Spot => intensity = self.0 / f32::pi4(),
			LightType::Directional => intensity = self.0,
		}

		return Candela(intensity);
	}
}

impl From<Candela> for f32
{
	fn from(cd:Candela) -> f32
	{
		cd.0
	}
}

impl From<Lumens> for f32
{
	fn from(lm:Lumens) -> f32
	{
		lm.0
	}
}

impl From<Incandescent> for Lumens
{
	fn from(watts:Incandescent) -> Lumens
	{
		Lumens(watts.0 * f32::pi4())
	}
}

impl From<Halogen> for Lumens
{
	fn from(watts:Halogen) -> Lumens
	{
		Lumens(watts.0 * 20.0)
	}
}

impl From<LED> for Lumens
{
	fn from(watts:LED) -> Lumens
	{
		Lumens(watts.0 * 80.0)
	}
}

impl From<Fluorescent> for Lumens
{
	fn from(watts:Fluorescent) -> Lumens
	{
		Lumens(watts.0 * 15.0)
	}
}

impl From<Lumens> for Incandescent
{
	fn from(lm:Lumens) -> Incandescent
	{
		Incandescent(lm.0 / f32::pi4())
	}
}

impl From<Lumens> for Halogen
{
	fn from(lm:Lumens) -> Halogen
	{
		Halogen(lm.0 / 20.0)
	}
}

impl From<Lumens> for LED
{
	fn from(lm:Lumens) -> LED
	{
		LED(lm.0 / 80.0)
	}
}

impl From<Lumens> for Fluorescent
{
	fn from(lm:Lumens) -> Fluorescent
	{
		Fluorescent(lm.0 / 15.0)
	}
}