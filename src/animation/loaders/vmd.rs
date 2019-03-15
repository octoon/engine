use std::io::{Cursor};
use std::f32;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use byteorder::{LittleEndian, ReadBytesExt};
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J;
use super::super::{Error, Result, Animator, AnimationClip, AnimationCurve};

pub struct VMDName{}
pub struct VMDVector2{}
pub struct VMDVector3{}
pub struct VMDVector4{}

#[derive(Debug)]
pub struct VMDHeader
{
	pub magic:String, // Fixed: 30 length
	pub name:String, // Fixed: 20 length
}

#[derive(Debug)]
pub struct VMDMotion
{
	pub name:String, // Fixed: 15 length
	pub frame:u32,
	pub position:(f32,f32,f32),
	pub rotate:(f32,f32,f32,f32),
	pub bezier_x:[u8; 16], // X1(x,y,z,r), Y1(x,y,z,r), X2(x,y,z,r), Y2(x,y,z,r)
	pub bezier_y:[u8; 16], // X1(x,y,z,r), Y1(x,y,z,r), X2(x,y,z,r), Y2(x,y,z,r)
	pub bezier_z:[u8; 16], // X1(x,y,z,r), Y1(x,y,z,r), X2(x,y,z,r), Y2(x,y,z,r)
	pub bezier_r:[u8; 16], // X1(x,y,z,r), Y1(x,y,z,r), X2(x,y,z,r), Y2(x,y,z,r)
}

#[derive(Debug)]
pub struct VMDMorph
{
	pub name:String, // Fixed: 15 length
	pub frame:u32,
	pub weight:f32,
}

#[derive(Debug)]
pub struct VMDCamera
{
	pub frame:u32,
	pub length:f32,
	pub position:(f32,f32,f32),
	pub rotation:(f32,f32,f32),
	pub bezier:[u8;24],
	pub view_angle:u32,
	pub perspective:u8,
}

#[derive(Debug)]
pub struct VMDLight
{
	pub frame:u32,
	pub rgb:(f32,f32,f32),
	pub position:(f32,f32,f32),
}

#[derive(Debug)]
pub struct VMDSelfShadow
{
	pub frame:u32,
	pub mode:u8, // 00-02
	pub distance:f32, // 0.1 - (dist * 0.000001)
}

#[derive(Debug)]
pub struct VMDFile
{
	pub header:VMDHeader,
	pub motions:Vec<VMDMotion>,
	pub morphs:Vec<VMDMorph>,
	pub cameras:Vec<VMDCamera>,
	pub lights:Vec<VMDLight>,
	pub self_shadows:Vec<VMDSelfShadow>,
}

impl VMDName
{
	pub fn load(reader:&mut Cursor<&[u8]>, length:usize) -> Result<String>
	{
		let mut bytes = Vec::with_capacity(length);

		for _ in 0..length
		{
			let value = reader.read_u8()?;
			if value != 0
			{
				bytes.push(value);
			}
		}

		Ok(WINDOWS_31J.decode(&bytes, DecoderTrap::Ignore).unwrap())
	}
}

impl VMDVector2
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		Ok((x, y))
	}
}

impl VMDVector3
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		let z = reader.read_f32::<LittleEndian>()?;
		Ok((x, y, z))
	}
}

impl VMDVector4
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32,f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		let z = reader.read_f32::<LittleEndian>()?;
		let w = reader.read_f32::<LittleEndian>()?;
		Ok((x, y, z, w))
	}
}

