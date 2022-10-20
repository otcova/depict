// #![feature(impl)]
pub mod console;
mod draw;
mod request_frame;
mod setup;
mod time;
mod web;

use std::{cell::RefCell, rc::Rc};

pub use draw::*;
pub use vecn::*;
pub use wasm_bindgen;

use request_frame::*;
use time::*;
use webgl::*;

type Result<T> = std::result::Result<T, String>;

pub struct Depict {
    gl: WebGl,
    size: Vec2,
    time: Rc<RefCell<FrameTime>>,
}

impl Depict {
    pub fn new() -> Result<Self> {
        let gl = WebGl::new()?;
        Ok(Self {
            size: Self::calc_size(&gl),
            time: Rc::new(RefCell::new(FrameTime::new())),
            gl,
        })
    }

    pub fn draw_loop<F: FnMut(&Self, &mut Draw) + 'static>(self, mut f: F) -> Result<()> {
        let mut draw = Draw::new(&self.gl)?;

        let t = self.time.clone();
        web::add_event_listener("visibilitychange", move || {
            if web::Document::visibility_state() == "hidden" {
                t.borrow_mut().pause_time()
            }
        });

        start_animation_loop(move || {
            self.time.borrow_mut().start_frame();
            f(&self, &mut draw);
            draw.draw(&self);
            self.time.borrow_mut().end_frame();
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
