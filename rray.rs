use lmath::funs::common::*;

use geometry::*;
use scene::*;

type Pixel = (float, float);
type Colour = (float, float, float);
type SceneParams = (float, float, Vec3<float>, Vec3<float>, bool);

fn deg2rad(d: float) -> float {
    d * float::consts::pi / 180.0f
}

fn makePixels(w: uint, h: uint) -> ~[~[Pixel]] {
    let xs: ~[float] = vec::from_fn(w, |n| n as float);
    let ys: ~[float] = vec::from_fn(h, |n| n as float);

    vec::foldr(ys, ~[], |y, result| {
        result + ~[vec::map(xs, |x| (*x, *y))]
    })
}

fn setupScene(s: &Scene, aa: bool) -> SceneParams {
    let aspectRatio = (s.width as float) / (s.height as float);
    let viewLen = (s.height as float) / float::tan(deg2rad(s.fov));
    let horVec = s.view.cross(&s.up).normalize();
    let centerPixel = s.camera.add_v(&s.view.mul_t(viewLen));
    let topPixel = centerPixel
                    .add_v(&horVec.mul_t((s.width as float) / -2.0f))
                    .add_v(&s.up.mul_t((s.height as float) / -2.0f));

    (aspectRatio, viewLen, horVec, topPixel, aa)
}

fn intersectNodes(ps: &[Primitive], ray: Vec3<float>, origin: Vec3<float>) -> Option<Intersection> {
    vec::foldr(ps, None, |x, y| {
        match move intersect(x, ray, origin) {
            Some(move newIntersection) => {
                let (rayLen, _, _) = newIntersection;
                match y {
                    Some(oldIntersection) => {
                        let (oRayLen, _, _): Intersection = oldIntersection;
                        if oRayLen > rayLen { Some(newIntersection) }
                        else { Some(oldIntersection) }
                    }
                    None => Some(newIntersection)
                }
            },
            None => y
        }
    })
}

#[inline(always)]
fn vecMult(a: &Vec3<float>, b: &Vec3<float>) -> Vec3<float> {
    Vec3::new(a[0] * b[0], a[1] * b[1], a[2] * b[2])
}

fn trace(ps: &[Primitive], amb: Vec3<float>, ray: Vec3<float>, origin: Vec3<float>, lights: &[Light]) -> Colour {
    match move intersectNodes(ps, ray, origin) {
        Some((iRayLen, iRay, iP)) => {
            let intersection = origin.add_v(&ray.mul_t(iRayLen));
            let normal = iRay.normalize();
            let normalizedRay = ray.normalize();
            let mat = iP.mat;
            let lightIntersections = vec::filter(vec::map(lights, |light| {
                let shadowRay = light.pos.sub_v(&intersection);
                (*light, intersectNodes(ps, shadowRay, intersection))
            }), |r| {
                let (_, r) = *r;
                option::is_none(&r)
            });
            let shadedColours = vec::map(lightIntersections, |li| {
                let (light, _) = *li;
                let shadowRay = light.pos.sub_v(&intersection);
                let normalizedShadowRay = shadowRay.normalize();
                let diffuseCoef = normal.dot(&normalizedShadowRay);
                let reflectedShadowRay = normalizedShadowRay.sub_v(&normal.mul_t(2.0f * diffuseCoef));
                let specCoef = float::abs(float::pow(reflectedShadowRay.dot(&normalizedRay) as libc::c_double,
                                                     mat.shininess as libc::c_double) as float);
                let diffuseColours =
                    if diffuseCoef > EPSILON {
                        vecMult(&mat.diffuse.mul_t(diffuseCoef), &light.colour)
                    } else {
                        Vec3::new(0.0f, 0.0f, 0.0f)
                    };
                let specularColours =
                    if specCoef > EPSILON {
                        vecMult(&mat.specular.mul_t(specCoef), &light.colour)
                    } else {
                        Vec3::new(0.0f, 0.0f, 0.0f)
                    };

                (diffuseColours, specularColours)
            });
            let diffuse = vec::foldr(shadedColours, Vec3::new(0.0f, 0.0f, 0.0f), |colour, r| {
                let (diffuseColour, _) = *colour;
                diffuseColour.add_v(&r)
            });
            let specular = vec::foldr(shadedColours, Vec3::new(0.0f, 0.0f, 0.0f), |colour, r| {
                let (_, specularColour) = *colour;
                specularColour.add_v(&r)
            });

            let colours = diffuse.add_v(&specular.add_v(&vecMult(&amb, &mat.diffuse)));

            (colours.x, colours.y, colours.z)

        },
        None => (0.0f, 0.0f, 0.0f)
    }
}

fn doTrace(s: &Scene, params: SceneParams, posn: Pixel) -> Colour {
    let (aspectRatio, _viewLen, horVec, topPixel, aa) = params;
    let (x, y) = posn;
    let subPixels =
        if aa {
            ~[(x, y), (x, y + 0.5f), (x + 0.5f, y), (x + 0.5f, y + 0.5f)]
        } else {
            ~[(x, y)]
        };
    let coef = 1.0f / (vec::len(subPixels) as float);

    vec::foldr(subPixels, (0.0f, 0.0f, 0.0f), |cs, results| {
        let (sx, sy) = *cs;
        let currentPixel = topPixel
                            .add_v(&horVec.mul_t(aspectRatio * sx))
                            .add_v(&s.up.mul_t(sy));
        let ray = currentPixel.sub_v(&s.camera);
        let (r, g, b) = trace(s.primitives, s.ambient, ray, s.camera, s.lights);
        let (rr, rg, rb) = results;
        (coef * r + rr, coef * g + rg, coef * b + rb)
    })
}

fn render(s: &Scene, aa: bool) -> ~[~[Colour]] {
    let params = setupScene(s, aa);
    vec::map(makePixels(s.width, s.height), |column| {
        vec::map(*column, |pix| {
            doTrace(s, params, *pix)
        })
    })
}

fn main() {

    let antialias = false;
    let refScene = getRefScene();
    let r = render(&refScene, antialias);

    io::println("P3");
    io::println(#fmt("%u %u", refScene.width, refScene.height));
    io::println("255");

    for uint::range(0, refScene.height) |y| {
        for uint::range(0, refScene.width) |x| {
            let (r, g, b) = r[y][x];
            let r = (r * 255.0f).round().clamp(&(0.0f), &(255.0f));
            let g = (g * 255.0f).round().clamp(&(0.0f), &(255.0f));
            let b = (b * 255.0f).round().clamp(&(0.0f), &(255.0f));

            io::print(#fmt("%? %? %? ", r as u8, g as u8, b as u8));
        }
        io::println("");
    }
}
