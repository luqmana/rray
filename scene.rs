use geometry::*;
use math::*;

pub struct SceneParams {
    pub aspectRatio: f32,
    pub viewLen: f32,
    pub horVec: Vec3f32,
    pub topPixel: Vec3f32,
    pub antialias: bool
}

// Light source
pub struct Light {
    pub pos: Vec3f32,
    pub colour: Colour
}

// Complete scene
pub struct Scene {
    // Light sources
    pub lights: Vec<Light>,

    // Objects to possibly render
    pub primitives: Vec<Box<Object>>,

    // Ambient colour
    pub ambient: Colour,

    // Camera location
    pub camera: Vec3f32,

    // Viewpoint
    pub view: Vec3f32,

    // Which way is "up"
    pub up: Vec3f32,

    // Dimensions
    pub width: uint,
    pub height: uint,

    // Field of view
    pub fov: f32
}

// Setup some of the scene parameters
pub fn setup_scene(s: &Scene, aa: bool) -> SceneParams {
    let aspectRatio = (s.width as f32) / (s.height as f32);
    let viewLen = (s.height / 2) as f32 / s.fov.div(&2.0).to_radians().tan();
    let horVec = s.view.cross(&s.up).normalize();
    let centerPixel = s.camera.add_v(&s.view.mul_t(viewLen));
    let topPixel = centerPixel
                    .add_v(&horVec.mul_t((s.width as f32) / -2.0))
                    .add_v(&s.up.mul_t((s.height as f32) / 2.0));

    SceneParams {
        aspectRatio: aspectRatio,
        viewLen: viewLen,
        horVec: horVec,
        topPixel: topPixel,
        antialias: aa
    }
}

// Create a reference scene
pub fn get_ref_scene() -> Scene {
    // The materials!
    let mat1 = Material {
        diffuse: Vec3::new(0.7, 1.0, 0.7),
        specular: Vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    };
    let mat2 = Material {
        diffuse: Vec3::new(0.5, 0.5, 0.5),
        specular: Vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    };
    let mat3 = Material {
        diffuse: Vec3::new(1.0, 0.6, 0.1),
        specular: Vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    };

    // Now build the scene
    Scene {
        lights: vec![
            Light {
                pos: Vec3::new(-100.0, 150.0, 400.0),
                colour: Vec3::new(0.7, 0.7, 0.7)
            },
            Light {
                pos: Vec3::new(400.0, 100.0, 150.0),
                colour: Vec3::new(0.7, 0.0, 0.7)
            }
        ],
        primitives: vec![
            box Sphere {
                pos: Vec3::new(0.0, 0.0, -400.0),
                rad: 100.0,
                mat: mat1
            } as Box<Object>,
            box Sphere {
                pos: Vec3::new(200.0, 50.0, -100.0),
                rad: 150.0,
                mat: mat1
            } as Box<Object>,
            box Sphere {
                pos: Vec3::new(0.0, -1200.0, -500.0),
                rad: 1000.0,
                mat: mat2
            } as Box<Object>,
            box Sphere {
                pos: Vec3::new(-100.0, 25.0, -300.0),
                rad: 50.0,
                mat: mat3
            } as Box<Object>,
            box Sphere {
                pos: Vec3::new(0.0, 100.0, -250.0),
                rad: 25.0,
                mat: mat1
            } as Box<Object>
        ],
        ambient: Vec3::new(0.3, 0.3, 0.3),
        camera: Vec3::new(0.0, 0.0, 800.0),
        view: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        width: 2048,
        height: 2048,
        fov: 45.0
    }
}
