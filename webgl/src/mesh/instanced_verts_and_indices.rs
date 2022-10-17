use crate::*;

use super::vao::VAO;

pub struct MeshInstancedVertsAndIndices {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    indices: ElementBuffer,
    instances: ArrayBuffer,
}

impl WebGl {
    pub fn instanced_mesh_from_verts_and_indices(
        &self,
        verts: &ArrayBuffer,
        indices: &ElementBuffer,
        instances: &ArrayBuffer,
    ) -> Result<MeshInstancedVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts);
        vao.link_instance_buffer(instances);
        Ok(MeshInstancedVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            indices: indices.clone(),
            instances: instances.clone(),
        })
    }
}

impl MeshInstancedVertsAndIndices {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.indices.bind();
        self.ctx.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices.len(),
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
            self.instances.len() / ShaderAttrib::count_floats(self.instances.attribs()),
        );
    }
}
