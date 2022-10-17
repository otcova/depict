use std::ops::{DerefMut, Deref};

use super::*;

type RawElementBuffer = Buffer<{ WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER }>;
#[derive(Clone)]
pub struct ElementBuffer(RawElementBuffer);

impl Deref for ElementBuffer {
    type Target = RawElementBuffer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElementBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WebGl {
    pub fn new_element_buffer(&self) -> Result<ElementBuffer> {
        Ok(ElementBuffer(Buffer::new(self)?))
    }
}
