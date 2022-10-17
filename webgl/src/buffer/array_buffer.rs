use std::ops::{Deref, DerefMut};

use super::*;

type RawArrayBuffer = Buffer<{ WebGl2RenderingContext::ARRAY_BUFFER }>;
#[derive(Clone)]
pub struct ArrayBuffer(RawArrayBuffer, &'static [ShaderAttrib]);

impl Deref for ArrayBuffer {
    type Target = RawArrayBuffer;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ArrayBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ArrayBuffer {
    pub fn attribs(&self) -> &'static [ShaderAttrib] {
        self.1
    }
}

impl WebGl {
    pub fn new_array_buffer(
        &self,
        attribs: &'static [ShaderAttrib],
    ) -> Result<ArrayBuffer> {
		Ok(ArrayBuffer(Buffer::new(self)?, attribs))
    }
}
