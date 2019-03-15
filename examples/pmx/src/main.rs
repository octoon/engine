extern crate octoon;

fn main()
{
	let model = octoon::models::open("./安特.pmx").unwrap();

	println!("model count: {}", model.meshes.len());
	println!("texture count: {}", model.textures.len());
	println!("material count: {}", model.materials.len());
	println!("bone count: {}", model.bones.len());

	for (i, mesh) in model.meshes.iter().enumerate()
	{
		println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
		println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
		println!("model[{}].normals: {}", i, mesh.normals.len() / 3);
		println!("model[{}].texcoords: {}", i, mesh.texcoords.len() / 2);
		println!("model[{}].indices: {}", i, mesh.indices.len());
	}
}