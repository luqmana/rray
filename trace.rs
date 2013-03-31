use core::cell::Cell;
use geometry::*;
use lmath::vec::*;
use numeric::*;
use scene::*;
use sdl;

use powf = core::unstable::intrinsics::powf32;

fn makeGrid(w: uint, h: uint, x: uint, y: uint) -> ~[~[Pixel]] {
    vec::from_fn(h, |j| {
        vec::from_fn(w, |i| vec2::new((i + x) as f32, (j + y) as f32))
    })
}

// Basically shoot a ray out to every primitive in our scene and find the one in front
fn intersectNodes(ps: &[@Primitive], ray: &vec3, origin: &vec3) -> Option<Intersection> {
    ps.foldr(None, |x, y: Option<Intersection>| {
        match x.intersect(ray, origin) {
            Some(newIntersection @ (rayLen, _, _)) => {
                match y {
                    Some((oRayLen, _, _)) if oRayLen < rayLen => y,
                    _ => Some(newIntersection)
                }
            }
            None => y
        }
    })
}

// Calculate the colour value for some ray
fn trace(ps: &[@Primitive], amb: &vec3, ray: &vec3, origin: &vec3, lights: &[Light]) -> Colour {
    match intersectNodes(ps, ray, origin) {
        Some((iRayLen, iRay, iP)) => {
            // We've hit something!
        
            let intersection = origin.add_v(&ray.mul_t(iRayLen));
            let normal = iRay.normalize();
            let normalizedRay = ray.normalize();
            let mat = iP.mat();

            // Find where the lights in the scene intersect with the current object
            let lightIntersections = do lights.filter_mapped |&light| {
                let shadowRay = light.pos.sub_v(&intersection);

                match intersectNodes(ps, &shadowRay, &intersection) {
                    None => Some((light, shadowRay)),
                    _ => None
                }
            };

            // Calculate colour values based on the object's material
            let shadedColours = do lightIntersections.map |&(light, shadowRay)| {
                let normalizedShadowRay = shadowRay.normalize();

                // calculate the diffuse coefficient
                let diffuseCoef = normal.dot(&normalizedShadowRay);

                // and the specular coefficient
                let refShadowRay = normalizedShadowRay.sub_v(&normal.mul_t(2.0 * diffuseCoef));
                let specularCoef = powf(refShadowRay.dot(&normalizedRay), mat.shininess);

                // Now for the colours

                // the diffuse component
                let diffuseColours = if diffuseCoef > EPSILON {
                    mat.diffuse.mul_t(diffuseCoef).mul_v(&light.colour)
                } else { vec3::zero() };

                // and the specular component
                let specularColours = if specularCoef > EPSILON {
                    mat.specular.mul_t(specularCoef).mul_v(&light.colour)
                } else { vec3::zero() };

                (diffuseColours, specularColours)
            };

            // Now add the colours up from all the light sources
            let (diffuse, specular) =
                do shadedColours.foldr((vec3::zero(), vec3::zero()))
                  |&(diffuseColour, specularColour), (rDiffuseColour, rSpecularColour)| {

                    (diffuseColour.add_v(&rDiffuseColour), specularColour.add_v(&rSpecularColour))
                };

            // Add in the ambient colour and voilà, we have our final colour value
            diffuse.add_v(&specular.add_v(&amb.mul_v(&mat.diffuse)))
        }

        // No intersection, so just give back black
        _ => vec3::zero()
    }
}

// Intiate the actual shooting of rays and tracing for a given pixel
fn doTrace(s: &Scene, sp: &SceneParams, posn: &Pixel) -> Colour {

    // If antialias is on break the pixel into 4 'sub pixels'
    let subPixels = if sp.antialias {
        ~[vec2::new(posn.x + 0.25, posn.y + 0.25),
          vec2::new(posn.x + 0.25, posn.y + 0.75),
          vec2::new(posn.x + 0.75, posn.y + 0.25),
          vec2::new(posn.x + 0.75, posn.y + 0.75)]
    } else {
        ~[vec2::new(posn.x, posn.y)]
    };

    // Evenly weight the colour contribution of each sub pixel
    let coef = 1.0 / (subPixels.len() as f32);

    subPixels.foldr(vec3::zero(), |cs, results| {
        let currentPixel = sp.topPixel
                            .add_v(&sp.horVec.mul_t(sp.aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(-cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives, &s.ambient, &ray, &s.camera, s.lights);

        colour.mul_t(coef).add_v(&results)
    })
}

// Let's render our beautiful scene
fn render(s: &Scene, antialias: bool, out: comm::Chan<(uint, uint, Colour)>) {
    let params = setupScene(s, antialias);

    let grid = makeGrid(s.width, s.height, 0, 0);
    for grid.each |column| {
        for column.each |pix| {
            out.send((pix.x as uint, pix.y as uint, doTrace(s, &params, pix)));
        }
    }
}

fn main() {

    // Create a port-channel pair so we can get the pixels back
    let (rport, rchan) = comm::stream();
    let rchan = Cell(rchan);

    do task::spawn_sched(task::ThreadPerCore) {
        // Get our reference scene
        let antialias = true;
        let scene = getRefScene();

        // Kick off the ray tracing
        render(&scene, antialias, rchan.take());

    }

    do sdl::start {
        sdl::init([sdl::InitVideo]);

        sdl::wm::set_caption("Rust Ray Tracer", "rray");

        let screen = match sdl::video::set_video_mode(
                256, 256, 32, [sdl::video::HWSurface], []) {
            Ok(screen) => screen,
            Err(e) => fail!(fmt!("Failed to set video mode: %s", e))
        };

        // Start with a white screen
        screen.fill(sdl::video::RGB(255, 255, 255));
        screen.flip();

        loop main: {

            // Wait for a pixel
            match rport.try_recv() {
                Some((x, y, pix)) => {

                    // Clamp our rgb values to 0-255
                    let r = (pix.x * 255.0).clamp(0.0, 255.0) as u8;
                    let g = (pix.y * 255.0).clamp(0.0, 255.0) as u8;
                    let b = (pix.z * 255.0).clamp(0.0, 255.0) as u8;

                    // and draw the pixel!
                    screen.fill_rect(Some(sdl::Rect { x: x as i16, y: y as i16, w: 1, h: 1 }),
                                     sdl::video::RGB(r, g, b));
                }
                _ => break main
            }

            screen.flip();

        }

        loop {
            match sdl::event::poll_event() {
                sdl::event::QuitEvent => break,
                sdl::event::KeyEvent(k, _, _, _) => {
                    match k {
                        sdl::event::EscapeKey => break,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        sdl::quit();
    }

}
