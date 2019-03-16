use std::f32;
use std::io::{Cursor, SeekFrom};
use std::io::prelude::*;
use byteorder::{LittleEndian, ReadBytesExt};
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_31J;
use crate::math::float3;
use super::super::{Error, Result, Loader, ModelLoader, Object, Scene, PerspectiveCamera};

// https://github.com/oigami/PMMEditor/blob/master/PMMEditor/MMDFileParser/PmmReader.cs
pub struct PmmInt2();
pub struct PmmVector2();
pub struct PmmVector3();
pub struct PmmVector4();
pub struct PmmName();
pub struct PmmUint8();
pub struct PmmInt32();
pub struct PmmFloat();

#[derive(Debug)]
pub struct PmmHeader
{
	pub magic:String,
	pub reserve1:u16,
	pub reserve2:u16,
	pub reserve3:u16,
	pub view_width:u32,
	pub view_height:u32,
	pub frame_width:u32,
	pub edit_view_angle:f32,
	pub is_edit_camera_light_accessory :u8,
	pub is_open_camera_panel:u8,
	pub is_open_light_panel:u8,
	pub is_open_accessory_panel:u8,
	pub is_open_bone_panel:u8,
	pub is_open_morph_panel:u8,
	pub is_open_selfshadow_panel:u8,
	pub selected_model_index:u8,
}

#[derive(Debug)]
pub struct PmmBoneFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub interpolation_x:[u8;4],
	pub interpolation_y:[u8;4],
	pub interpolation_z:[u8;4],
	pub interpolation_rotation:[u8;4],
	pub translation:(f32,f32,f32),
	pub quaternion:(f32,f32,f32,f32),
	pub is_selected:u8,
	pub is_physics_disabled:u8,
}

#[derive(Debug)]
pub struct PmmMorphFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub value:f32,
	pub is_selected:u8,
}

#[derive(Debug)]
pub struct PmmOpFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub is_display:u8,
	pub is_ik_enabled:Vec<u8>,
	pub op_data:Vec<(i32,i32)>,
	pub is_selected:u8
}

#[derive(Debug)]
pub struct PmmBoneCurrentData
{
	pub translation:(f32,f32,f32),
	pub quaternion:(f32,f32,f32,f32),
	pub is_edit_un_commited:u8,
	pub is_physics_disabled:u8,
	pub is_row_selected:u8,
}

#[derive(Debug)]
pub struct PmmOpCurrentData
{
	pub keyframe_begin:i32,
	pub keyframe_end:i32,
	pub model_index:i32,
	pub parent_bone_index:i32,
}

#[derive(Debug)]
pub struct PmmModel
{
	pub number:u8,
	pub name:String,
	pub name_en:String,
	pub path:String,
	pub keyframe_editor_toplevel_rows:u8,
	pub bone_name:Vec<String>,
	pub morph_name:Vec<String>,
	pub ik_index:Vec<i32>,
	pub op_index:Vec<i32>,
	pub draw_order:u8,
	pub edit_is_display:u8,
	pub edit_selected_bone:i32,
	pub skin_panel:[i32;4],
	pub is_frame_open:Vec<u8>,
	pub vscroll:i32,
	pub last_frame:i32,
	pub bone_init_frame:Vec<PmmBoneFrame>,
	pub bone_key_frame:Vec<PmmBoneFrame>,
	pub morph_init_frame:Vec<PmmMorphFrame>,
	pub morph_key_frame:Vec<PmmMorphFrame>,
	pub op_init_frame:PmmOpFrame,
	pub op_key_frames:Vec<PmmOpFrame>,
	pub bone_current_datas:Vec<PmmBoneCurrentData>,
	pub morph_current_datas:Vec<f32>,
	pub is_current_ik_enabled_datas:Vec<u8>,
	pub op_current_data:Vec<PmmOpCurrentData>,
	pub is_add_blend:u8,
	pub edge_width:f32,
	pub is_selfshadow_enabled:u8,
	pub calc_order:u8
}

#[derive(Debug)]
pub struct PmmCameraFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub distance:f32,
	pub eye_position:(f32,f32,f32),
	pub rotation:(f32,f32,f32),
	pub looking_model_index:i32,
	pub looking_bone_index:i32,
	pub interpolation_x:[u8; 4],
	pub interpolation_y:[u8; 4],
	pub interpolation_z:[u8; 4],
	pub interpolation_rotation:[u8; 4],
	pub interpolation_distance:[u8; 4],
	pub interpolation_angleview:[u8; 4],
	pub is_parse:u8,
	pub angle_view:u32,
	pub is_selected:u8,
}

#[derive(Debug)]
pub struct PmmCameraCurrentData
{
	pub eye_position:(f32,f32,f32),
	pub target_position:(f32,f32,f32),
	pub rotation:(f32,f32,f32),
	pub isorthro: u8 ,
}

