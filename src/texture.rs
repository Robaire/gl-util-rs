extern crate image;

/// Generate a texture buffer
pub fn generate() -> u32 {
    let mut id = 0;

    unsafe {
        gl::GenTextures(1, &mut id);
    };

    return id;
}

/// Bind a texture
/// # Arguments
/// * `id` - Texture ID
pub fn bind(id: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, id);
    }
}

/// Set texture data
/// # Arguments
/// * `id` - Texture ID
/// * `texture` - Texture Data
pub fn set(id: u32, texture: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            texture.width() as i32,
            texture.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            texture.as_ptr() as *const gl::types::GLvoid,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    }
}

/// Set texture data directly from a file
/// # Arguments
/// * `file` - File path
pub fn create_from_file(file: &str) -> Result<u32, String> {
    // Try to load the texture
    let texture = match image::open(file) {
        Ok(image) => image.flipv().into_rgba8(),
        Err(message) => return Err(message.to_string())
    };

    // Create a texture and set the image data
    let id = generate();
    set(id, texture);

    return Ok(id);
}
