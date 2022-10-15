use crate::*;
use js_sys::{Float32Array, Int32Array};
use web_sys::*;

pub type ArrayBuffer = Buffer<{WebGl2RenderingContext::ARRAY_BUFFER}>;
pub type ElementBuffer = Buffer<{WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER}>;

pub struct Buffer<const T: u32> {
    ctx: WebGl2RenderingContext,
    buffer: WebGlBuffer,
    len: usize,
}

impl WebGl {
    pub fn new_buffer<const T: u32>(&self) -> Result<Buffer<T>> {
        Ok(Buffer {
            ctx: self.ctx.clone(),
            buffer: self
                .ctx
                .create_buffer()
                .ok_or("Unable to create gl buffer")?,
            len: 0,
        })
    }
}

impl<const T: u32> Buffer<T> {
    pub(crate) fn bind(&self) {
        self.ctx.bind_buffer(T, Some(&self.buffer));
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn update_i32(&mut self, data: &[i32]) {
        if self.len < data.len() {
            self.allocate_i32(data, WebGl2RenderingContext::DYNAMIC_DRAW);
        } else {
            self.update_slice_i32(data, 0);
        }
    }

    pub fn update_f32(&mut self, data: &[f32]) {
        if self.len < data.len() {
            self.allocate_f32(data, WebGl2RenderingContext::DYNAMIC_DRAW);
        } else {
            self.update_slice_f32(data, 0);
        }
    }

    /// It's faster than allocate if you don't want to update data frequently
    pub fn allocate_static_i32(&mut self, data: &[i32]) {
        self.allocate_i32(data, WebGl2RenderingContext::STATIC_DRAW);
    }
    pub fn allocate_static_f32(&mut self, data: &[f32]) {
        self.allocate_f32(data, WebGl2RenderingContext::STATIC_DRAW);
    }

    pub fn update_slice_i32(&self, data: &[i32], dst_byte_offset: i32) {
        into_js_i32_array(data, |array| self.update_slice(&array, dst_byte_offset))
    }
    pub fn update_slice_f32(&self, data: &[f32], dst_byte_offset: i32) {
        into_js_f32_array(data, |array| self.update_slice(&array, dst_byte_offset))
    }

    pub fn update_slice(&self, data: &js_sys::Object, dst_byte_offset: i32) {
        self.bind();

        self.ctx.buffer_sub_data_with_i32_and_array_buffer_view(
            T,
            dst_byte_offset,
            data,
        );
    }

    fn allocate_i32(&mut self, data: &[i32], usage: u32) {
        into_js_i32_array(data, |array| self.allocate(&array, data.len(), usage));
    }
    fn allocate_f32(&mut self, data: &[f32], usage: u32) {
        into_js_f32_array(data, |array| self.allocate(&array, data.len(), usage));
    }
    fn allocate(&mut self, data: &js_sys::Object, data_len: usize, usage: u32) {
        self.len = data_len;
        self.bind();
        self.ctx
            .buffer_data_with_array_buffer_view(T, data, usage);
    }
}

impl<const T: u32> Drop for Buffer<T> {
    fn drop(&mut self) {
        self.ctx.delete_buffer(Some(&self.buffer));
    }
}

pub fn into_js_i32_array<F: FnOnce(Int32Array)>(data: &[i32], scope: F) {
    // Note that `Int32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Int32Array` to be invalid.
    //
    // As a result, after `Int32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let array_buffer_view = js_sys::Int32Array::view(&data);
        scope(array_buffer_view);
    }
}

pub fn into_js_f32_array<F: FnOnce(Float32Array)>(data: &[f32], scope: F) {
    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let array_buffer_view = js_sys::Float32Array::view(&data);
        scope(array_buffer_view);
    }
}
