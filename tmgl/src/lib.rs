#![macro_use]

extern crate gl;

use std::ffi::{CStr, CString};

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

//////////////////////
// Creating shaders //
//////////////////////

/// Load a vertex or a fragment shader from a specified file.
pub fn shader_from_file(name_and_content: (&str, &str)) {
    let kind: gl::types::GLenum;
    let name = name_and_content.0;
    if name.ends_with(".vert") {
        kind = gl::VERTEX_SHADER;
    } else if name.ends_with(".frag") {
        kind = gl::FRAGMENT_SHADER;
    } else {
        panic!("Unknown shader type: {}", name);
    }
    println!("{}", name_and_content.1);
    //let res = shader_from_source(&CString::new(file_content).unwrap(), kind);
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}