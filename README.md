octoon-math
-------------
octoon-math is designed for rust developers working on games and compute graphics

Usage
---------
Add this to your Cargo.toml:

```toml
[dependencies]
octoon-math = { version = "0.1", features = ["serialize"] }
```

Examples
--------
```rust
extern crate octoon_math;

use octoon_math::*;
use octoon_math::consts::*;

fn main()
{
// new float2
    let a = float2::new(1.0,0.0);
    let b = float!(0.0,1.0); // same as float2::new

// new float3
    let c = float3::new(1.0,0.0,0.0);
    let d = float!(0.0,1.0,0.0); // same as float3::new

// new float4
    let e = float4::new(1.0,0.0,0.0,0.0);
    let f = float!(0.0,1.0,0.0,0.0); // same as float4::new

// new array
	let x = 0.5;
	let y = 0.5;
	let z = 0.5;

	let vertices:Vec<float3> = vec![
		float!(-x,-y,-z), float!( x,-y,-z), float!( x, y,-z), float!(-x, y,-z), 
		float!(-x,-y, z), float!( x,-y, z), float!( x, y, z), float!(-x, y, z),
		float!(-x,-y,-z), float!(-x, y,-z), float!(-x, y, z), float!(-x,-y, z),
		float!( x,-y,-z), float!( x, y,-z), float!( x, y, z), float!( x,-y, z), 
		float!(-x,-y,-z), float!(-x,-y, z), float!( x,-y, z), float!( x,-y,-z),
		float!(-x, y,-z), float!(-x, y, z), float!( x, y, z), float!( x, y,-z), 
	];

// operators
	let ab = a.dot(b);
	let cd = c.cross(d);
	let ef = e.lerp(f, 0.5);
	let len = a.length();

// consts
	let pi = f32::pi();
	let pi2 = f32::pi2();
	let pi4 = f32::pi4();
	let one = float2::one();
	let zero = float2::zero();
	let unitx = float3::unit_x(); // say: float3::new(1.0, 0.0, 0.0)
	let unity = float3::unit_y(); // say: float3::new(0.0, 1.0, 0.0)
	let unitz = float3::unit_z(); // say: float3::new(0.0, 0.0, 1.0)
}
```