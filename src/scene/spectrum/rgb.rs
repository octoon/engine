use std::f32;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use crate::math::consts::{Zero, One};

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGB(pub f32, pub f32, pub f32);

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct RGBSpectrum
{
	pub x:f32,
	pub y:f32,
	pub z:f32,
}

impl RGBSpectrum 
{
	/// Creates a new RGBSpectrum from multiple components
	pub fn new(x: f32, y: f32, z: f32) -> Self 
	{
		Self { x, y, z } 
	}

	pub fn to_rgb(&self) -> (f32, f32, f32)
	{
		(self.x, self.y, self.z)
	}

	/// return the length of element
	pub fn len() -> usize 
	{
		return 3; 
	}

    pub fn lum(&self) -> f32
    {
        let lum = (0.212671, 0.715160, 0.072169);

        return 
        	lum.0 * self.x + 
        	lum.1 * self.y + 
        	lum.2 * self.z;
    }
}

impl Neg for RGBSpectrum
{
    type Output = RGBSpectrum;

    fn neg(self) -> RGBSpectrum
    {
        RGBSpectrum 
        { 
        	x: -self.x,
        	y: -self.y,
        	z: -self.z
        }
    }
}

impl Add for RGBSpectrum 
{
	type Output = RGBSpectrum;

	fn add(self, other: RGBSpectrum) -> RGBSpectrum
	{
		RGBSpectrum
		{ 
			x: self.x + other.x, 
			y: self.y + other.y, 
			z: self.z + other.z  
		}
	}
}

impl Sub for RGBSpectrum
{
	type Output = RGBSpectrum;

	fn sub(self, other: RGBSpectrum) -> RGBSpectrum
	{
		RGBSpectrum
		{ 
			x: self.x - other.x, 
			y: self.y - other.y, 
			z: self.z - other.z  
		}
	}
}

impl Mul for RGBSpectrum
{
	type Output = RGBSpectrum;

	fn mul(self, other: RGBSpectrum) -> RGBSpectrum
	{
		RGBSpectrum
		{ 
			x: self.x * other.x, 
			y: self.y * other.y, 
			z: self.z * other.z  
		}
	}
}

impl Div for RGBSpectrum
{
	type Output = RGBSpectrum;

	fn div(self, other: RGBSpectrum) -> RGBSpectrum
	{
		RGBSpectrum
		{ 
			x: self.x / other.x, 
			y: self.y / other.y, 
			z: self.z / other.z  
		}
	}
}

impl AddAssign for RGBSpectrum
{
	fn add_assign(&mut self, other: RGBSpectrum)
	{
		self.x += other.x;
		self.y += other.y; 
		self.z += other.z; 
	}
}

impl SubAssign for RGBSpectrum
{
	fn sub_assign(&mut self, other: RGBSpectrum)
	{
		self.x -= other.x;
		self.y -= other.y; 
		self.z -= other.z; 
	}
}

impl MulAssign for RGBSpectrum
{
	fn mul_assign(&mut self, other: RGBSpectrum)
	{
		self.x *= other.x;
		self.y *= other.y; 
		self.z *= other.z; 
	}
}

impl DivAssign for RGBSpectrum
{
	fn div_assign(&mut self, other: RGBSpectrum)
	{
		self.x /= other.x;
		self.y /= other.y; 
		self.z /= other.z; 
	}
}

impl Zero for RGBSpectrum
{
    #[inline(always)]
    fn zero() -> RGBSpectrum
    {
        RGBSpectrum
        { 
            x: 0., y: 0., z: 0. 
        }
    }
}

impl One for RGBSpectrum
{
    #[inline(always)]
    fn one() -> RGBSpectrum
    {
        RGBSpectrum
        { 
            x: 1., y: 1., z: 1. 
        }
    }
}

impl From<RGB> for RGBSpectrum
{
	fn from(rgb:RGB) -> Self
	{
		RGBSpectrum::new(rgb.0, rgb.1, rgb.2)
	}
}