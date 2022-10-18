use crate::*;

pub(crate) struct RectMesh {
    instances: ArrayBuffer,
    instances_buffer: Vec<f32>,
    mesh: MeshInstancedVerts,
    shader: Shader,
}

pub struct Rect<'a> {
    rgb: Vec3,
    z_index: f32,
    matrix: Mat3x2,
    mesh: &'a mut RectMesh,
}

impl Draw {
    pub fn rect(&mut self) -> Rect {
        self.z_index += 1.;
       self.rect.instantiate(self.z_index)
    }
}

impl RectMesh {
    pub(crate) fn new(gl: &WebGl) -> Result<Self> {
        let mut vertices = gl.new_array_buffer(VERTICES_ATTRIBS)?;
        vertices.update_f32(&[0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5]);
        let instances = gl.new_array_buffer(INSTANCES_ATTRIBS)?;
        let mesh = gl.instanced_mesh_from_verts(&vertices, &instances)?;
        let shader = gl.new_shader(VERTEX_SHADER, FRAGMENT_SHADER)?;
        Ok(Self {
            instances,
            instances_buffer: vec![],
            mesh,
            shader,
        })
    }

    pub fn instantiate(&mut self, z_index: f32) -> Rect {
        Rect {
            rgb: vec3![1., 0.5, 0.],
            z_index,
            matrix: mat3x2![1.],
            mesh: self,
        }
    }

    pub fn draw(&mut self, app: &Depict) {
        self.instances.update_f32(&self.instances_buffer);
        self.shader.set_uniform("screen_size", Uniform::Vec2F32(&*app.size()));
        self.mesh.draw(&self.shader);

        self.instances_buffer.truncate(0);
    }
}

impl<'a> Transform2D for Rect<'a> {
    fn mut_matrix(&mut self) -> &mut Mat3x2 {
        &mut self.matrix
    }
}

impl<'a> Rgb for Rect<'a> {
    fn mut_rgb(&mut self) -> &mut Vec3 {
        &mut self.rgb
    }
}
impl<'a> Drop for Rect<'a> {
    fn drop(&mut self) {
        self.mesh.instances_buffer.extend_from_slice(&[
            self.rgb.x,
            self.rgb.y,
            self.rgb.z,
            self.z_index,
            self.matrix[0],
            self.matrix[1],
            self.matrix[2],
            self.matrix[3],
            self.matrix[4],
            self.matrix[5],
        ]);
    }
}

const VERTICES_ATTRIBS: &[ShaderAttrib] = &[ShaderAttrib::VecF32 { loc: 0, len: 2 }];
const INSTANCES_ATTRIBS: &[ShaderAttrib] = &[
    ShaderAttrib::VecF32 { loc: 1, len: 3 },
    ShaderAttrib::F32 { loc: 2 },
    ShaderAttrib::MatF32 {
        loc: 3,
        cols: 3,
        rows: 2,
    },
];
const VERTEX_SHADER: &str = r##"#version 300 es
precision highp float;

layout(location=0) in vec2 vertex_coord;
layout(location=1) in vec3 color;
layout(location=2) in float z_index;
layout(location=3) in mat3x2 transformation;

out vec3 f_color;

uniform vec2 screen_size;

void main() {
	f_color = color;
    vec2 pos = (transformation * vec3(vertex_coord, 1.)).xy;
	gl_Position = vec4(pos / screen_size, z_index / -float(1 << 24), 1.);
}
"##;

pub const FRAGMENT_SHADER: &str = r##"#version 300 es
precision mediump float;

in vec3 f_color;
out vec4 outColor;

void main() {
	outColor = vec4(f_color, 1.);
}
"##;
