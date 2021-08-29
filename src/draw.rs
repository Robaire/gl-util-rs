extern crate gl;
use gl::types::{GLenum, GLint, GLsizei};

pub fn arrays(mode: GLenum, first: usize, count: usize) {
    unsafe { gl::DrawArrays(mode, first as GLint, count as GLsizei); }
}