#[derive(Debug)]
pub struct PmmLightFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub rgb:(f32,f32,f32),
	pub xyz:(f32,f32,f32),
	pub is_selected:u8,
}

#[derive(Debug)]
pub struct PmmLightCurrentData
{
	pub rgb:(f32,f32,f32),
	pub xyz:(f32,f32,f32),
	pub is_selected:u8,
}

#[derive(Debug)]
pub struct PmmKeyFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub is_selected:u8
}

#[derive(Debug)]
pub struct PmmDataBody
{
	pub transparency:u8,
	pub is_visible:u8,
	pub parent_model_index:i32,
	pub parent_bone_index:i32,
	pub translation:(f32,f32,f32),
	pub rotation:(f32,f32,f32),
	pub scale:f32,
	pub is_shadow_enabled:u8,
}

#[derive(Debug)]
pub struct PmmAccessoryData
{
	pub index:u8,
	pub name:String,
	pub path:String,
	pub draw_order:u8,
	pub init_frame:PmmKeyFrame,
	pub key_frames:Vec<PmmKeyFrame>,
	pub current_data:PmmDataBody,
	pub is_add_blend:u8
}

#[derive(Debug)]
pub struct PmmGravityCurrentData
{
	pub acceleration:f32,
	pub noize_amount:u32,
	pub direction:(f32,f32,f32),
	pub is_add_noize:u8
}

#[derive(Debug)]
pub struct PmmGravityKeyFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub is_add_noize:u8,
	pub noize_amount:u32,
	pub acceleration:f32,
	pub direction:(f32,f32,f32),
	pub is_selected:u8
}

#[derive(Debug)]
pub struct PmmSelfShadowKeyFrame
{
	pub data_index:i32,
	pub frame_number:i32,
	pub pre_index:i32,
	pub next_index:i32,
	pub mode:u8,
	pub distance:f32,
	pub is_selected:u8
}

#[derive(Debug)]
pub struct PmmCSelectorChoiceData
{
	pub mode_index:u8,
	pub selector_choice:u32
}

#[derive(Debug)]
pub struct PMMFile
{
	pub header:PmmHeader,
	pub model:Vec<PmmModel>,
	pub camera_init_frame:PmmCameraFrame,
	pub camera_key_frames:Vec<PmmCameraFrame>,
	pub camera_current_data:PmmCameraCurrentData,
	pub light_init_frame:PmmLightFrame,
	pub light_key_frames:Vec<PmmLightFrame>,
	pub light_current_data:PmmLightCurrentData,
	pub selected_accessory_index:u8,
	pub accessory_vscroll:u32,
	pub accessory_count:u8,
	pub accessory_name:Vec<String>,
	pub accessory_datas:Vec<PmmAccessoryData>,
	pub current_frame_position:u32,
	pub h_scroll_position:u32,
	pub h_scroll_scale:u32,
	pub bone_operation_kind:u32,
	pub looking_at:u8,
	pub is_repeat:u8,
	pub is_play_from_frame:u8,
	pub is_play_to_frame:u8,
	pub play_start_frame:u32,
	pub play_end_frame:u32,
	pub is_wave_enabled:u8,
	pub wave_path:String,
	pub avi_offset_x: u32,
	pub avi_offset_y: u32,
	pub avi_scale: f32,
	pub avi_path: String,
	pub is_show_avi: u32,
	pub background_image_offset_x: u32,
	pub background_image_offset_y: u32,
	pub background_image_scale: u32,
	pub background_image_path: String,
	pub is_show_background_image: u8,
	pub is_show_infomation: u8,
	pub is_show_axis: u8,
	pub is_show_groundshadow: u8,
	pub fps_limit: f32,
	pub screen_capture_mode: u32,
	pub accessory_number_render_after_model: u32,
	pub ground_shadow_brightness: f32,
	pub is_transparent_ground_shadow: u8,
	pub physics_mode: u8,
	pub gravity_current_data:PmmGravityCurrentData,
	pub gravity_init_frame:PmmGravityKeyFrame,
	pub gravity_key_frames:Vec<PmmGravityKeyFrame>,
	pub is_show_selfshadow:u8,
	pub selfshadow_current_data:f32,
	pub selfshadow_init_frame:PmmSelfShadowKeyFrame,
	pub selfshadow_keyframes:Vec<PmmSelfShadowKeyFrame>,
	pub edge_color_r:u32,
	pub edge_color_g:u32,
	pub edge_color_b:u32,
	pub is_black_background:u8,
	pub camera_current_looking_at_model:i32,
	pub camera_current_looking_at_bone:i32,
	pub unknown_array:[f32;16],
	pub is_view_look_at_enabled:u8,
	pub unknown:u8,
	pub is_physics_ground_enabled:u8,
	pub frame_text_box:u32,
	pub selector_choice_selection_following:u8,
	pub selector_choice_datas:Vec<PmmCSelectorChoiceData>,
}

