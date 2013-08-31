use geometry::*;
use math::*;

pub struct SceneParams {
    aspectRatio: f32,
    viewLen: f32,
    horVec: Vec3f32,
    topPixel: Vec3f32,
    antialias: bool
}

// Light source
pub struct Light {
    pos: Vec3f32,
    colour: Colour
}

// Complete scene
pub struct Scene {
    // Light sources
    lights: ~[Light],

    // Objects to possibly render
    primitives: ~[@Object],

    // Ambient colour
    ambient: Colour,

    // Camera location
    camera: Vec3f32,

    // Viewpoint
    view: Vec3f32,

    // Which way is "up"
    up: Vec3f32,

    // Dimensions
    width: uint,
    height: uint,

    // Field of view
    fov: f32
}

// Setup some of the scene parameters
pub fn setupScene(s: &Scene, aa: bool) -> SceneParams {
    let aspectRatio = (s.width as f32) / (s.height as f32);
    let viewLen = (s.height as f32) / s.fov.to_radians().tan();
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
pub fn getRefScene() -> Scene {
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
        lights: ~[
            Light {
                pos: Vec3::new(-100.0, 150.0, 400.0),
                colour: Vec3::new(0.7, 0.7, 0.7)
            },
            Light {
                pos: Vec3::new(400.0, 100.0, 150.0),
                colour: Vec3::new(0.7, 0.0, 0.7)
            }
        ],
        primitives: ~[
            @Sphere {
                pos: Vec3::new(0.0, 0.0, -400.0),
                rad: 100.0,
                mat: mat1
            } as @Object,
            @Sphere {
                pos: Vec3::new(200.0, 50.0, -100.0),
                rad: 150.0,
                mat: mat1
            } as @Object,
            @Sphere {
                pos: Vec3::new(0.0, -1200.0, -500.0),
                rad: 1000.0,
                mat: mat2
            } as @Object,
            @Sphere {
                pos: Vec3::new(-100.0, 25.0, -300.0),
                rad: 50.0,
                mat: mat3
            } as @Object,
            @Sphere {
                pos: Vec3::new(0.0, 100.0, -250.0),
                rad: 25.0,
                mat: mat1
            } as @Object
        ],
        ambient: Vec3::new(0.3, 0.3, 0.3),
        camera: Vec3::new(0.0, 0.0, 800.0),
        view: Vec3::new(0.0, 0.0, -1.0),
        up: Vec3::new(0.0, 1.0, 0.0),
        width: 512,
        height: 512,
        fov: 45.0
    }
}
