
use std::mem::size_of;

use crate::*;

use super::vao::VAO;

pub struct GeometryInstancedVertsAndIndices {
    ctx: WebGl2RenderingContext,
    vao: VAO,
	indices: ElementBuffer,
    instances: ArrayBuffer,
    instance_size: i32,
}

impl WebGl {
    pub fn instanced_triangles_from_verts_and_indices(
        &self,
        verts: (&ArrayBuffer, &[ShaderAttrib]),
		indices: &ElementBuffer,
        instances: (&ArrayBuffer, &[ShaderAttrib]),
    ) -> Result<GeometryInstancedVertsAndIndices> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts.0, verts.1);
        vao.link_instance_buffer(instances.0, instances.1);
        Ok(GeometryInstancedVertsAndIndices {
            ctx: self.ctx.clone(),
            vao,
            indices: indices.clone(),
            instances: instances.0.clone(),
            instance_size: ShaderAttrib::count_bytes(instances.1) / size_of::<f32>() as i32,
        })
    }
}

impl GeometryInstancedVertsAndIndices {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.indices.bind();
        self.ctx.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.indices.len() as i32,
			WebGl2RenderingContext::UNSIGNED_INT,
			0,
            self.instances.len() as i32 / self.instance_size,
        );
    }
}
