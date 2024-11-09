use std::ffi::CStr;
use gl::{GetShaderInfoLog, GetProgramInfoLog, types};

macro_rules! log_error {
    ($func_name:ident, $id:ident) => {
        let mut error_buf = [0u8; 512];
        let mut error_len = 0;
        $func_name($id, error_buf.len() as i32, &mut error_len, error_buf.as_mut_ptr() as *mut i8);
        let error_len = error_len as usize;
        let err_str = std::str::from_utf8_unchecked(&error_buf[..error_len]);
        eprintln!("{err_str}");
    };
}

pub(crate) struct ShaderProgram {
    id: u32,
}
impl ShaderProgram {
    pub(crate) fn compile_vert_and_frag(vert_code: &str, frag_code: &str) -> Option<Self> {
        let vert = Shader::new_compile(vert_code, ShaderKind::Vertex)?;
        let frag = Shader::new_compile(frag_code, ShaderKind::Fragment)?;
        let id = unsafe {
            let id = gl::CreateProgram();
            gl::AttachShader(id, vert.id);
            gl::AttachShader(id, frag.id);
            gl::LinkProgram(id);
            let mut success = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                eprintln!("error linking shader program");
                log_error!(GetProgramInfoLog, id);
                gl::DeleteProgram(id);
                return None;
            }
            id
        };
        Some(Self { id })
    }
    pub(crate) fn get_uniform_location(&self, name: &CStr) -> i32 {
        let loc = unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) };
        if loc < 0 {
            eprintln!("failed to get location of uniform {name:?}");
        }
        loc
    }
    pub(crate) fn set_uniform(&self, location: i32, v1: f32, v2: f32, v3: f32, v4: f32) {
        unsafe { gl::Uniform4f(location, v1, v2, v3, v4) };
    }
    pub(crate) fn list_uniforms(&self) {
        unsafe {
            let mut count = 0;
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORMS, &mut count);
            println!("Active Uniforms: {count}");

            let mut buf = [0u8; 512];
            let mut len = 0;
            let mut size = 0;
            let mut kind = 0;
            for i in 0..count {
                gl::GetActiveUniform(self.id, i as u32, buf.len() as i32, &mut len, &mut size, &mut kind, buf.as_mut_ptr() as *mut i8);
                let len = len as usize;
                let str = std::str::from_utf8_unchecked(&buf[..len]);
                println!("Uniform #{i} Type: {kind} Name: {str}");
            }
        }
    }
}
impl IProgram for ShaderProgram {
    fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) }
    }
}
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}

pub(crate) enum ShaderKind {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}
pub(crate) struct Shader {
    id: u32,
}
impl Shader {
    pub(crate) fn new_compile(code: &str, kind: ShaderKind) -> Option<Self> {
        let id = unsafe {
            let id = gl::CreateShader(kind as u32);
            if id == 0 {
                eprintln!("failed to create shader id");
                return None;
            }
            let codes = [str_to_gl_ptr(code)];
            gl::ShaderSource(id, 1, codes.as_ptr(), &(code.len() as i32));
            gl::CompileShader(id);
            let mut success = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                eprintln!("error compiling shader");
                log_error!(GetShaderInfoLog, id);
                gl::DeleteShader(id);
                return None;
            }
            id
        };
        Some(Self { id })
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

pub(crate) trait IProgram {
    fn use_program(&self);
}

#[inline]
unsafe fn str_to_gl_ptr(str: &str) -> *const types::GLchar {
    str.as_bytes().as_ptr() as *const types::GLchar
}
