pub mod primitives;
pub use primitives::*;

use crate::*;

pub struct Draw {
    z_index: f32,
    rect: RectMesh,
}

impl Draw {
	pub(crate) fn new(gl: &WebGl) -> Result<Self> {
		Ok(Self {
			z_index: 0.,
			rect: RectMesh::new(gl)?,
		})
	}
	pub(crate) fn draw(&mut self, app: &Depict) {
		app.gl.clear_canvas([0.1, 0.1, 0.1, 1.]);
		self.rect.draw(app);
	}
}
