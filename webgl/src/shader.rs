use crate::*;

pub struct Shader {
    ctx: WebGl2RenderingContext,
    program: WebGlProgram,
}

#[derive(Debug)]
pub enum Uniform<'a> {
    Vec1F32(&'a [f32]),
    Vec2F32(&'a [f32]),
    Vec3F32(&'a [f32]),
    Vec4F32(&'a [f32]),
    Vec1I32(&'a [i32]),
    Vec2I32(&'a [i32]),
    Vec3I32(&'a [i32]),
    Vec4I32(&'a [i32]),
}

impl Shader {
    pub(crate) fn bind(&self) {
        self.ctx.use_program(Some(&self.program));
    }
    
    pub fn set_uniform(&self, name: &str, value: Uniform) {
        self.bind();
        let location = self.ctx.get_uniform_location(&self.program, name);
        let loc = location.as_ref();
        match value {
            Uniform::Vec1F32(data) => self.ctx.uniform1fv_with_f32_array(loc, data),
            Uniform::Vec2F32(data) => self.ctx.uniform2fv_with_f32_array(loc, data),
            Uniform::Vec3F32(data) => self.ctx.uniform3fv_with_f32_array(loc, data),
            Uniform::Vec4F32(data) => self.ctx.uniform4fv_with_f32_array(loc, data),
            Uniform::Vec1I32(data) => self.ctx.uniform1iv_with_i32_array(loc, data),
            Uniform::Vec2I32(data) => self.ctx.uniform2iv_with_i32_array(loc, data),
            Uniform::Vec3I32(data) => self.ctx.uniform3iv_with_i32_array(loc, data),
            Uniform::Vec4I32(data) => self.ctx.uniform4iv_with_i32_array(loc, data),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.ctx.delete_program(Some(&self.program));
    }
}

impl WebGl {
    pub fn new_shader(&self, vertex_source: &str, fragment_source: &str) -> Result<Shader> {
        let vertex_shader =
            self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vertex_source)?;
        let fragment_shader =
            self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fragment_source)?;

        Ok(Shader {
            ctx: self.ctx.clone(),
            program: self.link_shaders(vertex_shader, fragment_shader)?,
        })
    }

    fn link_shaders(
        &self,
        vertex_shader: WebGlShader,
        fragment_shader: WebGlShader,
    ) -> Result<WebGlProgram> {
        let program = self
            .ctx
            .create_program()
            .ok_or_else(|| String::from("Couldn't create shader object"))?;

        self.ctx.attach_shader(&program, &vertex_shader);
        self.ctx.attach_shader(&program, &fragment_shader);
        self.ctx.link_program(&program);

        if self
            .ctx
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            // self.ctx.use_program(Some(&program));
            Ok(program)
        } else {
            Err(self
                .ctx
                .get_program_info_log(&program)
                .map(|e| format!("Couldn't link shaders because:\n{}", e))
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader> {
        let shader = self
            .ctx
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Couldn't create shader object"))?;
        self.ctx.shader_source(&shader, source);
        self.ctx.compile_shader(&shader);

        if self
            .ctx
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            let shader_name = if shader_type == WebGl2RenderingContext::VERTEX_SHADER {
                "vertex"
            } else {
                "fragment"
            };
            Err(self
                .ctx
                .get_shader_info_log(&shader)
                .map(|s| {
                    format!(
                        "Could not compile {} shaders because of:\n{}",
                        shader_name, s
                    )
                })
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}
