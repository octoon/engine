#![recursion_limit = "512"]

#[macro_use]
extern crate octoon_math;
extern crate octoon_scene;

use std::sync::Arc;
use std::path::Path;

use octoon_math::{float3, float4x4, Quaternion, One};
use octoon_scene::core::{Object, Light, LightType};
use octoon_scene::lights::{ PointLight, SkyLight };
use octoon_scene::cameras::{ PerspectiveCamera };
use octoon_scene::geometries::{ SphereGeometry };
use octoon_scene::loader::{TextureLoader, ModelLoader};
use octoon_scene::scene::Scene;
use octoon_scene::spectrum::{Kelvin, Lumens, LED};
use octoon_scene::materials::{StandardMaterial, SkyboxMaterial};
use octoon_scene::shapes::MeshShape;
use octoon_renderer::window::Window;

#[macro_use]
#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
extern crate stdweb;

#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
use stdweb::unstable::TryInto;

pub fn main()
{
	octoon_scene::core::log::init_log().unwrap();

	let irradiance = Arc::new(TextureLoader::load_from_memory(include_bytes!("../static/output_iem.png")).unwrap());

	let material = StandardMaterial::builder()
		.set_albedo(float!(192./255.,185./255.,176./255.).into())
		.set_smoothness(0.6_f32.into())
		.build();

	let sky_material = SkyboxMaterial::builder()
		.set_color(float3::one().into())
		.set_texture(Some(irradiance.clone()))
		.build();

	let sky_geometry = SphereGeometry::builder()
		.set_radius(1000.0)
		.set_width_segments(10)
		.set_height_segments(10)
		.build();

	let sphere_geometry = SphereGeometry::builder()
		.set_radius(1.0)
		.set_width_segments(32)
		.set_height_segments(32)
		.build();

	let mut camera = PerspectiveCamera::builder()
		.main(true)
		.set_film_size(36.0)
		.set_focal_length(50.0)
		.set_translate(float!(0.0,0.1,10.0))
		.build();

	let mut light = PointLight::builder()
		.set_color(Kelvin(6000.0).into())
		.set_intensity(Lumens::from(LED(10.0)).to_cd(LightType::Point).into())
		.set_translate(float3::new(0.0,10.0,0.0))
		.build();

	let sky = SkyLight::builder(irradiance.clone(), irradiance.clone())
		.build();

	let sky_shape = MeshShape::builder()
		.set_geometry(sky_geometry.into())
		.set_material(sky_material.into())
		.set_rotation(float!(0.0,3.1415926,0.0))
		.build();

	let sphere = MeshShape::builder()
		.set_geometry(sphere_geometry.into())
		.set_material(material.into())
		.set_translate(float!(0.0,0.0,20.0))
		.set_rotation(float!(0.0,3.1415926,0.0))
		.build();

	/*let mut model = ModelLoader::load_from_memory(include_bytes!("../static/安特.pmx")).unwrap();
	model.set_scale(float!(0.1,0.1,0.1));
	model.set_translate(float!(0.0,-0.8,20.0));*/

	let mut scene = Scene::new();
	scene.add(&camera);
	scene.add(&sky_shape);
	scene.add(&sphere);
	scene.add(&light);
	scene.add(&sky);
	//scene.add(model);

	#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
	js!{
		window.dat.light = {
			x:30,
			y:30,
			z:30,
			temperature:6000,
			intensity:10,
		};

		window.dat.camera = {
			x:0,
			y:0,
			z:10,
			film_size:"35mm",
			focal_length:"50mm"
		};

		window.dat.exposure = {
			aperture:"1.0",
			iso:"100",
			shutter_speed:"1/125",
		};

		var gui = new dat.GUI();

		var light = gui.addFolder("Light");
		light.add(window.dat.light,"x",-180,180);
		light.add(window.dat.light,"y",-180,180);
		light.add(window.dat.light,"z",-180,180);
		light.add(window.dat.light,"temperature",1000,40000);
		light.add(window.dat.light,"intensity",0,20);

		var camera = gui.addFolder("Camera");
		camera.add(window.dat.camera,"x",-3,3);
		camera.add(window.dat.camera,"y",-3,3);
		camera.add(window.dat.camera,"z",0,40);
		camera.add(window.dat.camera,"film_size",["7mm","35mm","41mm","62mm"],);
		camera.add(window.dat.camera,"focal_length",["15mm","20mm","24mm","28mm","35mm","50mm","80mm","135mm","200mm"]);

		var exposure = gui.addFolder("Exposure");
		exposure.add(window.dat.exposure,"aperture", [,"1.0","1.2","1.4","1.8","2.5","2.8","3.2","4.8","5.6","6.7","2","4","8","11","13","16","18","22","27","32"]);
		exposure.add(window.dat.exposure,"iso",["50","100","125","160","200","250","320","400","500","640","800","1000","1250","1600","2000","2500","3200","4000","5000","6400"]);
		exposure.add(window.dat.exposure,"shutter_speed",["1","2","4","1/4000","1/2000","1/1000","1/500","1/250","1/125","1/60","1/30","1/15","1/8","1/4","1/2"]);
	};

	let mut window = Window::new("Webgl Example");
	window.update(move |renderer, time|
	{
		#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
		{
			let light_x:i32 = (js! { return parseInt(window.dat.light.x); } ).try_into().unwrap();
			let light_y:i32 = (js! { return parseInt(window.dat.light.y); } ).try_into().unwrap();
			let light_z:i32 = (js! { return parseInt(window.dat.light.z); } ).try_into().unwrap();
			let temperature:i32 = (js! { return parseInt(window.dat.light.temperature); } ).try_into().unwrap();
			let intensity:i32 = (js! { return parseInt(window.dat.light.intensity); } ).try_into().unwrap();
			let rotation:float4x4 = Quaternion::euler_xyz(&float!((light_x as f32).to_degrees(),(light_y as f32).to_degrees(),(light_z as f32).to_degrees())).into();

			light.set_color(Kelvin(temperature as f32).into());
			light.set_intensity(Lumens::from(LED(intensity as f32)).to_cd(LightType::Point).into());
			light.set_translate(float!(0.0,10.0,0.0) * rotation);
		}

		#[cfg(any(target_arch = "wasm32", target_arch = "asmjs"))]
		{
			let camera_x:i32 = (js! { return parseInt(window.dat.camera.x); } ).try_into().unwrap();
			let camera_y:i32 = (js! { return parseInt(window.dat.camera.y); } ).try_into().unwrap();
			let camera_z:i32 = (js! { return parseInt(window.dat.camera.z); } ).try_into().unwrap();

			camera.set_translate(float!(camera_x as f32,camera_y as f32,camera_z as f32));
		}

		#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
		{
			let rotation:float4x4 = Quaternion::euler_xyz(&float!(time, time, time)).into();
			light.set_translate(float!(0.0,10.0,0.0) * rotation);
		}

		camera.upload(renderer);

		renderer.render(&scene)
	});
}