impl VMDHeader
{
	pub fn new() -> Self
	{
		Self
		{
			magic:String::new(),
			name:String::new(),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		if self.magic != "Vocaloid Motion Data 0002" { return Err(Error("Invalid header of magic.".to_string())); };
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDHeader::new();
		this.magic = VMDName::load(reader, 30)?;
		this.name = VMDName::load(reader, 20)?;

		this.valid()
	}
}

impl VMDMotion
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			frame:0,
			position:(0.0,0.0,0.0),
			rotate:(0.0,0.0,0.0,0.0),
			bezier_x:[0;16],
			bezier_y:[0;16],
			bezier_z:[0;16],
			bezier_r:[0;16],
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDMotion::new();
		this.name = VMDName::load(reader, 15)?;
		this.frame = reader.read_u32::<LittleEndian>()?;
		this.position = VMDVector3::load(reader)?;
		this.rotate = VMDVector4::load(reader)?;
		for i in 0..this.bezier_x.len() { this.bezier_x[i] = reader.read_u8()?; }
		for i in 0..this.bezier_y.len() { this.bezier_y[i] = reader.read_u8()?; }
		for i in 0..this.bezier_z.len() { this.bezier_z[i] = reader.read_u8()?; }
		for i in 0..this.bezier_r.len() { this.bezier_r[i] = reader.read_u8()?; }

		this.valid()
	}

	pub fn load_array(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let num_motion = reader.read_u32::<LittleEndian>()?;
		let mut motions = Vec::with_capacity(num_motion as usize);

		for _ in 0..num_motion
		{
			motions.push(VMDMotion::load(reader)?);
		}

		Ok(motions)
	}
}

impl VMDMorph
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			frame:0,
			weight:0.0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDMorph::new();
		this.name = VMDName::load(reader, 15)?;
		this.frame = reader.read_u32::<LittleEndian>()?;
		this.weight = reader.read_f32::<LittleEndian>()?;

		this.valid()
	}

	pub fn load_array(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let num_morph = reader.read_u32::<LittleEndian>()?;
		let mut morphs = Vec::with_capacity(num_morph as usize);

		for _ in 0..num_morph
		{
			morphs.push(VMDMorph::load(reader)?);
		}

		Ok(morphs)
	}
}

impl VMDCamera
{
	pub fn new() -> Self
	{
		Self
		{
			frame:0,
			length:0.0,
			position:(0.0,0.0,0.0),
			rotation:(0.0,0.0,0.0),
			bezier:[0;24],
			view_angle:0,
			perspective:0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDCamera::new();
		this.frame = reader.read_u32::<LittleEndian>()?;
		this.position = VMDVector3::load(reader)?;
		this.rotation = VMDVector3::load(reader)?;
		for i in 0..this.bezier.len() { this.bezier[i] = reader.read_u8()?; }
		this.view_angle = reader.read_u32::<LittleEndian>()?;
		this.perspective = reader.read_u8()?;

		this.valid()
	}

	pub fn load_array(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let num_camera = reader.read_u32::<LittleEndian>()?;
		let mut cameras = Vec::with_capacity(num_camera as usize);

		for _ in 0..num_camera
		{
			cameras.push(VMDCamera::load(reader)?);
		}

		Ok(cameras)
	}
}

impl VMDLight
{
	pub fn new() -> Self
	{
		Self
		{
			frame:0,
			rgb:(0.0,0.0,0.0),
			position:(0.0,0.0,0.0),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDLight::new();
		this.frame = reader.read_u32::<LittleEndian>()?;
		this.rgb = VMDVector3::load(reader)?;
		this.position = VMDVector3::load(reader)?;

		this.valid()
	}

	pub fn load_array(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let num_light = reader.read_u32::<LittleEndian>()?;
		let mut lights = Vec::with_capacity(num_light as usize);

		for _ in 0..num_light
		{
			lights.push(VMDLight::load(reader)?);
		}

		Ok(lights)
	}
}

impl VMDSelfShadow
{
	pub fn new() -> Self
	{
		Self
		{
			frame:0,
			mode:0,
			distance:0.0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = VMDSelfShadow::new();
		this.frame = reader.read_u32::<LittleEndian>()?;
		this.mode = reader.read_u8()?;
		this.distance = reader.read_f32::<LittleEndian>()?;

		this.valid()
	}

	pub fn load_array(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let num_shadow = reader.read_u32::<LittleEndian>()?;
		let mut shadows = Vec::with_capacity(num_shadow as usize);

		for _ in 0..num_shadow
		{
			shadows.push(VMDSelfShadow::load(reader)?);
		}

		Ok(shadows)
	}
}

impl VMDFile
{
	pub fn new() -> Self
	{
		Self
		{
			header:VMDHeader::new(),
			motions:Vec::new(),
			morphs:Vec::new(),
			cameras:Vec::new(),
			lights:Vec::new(),
			self_shadows:Vec::new(),
		}
	}

