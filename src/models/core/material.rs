use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material
{
	pub name: String,
	pub ambient: [f32; 3],
	pub diffuse: [f32; 3],
	pub specular: [f32; 3],
	pub shininess: f32,
	pub dissolve: f32,
	pub optical_density: f32,
	pub ambient_texture: Option<usize>,
	pub diffuse_texture: Option<usize>,
	pub specular_texture: Option<usize>,
	pub normal_texture: Option<usize>,
	pub dissolve_texture: Option<usize>,
	pub illumination_model: Option<u8>,
	pub unknown_param: HashMap<String, String>,
}

impl Material
{
	pub fn new() -> Self
	{
		Self
		{
			name: String::new(),
			ambient: [0.0; 3],
			diffuse: [0.0; 3],
			specular: [0.0; 3],
			shininess: 0.0,
			dissolve: 1.0,
			optical_density: 1.0,
			ambient_texture: None,
			diffuse_texture: None,
			specular_texture: None,
			normal_texture: None,
			dissolve_texture: None,
			illumination_model: None,
			unknown_param: HashMap::new(),
		}
	}
}