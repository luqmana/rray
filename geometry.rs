pub struct Material {
    diffuse: Vec3<float>,
    specular: Vec3<float>,
    shininess: float,
    mirror: float
}

// -- Primitives
pub enum Primitive {
    Sphere(Sphere)
}

pub struct Sphere {
    pos: Vec3<float>,
    rad: float,
    mat: Material
}


pub const EPSILON: float = 1.0e-4;

pub type Intersection = (float, Vec3<float>, Primitive);

fn rayEpsilonCheck(rayLen: float, ray: Vec3<float>, line: Vec3<float>, node: &Primitive) -> Option<Intersection> {
    if rayLen > EPSILON {
        Some((rayLen, ray.mul_t(rayLen).sub_v(&line), *node))
    } else {
        None
    }
}
use std::cmp::FuzzyEq;
fn quadRoot(A: float, B: float, C: float) -> ~[float] {
    if float::abs(A) < EPSILON {
        ~[(-C) / B]
    } else {
        let d = (B * B) - (4.0f * A * C);
        if float::is_positive(d) {
            ~[((-B) + float::sqrt(d)) / (2.0 * A),
              ((-B) - float::sqrt(d)) / (2.0 * A)]
        } else {
            ~[]
        }
    }
}

pub fn intersect(p: &Primitive, ray: Vec3<float>, origin: Vec3<float>) -> Option<Intersection> {

    // Now do primitive specific intersect testing
    match *p {
        Sphere(_) => {
            let line = p.pos.sub_v(&origin);
            let rayLens = quadRoot(ray.length2(), -2.0f * line.dot(&ray), line.length2() - (p.rad * p.rad));
            match vec::len(rayLens) {
                1 => rayEpsilonCheck(rayLens[0], ray, line, p),
                2 if rayLens[0] < rayLens[1] => rayEpsilonCheck(rayLens[0], ray, line, p),
                2 => rayEpsilonCheck(rayLens[1], ray, line, p),
                _ => None
            }
        }
    }
}
