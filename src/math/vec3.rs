use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use super::vec::{Vec, Math, Lerp, Slerp};
use super::vec2::Vec2;
use super::vec4::Vec4;
use super::consts::{Zero, One, UnitX, UnitY, UnitZ};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
pub struct Vec3<T>
{
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Neg for Vec3<T> where T:Neg<Output=T>
{
	type Output = Self;

	fn neg(self) -> Self
	{
		Self
		{
			x: -self.x,
			y: -self.y,
			z: -self.z
		}
	}
}

impl<T> Add for Vec3<T>  where T:Add<Output=T>
{
	type Output = Self;

	fn add(self, other: Self) -> Self
	{
		Self
		{
			x: self.x + other.x, 
			y: self.y + other.y, 
			z: self.z + other.z  
		}
	}
}

impl<T> Sub for Vec3<T> where T:Sub<Output=T>
{
	type Output = Self;

	fn sub(self, other: Self) -> Self
	{
		Self
		{
			x: self.x - other.x, 
			y: self.y - other.y, 
			z: self.z - other.z  
		}
	}
}

impl<T> Mul<T> for Vec3<T> where T:Mul<Output=T> + Copy
{
	type Output = Self;

	fn mul(self, s: T) -> Self
	{
		Self
		{
			x:self.x * s,
			y:self.y * s,
			z:self.z * s
		}
	}
}

impl<T> Mul for Vec3<T> where T:Mul<Output=T>
{
	type Output = Self;

	fn mul(self, other: Self) -> Self
	{
		Self
		{
			x: self.x * other.x, 
			y: self.y * other.y, 
			z: self.z * other.z  
		}
	}
}

impl<T> Div<T> for Vec3<T> where T:Div<Output=T> + Copy
{
	type Output = Self;

	fn div(self, s: T) -> Self
	{
		Self
		{
			x:self.x / s,
			y:self.y / s,
			z:self.z / s,
		}
	}
}

impl<T> Div for Vec3<T> where T:Div<Output=T>
{
	type Output = Self;

	fn div(self, other: Self) -> Self
	{
		Self
		{
			x: self.x / other.x, 
			y: self.y / other.y, 
			z: self.z / other.z  
		}
	}
}

impl<T> AddAssign for Vec3<T> where T: AddAssign<T>
{
	fn add_assign(&mut self, other: Self)
	{
		self.x += other.x;
		self.y += other.y; 
		self.z += other.z; 
	}
}

impl<T> SubAssign for Vec3<T> where T: SubAssign<T>
{
	fn sub_assign(&mut self, other: Self)
	{
		self.x -= other.x;
		self.y -= other.y; 
		self.z -= other.z; 
	}
}

impl<T> MulAssign<T> for Vec3<T> where T:MulAssign<T> + Copy
{
	fn mul_assign(&mut self, s: T)
	{
		self.x *= s;
		self.y *= s;
		self.z *= s;
	}
}

impl<T> MulAssign for Vec3<T> where T: MulAssign<T>
{
	fn mul_assign(&mut self, other: Self)
	{
		self.x *= other.x;
		self.y *= other.y; 
		self.z *= other.z; 
	}
}

impl<'a, T> MulAssign<&'a T> for Vec3<T> where T:MulAssign<T> + Copy
{
	fn mul_assign(&mut self, other: &'a T)
	{
		self.x *= *other;
		self.y *= *other;
		self.z *= *other;
	}
}

impl<T> DivAssign<T> for Vec3<T> where T:DivAssign<T> + Copy
{
	fn div_assign(&mut self, s: T)
	{
		self.x /= s;
		self.y /= s;
		self.z /= s;
	}
}

impl<T> DivAssign for Vec3<T> where T: DivAssign<T>
{
	fn div_assign(&mut self, other: Self)
	{
		self.x /= other.x;
		self.y /= other.y; 
		self.z /= other.z; 
	}
}

impl<'a, T> DivAssign<&'a T> for Vec3<T> where T:DivAssign<T> + Copy
{
	fn div_assign(&mut self, s: &'a T)
	{
		self.x /= *s;
		self.y /= *s;
		self.z /= *s;
	}
}

impl<T> Vec3<T> where T: Copy
{
	/// Creates a new Vec3 from multiple components
	#[inline(always)]
	pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }

