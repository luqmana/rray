use std::num::{one, One};

pub struct Vec2f32 {
    pub x: f32,
    pub y: f32
}

pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub trait MathVector<T:Float + One> {
    fn add_v(&self, v: &Self) -> Self;
    fn sub_v(&self, v: &Self) -> Self;
    fn mul_v(&self, v: &Self) -> Self;
    fn mul_t(&self, t: T) -> Self;
    fn dot(&self, v: &Self) -> T;
    fn length(&self) -> T{
        self.length2().sqrt()
    }
    fn length2(&self) -> T {
        self.dot(self)
    }
    fn normalize(&self) -> Self {
        self.mul_t(one::<T>() / self.length())
    }
}

pub trait Vec2<T> {
    fn new(x: T, y: T) -> Self;
    fn zero() -> Self;
}

pub trait Vec3<T> {
    fn new(x: T, y: T, z: T) -> Self;
    fn zero() -> Self;

    fn cross(&self, v: &Self) -> Self;
}

impl Vec2<f32> for Vec2f32 {
    fn new(x: f32, y: f32) -> Vec2f32 {
        Vec2f32 {
            x: x,
            y: y
        }
    }

    fn zero() -> Vec2f32 {
        Vec2f32 {
            x: 0f32,
            y: 0f32
        }
    }
}

impl Vec3<f32> for Vec3f32 {
    fn new(x: f32, y: f32, z: f32) -> Vec3f32 {
        Vec3f32 {
            x: x,
            y: y,
            z: z
        }
    }

    fn zero() -> Vec3f32 {
        Vec3f32 {
            x: 0f32,
            y: 0f32,
            z: 0f32
        }
    }

    fn cross(&self, v: &Vec3f32) -> Vec3f32 {
        Vec3f32 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }
}

impl MathVector<f32> for Vec2f32 {
    fn add_v(&self, v: &Vec2f32) -> Vec2f32 {
        Vec2f32 {
            x: self.x + v.x,
            y: self.y + v.y
        }
    }

    fn sub_v(&self, v: &Vec2f32) -> Vec2f32 {
        Vec2f32 {
            x: self.x - v.x,
            y: self.y - v.y
        }
    }

    fn mul_v(&self, v: &Vec2f32) -> Vec2f32 {
        Vec2f32 {
            x: self.x * v.x,
            y: self.y * v.y
        }
    }

    fn mul_t(&self, t: f32) -> Vec2f32 {
        Vec2f32 {
            x: self.x * t,
            y: self.y * t
        }
    }

    fn dot(&self, v: &Vec2f32) -> f32 {
        self.x * v.x + self.y * v.y
    }
}

impl MathVector<f32> for Vec3f32 {
    fn add_v(&self, v: &Vec3f32) -> Vec3f32 {
        Vec3f32 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }

    fn sub_v(&self, v: &Vec3f32) -> Vec3f32 {
        Vec3f32 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }

    fn mul_v(&self, v: &Vec3f32) -> Vec3f32 {
        Vec3f32 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }

    fn mul_t(&self, t: f32) -> Vec3f32 {
        Vec3f32 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t
        }
    }

    fn dot(&self, v: &Vec3f32) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

pub trait Clamp {
    fn clamp(&self, min: f32, max: f32) -> Self;
}

impl Clamp for f32 {
    fn clamp(&self, min: f32, max: f32) -> f32 {
        if *self < min { min } else
        if *self > max { max } else { *self }
    }
}
