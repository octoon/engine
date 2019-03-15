use super::{Mesh, Material, Bone, Solver};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model
{
	pub name: String,
	pub meshes: Vec<Mesh>,
	pub materials: Vec<Material>,
	pub textures: Vec<String>,
	pub bones: Vec<Bone>,
	pub iks: Vec<Solver>,
}

impl Model
{
	pub fn new() -> Self
	{
		Self
		{
			name: String::new(),
			meshes: Vec::new(),
			materials: Vec::new(),
			textures: Vec::new(),
			bones:Vec::new(),
			iks:Vec::new(),
		}
	}

	pub fn add_mesh(&mut self, mesh:Mesh) -> &mut Self
	{
		self.meshes.push(mesh);
		self
	}

	pub fn add_material(&mut self, material:Material) -> &mut Self
	{
		self.materials.push(material);
		self
	}

	pub fn add_texture(&mut self, texture:String) -> &mut Self
	{
		self.textures.push(texture);
		self
	}

	pub fn add_bone(&mut self, bone:Bone) -> &mut Self
	{
		self.bones.push(bone);
		self
	}

	pub fn add_ik(&mut self, ik:Solver) -> &mut Self
	{
		self.iks.push(ik);
		self
	}
}
