use crate::*;

use super::vao::VAO;

pub struct MeshInstancedVerts {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    vertices: ArrayBuffer,
    instances: ArrayBuffer,
}

impl WebGl {
    pub fn instanced_mesh_from_verts(
        &self,
        verts: &ArrayBuffer,
        instances: &ArrayBuffer,
    ) -> Result<MeshInstancedVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts);
        vao.link_instance_buffer(instances);
        Ok(MeshInstancedVerts {
            ctx: self.ctx.clone(),
            vao,
            vertices: verts.clone(),
            instances: instances.clone(),
        })
    }
}

impl MeshInstancedVerts {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.ctx.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() / ShaderAttrib::count_floats(self.vertices.attribs()),
            self.instances.len() / ShaderAttrib::count_floats(self.instances.attribs()),
        );
    }
}
