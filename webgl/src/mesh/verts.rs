use crate::*;

use super::vao::VAO;

pub struct MeshVerts {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    vertices: ArrayBuffer,
}

impl WebGl {
    pub fn mesh_from_verts(&self, verts: &ArrayBuffer) -> Result<MeshVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts);
        Ok(MeshVerts {
            ctx: self.ctx.clone(),
            vao,
            vertices: verts.clone(),
        })
    }
}

impl MeshVerts {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.ctx.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() as i32 / ShaderAttrib::count_floats(self.vertices.attribs()),
        );
    }
}
