use math::*;

pub static EPSILON: f32 = 1.0e-4;

pub type Pixel = Vec2f32;
pub type Colour = Vec3f32;
pub type Intersection<'a> = (f32, Vec3f32, &'a Object+'a);

pub trait Object {
    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect<'a>(&'a self, ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection<'a>>;

    // Get the material
    #[inline(always)]
    fn mat(&self) -> Material;
}

// Material properties for primitives
pub struct Material {
    pub diffuse: Colour,    // Diffuse colour component
    pub specular: Colour,   // Specular colour component
    pub shininess: f32,     // Is the rock shiny????
    pub mirror: f32         // And mirror-y?
}

// Supported primitives
// TODO: Add more than just spheres

pub struct Sphere {
    pub pos: Vec3f32,      // Position
    pub rad: f32,          // Radius
    pub mat: Material      // Material
}

impl Object for Sphere {
    // Determine if a ray from some origin intersects with us
    // and if so give back the intersection point
    fn intersect<'a>(&'a self, ray: &Vec3f32, origin: &Vec3f32) -> Option<Intersection<'a>> {
        // Determine the ray from the origin to us and solve
        // the quadratic equation to check for an intersection
        let line = self.pos.sub_v(origin);
        let rayLens = quad_root(ray.length2(), -2.0 * line.dot(ray),
                                line.length2() - (self.rad * self.rad));

        // Find the shortest ray which hits us, if any
        let shortestRay = match rayLens {
            Zero => None,
            One(a) => Some(a),
            Two(a, b) if a < b => Some(a),
            Two(_, b) => Some(b)
        };

        // Finally, if we've found one, return the intersection point
        // that is intersection ray, it's length and a reference to the object
        shortestRay.and_then(|rayLen| {
            if rayLen > EPSILON {
                let intersect_ray = ray.mul_t(rayLen).sub_v(&line);
                Some((rayLen, intersect_ray, self as &Object))
            } else { None }
        })
    }

    // Get the material
    #[inline(always)]
    fn mat(&self) -> Material {
        self.mat
    }
}

enum QuadRootResult {
    Zero,
    One(f32),
    Two(f32, f32)
}

fn quad_root(a: f32, b: f32, c: f32) -> QuadRootResult {
    if a.abs() < EPSILON {
        One((-c) / b)
    } else {
        let d = (b * b) - (4.0 * a * c);
        if d.is_positive() {
            let sq = d.sqrt();
            let ta = a * 2.0;

            Two(((-b) + sq) / ta, ((-b) - sq) / ta)
        } else {
            Zero
        }
    }
}