impl PmmInt2
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(i32,i32)>
	{
		let x = reader.read_i32::<LittleEndian>()?;
		let y = reader.read_i32::<LittleEndian>()?;
		Ok((x, y))
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<(i32,i32)>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmInt2::load(reader)?);
		}

		Ok(array)
	}

	pub fn load_fixed_arrays(reader:&mut Cursor<&[u8]>, len:usize) -> Result<Vec<(i32,i32)>>
	{
		let mut array = Vec::with_capacity(len as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmInt2::load(reader)?);
		}

		Ok(array)
	}
}

impl PmmVector2
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		Ok((x, y))
	}
}

impl PmmVector3
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<(f32,f32,f32)>
	{
		let x = reader.read_f32::<LittleEndian>()?;
		let y = reader.read_f32::<LittleEndian>()?;
		let z = reader.read_f32::<LittleEndian>()?;
		Ok((x, y, z))
	}
}

impl PmmVector4
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

impl PmmName
{
	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<String>
	{
		let mut bytes = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..bytes.capacity()
		{
			bytes.push(reader.read_u8()?);
		}

		Ok(WINDOWS_31J.decode(&bytes, DecoderTrap::Ignore).unwrap())
	}

	pub fn load_fixed_utf8(reader:&mut Cursor<&[u8]>, len:usize) -> Result<String>
	{
		let mut bytes = Vec::new();
		for _ in 0..len
		{
			let ch = reader.read_u8()?;
			if ch == 0 { break; }
			bytes.push(ch);
		}

		if bytes.len() < len-1
		{
			reader.seek(SeekFrom::Current((len - bytes.len() - 1) as i64))?;
		}

		Ok(WINDOWS_31J.decode(&bytes, DecoderTrap::Ignore).unwrap())
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<String>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmName::load(reader)?);
		}

		Ok(array)
	}
}

impl PmmUint8
{
	pub fn load_array_from_u8(reader:&mut Cursor<&[u8]>) -> Result<Vec<u8>>
	{
		let mut array = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(reader.read_u8()?);
		}

		Ok(array)
	}

	pub fn load_fixed_arrays(reader:&mut Cursor<&[u8]>, len:usize) -> Result<Vec<u8>>
	{
		let mut array = Vec::with_capacity(len);
		for _ in 0..array.capacity()
		{
			array.push(reader.read_u8()?);
		}

		Ok(array)
	}
}

impl PmmInt32
{
	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<i32>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(reader.read_i32::<LittleEndian>()?);
		}

		Ok(array)
	}
}

impl PmmFloat
{
	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<f32>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(reader.read_f32::<LittleEndian>()?);
		}

		Ok(array)
	}
}

impl PmmHeader
{
	pub fn new() -> Self
	{
		Self
		{
			magic:String::new(),
			reserve1:0,
			reserve2:0,
			reserve3:0,
			view_width:0,
			view_height:0,
			frame_width:0,
			edit_view_angle:0.0,
			is_edit_camera_light_accessory :0,
			is_open_camera_panel:0,
			is_open_light_panel:0,
			is_open_accessory_panel:0,
			is_open_bone_panel:0,
			is_open_morph_panel:0,
			is_open_selfshadow_panel:0,
			selected_model_index:0,
		}
	}

	pub fn valid(self) -> Result<Self>
	{
		if self.magic != "Polygon Movie maker 0002" { return Err(Error::LoaderError("Invalid magic in PMM Header".to_string())); }
		if self.view_width == 0 { return Err(Error::LoaderError("Invalid width in PMM Header".to_string())); }
		if self.view_height == 0 { return Err(Error::LoaderError("Invalid height in PMM Header".to_string())); }
		if self.frame_width == 0 { return Err(Error::LoaderError("Invalid frame width in PMM Header".to_string())); }
		if self.edit_view_angle <= 0.0 { return Err(Error::LoaderError("Invalid FOV in PMM Header".to_string())); }

		Ok(self)
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmHeader::new();
		this.magic = PmmName::load_fixed_utf8(reader, 24)?;
		this.reserve1 = reader.read_u16::<LittleEndian>()?;
		this.reserve2 = reader.read_u16::<LittleEndian>()?;
		this.reserve3 = reader.read_u16::<LittleEndian>()?;
		this.view_width = reader.read_u32::<LittleEndian>()?;
		this.view_height = reader.read_u32::<LittleEndian>()?;
		this.frame_width = reader.read_u32::<LittleEndian>()?;
		this.edit_view_angle = reader.read_f32::<LittleEndian>()?;
		this.is_edit_camera_light_accessory = reader.read_u8()?;
		this.is_open_camera_panel = reader.read_u8()?;
		this.is_open_light_panel = reader.read_u8()?;
		this.is_open_accessory_panel = reader.read_u8()?;
		this.is_open_bone_panel = reader.read_u8()?;
		this.is_open_morph_panel = reader.read_u8()?;
		this.is_open_selfshadow_panel = reader.read_u8()?;
		this.selected_model_index = reader.read_u8()?;

		this.valid()
	}
}

