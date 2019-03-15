#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexWeight
{
	pub bone:[u16;4],
	pub weight:[f32;4],
}

impl VertexWeight
{
	pub fn new() -> Self
	{
		Self
		{
			bone:[0,0,0,0],
			weight:[0.0,0.0,0.0,0.0]
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh
{
	pub name:String,
	pub material_id:Option<usize>,
	pub positions:Vec<f32>,
	pub normals:Vec<f32>,
	pub texcoords:Vec<f32>,
	pub indices:Vec<u32>,
	pub weights:Vec<VertexWeight>,
}

impl Mesh
{
	pub fn new() -> Self
	{
		Self
		{
			name:String::new(),
			material_id:None,
			positions:Vec::new(),
			normals:Vec::new(),
			texcoords:Vec::new(),
			indices:Vec::new(),
			weights:Vec::new(),
		}
	}

	pub fn with_capacity(vertex:usize, index:usize) -> Self
	{
		Self
		{
			name: String::new(),
			positions:Vec::with_capacity(vertex * 3),
			normals:Vec::with_capacity(vertex * 3),
			texcoords:Vec::with_capacity(vertex * 2),
			indices:Vec::with_capacity(index),
			weights:Vec::new(),
			material_id:None,
		}
	}
}