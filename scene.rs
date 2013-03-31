use geometry::*;
use lmath::vec::*;
use numeric::*;

pub struct SceneParams {
    aspectRatio: f32,
    viewLen: f32,
    horVec: vec3,
    topPixel: vec3,
    antialias: bool
}

// Light source
pub struct Light {
    pos: vec3,
    colour: Colour
}

// Complete scene
pub struct Scene {
    // Light sources
    lights: ~[Light],

    // Primitives to possibly render
    primitives: ~[@Primitive],

    // Ambient colour
    ambient: Colour,

    // Camera location
    camera: vec3,

    // Viewpoint
    view: vec3,

    // Which way is "up"
    up: vec3,

    // Dimensions
    width: uint,
    height: uint,

    // Field of view
    fov: f32
}

// Setup some of the scene parameters
pub fn setupScene(s: &Scene, aa: bool) -> SceneParams {
    let aspectRatio = (s.width as f32) / (s.height as f32);
    let viewLen = (s.height as f32) / f32::tan(s.fov.radians());
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
        diffuse: vec3::new(0.7, 1.0, 0.7),
        specular: vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    }, mat2 = Material {
        diffuse: vec3::new(0.5, 0.5, 0.5),
        specular: vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    }, mat3 = Material {
        diffuse: vec3::new(1.0, 0.6, 0.1),
        specular: vec3::new(0.5, 0.7, 0.5),
        shininess: 25.0,
        mirror: 0.3
    };

    // Now build the scene
    Scene {
        lights: ~[
            Light {
                pos: vec3::new(-100.0, 150.0, 400.0),
                colour: vec3::new(0.7, 0.7, 0.7)
            },
            Light {
                pos: vec3::new(400.0, 100.0, 150.0),
                colour: vec3::new(0.7, 0.0, 0.7)
            }
        ],
        primitives: ~[
            @Sphere {
                pos: vec3::new(0.0, 0.0, -400.0),
                rad: 100.0,
                mat: mat1
            } as @Primitive,
            @Sphere {
                pos: vec3::new(200.0, 50.0, -100.0),
                rad: 150.0,
                mat: mat1
            } as @Primitive,
            @Sphere {
                pos: vec3::new(0.0, -1200.0, -500.0),
                rad: 1000.0,
                mat: mat2
            } as @Primitive,
            @Sphere {
                pos: vec3::new(-100.0, 25.0, -300.0),
                rad: 50.0,
                mat: mat3
            } as @Primitive,
            @Sphere {
                pos: vec3::new(0.0, 100.0, -250.0),
                rad: 25.0,
                mat: mat1
            } as @Primitive
        ],
        ambient: vec3::new(0.3, 0.3, 0.3),
        camera: vec3::new(0.0, 0.0, 800.0),
        view: vec3::new(0.0, 0.0, -1.0),
        up: vec3::new(0.0, 1.0, 0.0),
        width: 2048,
        height: 2048,
        fov: 45.0
    }
}
