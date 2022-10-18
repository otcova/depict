pub mod console;
mod draw;
mod request_frame;
mod setup;
mod time;

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
    chrono: Chrono,
}

impl Depict {
    pub fn new() -> Result<Self> {
        let gl = WebGl::new()?;
        Ok(Self {
            size: Self::calc_size(&gl),
            chrono: Chrono::default(),
            gl,
        })
    }

    pub fn draw_loop<F: FnMut(&Self, &mut Draw) + 'static>(mut self, mut f: F) -> Result<()> {
        let mut draw = Draw::new(&self.gl)?;
        self.chrono = Chrono::default();

        start_animation_loop(move || {
            f(&self, &mut draw);
            draw.draw(&self);

            self.chrono.update();
        });

        Ok(())
    }
}

impl Depict {
    fn calc_size(gl: &WebGl) -> Vec2 {
        let pixels = (gl.width(), gl.height());
        if pixels.0 > pixels.1 {
            vec2![1., pixels.1 / pixels.0]
        } else {
            vec2![pixels.0 / pixels.1, 1.]
        }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn seconds(&self) -> f32 {
        self.chrono.seconds()
    }
}
