pub mod console;
mod draw;
mod request_frame;
mod setup;
mod time;
mod web;

pub use draw::*;
pub use vecn::*;
pub use wasm_bindgen;

use request_frame::*;
pub use time::*;
use webgl::*;

type Result<T> = std::result::Result<T, String>;

pub struct Depict {
    gl: WebGl,
    size: Vec2,
    time: FrameTime,
}

impl Depict {
    pub fn new() -> Result<Self> {
        let gl = WebGl::new()?;
        Ok(Self {
            size: Self::calc_size(&gl),
            time: FrameTime::new(),
            gl,
        })
    }

    pub fn draw_loop<F: FnMut(&Self, &mut Draw) + 'static>(mut self, mut f: F) -> Result<()> {
        let mut draw = Draw::new(&self.gl)?;

        let time_handle = self.time.pause_handle();
        web::add_event_listener("visibilitychange", move || {
            if web::Document::visibility_state() == "hidden" {
                time_handle.pause_time();
            }
        });

        start_animation_loop(move || {
            self.time.start_frame();
            f(&self, &mut draw);
            draw.draw(&self);
            self.time.end_frame();
        });

        Ok(())
    }
}

impl Depict {
    fn calc_size(gl: &WebGl) -> Vec2 {
        let pixels = (gl.width(), gl.height());
        if pixels.0 > pixels.1 {
            vec2![pixels.0 / pixels.1, 1.]
        } else {
            vec2![1., pixels.1 / pixels.0]
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}
