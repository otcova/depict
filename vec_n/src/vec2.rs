use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[macro_export]
macro_rules! vec2 {
    ($x: expr, $y: expr) => {
        $crate::Vec2 { x: ($x), y: ($y) }
    };
    ($elem: expr) => {
        $crate::Vec2::from($elem)
    };
}

impl From<&[f32; 2]> for Vec2 {
    fn from(vector: &[f32; 2]) -> Self {
        Vec2 {
            x: vector[0],
            y: vector[1],
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(vector: [f32; 2]) -> Self {
        Vec2 {
            x: vector[0],
            y: vector[1],
        }
    }
}

impl From<f32> for Vec2 {
    fn from(coord: f32) -> Self {
        Vec2 { x: coord, y: coord }
    }
}

impl Vec2 {
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
    pub fn resize(self, size: f32) -> Self {
        self.normalize() * size
    }
    pub fn rotate90(self) -> Self {
        [self.y, self.x].into()
    }
    pub fn rotate270(self) -> Self {
        vec2![self.y, -self.x]
    }
    pub fn dot(self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }
    pub fn cross(self, v: Vec2) -> f32 {
        self.x * v.y + self.y * v.x
    }
    pub fn as_slice(&self) -> &[f32; 2] {
        // Safety: this is OK because Vec2 is contiguous tanks to repr(C).
        unsafe { std::mem::transmute::<_, &[f32; 2]>(self) }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        vec2![self.x / rhs, self.y / rhs]
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Vec2 {
        vec2![self.x / rhs.x, self.y / rhs.y]
    }
}
impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Vec2 {
        vec2![self.x * rhs.x, self.y * rhs.y]
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        vec2![self.x * rhs, self.y * rhs]
    }
}
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        vec2![self.x - rhs.x, self.y - rhs.y]
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        vec2![self.x + rhs.x, self.y + rhs.y]
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        vec2![-self.x, -self.y]
    }
}
