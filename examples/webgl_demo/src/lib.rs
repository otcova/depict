mod console;
mod shaders;

use wasm_bindgen::prelude::*;
use webgl::*;

#[wasm_bindgen(start)]
pub fn start() {
    if let Err(error) = setup() {
        console::log!("[ERROR]\n{}", error);
    }
}

pub fn setup() -> Result<(), String> {
    let gl = WebGl::new()?;
    let shader = gl.new_shader(shaders::VERTEX_SOURCE, shaders::FRAGMENT_SOURCE)?;
    let shader2 = gl.new_shader(shaders::VERTEX_SOURCE2, shaders::FRAGMENT_SOURCE2)?;
    let shader3 = gl.new_shader(shaders::VERTEX_SOURCE3, shaders::FRAGMENT_SOURCE3)?;

    // BUFFERS
    let mut square_triangles_verts =
        gl.new_array_buffer(&[ShaderAttrib::VecF32 { loc: 0, len: 2 }])?;
    square_triangles_verts.update_f32(&[0., 0., 0.9, 0.9, 0.9, 0., 0., 0., 0.9, 0.9, 0., 0.9]);

    let mut square_verts = gl.new_array_buffer(&[ShaderAttrib::VecF32 { loc: 0, len: 2 }])?;
    square_verts.update_f32(&[0.5, 0.5, 0.9, 0.5, 0.9, 0.9, 0.5, 0.9]);
    let mut square_indices = gl.new_element_buffer()?;
    square_indices.update_i32(&[0, 1, 2, 0, 2, 3]);

    let mut instances = gl.new_array_buffer(&[
        ShaderAttrib::VecF32 { loc: 1, len: 2 },
        ShaderAttrib::VecF32 { loc: 2, len: 3 },
    ])?;
    instances.update_f32(&[0., 0., 1., 1., 1., 0.2, 0.1, 1., 1., 0.]);

    let mut instances2 = gl.new_array_buffer(&[
        ShaderAttrib::VecF32 { loc: 1, len: 2 },
        ShaderAttrib::VecF32 { loc: 2, len: 3 },
    ])?;
    instances2.update_f32(&[-1., 0., 1., 0., 0., -0.7, -0.7, 0., 1., 0.]);

    // MESH
    let square = gl.mesh_from_verts(&square_triangles_verts)?;

    let square2 = gl.mesh_from_verts_and_indices(&square_verts, &square_indices)?;

    let square3 = gl.instanced_mesh_from_verts(&square_triangles_verts, &instances)?;

    let square4 =
        gl.instanced_mesh_from_verts_and_indices(&square_verts, &square_indices, &instances2)?;

    // DRAW
    gl.clear_canvas([0.1, 0.1, 0.1, 0.9]);

    shader.set_uniform("color", Uniform::Vec3F32(&[0., 0.9, 0.9]));
    square2.draw(&shader);

    shader.set_uniform("color", Uniform::Vec3F32(&[0.9, 0.9, 0.9]));
    square.draw(&shader2);

    square3.draw(&shader3);

    square4.draw(&shader3);

    Ok(())
}
