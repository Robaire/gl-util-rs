use gl::types::GLenum;
use std::ffi::CString;

/// Set a shader program as used
/// # Arugments
/// * `id` - Shader Program ID
pub fn set_used(id: u32) {
    unsafe {
        gl::UseProgram(id);
    }
}

/// Create a program object on the GPU
pub fn create() -> u32 {
    unsafe { gl::CreateProgram() }
}

/// Attach a shader to a program object
/// # Arguments
/// `program` - The id of the program to attach the shader to
/// `shaders` - The ids of the shaders to attach to the program
pub fn attach_shaders(program: u32, shaders: Vec<u32>) {
    for shader in shaders {
        unsafe { gl::AttachShader(program, shader) };
    }
}

/// Detach shaders from a program
/// # Arguments
/// `program` - id of the program to attach the shader to
/// `shaders` - ids of the shaders to attach to the program
pub fn detach_shaders(program: u32, shaders: Vec<u32>) {
    for shader in shaders {
        unsafe { gl::DetachShader(program, shader) };
    }
}

/// Link a program
/// # Arguments
/// `program` - id of the program to link
pub fn link(program: u32) -> Result<(), String> {
    unsafe {
        gl::LinkProgram(program);
    };

    let link_status = get_parameter(program, gl::LINK_STATUS);

    if link_status == 1 {
        Ok(())
    } else {
        Err(get_info_log(program))
    }
}

/// Deletes a program object
/// # Arguments
/// `program` - id of the program to delete
pub fn delete(program: u32) {
    unsafe { gl::DeleteProgram(program) };
}

/// Get a program parameter
/// # Arguments
/// `id` - id of the program
/// `param` - the program parameter to retrieve
fn get_parameter(id: u32, param: GLenum) -> i32 {
    let mut status: i32 = 0;
    unsafe {
        gl::GetProgramiv(id, param, &mut status);
    };

    status
}

/// Get a programs's info log
/// `id` - id of the program
fn get_info_log(id: u32) -> String {
    let log_length = get_parameter(id, gl::INFO_LOG_LENGTH);

    let log: CString = {
        let mut buffer: Vec<u8> = Vec::with_capacity(log_length as usize + 1);
        buffer.extend([b' '].iter().cycle().take(log_length as usize));
        unsafe { CString::from_vec_unchecked(buffer) }
    };

    unsafe {
        gl::GetProgramInfoLog(
            id,
            log_length,
            std::ptr::null_mut(),
            log.as_ptr() as *mut i8,
        );
    };

    log.to_string_lossy().into_owned()
}
