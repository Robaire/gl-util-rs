use gl::types::GLenum;

/// Create a shader using a string as the source code
/// # Arguments
/// `kind` - The kind of shader to create
/// `source` - The shader source code string
pub fn new_from_string(kind: GLenum, source: String) -> Result<u32, String> {
    // Create a shader
    let shader_id: u32 = create(kind)?;

    // Compile the shader
    compile(shader_id, source);

    // Check if the shader compiled
    let status = get_parameter(shader_id, gl::COMPILE_STATUS)?;

    if status == 1 {
        Ok(shader_id)
    } else {
        Err(get_info_log(shader_id))
    }
}

/// Create a shader using a file as the source code
/// # Arguments
/// `path` - Path to the source code file
/// `kind` - The kind of shader to create
pub fn new_from_file(kind: GLenum, path: &str) -> Result<u32, String> {
    // Read the source file in as a string
    match std::fs::read_to_string(path) {
        Ok(source) => new_from_string(kind, source),
        Err(message) => Err(message.to_string()),
    }
}

/// Creates a shader object on the GPU
/// # Arguments
/// * `kind` - The kind of shader to generate
fn create(kind: GLenum) -> Result<u32, String> {
    let id;
    unsafe {
        id = gl::CreateShader(kind);
    };

    if id == 0 {
        // TODO: Get an error message from the GPU
        return Err("Shader could not be created".to_string());
    } else {
        return Ok(id);
    }
}

/// Compiles a shader program
/// # Arguments
/// * `id` - Shader Program ID
/// * `source` - Shader program source code
fn compile(id: u32, source: String) {
    unsafe {
        gl::ShaderSource(
            id,
            1,
            // TODO: Get rid of warning message
            &std::ffi::CString::new(source).unwrap().as_ptr(),
            std::ptr::null(),
        );
        gl::CompileShader(id);
    };

    // TODO: Check for compilation error
}

/// Get a shader parameter
/// # Arguments
/// `id` - Shader ID
/// `param` - The shader parameter to retrieve
fn get_parameter(id: u32, param: GLenum) -> Result<i32, String> {
    let mut status = 0;
    unsafe {
        gl::GetShaderiv(id, param, &mut status);
    };

    if status == 0 {
        Err("".to_string())
    } else {
        Ok(status)
    }
}

/// Get a shader's info log
/// `id` - Shader ID
fn get_info_log(id: u32) -> String {
    let log_length = get_parameter(id, gl::INFO_LOG_LENGTH).unwrap();

    let log: std::ffi::CString = {
        let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize + 1);
        buffer.extend([b' '].iter().cycle().take(log_length as usize));
        unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
    };

    unsafe {
        gl::GetShaderInfoLog(
            id,
            log_length,
            std::ptr::null_mut(),
            log.as_ptr() as *mut i8,
        );
    };

    log.to_string_lossy().into_owned()
}
