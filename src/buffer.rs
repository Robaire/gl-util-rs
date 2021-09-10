use gl::types::{GLenum, GLint, GLuint};

/// Generate a buffer
pub fn generate_buffer() -> Result<u32, String> {
    let mut id = 0;

    unsafe {
        gl::GenBuffers(1, &mut id);
    }

    if id == 0 {
        // TODO: Get an error message from the GPU
        Err("".to_string())
    } else {
        Ok(id)
    }
}

/// Bind a buffer
pub fn bind_buffer(id: u32, kind: GLenum) {
    unsafe {
        gl::BindBuffer(kind, id);
    }
}

/// Set the vertx data in a buffer
pub fn set_buffer_data<T>(id: u32, draw_type: GLenum, data: &Vec<T>) {
    unsafe {
        gl::BindBuffer(id, gl::ARRAY_BUFFER);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
            data.as_ptr() as *const gl::types::GLvoid,
            draw_type,
        );
    }
}

/// Generate a vertex attribute array
pub fn generate_vertex_array() -> Result<u32, String> {
    let mut id = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut id);
    }

    if id == 0 {
        Err("".to_string())
    } else {
        Ok(id)
    }
}

/// Bind an attribute array
pub fn bind_array(id: u32) {
    unsafe {
        gl::BindVertexArray(id);
    }
}

/// Set vertex attribute array data
/// # Arguments
/// * `id` - Vertex Array ID
/// * `buffer` - Buffer vertex data is stored in
/// * `index` - Vertex Array Index to modify
/// * `size` - The number of components per vertex
pub fn set_vertex_array_pointer(id: u32, buffer: u32, index: u32, size: u32) {
    if size > 4 || size < 1 {
        panic!("Size must be 1, 2, 3, or 4");
    }

    unsafe {
        gl::BindVertexArray(id);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size as i32,
            gl::FLOAT,
            gl::FALSE,
            0,
            std::ptr::null(),
        );
    }
}
