extern crate gl;
use gl::types::GLenum;

pub mod buffer;
pub mod draw;
pub mod program;
pub mod shader;
pub mod texture;
pub mod uniform;

/// Get error
pub fn get_error() -> GLenum {
    return unsafe { gl::GetError() };
}

/// Clear
pub fn clear() {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
}
