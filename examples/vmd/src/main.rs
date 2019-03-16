extern crate octoon;

fn main()
{
    let mut motion = octoon::animation::open("./M.vmd").unwrap();
    motion.clips[0].add_event(|name, value| println!("{:?}:{:?}", name, value));
    motion.evaluate(7.5);
}