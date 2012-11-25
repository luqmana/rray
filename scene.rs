use geometry::*;

pub struct Light {
    pos: Vec3<float>,
    colour: Vec3<float>
}

pub struct Scene {
    lights: ~[Light],
    primitives: ~[Primitive],
    ambient: Vec3<float>,
    camera: Vec3<float>,
    view: Vec3<float>,
    up: Vec3<float>,
    width: uint,
    height: uint,
    fov: float
}

// Returns a reference scene
pub fn getRefScene() -> Scene {

    // The materials!
    let mat1 = Material {
        diffuse: Vec3::new(0.7f, 1.0f, 0.7f),
        specular: Vec3::new(0.5f, 0.7f, 0.5f),
        shininess: 25.0f,
        mirror: 0.3f
    }, mat2 = Material {
        diffuse: Vec3::new(0.5f, 0.5f, 0.5f),
        specular: Vec3::new(0.5f, 0.7f, 0.5f),
        shininess: 25.0f,
        mirror: 0.3f
    }, mat3 = Material {
        diffuse: Vec3::new(1.0f, 0.6f, 0.1f),
        specular: Vec3::new(0.5f, 0.7f, 0.5f),
        shininess: 25.0f,
        mirror: 0.3f
    };

    //  Now build the scene
    Scene {
        lights: ~[
            Light {
                pos: Vec3::new(-100.0f, 150.0f, 400.0f),
                colour: Vec3::new(0.7f, 0.7f, 0.7f)
            },
            Light {
                pos: Vec3::new(400.0f, 100.0f, 150.0f),
                colour: Vec3::new(0.7f, 0.0f, 0.7f)
            }
        ],
        primitives: ~[
            Sphere(Sphere {
                pos: Vec3::new(0.0f, 0.0f, -400.0f),
                rad: 100.0f,
                mat: mat1
            }),
            Sphere(Sphere {
                pos: Vec3::new(200.0f, 50.0f, -100.0f),
                rad: 150.0f,
                mat: mat1
            }),
            Sphere(Sphere {
                pos: Vec3::new(0.0f, -1200.0f, -500.0f),
                rad: 1000.0f,
                mat: mat2
            }),
            Sphere(Sphere {
                pos: Vec3::new(-100.0f, 25.0f, -300.0f),
                rad: 50.0f,
                mat: mat3
            }),
            Sphere(Sphere {
                pos: Vec3::new(0.0f, 100.0f, -250.0f),
                rad: 25.0f,
                mat: mat1
            })
        ],
        ambient: Vec3::new(0.3f, 0.3f, 0.3f),
        camera: Vec3::new(0.0f, 0.0f, 800.0f),
        view: Vec3::new(0.0f, 0.0f, -1.0f),
        up: Vec3::new(0.0f, 1.0f, 0.0f),
        width: 1024,
        height: 1024,
        fov: 45.0f
    }
}
