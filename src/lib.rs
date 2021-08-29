use std::ffi::CString;

extern crate gl;
use gl::types::{GLenum, GLint};

pub mod shader;
pub mod program;
pub mod draw;
pub mod texture;

/// Get error
pub fn get_error() -> GLenum {
    return unsafe { gl::GetError() };
}

/// Clear
pub fn clear() {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
}

/// Generates a buffer on the GPU and returns its id
pub fn generate_buffer() -> u32 {
    let mut id = 0;

    unsafe {
        gl::GenBuffers(1, &mut id);
    };

    assert_ne!(id, 0);

    return id;
}

/// Sets the vertex data in a buffer
/// # Arguments
/// * `id` - Buffer ID
/// * `data` - Data to upload
pub fn set_buffer_data<T>(id: u32, data: &Vec<T>) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    };
}

/// Bind a buffer
/// # Arguments
/// * `id` - Buffer ID
pub fn bind_buffer(id: u32) {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, id);
    }
}

/// Generates a vertex attribute array on the GPU
pub fn generate_vertex_array() -> u32 {
    let mut id = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut id);
    };

    assert_ne!(id, 0);

    return id;
}

/// Bind an attribute array
/// # Arguments
/// * `id` - Vertex Array ID
pub fn bind_array(id: u32) {
    unsafe {
        gl::BindVertexArray(id);
    }
}

/// Set vertex attribute array
/// # Arguments
/// * `buffer` - Buffer vertex data is stored in
/// * `id` - Vertex Array ID
/// * `index` - Vertex Array Index to modify
/// * `size` - The number of components per vertex
pub fn set_vertex_array_pointer(buffer: u32, id: u32, index: u32, size: u32) {
    if size > 4 || size == 0 {
        panic!("Size must be 1, 2, 3, or 4");
    }

    unsafe {
        gl::BindVertexArray(id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(index, size as GLint, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}

/// Set the value of a vec3 uniform
/// # Arguments
/// * `uniform` - The name of the uniform to copy data to
/// * `program` - The shader program in use
/// * `data` - Data to copy to the uniform
pub fn set_uniform_float_vec3(uniform: &str, program: u32, data: &Vec<f32>) {
    unsafe {
        let location = gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr());
        gl::Uniform3fv(location, 1, data.as_ptr());
    }
}

/// Set the value of a vec2 uniform
/// # Arguments
/// * `uniform` - The name of the uniform to copy data to
/// * `program` - The shader program in use
/// * `data` - Data to copy to the uniform
pub fn set_uniform_float_vec2(uniform: &str, program: u32, data: &Vec<f32>) {
    unsafe {
        let location = gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr());
        gl::Uniform2fv(location, 1, data.as_ptr());
    }
}
