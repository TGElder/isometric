use shader::Shader;
use std::ffi::CString;
use utils::create_whitespace_cstring_with_len;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let out = Program { id };

        if !out.linked_succesfully() {
            Err(out.get_message())
        } else {
            for shader in shaders {
                unsafe {
                    gl::DetachShader(id, shader.id());
                }
            }

            Ok(out)
        }
    }

    fn linked_succesfully(&self) -> bool {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        };
        return success != 0;
    }

    fn get_log_length(&self) -> i32 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);
        }
        return len;
    }

    fn get_message(&self) -> String {
        let length = self.get_log_length();
        let error = create_whitespace_cstring_with_len(length as usize);
        unsafe {
            gl::GetProgramInfoLog(
                self.id,
                length,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }
        error.to_string_lossy().into_owned()
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn load_matrix(&self, variable: &str, matrix: na::Matrix4<f32>) {
        unsafe {
            let mvp_location = gl::GetUniformLocation(
                self.id(),
                CString::new(variable).unwrap().as_ptr() as *const gl::types::GLchar,
            );
            let proj_ptr = matrix.as_slice().as_ptr();
            gl::UniformMatrix4fv(mvp_location, 1, gl::FALSE, proj_ptr);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
