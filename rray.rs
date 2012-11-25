use core::pipes;
use lmath::funs::common::*;

use geometry::*;
use scene::*;

type Pixel = Vec2<float>;
type Colour = Vec3<float>;
type SceneParams = (float, float, Vec3<float>, Vec3<float>, bool);

fn deg2rad(d: float) -> float {
    d * float::consts::pi / 180.0f
}

fn makePixels(w: uint, h: uint, x: uint, y: uint) -> ~[~[Pixel]] {
    let xs = vec::from_fn(w, |n| (n + x) as float);
    let ys = vec::from_fn(h, |n| (n + y) as float);

    vec::foldr(ys, ~[], |y, result| {
        result + ~[vec::map(xs, |x| Vec2::new(*x, *y))]
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

fn intersectNodes(ps: &[Primitive], ray: &Vec3<float>, origin: &Vec3<float>) -> Option<Intersection> {
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

fn trace(ps: &[Primitive], amb: &Vec3<float>, ray: &Vec3<float>, origin: &Vec3<float>, lights: &[Light]) -> Colour {
    match move intersectNodes(ps, ray, origin) {
        Some((iRayLen, iRay, iP)) => {
            let intersection = origin.add_v(&ray.mul_t(iRayLen));
            let normal = iRay.normalize();
            let normalizedRay = ray.normalize();
            let mat = iP.mat;
            let lightIntersections = vec::filter(vec::map(lights, |light| {
                let shadowRay = light.pos.sub_v(&intersection);
                (*light, intersectNodes(ps, &shadowRay, &intersection))
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

            // Add up the diffuse, specular, and ambience
            // components for the final colour
            diffuse.add_v(&specular.add_v(&vecMult(amb, &mat.diffuse)))
        },
        None => Vec3::new(0.0f, 0.0f, 0.0f)
    }
}

fn doTrace(s: &Scene, params: SceneParams, posn: &Pixel) -> Colour {
    let (aspectRatio, _viewLen, horVec, topPixel, aa) = params;
    let subPixels =
        if aa {
            ~[*posn,
              Vec2::new(posn.x, posn.y + 0.5f),
              Vec2::new(posn.x + 0.5f, posn.y),
              Vec2::new(posn.x + 0.5f, posn.y + 0.5f)]
        } else {
            ~[*posn]
        };
    let coef = 1.0f / (vec::len(subPixels) as float);

    vec::foldr(subPixels, Vec3::new(0.0f, 0.0f, 0.0f), |cs, results| {
        let currentPixel = topPixel
                            .add_v(&horVec.mul_t(aspectRatio * cs.x))
                            .add_v(&s.up.mul_t(cs.y));
        let ray = currentPixel.sub_v(&s.camera);
        let colour = trace(s.primitives, &s.ambient, &ray, &s.camera, s.lights);

        colour.mul_t(coef).add_v(&results)
    })
}

fn render(s: &Scene, aa: bool) -> ~[~[Colour]] {
    let params = setupScene(s, aa);

    let p = 256;

    let wFit = ((s.width as float) / (p as float)).ceil();
    let hFit = ((s.height as float) / (p as float)).ceil();
    let wFit = wFit as uint, hFit = hFit as uint;

    let tasks = pipes::PortSet();

    for uint::range(0, hFit) |i| {
        for uint::range(0, wFit) |k| {

            let w = if k == (wFit - 1) {
                        s.width - (p * k)
                    } else { p };
            let h = if i == (hFit - 1) {
                        s.height - (p * i)
                    } else { p };

            let (to_master, from_task) = pipes::stream();
            tasks.add(move from_task);

            let grid = makePixels(w, h, p * k, p * i);

            do task::spawn_sched(task::ThreadPerCore) |move to_master, move grid| {
                vec::map(grid, |col| {
                    vec::map(*col, |pix| {
                        // TODO use the same scene that's passed
                        let scene = getRefScene();
                        to_master.send((*pix, doTrace(&scene, params, pix)));
                    });
                });
            };
        }
    }

    // Our final result
    let mut result: ~[~[Colour]] =
        vec::map(makePixels(s.width, s.height, 0, 0), |col| {
            vec::map(*col, |_| {
                Vec3::new(0.0f, 0.0f, 0.0f)
            })
        });

    // Now just wait for the tasks
    let mut left = s.width * s.height;
    while left > 0 {
        let (pos, colour) = tasks.recv();

        // Flip the y axis
        result[s.height - 1 - (pos.y as uint)][pos.x as uint] = colour;
        left -= 1;
    }

    move result 
}

fn main() {

    let antialias = true;
    let refScene = getRefScene();
    let r = render(&refScene, antialias);

    io::println("P3");
    io::println(#fmt("%u %u", refScene.width, refScene.height));
    io::println("255");

    for uint::range(0, refScene.height) |y| {
        for uint::range(0, refScene.width) |x| {
            let colour = r[y][x];
            let r = (colour.x * 255.0f).round().clamp(&(0.0f), &(255.0f));
            let g = (colour.y * 255.0f).round().clamp(&(0.0f), &(255.0f));
            let b = (colour.z * 255.0f).round().clamp(&(0.0f), &(255.0f));

            io::print(#fmt("%? %? %? ", r as u8, g as u8, b as u8));
        }
        io::println("");
    }
}
