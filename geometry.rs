use lmath::vec::*;

pub static EPSILON: f32 = 1.0e-4;

pub type Pixel = vec2;
pub type Colour = vec3;
pub type Intersection = (f32, vec3, @Primitive);

pub trait Primitive {
    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect(&self, ray: &vec3, origin: &vec3) -> Option<Intersection>;

    // Get the material
    #[inline(always)]
    fn mat(&self) -> Material;
}

// Material properties for primitives
pub struct Material {
    diffuse: Colour,    // Diffuse colour component
    specular: Colour,   // Specular colour component
    shininess: f32,     // Is the rock shiny????
    mirror: f32         // And mirror-y?
}

// Supported primitives
// TODO: Add more than just spheres

pub struct Sphere {
    pos: vec3,         // Position
    rad: f32,          // Radius
    mat: Material      // Material
}

impl Primitive for Sphere {

    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect(&self, ray: &vec3, origin: &vec3) -> Option<Intersection> {

        // Determine the ray from the origin to us and solve
        // the quadratic equation to check for an intersection
        let line = self.pos.sub_v(origin);
        let rayLens = quadRoot(ray.length2(), -2.0 * line.dot(ray),
                               line.length2() - self.rad.pow(2.0));

        // Find the shortest ray which hits us, if any
        let shortestRay = match rayLens.len() {
            1 => Some(rayLens[0]),
            2 if rayLens[0] < rayLens[1] => Some(rayLens[0]),
            2 => Some(rayLens[1]),
            _ => None
        };

        // Finally, if we've found one, return the intersection point
        // that is intersection ray, it's length and a reference to the object
        match shortestRay {
            Some(rayLen) if rayLen > EPSILON => {
                // Calculate the intersection ray
                let iRay = ray.mul_t(rayLen).sub_v(&line);

                Some((rayLen, iRay, @*self as @Primitive))
            }
            _ => None
        }
    }

    // Get the material
    #[inline(always)]
    fn mat(&self) -> Material {
        self.mat
    }
}

fn quadRoot(a: f32, b: f32, c: f32) -> ~[f32] {
    if a.abs() < EPSILON {
        ~[(-c) / b]
    } else {
        let d = b.pow(2.0) - (4.0 * a * c);
        if d.is_positive() {
            let sq = d.sqrt();
            let ta = a * 2.0;

            ~[((-b) + sq) / ta, ((-b) - sq) / ta]
        } else {
            ~[]
        }
    }
}
