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
    fn translate<V: Into<Vec2>>(&mut self, position: V) -> &mut Self {
        self.mut_matrix().translate(position);
        self
    }
    fn rotate_rad(&mut self, angle: f32) -> &mut Self {
        self.mut_matrix().rotate_rad(angle);
        self
    }
    fn rotate_deg(&mut self, angle: f32) -> &mut Self {
        self.mut_matrix().rotate_deg(angle);
        self
    }
    fn scale<V: Into<Vec2>>(&mut self, size: V) -> &mut Self {
        self.mut_matrix().scale(size);
        self
    }
    // Shortcuts
    fn xy<V: Into<Vec2>>(&mut self, position: V) -> &mut Self {
        self.translate(position);
        self
    }
    fn z_rad(&mut self, angle: f32) -> &mut Self {
        self.rotate_rad(angle);
        self
    }
    fn z_deg(&mut self, angle: f32) -> &mut Self {
        self.rotate_deg(angle);
        self
    }
}
