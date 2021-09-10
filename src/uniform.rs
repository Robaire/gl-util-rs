use std::ffi::CString;

fn get_location(program: u32, uniform: &str) -> i32 {
    let location;

    unsafe {
        location = gl::GetUniformLocation(program, CString::new(uniform).unwrap().as_ptr());
    }

    location
}

pub fn set_float_vec3(program: u32, uniform: &str, data: &Vec<f32>) {
    if data.len() < 3 {
        panic!("Vec must have at least 3 elements");
    }

    let location = get_location(program, uniform);

    unsafe {
        gl::Uniform3fv(location, 1, data.as_ptr());
    }
}

pub fn set_float_vec2(program: u32, uniform: &str, data: &Vec<f32>) {
    if data.len() < 2 {
        panic!("Vec must have at least 2 elements");
    }

    let location = get_location(program, uniform);

    unsafe {
        gl::Uniform2fv(location, 1, data.as_ptr());
    }
}
