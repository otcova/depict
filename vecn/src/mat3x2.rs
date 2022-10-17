use crate::*;
use std::ops::{Deref, DerefMut};

pub struct Mat3x2(pub [f32; 6]);

impl<V: Into<Vec2>> From<V> for Mat3x2 {
	fn from(size: V) -> Self {
		let vec2 = size.into();
		Mat3x2([vec2.x, 0., 0., 0., vec2.y, 0.])
	}
}

#[macro_export]
macro_rules! mat3x2 {
    () => {
        Mat3x2([1., 0., 0., 0., 1., 0.])
    };
    ($m: expr) => {
        Mat3x2::from($m)
    };
    ($a: literal, $b: literal, $c: literal, $d: literal, $e: literal, $f: literal) => {
        Mat3x2([$a, $b, $c, $d, $e, $f])
    };
}

impl Deref for Mat3x2 {
    type Target = [f32; 6];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mat3x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
