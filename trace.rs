use geometry::*;
use math::*;
use scene::*;

fn make_grid(w: uint, h: uint, x: uint, y: uint) -> Vec<Vec<Pixel>> {
    Vec::from_fn(h, |j| {
        Vec::from_fn(w, |i| {
            Vec2::new((i + x) as f32, (j + y) as f32)
        })
    })
}

// Basically shoot a ray out to every primitive in our scene and find the one in front
fn intersect_nodes<'a>(ps: &'a [Box<Object>], ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection<'a>> {
    ps.iter().rev().fold(None, |y: Option<Intersection<'a>>, x| {
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
fn trace(ps: &[Box<Object>], amb: &Vec3f32, ray: &Vec3f32, origin: &Vec3f32, lights: &[Light]) -> Colour {
    intersect_nodes(ps, ray, origin).map_or(
        Vec3::zero(), // No intersection, so just give back black
        |(iRayLen, iRay, iP)| {
        // We've hit something!
        let intersection = origin.add_v(&ray.mul_t(iRayLen));
        let normal = iRay.normalize();
        let normalizedRay = ray.normalize();
        let mat = iP.mat();

        // Find where the lights in the scene intersect with the current object
        let lightIntersections = lights.iter().filter_map(|&light| {

            let shadowRay = light.pos.sub_v(&intersection);

            match intersect_nodes(ps, &shadowRay, &intersection) {
                None => Some((light, shadowRay)),
                _ => None
            }
        }).collect::<Vec<(Light, Vec3f32)>>();

        // Calculate colour values based on the object's material
        let shadedColours = lightIntersections.iter().map(|&(light, shadowRay)| {
            let normalizedShadowRay = shadowRay.normalize();

            // calculate the diffuse coefficient
            let diffuseCoef = normal.dot(&normalizedShadowRay);

            // and the specular coefficient
            let refShadowRay = normalizedShadowRay.sub_v(&normal.mul_t(2.0 * diffuseCoef));
            let specularCoef = refShadowRay.dot(&normalizedRay).powf(mat.shininess);

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
        });

        // Now add the colours up from all the light sources
        let (diffuse, specular) =
            shadedColours.rev().fold((Vec3::zero(), Vec3::zero()),
              |(rDiffuseColour, rSpecularColour), (diffuseColour, specularColour)| {
                (diffuseColour.add_v(&rDiffuseColour), specularColour.add_v(&rSpecularColour))
            });

        // Add in the ambient colour and voilà, we have our final colour value
        diffuse.add_v(&specular.add_v(&amb.mul_v(&mat.diffuse)))
    })
}

// Intiate the actual shooting of rays and tracing for a given pixel
fn do_trace(s: &Scene, sp: &SceneParams, posn: &Pixel) -> Colour {

    // If antialias is on break the pixel into 4 'sub pixels'
    let subPixels: Vec<Pixel> = if sp.antialias {
        vec![
            Vec2::new(posn.x + 0.25, posn.y + 0.25),
            Vec2::new(posn.x + 0.25, posn.y + 0.75),
            Vec2::new(posn.x + 0.75, posn.y + 0.25),
            Vec2::new(posn.x + 0.75, posn.y + 0.75)
        ]
    } else {
        vec![Vec2::new(posn.x, posn.y)]
    };

    // Evenly weight the colour contribution of each sub pixel
    let coef = 1.0 / (subPixels.len() as f32);

    subPixels.iter().rev().fold(Vec3::zero(), |results, &cs: &Pixel| {
        let currentPixel = sp.topPixel
                            .add_v(&sp.horVec.mul_t(sp.aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(-cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives.as_slice(), &s.ambient, &ray, &s.camera, s.lights.as_slice());

        colour.mul_t(coef).add_v(&results)
    })
}

// Let's render our beautiful scene
pub fn render(s: &Scene, antialias: bool) -> Vec<Vec<Colour>> {
    let params = setup_scene(s, antialias);

    make_grid(s.width, s.height, 0, 0).move_iter().map(|column| {
        column.move_iter().map(|pix| do_trace(s, &params, &pix)).collect()
    }).collect()
}
