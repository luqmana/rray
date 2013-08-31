use std::vec;

use geometry::*;
use math::*;
use scene::*;

fn makeGrid(w: uint, h: uint, x: uint, y: uint) -> ~[~[Pixel]] {
    do vec::from_fn(h) |j| {
        do vec::from_fn(w) |i| {
            Vec2::new((i + x) as f32, (j + y) as f32)
        }
    }
}

// Basically shoot a ray out to every primitive in our scene and find the one in front
fn intersectNodes(ps: &[@Object], ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection> {
    do ps.rev_iter().fold(None) |y: Option<Intersection>, x| {
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
fn trace(ps: &[@Object], amb: &Vec3f32, ray: &Vec3f32, origin: &Vec3f32, lights: &[Light]) -> Colour {
    do intersectNodes(ps, ray, origin).map_default(
        Vec3::zero() // No intersection, so just give back black
    ) |&(iRayLen, iRay, iP)| {
        // We've hit something!
        let intersection = origin.add_v(&ray.mul_t(iRayLen));
        let normal = iRay.normalize();
        let normalizedRay = ray.normalize();
        let mat = iP.mat();

        // Find where the lights in the scene intersect with the current object
        let lightIntersections = do lights.iter().filter_map |&light| {

            let shadowRay = light.pos.sub_v(&intersection);

            match intersectNodes(ps, &shadowRay, &intersection) {
                None => Some((light, shadowRay)),
                _ => None
            }
        }.collect::<~[(Light, Vec3f32)]>();

        // Calculate colour values based on the object's material
        let shadedColours = do lightIntersections.map |&(light, shadowRay)| {
            let normalizedShadowRay = shadowRay.normalize();

            // calculate the diffuse coefficient
            let diffuseCoef = normal.dot(&normalizedShadowRay);

            // and the specular coefficient
            let refShadowRay = normalizedShadowRay.sub_v(&normal.mul_t(2.0 * diffuseCoef));
            let specularCoef = refShadowRay.dot(&normalizedRay).pow(&mat.shininess);

            // Now for the colours

            // the diffuse component
            let diffuseColours = if diffuseCoef > EPSILON {
                mat.diffuse.mul_t(diffuseCoef).mul_v(&light.colour)
            } else { Vec3::zero() };

            // and the specular component
            let specularColours = if specularCoef > EPSILON {
                mat.specular.mul_t(specularCoef).mul_v(&light.colour)
            } else { Vec3::zero() };

            (diffuseColours, specularColours)
        };

        // Now add the colours up from all the light sources
        let (diffuse, specular) =
            do shadedColours.rev_iter().fold((Vec3::zero(), Vec3::zero()))
              |(rDiffuseColour, rSpecularColour), &(diffuseColour, specularColour)| {
                (diffuseColour.add_v(&rDiffuseColour), specularColour.add_v(&rSpecularColour))
            };

        // Add in the ambient colour and voilà, we have our final colour value
        diffuse.add_v(&specular.add_v(&amb.mul_v(&mat.diffuse)))
    }
}

// Intiate the actual shooting of rays and tracing for a given pixel
fn doTrace(s: &Scene, sp: &SceneParams, posn: &Pixel) -> Colour {

    // If antialias is on break the pixel into 4 'sub pixels'
    let subPixels: ~[Pixel] = if sp.antialias {
        ~[Vec2::new(posn.x + 0.25, posn.y + 0.25),
          Vec2::new(posn.x + 0.25, posn.y + 0.75),
          Vec2::new(posn.x + 0.75, posn.y + 0.25),
          Vec2::new(posn.x + 0.75, posn.y + 0.75)]
    } else {
        ~[Vec2::new(posn.x, posn.y)]
    };

    // Evenly weight the colour contribution of each sub pixel
    let coef = 1.0 / (subPixels.len() as f32);

    do subPixels.rev_iter().fold(Vec3::zero()) |results, &cs: &Pixel| {
        let currentPixel = sp.topPixel
                            .add_v(&sp.horVec.mul_t(sp.aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(-cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives, &s.ambient, &ray, &s.camera, s.lights);

        colour.mul_t(coef).add_v(&results)
    }
}

// Let's render our beautiful scene
pub fn render(s: &Scene, antialias: bool) -> ~[~[Colour]] {
    let params = setupScene(s, antialias);

    do makeGrid(s.width, s.height, 0, 0).map |column| {
        column.map(|pix| doTrace(s, &params, pix))
    }
}
