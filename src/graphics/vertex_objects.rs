use std::marker::PhantomData;

pub struct VBO<T: BufferType> {
    id: gl::types::GLuint,
    draw_mode: gl::types::GLenum,
    vao: VAO<T>,
    vertex_count: usize,
}

impl<T: BufferType> VBO<T> {
    pub fn new(draw_mode: gl::types::GLenum) -> VBO<T> {
        let mut id: gl::types::GLuint = 0;
        let vao = VAO::new();
        unsafe {
            gl::GenBuffers(1, &mut id);
            let out = VBO{
                id,
                vao,
                vertex_count: 0,
                draw_mode
            };
            out.set_vao();
            out
        }
    }

    unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    pub fn load(&mut self, vertices: Vec<f32>) {
        self.vertex_count = vertices.len();
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            self.unbind();
        }
    }

    pub unsafe fn set_vao(&self) {
        self.bind();
        self.vao.set();
        self.unbind();
    }

    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            gl::DrawArrays(
                self.draw_mode,
                0,
                self.vertex_count as i32,
            );
            self.vao.unbind();
        }
    }
}

impl<T: BufferType> Drop for VBO<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

pub struct VAO<T: BufferType> {
    id: gl::types::GLuint,
    buffer_type: PhantomData<T>,
}

impl<T: BufferType> VAO<T> {
    pub fn new() -> VAO<T> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO {
            id,
            buffer_type: PhantomData,
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
    }

    pub unsafe fn set(&self) {
        self.bind();
        T::setup_vao();
        self.unbind();
    }
}


impl<T: BufferType> Drop for VAO<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}

pub trait BufferType {
    fn setup_vao();
}

pub struct Vertex {}

impl BufferType for Vertex {
    fn setup_vao() {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
        }
    }
}

pub struct ColoredVertex {}

impl BufferType for ColoredVertex {
    fn setup_vao() {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }
}

pub struct TexturedVertex {}

impl BufferType for TexturedVertex {
    fn setup_vao() {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                (10 * std::mem::size_of::<f32>()) as gl::types::GLint,
                (8 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }
}