	/// return the length of element
	#[inline(always)]
	pub fn len() -> usize 
	{
		return 3;
	}

	#[inline(always)]
	pub fn to_tuple(&self) -> (T, T, T)
	{
		(self.x, self.y, self.z)
	}

	#[inline(always)]
	pub fn xx(&self) -> Vec2<T> { Vec2::new(self.x, self.x) }
	#[inline(always)]
	pub fn xy(&self) -> Vec2<T> { Vec2::new(self.x, self.y) }
	#[inline(always)]
	pub fn xz(&self) -> Vec2<T> { Vec2::new(self.x, self.z) }
	#[inline(always)]
	pub fn yx(&self) -> Vec2<T> { Vec2::new(self.y, self.x) }
	#[inline(always)]
	pub fn yy(&self) -> Vec2<T> { Vec2::new(self.y, self.y) }
	#[inline(always)]
	pub fn yz(&self) -> Vec2<T> { Vec2::new(self.y, self.z) }
	#[inline(always)]
	pub fn zx(&self) -> Vec2<T> { Vec2::new(self.z, self.x) }
	#[inline(always)]
	pub fn zy(&self) -> Vec2<T> { Vec2::new(self.z, self.y) }
	#[inline(always)]
	pub fn zz(&self) -> Vec2<T> { Vec2::new(self.z, self.z) }
	#[inline(always)]
	pub fn xxx(&self) -> Self { Self::new(self.x, self.x, self.x) }
	#[inline(always)]
	pub fn xxy(&self) -> Self { Self::new(self.x, self.x, self.y) }
	#[inline(always)]
	pub fn xxz(&self) -> Self { Self::new(self.x, self.x, self.z) }
	#[inline(always)]
	pub fn xyx(&self) -> Self { Self::new(self.x, self.y, self.x) }
	#[inline(always)]
	pub fn xyy(&self) -> Self { Self::new(self.x, self.y, self.y) }
	#[inline(always)]
	pub fn xyz(&self) -> Self { Self::new(self.x, self.y, self.z) }
	#[inline(always)]
	pub fn xzx(&self) -> Self { Self::new(self.x, self.z, self.x) }
	#[inline(always)]
	pub fn xzy(&self) -> Self { Self::new(self.x, self.z, self.y) }
	#[inline(always)]
	pub fn xzz(&self) -> Self { Self::new(self.x, self.z, self.z) }
	#[inline(always)]
	pub fn yxx(&self) -> Self { Self::new(self.y, self.x, self.x) }
	#[inline(always)]
	pub fn yxy(&self) -> Self { Self::new(self.y, self.x, self.y) }
	#[inline(always)]
	pub fn yxz(&self) -> Self { Self::new(self.y, self.x, self.z) }
	#[inline(always)]
	pub fn yyx(&self) -> Self { Self::new(self.y, self.y, self.x) }
	#[inline(always)]
	pub fn yyy(&self) -> Self { Self::new(self.y, self.y, self.y) }
	#[inline(always)]
	pub fn yyz(&self) -> Self { Self::new(self.y, self.y, self.z) }
	#[inline(always)]
	pub fn yzx(&self) -> Self { Self::new(self.y, self.z, self.x) }
	#[inline(always)]
	pub fn yzy(&self) -> Self { Self::new(self.y, self.z, self.y) }
	#[inline(always)]
	pub fn yzz(&self) -> Self { Self::new(self.y, self.z, self.z) }
	#[inline(always)]
	pub fn zxx(&self) -> Self { Self::new(self.z, self.x, self.x) }
	#[inline(always)]
	pub fn zxy(&self) -> Self { Self::new(self.z, self.x, self.y) }
	#[inline(always)]
	pub fn zxz(&self) -> Self { Self::new(self.z, self.x, self.z) }
	#[inline(always)]
	pub fn zyx(&self) -> Self { Self::new(self.z, self.y, self.x) }
	#[inline(always)]
	pub fn zyy(&self) -> Self { Self::new(self.z, self.y, self.y) }
	#[inline(always)]
	pub fn zyz(&self) -> Self { Self::new(self.z, self.y, self.z) }
	#[inline(always)]
	pub fn zzx(&self) -> Self { Self::new(self.z, self.z, self.x) }
	#[inline(always)]
	pub fn zzy(&self) -> Self { Self::new(self.z, self.z, self.y) }
	#[inline(always)]
	pub fn zzz(&self) -> Self { Self::new(self.z, self.z, self.z) }
	#[inline(always)]
	pub fn xxxx(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.x, self.x) }
	#[inline(always)]
	pub fn xxxy(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.x, self.y) }
	#[inline(always)]
	pub fn xxxz(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.x, self.z) }
	#[inline(always)]
	pub fn xxyx(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.y, self.x) }
	#[inline(always)]
	pub fn xxyy(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.y, self.y) }
	#[inline(always)]
	pub fn xxyz(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.y, self.z) }
	#[inline(always)]
	pub fn xxzx(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.z, self.x) }
	#[inline(always)]
	pub fn xxzy(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.z, self.y) }
	#[inline(always)]
	pub fn xxzz(&self) -> Vec4<T> { Vec4::new(self.x, self.x, self.z, self.z) }
	#[inline(always)]
	pub fn xyxx(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.x, self.x) }
	#[inline(always)]
	pub fn xyxy(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.x, self.y) }
	#[inline(always)]
	pub fn xyxz(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.x, self.z) }
	#[inline(always)]
	pub fn xyyx(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.y, self.x) }
	#[inline(always)]
	pub fn xyyy(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.y, self.y) }
	#[inline(always)]
	pub fn xyyz(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.y, self.z) }
	#[inline(always)]
	pub fn xyzx(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.z, self.x) }
	#[inline(always)]
	pub fn xyzy(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.z, self.y) }
	#[inline(always)]
	pub fn xyzz(&self) -> Vec4<T> { Vec4::new(self.x, self.y, self.z, self.z) }
	#[inline(always)]
	pub fn xzxx(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.x, self.x) }
	#[inline(always)]
	pub fn xzxy(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.x, self.y) }
	#[inline(always)]
	pub fn xzxz(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.x, self.z) }
	#[inline(always)]
	pub fn xzyx(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.y, self.x) }
	#[inline(always)]
	pub fn xzyy(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.y, self.y) }
	#[inline(always)]
	pub fn xzyz(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.y, self.z) }
	#[inline(always)]
	pub fn xzzx(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.z, self.x) }
	#[inline(always)]
	pub fn xzzy(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.z, self.y) }
	#[inline(always)]
	pub fn xzzz(&self) -> Vec4<T> { Vec4::new(self.x, self.z, self.z, self.z) }
	#[inline(always)]
	pub fn yxxx(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.x, self.x) }
	#[inline(always)]
	pub fn yxxy(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.x, self.y) }
	#[inline(always)]
	pub fn yxxz(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.x, self.z) }
	#[inline(always)]
	pub fn yxyx(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.y, self.x) }
	#[inline(always)]
	pub fn yxyy(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.y, self.y) }
	#[inline(always)]
	pub fn yxyz(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.y, self.z) }
	#[inline(always)]
	pub fn yxzx(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.z, self.x) }
	#[inline(always)]
	pub fn yxzy(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.z, self.y) }
	#[inline(always)]
	pub fn yxzz(&self) -> Vec4<T> { Vec4::new(self.y, self.x, self.z, self.z) }
	#[inline(always)]
	pub fn yyxx(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.x, self.x) }
	#[inline(always)]
	pub fn yyxy(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.x, self.y) }
	#[inline(always)]
	pub fn yyxz(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.x, self.z) }
	#[inline(always)]
	pub fn yyyx(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.y, self.x) }
	#[inline(always)]
	pub fn yyyy(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.y, self.y) }
	#[inline(always)]
	pub fn yyyz(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.y, self.z) }
	#[inline(always)]
	pub fn yyzx(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.z, self.x) }
	#[inline(always)]
	pub fn yyzy(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.z, self.y) }
	#[inline(always)]
	pub fn yyzz(&self) -> Vec4<T> { Vec4::new(self.y, self.y, self.z, self.z) }
	#[inline(always)]
	pub fn yzxx(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.x, self.x) }
	#[inline(always)]
	pub fn yzxy(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.x, self.y) }
	#[inline(always)]
	pub fn yzxz(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.x, self.z) }
	#[inline(always)]
	pub fn yzyx(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.y, self.x) }
	#[inline(always)]
	pub fn yzyy(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.y, self.y) }
	#[inline(always)]
	pub fn yzyz(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.y, self.z) }
	#[inline(always)]
	pub fn yzzx(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.z, self.x) }
	#[inline(always)]
	pub fn yzzy(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.z, self.y) }
	#[inline(always)]
	pub fn yzzz(&self) -> Vec4<T> { Vec4::new(self.y, self.z, self.z, self.z) }
	#[inline(always)]
	pub fn zxxx(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.x, self.x) }
	#[inline(always)]
	pub fn zxxy(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.x, self.y) }
	#[inline(always)]
	pub fn zxxz(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.x, self.z) }
	#[inline(always)]
	pub fn zxyx(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.y, self.x) }
	#[inline(always)]
	pub fn zxyy(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.y, self.y) }
	#[inline(always)]
	pub fn zxyz(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.y, self.z) }
	#[inline(always)]
	pub fn zxzx(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.z, self.x) }
	#[inline(always)]
	pub fn zxzy(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.z, self.y) }
	#[inline(always)]
	pub fn zxzz(&self) -> Vec4<T> { Vec4::new(self.z, self.x, self.z, self.z) }
	#[inline(always)]
	pub fn zyxx(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.x, self.x) }
	#[inline(always)]
	pub fn zyxy(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.x, self.y) }
	#[inline(always)]
	pub fn zyxz(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.x, self.z) }
	#[inline(always)]
	pub fn zyyx(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.y, self.x) }
	#[inline(always)]
	pub fn zyyy(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.y, self.y) }
	#[inline(always)]
	pub fn zyyz(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.y, self.z) }
	#[inline(always)]
	pub fn zyzx(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.z, self.x) }
	#[inline(always)]
	pub fn zyzy(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.z, self.y) }
	#[inline(always)]
	pub fn zyzz(&self) -> Vec4<T> { Vec4::new(self.z, self.y, self.z, self.z) }
	#[inline(always)]
	pub fn zzxx(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.x, self.x) }
	#[inline(always)]
	pub fn zzxy(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.x, self.y) }
	#[inline(always)]
	pub fn zzxz(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.x, self.z) }
	#[inline(always)]
	pub fn zzyx(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.y, self.x) }
	#[inline(always)]
	pub fn zzyy(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.y, self.y) }
	#[inline(always)]
	pub fn zzyz(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.y, self.z) }
	#[inline(always)]
	pub fn zzzx(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.z, self.x) }
	#[inline(always)]
	pub fn zzzy(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.z, self.y) }
	#[inline(always)]
	pub fn zzzz(&self) -> Vec4<T> { Vec4::new(self.z, self.z, self.z, self.z) }
}