	pub fn load(buf:&[u8]) -> Result<Self>
	{
		let mut reader = Cursor::new(buf);

		Ok(Self
		{
			header:VMDHeader::load(&mut reader)?,
			motions:VMDMotion::load_array(&mut reader)?,
			morphs:VMDMorph::load_array(&mut reader)?,
			cameras:VMDCamera::load_array(&mut reader)?,
			lights:VMDLight::load_array(&mut reader)?,
			self_shadows:VMDSelfShadow::load_array(&mut reader)?,
		})
	}

	fn collect_motions(&self) -> Animator
	{
		let mut curves = HashMap::new();
		for motion in self.motions.iter()
		{
			if !curves.contains_key(&motion.name)
			{
				curves.insert(motion.name.clone(), 
					(
						AnimationCurve::new(),
						AnimationCurve::new(),
						AnimationCurve::new(),
						AnimationCurve::new(), 
						AnimationCurve::new(),
						AnimationCurve::new(),
						AnimationCurve::new()
					));
			}

			if let Some(curve) = curves.get_mut(&motion.name)
			{
				curve.0.add_keyframe(motion.frame as f32, motion.position.0, None);
				curve.1.add_keyframe(motion.frame as f32, motion.position.1, None);
				curve.2.add_keyframe(motion.frame as f32, motion.position.2, None);
				curve.3.add_keyframe(motion.frame as f32, motion.rotate.0, None);
				curve.4.add_keyframe(motion.frame as f32, motion.rotate.1, None);
				curve.5.add_keyframe(motion.frame as f32, motion.rotate.2, None);
				curve.6.add_keyframe(motion.frame as f32, motion.rotate.3, None);
			}
		}

		let mut animator = Animator::new();
		for (key, curve) in curves
		{
			let mut clip = AnimationClip::new();
			clip.set_name(&key);
			clip.set_curve("Position.X", curve.0);
			clip.set_curve("Position.Y", curve.1);
			clip.set_curve("Position.Z", curve.2);
			clip.set_curve("Rotation.X", curve.3);
			clip.set_curve("Rotation.Y", curve.4);
			clip.set_curve("Rotation.Z", curve.5);
			clip.set_curve("Rotation.W", curve.6);
			
			animator.add_clip(clip);
		}

		animator
	}

	fn collect_morphs(&self) -> Animator
	{
		let mut curves = HashMap::new();
		for morph in self.morphs.iter()
		{
			if !curves.contains_key(&morph.name)
			{
				curves.insert(morph.name.clone(), AnimationCurve::new());
			}

			if let Some(curve) = curves.get_mut(&morph.name)
			{
				curve.add_keyframe(morph.frame as f32, morph.weight, None);
			}
		}

		let mut animator = Animator::new();
		for (key, curve) in curves
		{
			let mut clip = AnimationClip::new();
			clip.set_name(&key);
			clip.set_curve("Weight", curve);

			animator.add_clip(clip);
		}

		animator
	}
}

#[derive(Debug)]
pub struct VMDLoader {}

impl VMDLoader
{
	pub fn new() -> Self
	{
		Self
		{			
		}
	}

	pub fn can_read(&self, buf:&[u8]) -> bool
	{
		VMDHeader::load(&mut Cursor::new(buf)).is_ok()
	}

	pub fn do_save(_buf:&[u8], _model:&mut VMDFile) -> Result<()>
	{
		Err(Error("Not Implmention yet".to_string()))
	}

	pub fn do_load(&self, buf:&[u8]) -> Result<Animator>
	{
		let vmd = VMDFile::load(buf)?;
		let motions = vmd.collect_motions();

		Ok(motions)
	}

	pub fn open<P: AsRef<std::path::Path>>(path:P) -> Result<Animator>
	{
		let mut buffer = Vec::new();
		File::open(path)?.read_to_end(&mut buffer)?;
		VMDLoader::new().do_load(&buffer)
	}
}