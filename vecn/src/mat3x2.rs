use crate::*;
use std::{ops::{Deref, DerefMut}, f32::consts::PI};

/// Column major matrix
pub struct Mat3x2(pub [f32; 6]);

impl<V: Into<Vec2>> From<V> for Mat3x2 {
    fn from(size: V) -> Self {
        let vec2 = size.into();
        Mat3x2([vec2.x, 0., 0., vec2.y, 0., 0.])
    }
}

#[macro_export]
macro_rules! mat3x2 {
    () => {
        Mat3x2(1.)
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

impl Mat3x2 {
    pub fn translate<V: Into<Vec2>>(&mut self, pos: V) -> &mut Self {
        let vec = pos.into();
        self.0[4] += vec.x;
        self.0[5] += vec.y;
        self
    }
    pub fn rotate_rad(&mut self, angle: f32) -> &mut Self {
        let cos = angle.cos();
        let sin = angle.sin();

        let m11 = self.0[0];
        let m21 = self.0[1];
        let m12 = self.0[2];
        let m22 = self.0[3];
        
        self.0[0] = m11 * cos - m12 * sin;
        self.0[1] = m21 * cos - m22 * sin;
        self.0[2] = m11 * sin + m12 * cos;
        self.0[3] = m21 * sin + m22 * cos;

        self
    }

    pub fn rotate_deg(&mut self, angle: f32) -> &mut Self {
        self.rotate_rad(angle * PI / 180.)
    }
    
    pub fn scale<V: Into<Vec2>>(&mut self, size: V) -> &mut Self {
        let vec = size.into();
        self.0[0] *= vec.x;
        self.0[3] *= vec.y;
        self
    }
}