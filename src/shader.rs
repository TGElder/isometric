use std::ffi::{CString, CStr};

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = unsafe{ gl::CreateShader(kind) };
        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        
        let out = Shader{
            id
        };

        if out.compiled_succesfully() {
            Ok(out)
        } else {
            Err(out.get_message())
        }
    }

    fn compiled_succesfully(&self) -> bool {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut success);
        };
        return success != 0;
    }

    fn get_log_length(&self) -> i32 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        return len;
    }

    fn get_message(&self) -> String {
        let length = self.get_log_length();
        let mut buffer: Vec<u8> = vec![b' '; length as usize + 1];
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
        unsafe {
            gl::GetShaderInfoLog(
                self.id,
                length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        error.to_string_lossy().into_owned()
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}