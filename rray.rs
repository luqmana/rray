#[link(name = "rray", vers = "0.1")];

mod geometry;
mod math;
mod scene;
mod trace;

fn main() {
    let antialias = true;
    let scene = scene::getRefScene();

    // Create!
    let r = trace::render(&scene, antialias);

    println("P3");
    printfln!("%u %u", scene.width, scene.height);
    println("255");

    let mut data = std::str::with_capacity(scene.width * scene.height * 12);

    for y in range(0u, scene.height) {
        for x in range(0u, scene.width) {
            let pix = r[y][x];

            // Clamp our rgb values to 0-255
            let r = (pix.x * 255.0).clamp(&0.0, &255.0) as int;
            let g = (pix.y * 255.0).clamp(&0.0, &255.0) as int;
            let b = (pix.z * 255.0).clamp(&0.0, &255.0) as int;

            data.push_str(fmt!("%d %d %d ", r, g, b));
        }
    }

    println(data);
}
