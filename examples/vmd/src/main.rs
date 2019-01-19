extern crate octoon_animation as anim;

use anim::{VMDLoader};

fn main()
{
    let mut motion = VMDLoader::open("./M.vmd").unwrap();
    motion.clips[0].add_event(|name, value| println!("{:?}:{:?}", name, value));
    motion.evaluate(7.5);
}