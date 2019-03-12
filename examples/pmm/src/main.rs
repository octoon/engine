extern crate octoon_scene as scene;
extern crate octoon_renderer;

#[macro_use]
extern crate octoon_math;

use scene::{SkyLight, TextureLoader, PerspectiveCamera};
use scene::loaders::pmm::{PMMLoader};
use octoon_renderer::window::Window;
use std::sync::Arc;
use octoon_math::{float3};

fn main()
{
	let irradiance = Arc::new(TextureLoader::load_from_memory(include_bytes!("../static/output_iem.png")).unwrap());
	let sky = SkyLight::builder(irradiance.clone(), irradiance.clone())
		.build();

	let mut camera = PerspectiveCamera::builder()
		.main(true)
		.set_fov(30.0)
		.set_translate(float!(0.0,0.1,10.0))
		.set_clear_color(1.0,1.0,1.0,1.0)
		.build();

	let mut scene = PMMLoader::open("./1.pmm").unwrap();
	scene.add(sky);
	scene.add(&camera);

	//let mut window = Window::new("PMM Loader Example");
	//window.update(move |renderer, time|
	//{
	//	camera.upload(renderer);
	//	renderer.render(&scene);
	//});
}