mod rect;
pub use rect::*;

use crate::*;

pub trait Rgb {
    fn mut_rgb(&mut self) -> &mut Vec3;
    fn rgb<V: Into<Vec3>>(&mut self, color: V) -> &mut Self {
        *self.mut_rgb() = color.into();
        self
    }
}

pub trait Transform2D {
    fn mut_matrix(&mut self) -> &mut Mat3x2;
    /// Translate
    fn xy<V: Into<Vec2>>(&mut self, position: V) -> &mut Self {
        let p = (self.mut_matrix() as &Mat3x2) * position.into();
        self.mut_matrix().translate(p);
        self
    }
    /// Rotate on z axis with radians
    fn rotate_rad(&mut self, angle: f32) -> &mut Self {
        self.mut_matrix().rotate_rad(angle);
        self
    }
    /// Rotate on z axis with degrees
    fn rotate_deg(&mut self, angle: f32) -> &mut Self {
        self.mut_matrix().rotate_deg(angle);
        self
    }
    /// Scale x and y dimension
    fn scale<V: Into<Vec2>>(&mut self, size: V) -> &mut Self {
        self.mut_matrix().scale(size);
        self
    }
    fn apply(&mut self, transformation: &Mat3x2) -> &mut Self {
        self.mut_matrix().apply(transformation);
        self
    }
}
