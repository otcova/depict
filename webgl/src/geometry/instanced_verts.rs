use std::mem::size_of;

use crate::*;

use super::vao::VAO;

pub struct GeometryInstancedVerts {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    vertices: ArrayBuffer,
    instances: ArrayBuffer,
    vertex_size: i32,
    instance_size: i32,
}

impl WebGl {
    pub fn instanced_triangles_from_verts(
        &self,
        verts: (&ArrayBuffer, &[ShaderAttrib]),
        instances: (&ArrayBuffer, &[ShaderAttrib]),
    ) -> Result<GeometryInstancedVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(verts.0, verts.1);
        vao.link_instance_buffer(instances.0, instances.1);
        Ok(GeometryInstancedVerts {
            ctx: self.ctx.clone(),
            vao,
            vertices: verts.0.clone(),
            instances: instances.0.clone(),
            vertex_size: ShaderAttrib::count_bytes(verts.1) / size_of::<f32>() as i32,
            instance_size: ShaderAttrib::count_bytes(instances.1) / size_of::<f32>() as i32,
        })
    }
}

impl GeometryInstancedVerts {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.ctx.draw_arrays_instanced(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.vertices.len() as i32 / self.vertex_size,
            self.instances.len() as i32 / self.instance_size,
        );
    }
}
