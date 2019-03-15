use std::io::{Cursor};
use std::collections::HashMap;
use std::f32;
use byteorder::{LittleEndian, ReadBytesExt};
use super::super::{Loader, Model, Mesh, Material, VertexWeight, Bone, BoneLink, Solver, Error, Result};

pub const PMX_VERSION_2_0: f32 = 2.0;
pub const PMX_BONE_INDEX: u16 = 1 << 0;
pub const PMX_BONE_MOVE: u16 = 1 << 1;
pub const PMX_BONE_DISPLAY: u16 = 1 << 2;
pub const PMX_BONE_OPERATOR: u16 = 1 << 3;
pub const PMX_BONE_ROOT: u16 = 1 << 4;
pub const PMX_BONE_IK: u16 = 1 << 5;
pub const PMX_BONE_PARENT: u16 = 1 << 8;
pub const PMX_BONE_AXIS: u16 = 1 << 10;
pub const PMX_BONE_ROTATE: u16 = 1 << 11;

// PmxVertexSkinningType
pub const PMX_BDEF1:u8 = 0;
pub const PMX_BDEF2:u8 = 1;
pub const PMX_BDEF4:u8 = 2;
pub const PMX_SDEF:u8 = 3;
pub const PMX_QDEF:u8 = 4;

pub struct PmxVector2();
pub struct PmxVector3();
pub struct PmxVector4();
pub struct PmxName();
pub struct PmxInteger();

#[derive(Debug)]
pub struct PmxHeader
{
	pub magic:[u8;3],       // PMX
	pub offset:u8,         	 // 0x20
	pub version:f32,         // 2.0
	pub data_size:u8,        // 0x08
	pub encode:u8,         	 // 0x00 UTF-16(LE) 0x01 UTF-8
	pub add_uv_count:u8,     // ( 1 ~ 4 )
	pub sizeof_indices:u8,  // ( 1 or 2 or 4 )
	pub sizeof_texture:u8,  // ( 1 or 2 or 4 )
	pub sizeof_material:u8, // ( 1 or 2 or 4 )
	pub sizeof_bone:u8,     // ( 1 or 2 )
	pub sizeof_morph:u8,    // ( 1 or 2 or 4 )
	pub sizeof_body:u8,     // ( 1 or 2 or 4 )
}

#[derive(Debug)]
pub struct PmxDescription
{
	pub japan_model_length:u32,
	pub japan_comment_length:u32,

	pub english_model_length:u32,
	pub english_comment_length:u32,

	pub japan_model_name:Vec<u8>,
	pub japan_comment_name:Vec<u8>,

	pub english_model_name:Vec<u8>,
	pub english_comment_name:Vec<u8>,
}

#[derive(Debug)]
pub struct PmxBoneWeight
{
	pub kind:u8, // PmxVertexSkinningType

	pub bone1:u16,
	pub bone2:u16,
	pub bone3:u16,
	pub bone4:u16,

	pub weight1:f32,
	pub weight2:f32,
	pub weight3:f32,
	pub weight4:f32,

	pub sdef_c:(f32,f32,f32),
	pub sdef_r0:(f32,f32,f32),
	pub sdef_r1:(f32,f32,f32),
}

#[derive(Debug)]
pub struct PmxVertex
{
	pub position:(f32,f32,f32),
	pub normal:(f32,f32,f32),
	pub coord:(f32,f32),
	pub add_coord:[(f32,f32,f32,f32);8],
	pub weight:PmxBoneWeight,
	pub edge:f32,
}

#[derive(Debug)]
pub enum PmxIndex
{
	Uint8(Vec<u8>),
	Uint16(Vec<u16>),
	Uint32(Vec<u32>),
}

#[derive(Debug)]
pub struct PmxMaterial
{
	pub name:String,
	pub name_eng:String,
	pub diffuse:(f32,f32,f32),
	pub opacity:f32,
	pub specular:(f32,f32,f32),
	pub shininess:f32,
	pub ambient:(f32,f32,f32),
	pub flag:u8,
	pub edge_color:(f32,f32,f32,f32),
	pub edge_size:f32,
	pub texture_index:i16,
	pub sphere_texture_index:i16,
	pub sphere_mode:u8,
	pub toon_index:u8,
	pub toon_texture:i16,
	pub mem:String,
	pub face_count:u32,
}

