use std::mem::size_of;

use crate::*;

use super::vao::VAO;

pub struct GeometryVerts {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    vertices: ArrayBuffer,
    vertex_len: i32,
}

impl WebGl {
    pub fn triangles_from_verts(
        &self,
        verts: (&ArrayBuffer, &[ShaderAttrib]),
    ) -> Result<GeometryVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts.0, verts.1);
        Ok(GeometryVerts {
            ctx: self.ctx.clone(),
            vao,
            vertices: verts.0.clone(),
            vertex_len: ShaderAttrib::count_bytes(verts.1) / size_of::<f32>() as i32,
        })
    }
}

impl GeometryVerts {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.ctx.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() as i32 / self.vertex_len,
        );
    }
}
