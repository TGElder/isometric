use ::image::{DynamicImage, GenericImageView};
use std::ffi::c_void;

pub struct Texture {
    id: gl::types::GLuint,
}

impl Texture {
    pub fn new() -> Texture {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); 
            let out = Texture{
                id,
            };
            out
        }
    }

    pub unsafe fn bind(&self) {        
        gl::BindTexture(gl::TEXTURE_2D, self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    pub fn load(&mut self, image: DynamicImage) {
        let (width, height) = image.dimensions();
        let image = image.to_rgba().into_raw();
        let image_ptr: *const c_void = image.as_ptr() as *const c_void;
        
        unsafe {
            self.bind();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image_ptr
            );
            self.unbind();
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}