#[derive(Debug)]
pub struct PmxIK
{
	pub bone_index:i16,
	pub rotate_limited:u8,
	pub minimum_radian:(f32,f32,f32),
	pub maximum_radian:(f32,f32,f32),
}

#[derive(Debug)]
pub struct PmxBone
{
	pub name:String,
	pub name_eng:String,
	pub position:(f32,f32,f32),
	pub parent:i16,
	pub level:u32,
	pub flag:u16,
	pub connected_bone_index:i16,
	pub provided_parent_bone_index:i16,
	pub provided_ratio:f32,
	pub offset:(f32,f32,f32),
	pub axis_direction:(f32,f32,f32),
	pub dimention_direction_x:(f32,f32,f32),
	pub dimention_direction_z:(f32,f32,f32),
	pub ik_target_bone_index:i16,
	pub ik_loop_count:u32,
	pub ik_limited_radian:f32,
	pub ik_links:Vec<PmxIK>,
}

#[derive(Debug)]
pub struct PMXFile
{
	pub hdr:PmxHeader,
	pub description:PmxDescription,
	pub vertices:Vec<PmxVertex>,
	pub indices:PmxIndex,
	pub textures:Vec<String>,
	pub materials:Vec<PmxMaterial>,
	pub bones:Vec<PmxBone>
}

impl PmxVector2
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		Ok((x, y))
	}
}

impl PmxVector3
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		let z = reader.read_f32::<LittleEndian>()?;
		Ok((x, y, z))
	}
}

impl PmxVector4
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

impl PmxName
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<String>
	{
		let length = reader.read_u32::<LittleEndian>()?;
		let mut name = Vec::new();

		for _ in 0..length / 2
		{
			name.push(reader.read_u16::<LittleEndian>()?);
		}

		Ok(String::from_utf16(&name)?)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<String>>
	{
		let mut names = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..names.capacity()
		{
			names.push(PmxName::load(reader)?);
		}

		Ok(names)
	}
}

impl PmxInteger
{
	pub fn load(reader:&mut Cursor<&[u8]>, len:u8) -> Result<i32>
	{
		let value;
		if len == 1 { value = reader.read_i8()? as i32; }
		else if len == 2 { value = reader.read_i16::<LittleEndian>()? as i32; }
		else if len == 3 { value = reader.read_i32::<LittleEndian>()? as i32; }
		else { return Err(Error("Invalid length of byte".to_string())); }

		Ok(value)
	}
}

