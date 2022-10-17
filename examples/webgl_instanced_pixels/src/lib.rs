mod console;
mod draw_loop;
mod shaders;

use draw_loop::start_draw_loop;
use draw_loop::*;
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
    let screen_size = if gl.width() > gl.height() {
        [gl.width() / gl.height(), 1.]
    } else {
        [1., gl.height() / gl.width()]
    };
    shader.set_uniform("screen_size", Uniform::Vec2F32(&screen_size));

    let mut vertices = gl.new_array_buffer(&[ShaderAttrib::VecF32 { loc: 0, len: 2 }])?;
    vertices.update_f32(&[
        0.5, 0.5, -0.5, 0.5, -0.5, -0.5, -0.5, -0.5, 0.5, -0.5, 0.5, 0.5,
    ]);

    let mut instances = gl.new_array_buffer(&[
        ShaderAttrib::VecF32 { loc: 1, len: 3 },
        ShaderAttrib::MatF32 {
            loc: 2,
            cols: 3,
            rows: 2,
        },
    ])?;
    let mut instances_data = vec![];

    let rect = gl.instanced_mesh_from_verts(&vertices, &instances)?;

    let draw_frame = move |time: FrameTime| -> Result<(), String> {
        const N: f32 = 500.;
        for x in (-N as i32)..=(N as i32) {
            for y in (-N as i32)..=(N as i32) {
                instances_data.extend_from_slice(&[
                    1. * (time.seconds * 10. + x as f32 / 10.).sin(),
                    0.2 * (0.5 + time.seconds * 10. + y as f32 / 10.).sin(),
                    0.,
                    1. / N,
                    0.,
                    0.,
                    1. / N,
                    x as f32 / N,
                    y as f32 / N,
                ]);
            }
        }

        instances.update_f32(&instances_data);
        instances_data.truncate(0);

        gl.clear_canvas([0.1, 0.1, 0.1, 1.]);
        rect.draw(&shader);

        console::log!("deltatime: {}ms", time.render_average * 1000.);
        Ok(())
    };

    start_draw_loop(draw_frame)?;

    Ok(())
}