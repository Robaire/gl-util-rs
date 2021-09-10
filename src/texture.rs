extern crate image;

use gl::types::{GLenum, GLuint};

pub struct Texture {
    texture_type: GLenum,
    id: GLuint,
}

impl Texture {
    /// Create a new texture object of type texture_type
    pub fn new(texture_type: GLenum) -> Texture {
        let id = generate_texture_buffer();

        Texture { texture_type, id }
    }

    /// Bind this texture
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(self.texture_type, self.id);
        }
    }

    /// Set a texture parameter
    /// Binds this texture
    pub fn parameter(&self, parameter: GLenum, value: GLenum) {
        self.bind();

        unsafe {
            gl::TexParameteri(self.texture_type, parameter, value as i32);
        }
    }
}

/// Set texture data
/// # Arguments
/// * `texture` - Texture Object
/// * `image` - Texture Data
pub fn set_2d(texture: &Texture, image: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>) {
    // This function only works on 2D textures
    assert_eq!(texture.texture_type, gl::TEXTURE_2D);

    texture.bind();

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            image.width() as i32,
            image.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            image.as_ptr() as *const gl::types::GLvoid,
        );
    }
}

/// Set texture data for a 2D_ARRAY texture
pub fn set_2d_array(texture: &Texture) {
    // This funciton only works on 2D_ARRAY textures
    assert_eq!(texture.texture_type, gl::TEXTURE_2D_ARRAY);

    texture.bind();

    unsafe {
        // gl::TexImage
    }

    todo!();
}

/// Set texture data directly from a file
/// # Arguments
/// * `file` - File path
pub fn create_2d_from_file(file: &str) -> Result<Texture, String> {
    // Try to load the texture
    let image = match image::open(file) {
        Ok(img) => img.flipv().into_rgba8(),
        Err(message) => return Err(message.to_string()),
    };

    // Create a texture and set the image data
    let texture = Texture::new(gl::TEXTURE_2D);
    set_2d(&texture, image);

    return Ok(texture);
}

/// Generate a texture buffer
fn generate_texture_buffer() -> GLuint {
    let mut id = 0;

    unsafe {
        gl::GenTextures(1, &mut id);
    };

    assert_ne!(id, 0);

    return id;
}
