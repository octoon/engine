use std::sync::Arc;
use std::path::Path;
use std::io::BufRead;
use std::io::prelude::Seek;
use log::*;

use crate::math::*;
use crate::models;

use super::super::core::Result;
use super::super::geometries::MeshGeometry;
use super::super::materials::StandardMaterial;
use super::super::shapes::{ MeshShape, ModelShape };
use super::super::loaders::TextureLoader;

#[derive(Debug)]
pub struct ModelLoader {}

impl ModelLoader
{
	pub fn load_from_model<P: AsRef<Path>>(model:models::Model, path:P) -> Result<ModelShape>
	{
		let mut shapes = Vec::new();
		let mut materials = Vec::new();
		let mut textures = Vec::new();

		info!("model count: {}", model.meshes.len());
		info!("texture count: {}", model.textures.len());
		info!("material count: {}", model.materials.len());
		info!("bone count: {}", model.bones.len());
		info!("ik count: {}", model.iks.len());

		for name in &model.textures
		{
			let texture = TextureLoader::load(path.as_ref().join(&name));
			if texture.is_ok()
			{
				info!("Loading: {} Ok", &name);
				textures.push(Some(Arc::new(texture?)));
			}
			else
			{
				info!("Loading: {} Failed", &name);
				textures.push(None);
			}
		}

		for material in &model.materials
		{
			let mut m = StandardMaterial::new();
			m.set_albedo(float!(material.diffuse[0],material.diffuse[1],material.diffuse[2]));
			m.set_specular(float!(material.specular[0],material.specular[1],material.specular[2]));

			if material.diffuse_texture.is_some()
			{
				m.set_albedo_map(textures[material.diffuse_texture.unwrap()].clone());
			}

			materials.push(Arc::new(m));
		}

		for (i, mesh) in model.meshes.iter().enumerate()
		{
			info!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
			info!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
			info!("model[{}].normals: {}", i, mesh.normals.len() / 3);
			info!("model[{}].texcoords: {}", i, mesh.texcoords.len() / 2);
			info!("model[{}].weights: {}", i, mesh.weights.len());
			info!("model[{}].indices: {}", i, mesh.indices.len());

			let geometry = MeshGeometry::builder()
				.set_vertices(mesh.positions.chunks(3).map(|i| float!(i[0], i[1], i[2])).collect())
				.set_normals(mesh.normals.chunks(3).map(|i| float!(i[0], i[1], i[2])).collect())
				.set_texcoords(mesh.texcoords.chunks(2).map(|i| float!(i[0], i[1])).collect())
				.set_weights(mesh.weights.clone())
				.set_indices(mesh.indices.chunks(1).map(|i| i[0] as u16).collect());

			shapes.push(MeshShape::new(Arc::new(geometry.build()), materials[mesh.material_id.unwrap() as usize].clone()));
		}

		return Ok(ModelShape::new(shapes));
	}

	pub fn open<P: AsRef<Path>>(path:P) -> Result<ModelShape>
	{
	    ModelLoader::load_from_model(models::open(&path)?, path.as_ref().parent().unwrap())
	}

	pub fn load<P: AsRef<Path>>(path:P) -> Result<ModelShape>
	{
	    ModelLoader::load_from_model(models::open(&path)?, path.as_ref().parent().unwrap())
	}

	pub fn load_from_buf<R:BufRead + Seek>(r:R) -> Result<ModelShape>
	{
		ModelLoader::load_from_model(models::load_from_buf(r)?, "")
	}

	pub fn load_from_memory(buffer:&[u8]) -> Result<ModelShape>
	{
		ModelLoader::load_from_model(models::load_from_memory(buffer)?, "")
	}
}