use std::mem::size_of;

use crate::*;

use super::{vao::VAO, Mode};

pub struct GeometryVerts {
    ctx: WebGl2RenderingContext,
    vao: VAO,
    geometry_mode: Mode,
    pub vertices: ArrayBuffer,
    vertex_len: i32,
}

impl WebGl {
    pub fn triangles_from_verts(&self, verts: (ArrayBuffer, &[ShaderAttrib])) -> Result<GeometryVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(&verts.0, verts.1);
        Ok(GeometryVerts {
            ctx: self.ctx.clone(),
            vao,
            geometry_mode: Mode::TRIANGLES,
            vertices: verts.0,
            vertex_len: ShaderAttrib::count_bytes(verts.1) / size_of::<f32>() as i32,
        })
    }

    pub fn lines_from_verts(&self, verts: (ArrayBuffer, &[ShaderAttrib])) -> Result<GeometryVerts> {
        let vao = VAO::new(&self.ctx)?;
        vao.link_buffer(&verts.0, verts.1);
        Ok(GeometryVerts {
            ctx: self.ctx.clone(),
            vao,
            geometry_mode: Mode::LINES,
            vertices: verts.0,
            vertex_len: ShaderAttrib::count_bytes(verts.1) / size_of::<f32>() as i32,
        })
    }
}

impl GeometryVerts {
    pub fn draw(&self, shader: &Shader) {
        shader.bind();
        self.vao.bind();
        self.ctx.draw_arrays(
            self.geometry_mode.into_webgl_mode(),
            0,
            self.vertices.len() as i32 / self.vertex_len,
        );
    }
}
