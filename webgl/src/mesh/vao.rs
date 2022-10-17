use std::mem::size_of;

use crate::*;
use web_sys::*;

pub struct VAO {
    context: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
}

#[derive(Debug)]
pub enum ShaderAttrib {
    F32 { loc: u32 },
    VecF32 { loc: u32, len: i32 },
    VecI32 { loc: u32, len: i32 },
    MatF32 { loc: u32, cols: i32, rows: i32 },
    Offset { len: i32 },
}

impl VAO {
    pub(crate) fn bind(&self) {
        self.context.bind_vertex_array(Some(&self.vao));
    }
    pub(crate) fn new(context: &WebGl2RenderingContext) -> Result<VAO> {
        Ok(Self {
            vao: context.create_vertex_array().ok_or("Couldn't create vao")?,
            context: context.clone(),
        })
    }

    pub(crate) fn link_instance_buffer(&self, buffer: &ArrayBuffer) {
        self.context.bind_vertex_array(Some(&self.vao));
        buffer.bind();

        let stride = ShaderAttrib::count_bytes(buffer.attribs());
        let mut offset = 0;
        for attrib in buffer.attribs() {
            attrib.vertex_attrib_pointer(&self.context, stride, offset);
            attrib.vertex_attrib_divisor(&self.context, 1);
            attrib.enable_vertex_attrib_array(&self.context);

            offset += attrib.bytes_count();
        }
    }
    pub(crate) fn link_buffer(&self, buffer: &ArrayBuffer) {
        self.context.bind_vertex_array(Some(&self.vao));
        buffer.bind();

        let stride = ShaderAttrib::count_bytes(buffer.attribs());
        let mut offset = 0;
        for attrib in buffer.attribs() {
            attrib.vertex_attrib_pointer(&self.context, stride, offset);
            attrib.enable_vertex_attrib_array(&self.context);

            offset += attrib.bytes_count();
        }
    }
}

impl ShaderAttrib {
    fn bytes_count(&self) -> i32 {
        match self {
            ShaderAttrib::F32 { loc: _ } => 4,
            ShaderAttrib::VecF32 { loc: _, len } => len * 4,
            ShaderAttrib::VecI32 { loc: _, len } => len * 4,
            ShaderAttrib::MatF32 { loc: _, rows, cols } => rows * cols * 4,
            ShaderAttrib::Offset { len } => *len,
        }
    }

    fn vertex_attrib_pointer(&self, ctx: &WebGl2RenderingContext, stride: i32, offset: i32) {
        match self {
            ShaderAttrib::F32 { loc } => ctx.vertex_attrib_pointer_with_i32(
                *loc,
                1,
                WebGl2RenderingContext::FLOAT,
                false,
                stride,
                offset,
            ),
            ShaderAttrib::VecF32 { loc, len } => ctx.vertex_attrib_pointer_with_i32(
                *loc,
                *len,
                WebGl2RenderingContext::FLOAT,
                false,
                stride,
                offset,
            ),
            ShaderAttrib::VecI32 { loc, len } => ctx.vertex_attrib_i_pointer_with_i32(
                *loc,
                *len,
                WebGl2RenderingContext::INT,
                stride,
                offset,
            ),
            ShaderAttrib::MatF32 { loc, rows, cols } => {
                for col in 0..*cols {
                    ctx.vertex_attrib_pointer_with_i32(
                        *loc + (col as u32),
                        *rows,
                        WebGl2RenderingContext::FLOAT,
                        false,
                        stride,
                        offset + col * rows * 4,
                    );
                }
            }
            ShaderAttrib::Offset { len: _ } => {}
        }
    }

    fn vertex_attrib_divisor(&self, ctx: &WebGl2RenderingContext, divisor: u32) {
        match self {
            ShaderAttrib::F32 { loc } => ctx.vertex_attrib_divisor(*loc, divisor),
            ShaderAttrib::VecF32 { loc, len: _ } => ctx.vertex_attrib_divisor(*loc, divisor),
            ShaderAttrib::VecI32 { loc, len: _ } => ctx.vertex_attrib_divisor(*loc, divisor),
            ShaderAttrib::MatF32 { loc, cols, rows: _ } => {
                for col in 0..*cols {
                    ctx.vertex_attrib_divisor(loc + col as u32, divisor);
                }
            }
            ShaderAttrib::Offset { len: _ } => {}
        }
    }

    fn enable_vertex_attrib_array(&self, ctx: &WebGl2RenderingContext) {
        match self {
            ShaderAttrib::F32 { loc } => ctx.enable_vertex_attrib_array(*loc),
            ShaderAttrib::VecF32 { loc, len: _ } => ctx.enable_vertex_attrib_array(*loc),
            ShaderAttrib::VecI32 { loc, len: _ } => ctx.enable_vertex_attrib_array(*loc),
            ShaderAttrib::MatF32 { loc, cols, rows: _ } => {
                for col in 0..*cols {
                    ctx.enable_vertex_attrib_array(loc + col as u32);
                }
            }
            ShaderAttrib::Offset { len: _ } => {}
        }
    }

    pub(crate) fn count_bytes(attributes: &[Self]) -> i32 {
        let mut stride = 0;
        for attrib in attributes {
            stride += attrib.bytes_count();
        }
        stride
    }
    pub(crate) fn count_floats(attributes: &[Self]) -> i32 {
        Self::count_bytes(attributes) / size_of::<f32>() as i32
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        self.context.delete_vertex_array(Some(&self.vao));
    }
}
