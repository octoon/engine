extern crate octoon_models as model;

use std::fs::File;
use std::io::Read;
use model::loaders::pmm::{PMMLoader};

fn main()
{
	let mut buffer = Vec::new();
	File::open("./1.pmm").unwrap().read_to_end(&mut buffer).unwrap();
	let loader = PMMLoader::new();
	loader.do_load(&buffer).unwrap();
}