use math::*;

pub static EPSILON: f32 = 1.0e-4;

pub type Pixel = Vec2f32;
pub type Colour = Vec3f32;
pub type Intersection = (f32, Vec3f32, @Object);

pub trait Object {
    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect(&self, ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection>;

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
    pos: Vec3f32,      // Position
    rad: f32,          // Radius
    mat: Material      // Material
}

impl Object for Sphere {
    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect(&self, ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection> {
        // Determine the ray from the origin to us and solve
        // the quadratic equation to check for an intersection
        let line = self.pos.sub_v(origin);
        let rayLens = quadRoot(ray.length2(), -2.0 * line.dot(ray),
                               line.length2() - self.rad.pow(&2.0));

        // Find the shortest ray which hits us, if any
        let shortestRay = match rayLens.len() {
            1 => Some(rayLens[0]),
            2 if rayLens[0] < rayLens[1] => Some(rayLens[0]),
            2 => Some(rayLens[1]),
            _ => None
        };

        // Finally, if we've found one, return the intersection point
        // that is intersection ray, it's length and a reference to the object
        do shortestRay.chain |rayLen| {
            if rayLen > EPSILON {
                let intersect_ray = ray.mul_t(rayLen).sub_v(&line);
                Some((rayLen, intersect_ray, @*self as @Object))
            } else { None }
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
        let d = b.pow(&2.0) - (4.0 * a * c);
        if d.is_positive() {
            let sq = d.sqrt();
            let ta = a * 2.0;

            ~[((-b) + sq) / ta, ((-b) - sq) / ta]
        } else {
            ~[]
        }
    }
}
