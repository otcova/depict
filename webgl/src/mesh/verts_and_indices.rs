use crate::*;

use super::vao::VAO;

pub struct MeshVertsAndIndices {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    indices: ElementBuffer,
}

impl WebGl {
    pub fn mesh_from_verts_and_indices(
        &self,
        verts: &ArrayBuffer,
        indices: &ElementBuffer,
    ) -> Result<MeshVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts);
        Ok(MeshVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            indices: indices.clone(),
        })
    }
}

impl MeshVertsAndIndices {
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