impl PmxHeader
{
	pub fn new() -> Self
	{
		Self
		{
			magic:[0;3],
			offset:0,
			version:2.0,
			data_size:0,
			encode:0,
			add_uv_count:0,
			sizeof_indices:0,
			sizeof_texture:0,
			sizeof_material:0,
			sizeof_bone:0,
			sizeof_morph:0,
			sizeof_body:0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		if self.magic[0] != 'p' as u8 && self.magic[0] != 'P' as u8 { return Err(Error("Invalid magic in PMX Header".to_string())); }
		if self.magic[1] != 'm' as u8 && self.magic[1] != 'M' as u8 { return Err(Error("Invalid magic in PMX Header".to_string())); }
		if self.magic[2] != 'x' as u8 && self.magic[2] != 'X' as u8 { return Err(Error("Invalid magic in PMX Header".to_string())); }
		if self.version != 2.0 { return Err(Error("Invalid version in PMX Header".to_string())); }
		if self.offset == 0 { return Err(Error("Invalid offset in PMX Header".to_string())); }
		if self.data_size == 0 { return Err(Error("Invalid data_size in PMX Header".to_string())); }
		if self.encode != 0x0 && self.encode != 0x1 { return Err(Error("Invalid encode in PMX Header".to_string())); }
		if self.add_uv_count > 8 { return Err(Error("Invalid add_uv_count in PMX Header".to_string())); }
		if self.sizeof_indices != 1 && self.sizeof_indices != 2 && self.sizeof_indices != 4 { return Err(Error("Invalid sizeof_indices in PMX Header".to_string())); }
		if self.sizeof_texture != 1 && self.sizeof_texture != 2 { return Err(Error("Invalid sizeof_texture in PMX Header".to_string())); }
		if self.sizeof_material != 1 && self.sizeof_material != 2 && self.sizeof_material != 4 { return Err(Error("Invalid sizeof_material in PMX Header".to_string())); }
		if self.sizeof_bone != 1 && self.sizeof_bone != 2 { return Err(Error("Invalid sizeof_bone in PMX Header".to_string())); }
		if self.sizeof_morph != 1 && self.sizeof_morph != 2 && self.sizeof_morph != 4 { return Err(Error("Invalid sizeof_morph in PMX Header".to_string())); }
		if self.sizeof_body != 1 && self.sizeof_body != 2 && self.sizeof_body != 4 { return Err(Error("Invalid sizeof_body in PMX Header".to_string())); }

		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmxHeader::new();
		this.magic[0] = reader.read_u8()?;
		this.magic[1] = reader.read_u8()?;
		this.magic[2] = reader.read_u8()?;
		this.offset = reader.read_u8()?;
		this.version = reader.read_f32::<LittleEndian>()?;
		this.data_size = reader.read_u8()?;
		this.encode = reader.read_u8()?;
		this.add_uv_count = reader.read_u8()?;
		this.sizeof_indices = reader.read_u8()?;
		this.sizeof_texture = reader.read_u8()?;
		this.sizeof_material = reader.read_u8()?;
		this.sizeof_bone = reader.read_u8()?;
		this.sizeof_morph = reader.read_u8()?;
		this.sizeof_body = reader.read_u8()?;
		this.valid()
	}
}

impl PmxDescription
{
	pub fn new() -> Self
	{
		Self
		{
			japan_model_length:0,
			japan_comment_length:0,
			english_model_length:0,
			english_comment_length:0,
			japan_model_name:Vec::new(),
			japan_comment_name:Vec::new(),
			english_model_name:Vec::new(),
			english_comment_name:Vec::new(),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmxDescription::new();
		this.japan_model_length = reader.read_u32::<LittleEndian>()?;
		for _ in 0..this.japan_model_length
		{
			this.japan_model_name.push(reader.read_u8()?);
		}

		this.english_model_length = reader.read_u32::<LittleEndian>()?;
		for _ in 0..this.english_model_length
		{
			this.japan_model_name.push(reader.read_u8()?);
		}

		this.japan_comment_length = reader.read_u32::<LittleEndian>()?;
		for _ in 0..this.japan_comment_length
		{
			this.japan_comment_name.push(reader.read_u8()?);
		}

		this.english_comment_length = reader.read_u32::<LittleEndian>()?;
		for _ in 0..this.english_comment_length
		{
			this.japan_comment_name.push(reader.read_u8()?);
		}

		this.valid()
	}
}

impl PmxBoneWeight
{
	pub fn new() -> Self
	{
		Self
		{
			kind:0,
			bone1:0,
			bone2:0,
			bone3:0,
			bone4:0,
			weight1:0.0,
			weight2:0.0,
			weight3:0.0,
			weight4:0.0,
			sdef_c:(0.0,0.0,0.0),
			sdef_r0:(0.0,0.0,0.0),
			sdef_r1:(0.0,0.0,0.0),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let mut this = PmxBoneWeight::new();
		this.kind = reader.read_u8()?;
		match this.kind
		{
			PMX_BDEF1 =>
			{
				this.bone1 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.weight1 = 1.0;
			},
			PMX_BDEF2 =>
			{
				this.bone1 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone2 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.weight1 = reader.read_f32::<LittleEndian>()?;
				this.weight2 = 1.0 - this.weight1;
			},
			PMX_BDEF4 =>
			{
				this.bone1 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone2 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone3 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone4 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.weight1 = reader.read_f32::<LittleEndian>()?;
				this.weight2 = reader.read_f32::<LittleEndian>()?;
				this.weight3 = reader.read_f32::<LittleEndian>()?;
				this.weight4 = reader.read_f32::<LittleEndian>()?;
			},
			PMX_SDEF =>
			{
				this.bone1 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone2 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.weight1 = reader.read_f32::<LittleEndian>()?;
				this.sdef_c.0 = reader.read_f32::<LittleEndian>()?;
				this.sdef_c.1 = reader.read_f32::<LittleEndian>()?;
				this.sdef_c.2 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r0.0 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r0.1 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r0.2 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r1.0 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r1.1 = reader.read_f32::<LittleEndian>()?;
				this.sdef_r1.2 = reader.read_f32::<LittleEndian>()?;
				this.weight2 = 1.0 - this.weight1;
			},
			PMX_QDEF =>
			{
				this.bone1 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.bone2 = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
				this.weight1 = reader.read_f32::<LittleEndian>()?;
				this.weight2 = 1.0 - this.weight1;
			},
			_ => 
			{
				return Err(Error("Invalid Token bone type".to_string()));
			}
		}

		this.valid()
	}
}

impl PmxVertex
{
	pub fn new() -> Self
	{
		Self
		{
			position:(0.0,0.0,0.0),
			normal:(0.0,0.0,0.0),
			coord:(0.0,0.0),
			add_coord:[(0.0,0.0,0.0,0.0);8],
			weight:PmxBoneWeight::new(),
			edge:1.0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let mut this = PmxVertex::new();
		this.position = PmxVector3::load(reader)?;
		this.normal = PmxVector3::load(reader)?;
		this.coord = PmxVector2::load(reader)?;

		for i in 0..hdr.add_uv_count
		{
			this.add_coord[i as usize] = PmxVector4::load(reader)?;
		}

		this.weight = PmxBoneWeight::load(reader, hdr)?;
		this.edge = reader.read_f32::<LittleEndian>()?;

		this.valid()
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Vec<Self>>
	{
		let mut vertices = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..vertices.capacity()
		{
			vertices.push(PmxVertex::load(reader, hdr)?);
		}

		Ok(vertices)
	}
}

impl PmxIndex
{
	pub fn load_arrays(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let num_indices = reader.read_u32::<LittleEndian>()?;
		match hdr.sizeof_indices
		{
			1 =>
			{
				let mut indices = Vec::with_capacity(num_indices as usize);

				for _ in 0..num_indices
				{
					indices.push(reader.read_u8()?);
				}

				Ok(PmxIndex::Uint8(indices))
			},
			2 =>
			{
				let mut indices = Vec::with_capacity(num_indices as usize);

				for _ in 0..num_indices
				{
					indices.push(reader.read_u16::<LittleEndian>()?);
				}

				Ok(PmxIndex::Uint16(indices))
			},
			4 =>
			{
				let mut indices = Vec::with_capacity(num_indices as usize);

				for _ in 0..num_indices
				{
					indices.push(reader.read_u32::<LittleEndian>()?);
				}

				Ok(PmxIndex::Uint32(indices))
			},
			_ =>
			{
				return Err(Error("Invalid sizeof of index".to_string()));
			}
		}
	}
}

impl PmxMaterial
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			name_eng:String::new(),
			diffuse:(0.0,0.0,0.0),
			opacity:0.0,
			specular:(0.0,0.0,0.0),
			shininess:0.0,
			ambient:(0.0,0.0,0.0),
			flag:0,
			edge_color:(0.0,0.0,0.0,0.0),
			edge_size:0.0,
			texture_index:0,
			sphere_texture_index:0,
			sphere_mode:0,
			toon_index:0,
			toon_texture:0,
			mem:String::new(),
			face_count:0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let mut this = PmxMaterial::new();
		this.name = PmxName::load(reader)?;
		this.name_eng = PmxName::load(reader)?;
		this.diffuse = PmxVector3::load(reader)?;
		this.opacity = reader.read_f32::<LittleEndian>()?;
		this.specular = PmxVector3::load(reader)?;
		this.shininess = reader.read_f32::<LittleEndian>()?;
		this.ambient = PmxVector3::load(reader)?;
		this.flag = reader.read_u8()?;
		this.edge_color = PmxVector4::load(reader)?;
		this.edge_size = reader.read_f32::<LittleEndian>()?;
		this.texture_index = PmxInteger::load(reader, hdr.sizeof_texture)? as _;
		this.sphere_texture_index = PmxInteger::load(reader, hdr.sizeof_texture)? as _;
		this.sphere_mode = reader.read_u8()?;
		this.toon_index = reader.read_u8()?;
		
		if this.toon_index == 1
		{
			this.toon_texture = reader.read_u8()? as i16;
		}
		else
		{
			this.toon_texture = PmxInteger::load(reader, hdr.sizeof_texture)? as _;
		}

		this.mem = PmxName::load(reader)?;
		this.face_count = reader.read_u32::<LittleEndian>()?;

		this.valid()
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Vec<Self>>
	{
		let mut materials = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..materials.capacity()
		{
			materials.push(Self::load(reader, hdr)?);
		}

		Ok(materials)
	}
}

impl PmxIK
{
	pub fn new() -> Self
	{
		Self
		{
			bone_index:0,
			rotate_limited:0,
			minimum_radian:(0.0,0.0,0.0),
			maximum_radian:(0.0,0.0,0.0),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let mut this = PmxIK::new();
		this.bone_index = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
		this.rotate_limited = reader.read_u8()?;
		
		if this.rotate_limited > 0
		{
			this.minimum_radian = PmxVector3::load(reader)?;
			this.maximum_radian = PmxVector3::load(reader)?;
		}

		this.valid()
	}
}

impl PmxBone
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			name_eng:String::new(),
			position:(0.0,0.0,0.0),
			parent:0,
			level:0,
			flag:0,
			connected_bone_index:0,
			provided_parent_bone_index:0,
			provided_ratio:0.0,
			offset:(0.0,0.0,0.0),
			axis_direction:(0.0,0.0,0.0),
			dimention_direction_x:(0.0,0.0,0.0),
			dimention_direction_z:(0.0,0.0,0.0),
			ik_target_bone_index:0,
			ik_loop_count:0,
			ik_limited_radian:0.0,
			ik_links:Vec::new(),
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Self>
	{
		let mut this = PmxBone::new();
		this.name = PmxName::load(reader)?;
		this.name_eng = PmxName::load(reader)?;
		this.position = PmxVector3::load(reader)?;
		this.parent = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
		this.level = reader.read_u32::<LittleEndian>()?;
		this.flag = reader.read_u16::<LittleEndian>()?;

		if (this.flag & PMX_BONE_INDEX) > 0
		{
			this.connected_bone_index = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
		}
		else
		{
			this.offset = PmxVector3::load(reader)?;
		}

		if (this.flag & PMX_BONE_PARENT) > 0
		{
			this.provided_parent_bone_index = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
			this.provided_ratio = reader.read_f32::<LittleEndian>()?;
		}

		if (this.flag & PMX_BONE_AXIS) > 0
		{
			this.axis_direction = PmxVector3::load(reader)?;
		}

		if (this.flag & PMX_BONE_ROTATE) > 0
		{
			this.dimention_direction_x = PmxVector3::load(reader)?;
			this.dimention_direction_z = PmxVector3::load(reader)?;
		}

		if (this.flag & PMX_BONE_IK) > 0
		{
			this.ik_target_bone_index = PmxInteger::load(reader, hdr.sizeof_bone)? as _;
			this.ik_loop_count = reader.read_u32::<LittleEndian>()?;
			this.ik_limited_radian = reader.read_f32::<LittleEndian>()?;

			for _ in 0..reader.read_u32::<LittleEndian>()?
			{
				this.ik_links.push(PmxIK::load(reader, hdr)?);
			}
		}

		this.valid()
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, hdr:&PmxHeader) -> Result<Vec<Self>>
	{
		let mut bones = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..bones.capacity()
		{
			bones.push(PmxBone::load(reader, hdr)?);
		}

		Ok(bones)
	}
}

impl PMXFile
{
	pub fn new() -> Self
	{
		Self
		{
			hdr:PmxHeader::new(),
			description:PmxDescription::new(),
			vertices:Vec::new(),
			indices:PmxIndex::Uint16(Vec::new()),
			textures:Vec::new(),
			materials:Vec::new(),
			bones:Vec::new(),
		}
	}