impl<T> Vec3<T> where T:Vec + Math
{
	#[inline]
	pub fn dot(&self, b: Self) -> T 
	{
		return self.x * b.x + self.y * b.y + self.z * b.z;
	}

	#[inline]	
	pub fn cross(&self, b: Self) -> Self 
	{
		Self
		{
			x:self.y * b.z - self.z * b.y,
			y:self.z * b.x - self.x * b.z,
			z:self.x * b.y - self.y * b.x 
		}
	}

	#[inline]	
	pub fn length2(&self) -> T 
	{
		return self.dot(*self);
	}

	#[inline]	
	pub fn length(&self) -> T
	{
		return self.length2().sqrt();
	}

	#[inline]	
	pub fn distance(&self, b: Self) -> T 
	{
		return (*self - b).length();
	}

	#[inline]
	pub fn normalize(&self) -> Self 
	{
		let mag_sq = self.length2();
		if mag_sq.gt(T::zero())
		{
			let inv_sqrt = T::one() / mag_sq.sqrt();
			return *self * inv_sqrt;
		}

		return *self;
	}
}

impl<T> Math for Vec3<T> where T:Copy + Math
	
{	#[inline]
	fn abs(self) -> Self
	{
		let mx = self.x.abs();
		let my = self.y.abs();
		let mz = self.z.abs();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn recip(self) -> Self
	{
		let mx = self.x.recip();
		let my = self.y.recip();
		let mz = self.z.recip();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn sqrt(self) -> Self
	{
		let mx = self.x.sqrt();
		let my = self.y.sqrt();
		let mz = self.z.sqrt();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn rsqrt(self) -> Self
	{
		let mx = self.x.rsqrt();
		let my = self.y.rsqrt();
		let mz = self.z.rsqrt();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn sin(self) -> Self
	{
		let mx = self.x.sin();
		let my = self.y.sin();
		let mz = self.z.sin();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn cos(self) -> Self
	{
		let mx = self.x.cos();
		let my = self.y.cos();
		let mz = self.z.cos();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn tan(self) -> Self
	{
		let mx = self.x.tan();
		let my = self.y.tan();
		let mz = self.z.tan();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn sincos(self) -> (Self, Self)
	{
		let mx = self.x.sincos();
		let my = self.y.sincos();
		let mz = self.z.sincos();
		(
			Self { x: mx.0, y: my.0, z: mz.0 },
			Self { x: mx.1, y: my.1, z: mz.1 }
		)
	}
	
	#[inline]
	fn acos(self) -> Self
	{
		let mx = self.x.acos();
		let my = self.y.acos();
		let mz = self.z.acos();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn asin(self) -> Self
	{
		let mx = self.x.asin();
		let my = self.y.asin();
		let mz = self.z.asin();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn atan(self) -> Self
	{
		let mx = self.x.atan();
		let my = self.y.atan();
		let mz = self.z.atan();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn exp(self) -> Self
	{
		let mx = self.x.exp();
		let my = self.y.exp();
		let mz = self.z.exp();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn exp2(self) -> Self
	{
		let mx = self.x.exp2();
		let my = self.y.exp2();
		let mz = self.z.exp2();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn log(self, _rhs:Self) -> Self
	{
		let mx = self.x.log(_rhs.x);
		let my = self.y.log(_rhs.y);
		let mz = self.z.log(_rhs.z);
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn log2(self) -> Self
	{
		let mx = self.x.log2();
		let my = self.y.log2();
		let mz = self.z.log2();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn log10(self) -> Self
	{
		let mx = self.x.log10();
		let my = self.y.log10();
		let mz = self.z.log10();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn to_radians(self) -> Self
	{
		let mx = self.x.to_radians();
		let my = self.y.to_radians();
		let mz = self.z.to_radians();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn to_degrees(self) -> Self
	{
		let mx = self.x.to_degrees();
		let my = self.y.to_degrees();
		let mz = self.z.to_degrees();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn min(self, _rhs: Self) -> Self
	{
		let mx = self.x.min(_rhs.x);
		let my = self.y.min(_rhs.y);
		let mz = self.z.min(_rhs.z);
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn max(self, _rhs: Self) -> Self
	{
		let mx = self.x.max(_rhs.x);
		let my = self.y.max(_rhs.y);
		let mz = self.z.max(_rhs.z);
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn saturate(self) -> Self
	{
		let mx = self.x.saturate();
		let my = self.y.saturate();
		let mz = self.z.saturate();
		Self { x: mx, y: my, z: mz }
	}

	#[inline]
	fn snorm2unorm(self) -> Self
	{
		let mx = self.x.snorm2unorm();
		let my = self.y.snorm2unorm();
		let mz = self.z.snorm2unorm();
		Self { x: mx, y: my, z: mz }
	}

	#[inline]
	fn unorm2snorm(self) -> Self
	{
		let mx = self.x.unorm2snorm();
		let my = self.y.unorm2snorm();
		let mz = self.z.unorm2snorm();
		Self { x: mx, y: my, z: mz }
	}
	
	#[inline]
	fn clamp(self, minval: Self, maxval: Self) -> Self
	{
		let mx = self.x.clamp(minval.x, maxval.x);
		let my = self.y.clamp(minval.y, maxval.y);
		let mz = self.z.clamp(minval.z, maxval.z);
		Self { x: mx, y: my, z: mz }
	}
}

impl<T> Lerp<T> for Vec3<T> where T: Copy + One + Mul<Output=T> + Add<Output=T> + Sub<Output=T>
{
	#[inline(always)]
	fn lerp(self, b: Self, t: T) -> Self 
	{
		return self*(T::one() - t) + b*t;
	}
}

impl<T> Slerp<T> for Vec3<T> where T: Vec + Math
{
	fn slerp(self, other: Self, factor: T) -> Self 
	{
		let dot = self.dot(other);

		let s = T::one() - factor;
		let t = if dot.gt(T::zero()) { factor } else { -factor };
		let q = self * s + other * t;

		q.normalize()
	}
}

impl<T> Zero for Vec3<T> where T:Zero
{
	#[inline(always)]
	fn zero() -> Self
	{
		Self
		{
			x: T::zero(), y: T::zero(), z: T::zero() 
		}
	}
}

impl<T> One for Vec3<T> where T:One
{
	#[inline(always)]
	fn one() -> Self
	{
		Self
		{
			x: T::one(), y: T::one(), z: T::one() 
		}
	}
}

impl<T> UnitX for Vec3<T> where T:One + Zero
{
	#[inline(always)]
	fn unit_x() -> Self
	{
		Self
		{ 
			x: T::one(), y: T::zero(), z: T::zero()
		}
	}
}

impl<T> UnitY for Vec3<T> where T:One + Zero
{
	#[inline(always)]
	fn unit_y() -> Self
	{
		Self
		{ 
			x: T::zero(), y: T::one(), z: T::zero()
		}
	}
}

impl<T> UnitZ for Vec3<T> where T:One + Zero
{
	#[inline(always)]
	fn unit_z() -> Self
	{
		Self
		{ 
			x: T::zero(), y: T::one(), z: T::zero()
		}
	}
}

impl<T> fmt::Display for Vec3<T> where T:Debug
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
	}
}

impl<T> fmt::Binary for Vec3<T> where T:Vec + Math
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let len = self.length();
		let decimals = f.precision().unwrap_or(3);
		let string = format!("{:.*?}", decimals, len);
		f.pad_integral(true, "", &string)
	}
}

impl<T> From<[T;3]> for Vec3<T> where T:Copy
{
	fn from(v:[T;3]) -> Self
	{
		Self
		{
			x:v[0],
			y:v[1],
			z:v[2],
		}
	}
}

impl<T> From<(T,T,T)> for Vec3<T> where T:Copy
{
	fn from(v:(T,T,T)) -> Self
	{
		Self
		{
			x:v.0,
			y:v.1,
			z:v.2,
		}
	}
}

impl<T> AsRef<Vec3<T>> for Vec3<T>
{
	fn as_ref(&self) -> &Vec3<T>
	{
		self
	}
}

impl<T> AsMut<Vec3<T>> for Vec3<T>
{
	fn as_mut(&mut self) -> &mut Vec3<T>
	{
		self
	}
}