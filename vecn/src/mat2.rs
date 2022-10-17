use crate::*;
use std::ops::{Deref, DerefMut};

pub struct Mat2(pub [f32; 4]);

impl<V: Into<Vec2>> From<V> for Mat2 {
    fn from(size: V) -> Self {
        let vec2 = size.into();
        Mat2([vec2.x, 0., 0., vec2.y])
    }
}

#[macro_export]
macro_rules! mat2 {
    () => {
        Mat2([1., 0., 0., 1.])
    };
    ($m: expr) => {
        Mat2::from($m)
    };
    ($a: literal, $b: literal, $c: literal, $d: literal) => {
        Mat2([$a, $b, $c, $d])
    };
}

impl Deref for Mat2 {
    type Target = [f32; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mat2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
