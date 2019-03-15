use std::f32;
use super::rgb::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Kelvin(pub f32);

impl From<Kelvin> for RGBSpectrum
{
	fn from(kelvin:Kelvin) -> Self
	{
		assert!(kelvin.0 >= 0.0 && kelvin.0 <= 40000.0);

		let temp = kelvin.0 / 100.0;
		let temp60 = f32::max(0.0, temp - 60.0);
		let red; let green; let blue;

		if temp <= 66.0 {
			red = 1.0;
		} else { 
			red = 1.292936186062745 * f32::powf(temp60, -0.1332047592);
		};

		if temp <= 66.0 {
			green = 0.3900815787690196 * f32::log(temp, f32::consts::E) - 0.6318414437886275;
		} else { 
			green = 1.129890860895294 * f32::powf(temp60, -0.0755148492);
		};

		if temp >= 66.0 {
			blue = 1.0;
		} else { 
			if temp <= 19.0 {
				blue = 0.0;
			} else { 
				blue = 0.5432067891101961 * f32::log(temp - 10.0, f32::consts::E) - 1.19625408914;
			}
		};

		RGBSpectrum::new(red, green, blue)
	}
}