	pub fn load(buf:&[u8]) -> Result<Self>
	{
		let mut reader = Cursor::new(buf);
		let hdr = PmxHeader::load(&mut reader)?;
		let description = PmxDescription::load(&mut reader)?;
		let vertices = PmxVertex::load_arrays(&mut reader, &hdr)?;
		let indices = PmxIndex::load_arrays(&mut reader, &hdr)?;
		let textures = PmxName::load_arrays(&mut reader)?;
		let materials = PmxMaterial::load_arrays(&mut reader, &hdr)?;
		let bones = PmxBone::load_arrays(&mut reader, &hdr)?;

		Ok(Self
		{
			hdr:hdr,
			description:description,
			vertices:vertices,
			indices:indices,
			textures:textures,
			materials:materials,
			bones:bones
		})
	}
}

#[derive(Debug)]
pub struct PMXLoader {}

impl PMXLoader
{
	pub fn new() -> Self
	{
		Self
		{			
		}
	}

	pub fn do_save(_buf:&[u8], _model:&mut Model) -> Result<()>
	{
		Err(Error("Not Implmention yet".to_string()))
	}
}

impl Loader for PMXLoader
{
	fn can_read(&self, buf:&[u8]) -> bool
	{
		PmxHeader::load(&mut Cursor::new(buf)).is_ok()
	}

