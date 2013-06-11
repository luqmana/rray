#[link(name = "rray", vers = "0.1")];

extern mod lmath;
extern mod extra;

use std::uint;

mod geometry;
mod scene;
mod trace;

fn main() {
    let antialias = true;
    let scene = scene::getRefScene();

    // Create!
    let r = trace::render(&scene, antialias);

    println("P3");
    println(fmt!("%u %u", scene.width, scene.height));
    println("255");

    let mut data = std::str::with_capacity(scene.width * scene.height * 8);

    for uint::range(0, scene.height) |y| {
        for uint::range(0, scene.width) |x| {
            let pix = r[y][x];

            // Clamp our rgb values to 0-255
            let r = (pix.x * 255.0).clamp(&0.0, &255.0) as u8;
            let g = (pix.y * 255.0).clamp(&0.0, &255.0) as u8;
            let b = (pix.z * 255.0).clamp(&0.0, &255.0) as u8;

            std::str::push_str(&mut data, fmt!("%? %? %?", r, g, b));
        }
    }

    println(data);
}
