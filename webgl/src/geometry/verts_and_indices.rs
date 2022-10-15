use crate::*;

use super::{vao::VAO, Mode};

pub struct GeometryVertsAndIndices {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    geometry_mode: Mode,
    pub verts: ArrayBuffer,
    pub indices: ElementBuffer,
}

impl WebGl {
    pub fn triangles_from_verts_and_indices(
        &self,
        verts: (ArrayBuffer, &[ShaderAttrib]),
        indices: ElementBuffer,
    ) -> Result<GeometryVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(&verts.0, verts.1);
        Ok(GeometryVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            geometry_mode: Mode::TRIANGLES,
            verts: verts.0,
            indices,
        })
    }

    pub fn lines_from_verts_and_indices(
        &self,
        verts: (ArrayBuffer, &[ShaderAttrib]),
        indices: ElementBuffer,
    ) -> Result<GeometryVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(&verts.0, verts.1);
        Ok(GeometryVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            geometry_mode: Mode::LINES,
            verts: verts.0,
            indices,
        })
    }
}

impl GeometryVertsAndIndices {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
		self.indices.bind();

        self.ctx.draw_elements_with_i32(
            self.geometry_mode.into_webgl_mode(),
            self.indices.len() as i32,
			WebGl2RenderingContext::UNSIGNED_INT,
			0,	
        );
    }
}