	fn do_load(&self, buf:&[u8]) -> Result<Model>
	{
		let pmx = PMXFile::load(buf)?;
		let mut model = Model::new();
		let mut start_indices:usize = 0;

		for (material_id, it) in pmx.materials.iter().enumerate()
		{
			let mut mesh = Mesh::new();
			mesh.name = (*it.name).to_string();
			mesh.material_id = Some(material_id);
			mesh.indices = Vec::with_capacity(it.face_count as usize);

			let mut map:HashMap<u32,u32> = HashMap::new();
			for i in start_indices..start_indices + it.face_count as usize
			{
				let index:u32;
				match pmx.indices
				{
					PmxIndex::Uint8(ref v) => { index = v[i] as u32; },
					PmxIndex::Uint16(ref v) => { index = v[i] as u32; },
					PmxIndex::Uint32(ref v) => { index = v[i] as u32; },
				}

				if let Some(n) = map.get(&index)
				{
					mesh.indices.push(*n);
					continue;
				}

				let len = map.len();
				map.insert(index, len as u32);
				mesh.indices.push(len as u32);
			}

			start_indices = start_indices + it.face_count as usize;

			mesh.positions.resize(map.len() * 3, 0.0);
			mesh.normals.resize(map.len() * 3, 0.0);
			mesh.texcoords.resize(map.len() * 2, 0.0);

			if pmx.bones.len() > 0
			{
				mesh.weights.resize(map.len(), VertexWeight::new());
			}

			for (read, write) in map
			{
				let v = pmx.vertices[read as usize].position;
				let n = pmx.vertices[read as usize].normal;
				let uv = pmx.vertices[read as usize].coord;
				let weight = &pmx.vertices[read as usize].weight;

				mesh.positions[(write * 3) as usize] = v.0;
				mesh.positions[(write * 3 + 1) as usize] = v.1;
				mesh.positions[(write * 3 + 2) as usize] = v.2;

				mesh.normals[(write * 3) as usize] = n.0;
				mesh.normals[(write * 3 + 1) as usize] = n.1;
				mesh.normals[(write * 3 + 2) as usize] = n.2;

				mesh.texcoords[(write * 2) as usize] = uv.0;
				mesh.texcoords[(write * 2 + 1) as usize] = uv.1;

				if mesh.weights.len() > 0
				{
					mesh.weights[write as usize] = 
						VertexWeight
						{
							bone:[weight.bone1,weight.bone2,weight.bone3,weight.bone4],
							weight:[weight.weight1,weight.weight2,weight.weight3,weight.weight4],
						}
				}
			}

			model.add_mesh(mesh);
		}

		for it in pmx.textures
		{
			model.add_texture(it);
		}

		for it in pmx.materials
		{
			let mut material = Material::new();
			material.name = it.name;
			material.ambient = [it.ambient.0, it.ambient.1, it.ambient.2];
			material.diffuse = [it.diffuse.0, it.diffuse.1, it.diffuse.2];
			material.specular = [it.specular.0, it.specular.1, it.specular.2];
			material.shininess = it.shininess;
			material.optical_density = it.opacity;

			if it.texture_index >= 0 { material.diffuse_texture = Some(it.texture_index as usize); }

			model.add_material(material);
		}

		for (bone_index, it) in pmx.bones.iter().enumerate()
		{
			let mut bone = Bone::new();
			bone.name = (*it.name).to_string();
			bone.parent = if it.parent >= 0 { Some(it.parent as u16) } else { None };
			bone.position = it.position;
			model.add_bone(bone);

			if (it.flag & PMX_BONE_IK) > 0
			{
				let mut ik = Solver::new();
				ik.bone = bone_index as _;
				ik.target_bone = it.ik_target_bone_index;
				ik.limited_radian = it.ik_limited_radian;
				ik.loop_count = it.ik_loop_count;

				for child in it.ik_links.iter()
				{
					let mut link = BoneLink::new();
					link.bone = child.bone_index;
					link.rotate_limited = if child.rotate_limited > 0 { true } else { false };
					link.minimum_radian.0 = child.minimum_radian.0;
					link.minimum_radian.1 = child.minimum_radian.1;
					link.minimum_radian.2 = child.minimum_radian.2;
					link.maximum_radian.0 = child.maximum_radian.0;
					link.maximum_radian.1 = child.maximum_radian.1;
					link.maximum_radian.2 = child.maximum_radian.2;

					ik.links.push(link);

				}

				model.add_ik(ik);
			}
		}

		Ok(model)
	}
}