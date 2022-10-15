use std::ops::{Add, Div, Mul, Neg, Sub};

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[macro_export]
macro_rules! vec3 {
    ($x: expr, $y: expr, $z: expr) => {
        $crate::Vec3 {
            x: ($x),
            y: ($y),
            z: ($z),
        }
    };
    ($elem: expr) => {
        $crate::Vec3::from($elem)
    };
}

impl From<&[f32; 3]> for Vec3 {
    fn from(vector: &[f32; 3]) -> Self {
        Vec3 {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(vector: [f32; 3]) -> Self {
        Vec3 {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
}

impl From<f32> for Vec3 {
    fn from(coord: f32) -> Self {
        Vec3 {
            x: coord,
            y: coord,
            z: coord,
        }
    }
}

impl Vec3 {
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
    pub fn resize(self, size: f32) -> Self {
        self.normalize() * size
    }
    pub fn dot(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y
    }
    pub fn cross(self, v: Vec3) -> f32 {
        self.x * v.y + self.y * v.x
    }
    pub fn as_slice(&self) -> &[f32; 3] {
        // Safety: this is OK because Vec3 is contiguous tanks to repr(C).
        unsafe { std::mem::transmute::<_, &[f32; 3]>(self) }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        vec3![self.x / rhs, self.y / rhs, self.z / rhs]
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Vec3 {
        vec3![self.x / rhs.x, self.y / rhs.y, self.z / rhs.z]
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        vec3![self.x * rhs.x, self.y * rhs.y, self.z * rhs.z]
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        vec3![self.x * rhs, self.y * rhs, self.z * rhs]
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3![self.x - rhs.x, self.y - rhs.y, self.z - rhs.z]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        vec3![self.x + rhs.x, self.y + rhs.y, self.z + rhs.z]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        vec3![-self.x, -self.y, -self.z]
    }
}
