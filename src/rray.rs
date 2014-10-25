#![link(name = "rray", vers = "0.1")]

#![feature(globs)]
#![allow(non_snake_case)]

use math::Clamp;

mod geometry;
mod math;
mod scene;
mod trace;

fn main() {
    let antialias = true;
    let scene = scene::get_ref_scene();

    // Create!
    let r = trace::render(&scene, antialias);

    println!("P3");
    println!("{} {}", scene.width, scene.height);
    println!("255");

    let mut data = String::with_capacity(scene.width * scene.height * 12);

    for y in range(0u, scene.height) {
        for x in range(0u, scene.width) {
            let pix = r[y][x];

            // Clamp our rgb values to 0-255
            let r = (pix.x * 255.0).clamp(0., 255.) as int;
            let g = (pix.y * 255.0).clamp(0., 255.) as int;
            let b = (pix.z * 255.0).clamp(0., 255.) as int;

            data.push_str(format!("{} {} {} ", r, g, b).as_slice());
        }
    }

    std::io::println(data.as_slice());
}
