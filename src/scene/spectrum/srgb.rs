use super::rgb::*;

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct SRGB(pub u8, pub u8, pub u8);

fn srgb2linear(v: f32) -> f32
{
    if v <= 0.04045
    {
        v / 12.92
    }
    else
    {
        ((v + 0.055) * 1.0 / 1.055).powf(2.4)
    }
}

fn linear2srgb(v: f32) -> f32
{
    if v <= 0.0031308
    {
        12.92 * v
    }
    else
    {
        1.055 * f32::powf(v, 1.0 / 2.4) - 0.055
    }
}

impl From<SRGB> for RGB
{
	fn from(srgb:SRGB) -> Self
	{
		Self
		{
    		0:srgb2linear(srgb.0 as f32 / 255.0),
    		1:srgb2linear(srgb.1 as f32 / 255.0),
    		2:srgb2linear(srgb.2 as f32 / 255.0),
		}
	}
}

impl From<RGB> for SRGB
{
	fn from(rgb:RGB) -> Self
	{
		Self
		{
    		0:(linear2srgb(rgb.0) * 255.0) as u8,
    		1:(linear2srgb(rgb.1) * 255.0) as u8,
    		2:(linear2srgb(rgb.2) * 255.0) as u8,
		}
	}
}

impl From<SRGB> for RGBSpectrum
{
	fn from(srgb:SRGB) -> Self
	{
		let rgb : RGB = srgb.into();
		RGBSpectrum::new(rgb.0, rgb.1, rgb.2)
	}
}