impl PmmBoneFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:0,
			pre_index:-1,
			next_index:-1,
			interpolation_x:[0;4],
			interpolation_y:[0;4],
			interpolation_z:[0;4],
			interpolation_rotation:[0;4],
			translation:(0.0,0.0,0.0),
			quaternion:(0.0,0.0,0.0,1.0),
			is_selected:0,
			is_physics_disabled:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmBoneFrame::new();
		this.data_index = if is_init { -1 } else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		for i in 0..this.interpolation_x.len() { this.interpolation_x[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_y.len() { this.interpolation_y[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_z.len() { this.interpolation_z[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_rotation.len() { this.interpolation_rotation[i] = reader.read_u8()?; }
		this.translation = PmmVector3::load(reader)?;
		this.quaternion = PmmVector4::load(reader)?;
		this.is_selected = reader.read_u8()?;
		this.is_physics_disabled = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_fixed_arrays(reader:&mut Cursor<&[u8]>, len:usize, is_init:bool) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(len as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmBoneFrame::load(reader, is_init)?);
		}

		Ok(array)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let len = reader.read_u32::<LittleEndian>()? as usize;
		PmmBoneFrame::load_fixed_arrays(reader, len, is_init)
	}
}

impl PmmMorphFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:0,
			pre_index:-1,
			next_index:-1,
			value:0.0,
			is_selected:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmMorphFrame::new();
		this.data_index = if is_init { -1 } else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.value = reader.read_f32::<LittleEndian>()?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_fixed_arrays(reader:&mut Cursor<&[u8]>, len:usize, is_init:bool) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(len as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmMorphFrame::load(reader, is_init)?);
		}

		Ok(array)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let len = reader.read_u32::<LittleEndian>()? as usize;
		PmmMorphFrame::load_fixed_arrays(reader, len, is_init)
	}
}

impl PmmOpFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:-1,
			pre_index:-1,
			next_index:-1,
			is_display:0,
			is_ik_enabled:Vec::new(),
			op_data:Vec::new(),
			is_selected:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, ik_count:usize, op_count:usize, is_init:bool) -> Result<Self>
	{
		let mut this = PmmOpFrame::new();
		this.data_index = if is_init { -1 } else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.is_display = reader.read_u8()?;
		this.is_ik_enabled = PmmUint8::load_fixed_arrays(reader, ik_count)?;
		this.op_data = PmmInt2::load_fixed_arrays(reader, op_count)?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, ik_count:usize, op_count:usize, is_init:bool) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmOpFrame::load(reader, ik_count, op_count, is_init)?);
		}

		Ok(array)
	}
}

impl PmmGravityCurrentData
{
	pub fn new() -> Self
	{
		Self
		{
			acceleration:0.0,
			noize_amount:0,
			direction:(0.0,0.0,0.0),
			is_add_noize:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmGravityCurrentData::new();
		this.acceleration = reader.read_f32::<LittleEndian>()?;
		this.noize_amount = reader.read_u32::<LittleEndian>()?;
		this.direction = PmmVector3::load(reader)?;
		this.is_add_noize = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmGravityCurrentData::load(reader)?);
		}

		Ok(array)
	}
}

impl PmmGravityKeyFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:-1,
			pre_index:-1,
			next_index:-1,
			is_add_noize:0,
			noize_amount:0,
			acceleration:0.0,
			direction:(0.0,0.0,0.0),
			is_selected:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmGravityKeyFrame::new();
		this.data_index = if is_init { -1} else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.is_add_noize = reader.read_u8()?;
		this.noize_amount = reader.read_u32::<LittleEndian>()?;
		this.acceleration = reader.read_f32::<LittleEndian>()?;
		this.direction = PmmVector3::load(reader)?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmGravityKeyFrame::load(reader, is_init)?);
		}

		Ok(array)
	}
}

impl PmmSelfShadowKeyFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:-1,
			pre_index:-1,
			next_index:-1,
			mode:0,
			distance:0.0,
			is_selected:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmSelfShadowKeyFrame::new();
		this.data_index = if is_init { -1} else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.mode = reader.read_u8()?;
		this.distance = reader.read_f32::<LittleEndian>()?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let mut array = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..array.capacity()
		{
			array.push(PmmSelfShadowKeyFrame::load(reader, is_init)?);
		}

		Ok(array)
	}
}

