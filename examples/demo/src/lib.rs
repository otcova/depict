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

    let mut square_triangles_verts = gl.new_buffer()?;
    square_triangles_verts.update_f32(&[0., 0., 1., 1., 1., 0., 0., 0., 1., 1., 0., 1.]);

    let mut square_verts = gl.new_buffer()?;
    square_verts.update_f32(&[0.5, 0.5, 1., 0.5, 1., 1., 0.5, 1.]);
    let mut square_indices = gl.new_buffer()?;
    square_indices.update_i32(&[0, 1, 2, 0, 2, 3]);

    let square = gl.triangles_from_verts((
        square_triangles_verts,
        &[ShaderAttrib::VecF32 { loc: 0, len: 2 }],
    ))?;

    let square2 = gl.triangles_from_verts_and_indices(
        (square_verts, &[ShaderAttrib::VecF32 { loc: 0, len: 2 }]),
        square_indices,
    )?;

    gl.clear_canvas([0.1, 0.1, 0.1, 1.]);

    shader.set_uniform("color", Uniform::Vec3F32(&[0., 1., 1.]));
    square2.draw(&shader);
    
    shader.set_uniform("color", Uniform::Vec3F32(&[1., 1., 1.]));
    square.draw(&shader);


    Ok(())
}
