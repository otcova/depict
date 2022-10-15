mod instanced_verts;
mod instanced_verts_and_indices;
mod vao;
mod verts;
mod verts_and_indices;

pub use instanced_verts::*;
pub use instanced_verts_and_indices::*;
pub use vao::*;
pub use verts::*;
pub use verts_and_indices::*;

use web_sys::WebGl2RenderingContext;

enum Mode {
    TRIANGLES,
    LINES,
}

impl Mode {
    fn into_webgl_mode(&self) -> u32 {
        match self {
            Self::TRIANGLES => WebGl2RenderingContext::TRIANGLES,
            Self::LINES => WebGl2RenderingContext::LINES,
        }
    }
}
