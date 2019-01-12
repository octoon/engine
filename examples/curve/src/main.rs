extern crate octoon_animation as anim;

use anim::{Keyframe, AnimationCurve};

fn main()
{
	let mut ks = Vec::new();

    for i in 0..50
    {
        ks.push(Keyframe::<f32>::new(i as f32, f32::sin(i as f32), None));
    }

    let anim = AnimationCurve::new(ks, None);

    for i in 0..100
    {
    	println!("{:?}", anim.evaluate(i as f32 / 2.0));
    }
}