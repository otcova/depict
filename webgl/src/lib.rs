mod buffer;
mod mesh;
mod shader;
mod web;

pub use buffer::*;
pub use mesh::*;
pub use shader::*;

use wasm_bindgen::*;
use web::*;
use web_sys::*;

type Result<T> = std::result::Result<T, String>;

pub struct WebGl {
    ctx: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
}

impl WebGl {
    pub fn new() -> Result<Self> {
        let canvas = load_canvas()?;
        let ctx = canvas
            .get_context("webgl2")
            .map_err(|e| format!("Couldn't get webgl2 context. {:?}", e))?
            .ok_or("Coudn't get webgl2 context")?
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|e| format!("Couldn't get webgl2 context. {:?}", e))?;
        // ctx.enable(WebGl2RenderingContext::DEPTH_TEST);

        Ok(Self { ctx, canvas })
    }

    pub fn clear_canvas(&self, color: [f32; 4]) {
        self.ctx.clear_color(color[0], color[1], color[2], color[3]);
        self.ctx.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        // self.ctx.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }

    pub fn width(&self) -> f32 {
        self.canvas.width() as f32
    }

    pub fn height(&self) -> f32 {
        self.canvas.height() as f32
    }
}

fn load_canvas() -> Result<HtmlCanvasElement> {
    let canvas = Document::get_element_by_id("canvas").ok_or("Couldn't get canvas")?;
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|e| format!("Invalid canvas. {:?}", e))?;

    canvas.set_width(
        web::Window::inner_width()
            .as_f64()
            .ok_or("Could not get inner width")? as u32,
    );
    canvas.set_height(
        web::Window::inner_height()
            .as_f64()
            .ok_or("Could not get inner height")? as u32,
    );

    Ok(canvas)
}
