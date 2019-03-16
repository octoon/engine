extern crate octoon;

use std::sync::Arc;
use octoon::scene::{SkyLight, TextureLoader};
use octoon::renderer::window::Window;

fn main()
{
	let irradiance = Arc::new(TextureLoader::load_from_memory(include_bytes!("../static/output_iem.png")).unwrap());
	let sky = SkyLight::builder(irradiance.clone(), irradiance.clone()).build();

	let mut scene = octoon::scene::open("./1.pmm").unwrap();
	scene.add(sky);

	let mut window = Window::new("PMM Loader Example");
	window.update(move |canvas, _|
	{
		scene.update(canvas);
		canvas.render(&scene);
	});
}