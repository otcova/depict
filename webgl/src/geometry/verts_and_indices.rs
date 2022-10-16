use crate::*;

use super::vao::VAO;

pub struct GeometryVertsAndIndices {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    indices: ElementBuffer,
}

impl WebGl {
    pub fn triangles_from_verts_and_indices(
        &self,
        verts: (&ArrayBuffer, &[ShaderAttrib]),
        indices: &ElementBuffer,
    ) -> Result<GeometryVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts.0, verts.1);
        Ok(GeometryVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            indices: indices.clone(),
        })
    }
}

impl GeometryVertsAndIndices {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.indices.bind();

        self.ctx.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices.len() as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }
}
