use geometry::*;
use lmath::vec::*;
use numeric::*;
use scene::*;

use powf = core::unstable::intrinsics::powf32;

fn makeGrid(w: uint, h: uint, x: uint, y: uint) -> ~[~[Pixel]] {
    do vec::from_fn(h) |j| {
        do vec::from_fn(w) |i| {
            Vec2f32::new((i + x) as f32, (j + y) as f32)
        }
    }
}

// Basically shoot a ray out to every primitive in our scene and find the one in front
fn intersectNodes(ps: &[@Primitive], ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection> {
    do ps.foldr(None) |x, y: Option<Intersection>| {
        match x.intersect(ray, origin) {
            Some(newIntersection @ (rayLen, _, _)) => {
                match y {
                    Some((oRayLen, _, _)) if oRayLen < rayLen => y,
                    _ => Some(newIntersection)
                }
            }
            None => y
        }
    }
}

// Calculate the colour value for some ray
fn trace(ps: &[@Primitive], amb: &Vec3f32, ray: &Vec3f32, origin: &Vec3f32, lights: &[Light]) -> Colour {
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
                } else { Vec3f32::zero() };

                // and the specular component
                let specularColours = if specularCoef > EPSILON {
                    mat.specular.mul_t(specularCoef).mul_v(&light.colour)
                } else { Vec3f32::zero() };

                (diffuseColours, specularColours)
            };

            // Now add the colours up from all the light sources
            let (diffuse, specular) =
                do shadedColours.foldr((Vec3f32::zero(), Vec3f32::zero()))
                  |&(diffuseColour, specularColour), (rDiffuseColour, rSpecularColour)| {

                    (diffuseColour.add_v(&rDiffuseColour), specularColour.add_v(&rSpecularColour))
                };

            // Add in the ambient colour and voilà, we have our final colour value
            diffuse.add_v(&specular.add_v(&amb.mul_v(&mat.diffuse)))
        }

        // No intersection, so just give back black
        _ => Vec3f32::zero()
    }
}

// Intiate the actual shooting of rays and tracing for a given pixel
fn doTrace(s: &Scene, sp: &SceneParams, posn: &Pixel) -> Colour {

    // If antialias is on break the pixel into 4 'sub pixels'
    let subPixels = if sp.antialias {
        ~[Vec2f32::new(posn.x + 0.25, posn.y + 0.25),
          Vec2f32::new(posn.x + 0.25, posn.y + 0.75),
          Vec2f32::new(posn.x + 0.75, posn.y + 0.25),
          Vec2f32::new(posn.x + 0.75, posn.y + 0.75)]
    } else {
        ~[Vec2f32::new(posn.x, posn.y)]
    };

    // Evenly weight the colour contribution of each sub pixel
    let coef = 1.0 / (subPixels.len() as f32);

    do subPixels.foldr(Vec3f32::zero()) |cs, results| {
        let currentPixel = sp.topPixel
                            .add_v(&sp.horVec.mul_t(sp.aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(-cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives, &s.ambient, &ray, &s.camera, s.lights);

        colour.mul_t(coef).add_v(&results)
    }
}

// Let's render our beautiful scene
fn render(s: &Scene, antialias: bool) -> ~[~[Colour]] {
    let params = setupScene(s, antialias);

    do makeGrid(s.width, s.height, 0, 0).map |column| {
        column.map(|pix| doTrace(s, &params, pix))
    }
}

fn main() {

    let antialias = true;
    let scene = getRefScene();

    // Create!
    let r = render(&scene, antialias);

    io::println("P3");
    io::println(fmt!("%u %u", scene.width, scene.height));
    io::println("255");

    for uint::range(0, scene.height) |y| {
        for uint::range(0, scene.width) |x| {
            let pix = r[y][x];

            // Clamp our rgb values to 0-255
            let r = (pix.x * 255.0).clamp(0.0, 255.0) as u8;
            let g = (pix.y * 255.0).clamp(0.0, 255.0) as u8;
            let b = (pix.z * 255.0).clamp(0.0, 255.0) as u8;

            io::println(fmt!("%? %? %?", r, g, b));
        }
    }

}
