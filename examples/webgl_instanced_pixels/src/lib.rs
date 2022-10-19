mod console;
mod request_frame;
mod shaders;
mod time;

use request_frame::start_animation_loop;
use time::*;
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

    let mut time = FrameTime::new();

    let draw_frame = move || {
        time.start_frame();
        
        const N: f32 = 500.;
        for x in (-N as i32)..=(N as i32) {
            for y in (-N as i32)..=(N as i32) {
                let data = [
                    1. * (time.seconds * 10. + x as f32 / 10.).sin(),
                    0.5 * (0.5 + time.seconds * 10. + y as f32 / 10.).sin(),
                    0.,
                    1. / 500.,
                    0.,
                    0.,
                    1. / 500.,
                    x as f32 / N,
                    y as f32 / N,
                ];
                instances_data.extend_from_slice(&data);
            }
        }

        gl.clear_canvas([0.1, 0.1, 0.1, 1.]);
        instances.update_f32(&instances_data);
        rect.draw(&shader);
        instances_data.truncate(0);

        time.end_frame();
        console::log!("{}ms", time.delta_seconds.average() * 1000.);
    };

    start_animation_loop(draw_frame);

    Ok(())
}
