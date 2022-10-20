use crate::*;
use std::{
    f32::consts::PI,
    ops::Mul,
    ops::{Deref, DerefMut},
};

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
        Mat3x2([1., 0., 0., 1., 0., 0.])
    };
    ($m: expr) => {
        Mat3x2::from($m)
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

    pub fn apply(&mut self, transform: &Mat3x2) -> &mut Self {
        let m00 = self.0[0];
        let m10 = self.0[1];
        let m01 = self.0[2];
        let m11 = self.0[3];
        let m02 = self.0[4];
        let m12 = self.0[5];

        self.0[0] = m00 * transform.0[0] + m01 * transform.0[1];
        self.0[1] = m10 * transform.0[0] + m11 * transform.0[1];
        self.0[2] = m00 * transform.0[2] + m01 * transform.0[3];
        self.0[3] = m10 * transform.0[2] + m11 * transform.0[3];
        self.0[4] = m00 * transform.0[4] + m01 * transform.0[5] + m02;
        self.0[5] = m10 * transform.0[4] + m11 * transform.0[5] + m12;

        self
    }
}

impl Mul<Vec2> for &Mat3x2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        vec2![
            self.0[0] * rhs.x + self.0[2] * rhs.y + self.0[4],
            self.0[1] * rhs.x + self.0[3] * rhs.y + self.0[5]
        ]
    }
}
