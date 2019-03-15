use super::rgb::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct XYZ(f32, f32, f32);

impl From<XYZ> for RGB
{
	fn from(xyz:XYZ) -> Self
	{
		Self
		{
    		0:xyz.0 *  3.2404542 - xyz.1 * 1.5371385 - xyz.2 * 0.4985314,
    		1:xyz.0 * -0.9692660 + xyz.1 * 1.8760108 + xyz.2 * 0.0415560,
    		2:xyz.0 *  0.0556434 - xyz.1 * 0.2040259 + xyz.2 * 1.0572252
		}
	}
}

impl From<XYZ> for RGBSpectrum
{
	fn from(xyz:XYZ) -> Self
	{
		let rgb : RGB = xyz.into();
		RGBSpectrum::new(rgb.0, rgb.1, rgb.2)
	}
}