impl PmmBoneCurrentData
{
	pub fn new() -> Self
	{
		Self
		{
			translation:(0.0,0.0,0.0),
			quaternion:(0.0,0.0,0.0,0.0),
			is_edit_un_commited:0,
			is_physics_disabled:0,
			is_row_selected:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmBoneCurrentData::new();
		this.translation = PmmVector3::load(reader)?;
		this.quaternion = PmmVector4::load(reader)?;
		this.is_edit_un_commited = reader.read_u8()?;
		this.is_physics_disabled = reader.read_u8()?;
		this.is_row_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmBoneCurrentData::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmOpCurrentData
{
	pub fn new() -> Self
	{
		Self
		{
			keyframe_begin:-1,
			keyframe_end:-1,
			model_index:-1,
			parent_bone_index:-1,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmOpCurrentData::new();
		this.keyframe_begin = reader.read_i32::<LittleEndian>()?;
		this.keyframe_end = reader.read_i32::<LittleEndian>()?;
		this.model_index = reader.read_i32::<LittleEndian>()?;
		this.parent_bone_index = reader.read_i32::<LittleEndian>()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmOpCurrentData::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmModel
{
	pub fn new() -> Self
	{
		Self
		{
			number:0,
			name:String::new(),
			name_en:String::new(),
			path:String::new(),
			keyframe_editor_toplevel_rows:0,
			bone_name:Vec::new(),
			morph_name:Vec::new(),
			ik_index:Vec::new(),
			op_index:Vec::new(),
			draw_order:0,
			edit_is_display:0,
			edit_selected_bone:0,
			skin_panel:[0;4],
			is_frame_open:Vec::new(),
			vscroll:0,
			last_frame:0,
			bone_init_frame:Vec::new(),
			bone_key_frame:Vec::new(),
			morph_init_frame:Vec::new(),
			morph_key_frame:Vec::new(),
			op_init_frame:PmmOpFrame::new(),
			op_key_frames:Vec::new(),
			bone_current_datas:Vec::new(),
			morph_current_datas:Vec::new(),
			is_current_ik_enabled_datas:Vec::new(),
			op_current_data:Vec::new(),
			is_add_blend:0,
			edge_width:1.0,
			is_selfshadow_enabled:1,
			calc_order:1
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmModel::new();
		this.number = reader.read_u8()?;
		this.name = PmmName::load(reader)?;
		this.name_en = PmmName::load(reader)?;
		this.path = PmmName::load_fixed_utf8(reader, 256)?;
		this.keyframe_editor_toplevel_rows = reader.read_u8()?;
		this.bone_name = PmmName::load_arrays(reader)?;
		this.morph_name = PmmName::load_arrays(reader)?;
		this.ik_index = PmmInt32::load_arrays(reader)?;
		this.op_index = PmmInt32::load_arrays(reader)?;
		this.draw_order = reader.read_u8()?;
		this.edit_is_display = reader.read_u8()?;
		this.edit_selected_bone = reader.read_i32::<LittleEndian>()?;
		this.skin_panel[0] = reader.read_i32::<LittleEndian>()?;
		this.skin_panel[1] = reader.read_i32::<LittleEndian>()?;
		this.skin_panel[2] = reader.read_i32::<LittleEndian>()?;
		this.skin_panel[3] = reader.read_i32::<LittleEndian>()?;
		this.is_frame_open = PmmUint8::load_array_from_u8(reader)?;
		this.vscroll = reader.read_i32::<LittleEndian>()?;
		this.last_frame = reader.read_i32::<LittleEndian>()?;
		this.bone_init_frame = PmmBoneFrame::load_fixed_arrays(reader, this.bone_name.len(), true)?;
		this.bone_key_frame = PmmBoneFrame::load_arrays(reader, false)?;
		this.morph_init_frame = PmmMorphFrame::load_fixed_arrays(reader, this.morph_name.len(), true)?;
		this.morph_key_frame = PmmMorphFrame::load_arrays(reader, false)?;
		this.op_init_frame = PmmOpFrame::load(reader, this.ik_index.len(), this.op_index.len(), true)?;
		this.op_key_frames = PmmOpFrame::load_arrays(reader, this.ik_index.len(), this.op_index.len(), false)?;

		for _ in 0..this.bone_name.len()
		{
			this.bone_current_datas.push(PmmBoneCurrentData::load(reader)?);
		}

		for _ in 0..this.morph_name.len()
		{
			this.morph_current_datas.push(reader.read_f32::<LittleEndian>()?);
		}

		for _ in 0..this.ik_index.len()
		{
			this.is_current_ik_enabled_datas.push(reader.read_u8()?);
		}

		for _ in 0..this.op_index.len()
		{
			this.op_current_data.push(PmmOpCurrentData::load(reader)?);
		}

		this.is_add_blend = reader.read_u8()?;
		this.edge_width = reader.read_f32::<LittleEndian>()?;
		this.is_selfshadow_enabled = reader.read_u8()?;
		this.calc_order = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmModel::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmCameraFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:0,
			frame_number:0,
			pre_index:0,
			next_index:0,
			distance:0.0,
			eye_position:(0.0,0.0,0.0),
			rotation:(0.0,0.0,0.0),
			looking_model_index:0,
			looking_bone_index:0,
			interpolation_x:[0; 4],
			interpolation_y:[0; 4],
			interpolation_z:[0; 4],
			interpolation_rotation:[0; 4],
			interpolation_distance:[0; 4],
			interpolation_angleview:[0; 4],
			is_parse:0,
			angle_view:0,
			is_selected:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmCameraFrame::new();
		this.data_index = if is_init { -1 } else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.distance = reader.read_f32::<LittleEndian>()?;
		this.eye_position = PmmVector3::load(reader)?;
		this.rotation = PmmVector3::load(reader)?;
		this.looking_model_index = reader.read_i32::<LittleEndian>()?;
		this.looking_bone_index = reader.read_i32::<LittleEndian>()?;
		for i in 0..this.interpolation_x.len() { this.interpolation_x[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_y.len() { this.interpolation_y[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_z.len() { this.interpolation_z[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_rotation.len() { this.interpolation_rotation[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_distance.len() { this.interpolation_distance[i] = reader.read_u8()?; }
		for i in 0..this.interpolation_angleview.len() { this.interpolation_angleview[i] = reader.read_u8()?; }
		this.is_parse = reader.read_u8()?;
		this.angle_view = reader.read_u32::<LittleEndian>()?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmCameraFrame::load(reader, is_init)?);
		}

		Ok(model)
	}
}

impl PmmCameraCurrentData
{
	pub fn new() -> Self
	{
		Self
		{
			eye_position:(0.0,0.0,0.0),
			target_position:(0.0,0.0,0.0),
			rotation:(0.0,0.0,0.0),
			isorthro:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmCameraCurrentData::new();
		this.eye_position = PmmVector3::load(reader)?;
		this.target_position = PmmVector3::load(reader)?;
		this.rotation = PmmVector3::load(reader)?;
		this.isorthro = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmCameraCurrentData::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmLightFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:-1,
			pre_index:-1,
			next_index:-1,
			rgb:(0.0,0.0,0.0),
			xyz:(0.0,0.0,0.0),
			is_selected:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Self>
	{
		let mut this = PmmLightFrame::new();
		this.data_index = if is_init { -1 } else { reader.read_i32::<LittleEndian>()? };
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.rgb = PmmVector3::load(reader)?;
		this.xyz = PmmVector3::load(reader)?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>, is_init:bool) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u32::<LittleEndian>()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmLightFrame::load(reader, is_init)?);
		}

		Ok(model)
	}
}

impl PmmLightCurrentData
{
	pub fn new() -> Self
	{
		Self
		{
			rgb:(0.0,0.0,0.0),
			xyz:(0.0,0.0,0.0),
			is_selected:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmLightCurrentData::new();
		this.rgb = PmmVector3::load(reader)?;
		this.xyz = PmmVector3::load(reader)?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmLightCurrentData::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmKeyFrame
{
	pub fn new() -> Self
	{
		Self
		{
			data_index:-1,
			frame_number:-1,
			pre_index:-1,
			next_index:-1,
			is_selected:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmKeyFrame::new();
		this.data_index = reader.read_i32::<LittleEndian>()?;
		this.frame_number = reader.read_i32::<LittleEndian>()?;
		this.pre_index = reader.read_i32::<LittleEndian>()?;
		this.next_index = reader.read_i32::<LittleEndian>()?;
		this.is_selected = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmKeyFrame::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmDataBody
{
	pub fn new() -> Self
	{
		Self
		{
			transparency:0,
			is_visible:0,
			parent_model_index:0,
			parent_bone_index:0,
			translation:(0.0,0.0,0.0),
			rotation:(0.0,0.0,0.0),
			scale:0.0,
			is_shadow_enabled:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmDataBody::new();
		this.transparency = reader.read_u8()?;
		this.is_visible = 0;
		this.parent_model_index = reader.read_i32::<LittleEndian>()?;
		this.parent_bone_index = reader.read_i32::<LittleEndian>()?;
		this.translation = PmmVector3::load(reader)?;
		this.rotation = PmmVector3::load(reader)?;
		this.scale = reader.read_f32::<LittleEndian>()?;
		this.is_shadow_enabled = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmDataBody::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmAccessoryData
{
	pub fn new() -> Self
	{
		Self
		{
			index:0,
			name:String::new(),
			path:String::new(),
			draw_order:0,
			init_frame:PmmKeyFrame::new(),
			key_frames:Vec::new(),
			current_data:PmmDataBody::new(),
			is_add_blend:0
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmAccessoryData::new();
		this.index = reader.read_u8()?;
		this.name = PmmName::load_fixed_utf8(reader, 100)?;
		this.path = PmmName::load_fixed_utf8(reader, 256)?;
		this.draw_order = reader.read_u8()?;
		this.init_frame = PmmKeyFrame::load(reader)?;
		this.key_frames = PmmKeyFrame::load_arrays(reader)?;
		this.current_data = PmmDataBody::load(reader)?;
		this.is_add_blend = reader.read_u8()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmAccessoryData::load(reader)?);
		}

		Ok(model)
	}
}

impl PmmCSelectorChoiceData
{
	pub fn new() -> Self
	{
		Self
		{
			mode_index:0,
			selector_choice:0,
		}
	}

	pub fn load(reader:&mut Cursor<&[u8]>) -> Result<Self>
	{
		let mut this = PmmCSelectorChoiceData::new();
		this.mode_index = reader.read_u8()?;
		this.selector_choice = reader.read_u32::<LittleEndian>()?;

		Ok(this)
	}

	pub fn load_arrays(reader:&mut Cursor<&[u8]>) -> Result<Vec<Self>>
	{
		let mut model = Vec::with_capacity(reader.read_u8()? as usize);
		for _ in 0..model.capacity()
		{
			model.push(PmmCSelectorChoiceData::load(reader)?);
		}

		Ok(model)
	}
}

impl PMMFile
{
	pub fn new() -> Self
	{
		Self
		{
			header:PmmHeader::new(),
			model:Vec::new(),
			camera_init_frame:PmmCameraFrame::new(),
			camera_key_frames:Vec::new(),
			camera_current_data:PmmCameraCurrentData::new(),
			light_init_frame:PmmLightFrame::new(),
			light_key_frames:Vec::new(),
			light_current_data:PmmLightCurrentData::new(),
			selected_accessory_index:0,
			accessory_vscroll:0,
			accessory_count:0,
			accessory_name:Vec::new(),
			accessory_datas:Vec::new(),
			current_frame_position:0,
			h_scroll_position:0,
			h_scroll_scale:0,
			bone_operation_kind:0,
			looking_at:0,
			is_repeat:0,
			is_play_from_frame:0,
			is_play_to_frame:0,
			play_start_frame:0,
			play_end_frame:0,
			is_wave_enabled:0,
			wave_path:String::new(),
			avi_offset_x: 0,
			avi_offset_y: 0,
			avi_scale: 0.0,
			avi_path: String::new(),
			is_show_avi: 0,
			background_image_offset_x: 0,
			background_image_offset_y: 0,
			background_image_scale: 0,
			background_image_path: String::new(),
			is_show_background_image: 0,
			is_show_infomation: 0,
			is_show_axis: 0,
			is_show_groundshadow: 0,
			fps_limit: 0.0,
			screen_capture_mode: 0,
			accessory_number_render_after_model: 0,
			ground_shadow_brightness: 0.0,
			is_transparent_ground_shadow: 0,
			physics_mode: 0,
			gravity_current_data:PmmGravityCurrentData::new(),
			gravity_init_frame:PmmGravityKeyFrame::new(),
			gravity_key_frames:Vec::new(),
			is_show_selfshadow:0,
			selfshadow_current_data:0.0,
			selfshadow_init_frame:PmmSelfShadowKeyFrame::new(),
			selfshadow_keyframes:Vec::new(),
			edge_color_r:0,
			edge_color_g:0,
			edge_color_b:0,
			is_black_background:0,
			camera_current_looking_at_model:0,
			camera_current_looking_at_bone:0,
			unknown_array:[0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],
			is_view_look_at_enabled:0,
			unknown:0,
			is_physics_ground_enabled:0,
			frame_text_box:0,
			selector_choice_selection_following:0,
			selector_choice_datas:Vec::new(),
		}
	}

	pub fn load(buf:&[u8]) -> Result<Self>
	{
		let mut reader = Cursor::new(buf);
		let mut pmm = PMMFile::new();
		pmm.header = PmmHeader::load(&mut reader)?;
		pmm.model = PmmModel::load_arrays(&mut reader)?;
		pmm.camera_init_frame = PmmCameraFrame::load(&mut reader, true)?;
		pmm.camera_key_frames = PmmCameraFrame::load_arrays(&mut reader, false)?;
		pmm.camera_current_data = PmmCameraCurrentData::load(&mut reader)?;
		pmm.light_init_frame = PmmLightFrame::load(&mut reader, true)?;
		pmm.light_key_frames = PmmLightFrame::load_arrays(&mut reader, false)?;
		pmm.light_current_data = PmmLightCurrentData::load(&mut reader)?;
		pmm.selected_accessory_index = reader.read_u8()?;
		pmm.accessory_vscroll = reader.read_u32::<LittleEndian>()?;
		pmm.accessory_count = reader.read_u8()?;

		for _ in 0..pmm.accessory_count
		{
			pmm.accessory_name.push(PmmName::load_fixed_utf8(&mut reader, 100)?);
		}

		for _ in 0..pmm.accessory_count
		{
			pmm.accessory_datas.push(PmmAccessoryData::load(&mut reader)?);
		}

		pmm.current_frame_position = reader.read_u32::<LittleEndian>()?;
		pmm.h_scroll_position = reader.read_u32::<LittleEndian>()?;
		pmm.h_scroll_scale = reader.read_u32::<LittleEndian>()?;
		pmm.bone_operation_kind = reader.read_u32::<LittleEndian>()?;
		pmm.looking_at = reader.read_u8()?;
		pmm.is_repeat = reader.read_u8()?;
		pmm.is_play_from_frame = reader.read_u8()?;
		pmm.is_play_to_frame = reader.read_u8()?;
		pmm.play_start_frame = reader.read_u32::<LittleEndian>()?;
		pmm.play_end_frame = reader.read_u32::<LittleEndian>()?;
		pmm.is_wave_enabled = reader.read_u8()?;
		pmm.wave_path = PmmName::load_fixed_utf8(&mut reader, 256)?;
		pmm.avi_offset_x = reader.read_u32::<LittleEndian>()?;
		pmm.avi_offset_y = reader.read_u32::<LittleEndian>()?;
		pmm.avi_scale = reader.read_f32::<LittleEndian>()?;
		pmm.avi_path = PmmName::load_fixed_utf8(&mut reader, 256)?;
		pmm.is_show_avi = reader.read_u32::<LittleEndian>()?;
		pmm.background_image_offset_x = reader.read_u32::<LittleEndian>()?;
		pmm.background_image_offset_y = reader.read_u32::<LittleEndian>()?;
		pmm.background_image_scale = reader.read_u32::<LittleEndian>()?;
		pmm.background_image_path = PmmName::load_fixed_utf8(&mut reader, 255)?;
		pmm.is_show_background_image = reader.read_u8()?;
		pmm.is_show_infomation = reader.read_u8()?;
		pmm.is_show_axis = reader.read_u8()?;
		pmm.is_show_groundshadow = reader.read_u8()?;
		pmm.fps_limit = reader.read_f32::<LittleEndian>()?;
		pmm.screen_capture_mode = reader.read_u32::<LittleEndian>()?;
		pmm.accessory_number_render_after_model = reader.read_u32::<LittleEndian>()?;
		pmm.ground_shadow_brightness = reader.read_f32::<LittleEndian>()?;
		pmm.is_transparent_ground_shadow = reader.read_u8()?;
		pmm.physics_mode =  reader.read_u8()?;
		pmm.gravity_current_data = PmmGravityCurrentData::load(&mut reader)?;
		pmm.gravity_init_frame = PmmGravityKeyFrame::load(&mut reader, true)?;
		pmm.gravity_key_frames = PmmGravityKeyFrame::load_arrays(&mut reader, false)?;
		pmm.is_show_selfshadow = reader.read_u8()?;
		pmm.selfshadow_current_data = reader.read_f32::<LittleEndian>()?;
		pmm.selfshadow_init_frame = PmmSelfShadowKeyFrame::load(&mut reader, true)?;
		pmm.selfshadow_keyframes = PmmSelfShadowKeyFrame::load_arrays(&mut reader, false)?;
		pmm.edge_color_r = reader.read_u32::<LittleEndian>()?;
		pmm.edge_color_g = reader.read_u32::<LittleEndian>()?;
		pmm.edge_color_b = reader.read_u32::<LittleEndian>()?;
		pmm.is_black_background = reader.read_u8()?;
		pmm.camera_current_looking_at_model = reader.read_i32::<LittleEndian>()?;
		pmm.camera_current_looking_at_bone = reader.read_i32::<LittleEndian>()?;
		for i in 0..pmm.unknown_array.len() { pmm.unknown_array[i] = reader.read_f32::<LittleEndian>()?; }
		pmm.is_view_look_at_enabled = reader.read_u8()?;
		pmm.unknown = reader.read_u8()?;
		pmm.is_physics_ground_enabled = reader.read_u8()?;
		pmm.frame_text_box = reader.read_u32::<LittleEndian>()?;
		pmm.selector_choice_selection_following = reader.read_u8()?;
		pmm.selector_choice_datas = PmmCSelectorChoiceData::load_arrays(&mut reader)?;

		Ok(pmm)
	}
}

#[derive(Debug)]
pub struct PMMLoader {}

impl PMMLoader
{
	pub fn new() -> Self
	{
		Self
		{
		}
	}
}

impl Loader for PMMLoader
{
	fn can_read(&self, buf:&[u8]) -> bool
	{
		PmmHeader::load(&mut Cursor::new(buf)).is_ok()
	}

	fn do_load(&self, buf:&[u8]) -> Result<Scene>
	{
		let pmm = PMMFile::load(buf)?;
		let mut scene = Scene::new();

		let camera = PerspectiveCamera::builder()
			.main(true)
			.set_fov(30.0)
			.set_translate(float!(0.0,0.1,10.0))
			.build();

		scene.add(camera);

		for model in pmm.model
		{
			let mut model = ModelLoader::open(model.path)?;
			model.set_scale(float!(0.1,0.1,0.1));
			model.set_translate(float!(0.0,-0.8,20.0));
			scene.add(model);
		}

		Ok(scene)
	}
}