use super::engine::DrawingType;
use std::sync::Arc;

#[derive(Clone)]
pub struct VBO {
    id: gl::types::GLuint,
    vao: VAO,
    vertex_count: usize,
}

fn get_bytes<T> (floats: usize) -> usize {
    floats * std::mem::size_of::<T>()
}

impl VBO {
    pub fn new(drawing_type: DrawingType) -> VBO {
        let mut id: gl::types::GLuint = 0;
        let vao = VAO::new(drawing_type);
        unsafe {
            gl::GenBuffers(1, &mut id);
            let out = VBO {
                id,
                vao,
                vertex_count: 0,
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
        if self.vertex_count > 0 {
            unsafe {
                self.vao.bind();
                gl::DrawArrays(
                    get_draw_mode(&self.drawing_type()),
                    0,
                    (self.vertex_count / self.vao.stride()) as i32,
                );     
                self.vao.unbind();
            }
        }
    }

    pub fn drawing_type(&self) -> &DrawingType {
        &self.vao.drawing_type
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

#[derive(Clone)]
pub struct MultiVBO {
    id: gl::types::GLuint,
    vao: Arc<VAO>,
    indices: usize,
    max_floats_per_index: usize,
    floats_at_index: Vec<usize>,
}

impl MultiVBO {
    const MAX_BYTES: usize = 2147483648;
                             
    pub fn new(drawing_type: DrawingType, indices: usize, max_floats_per_index: usize) -> MultiVBO {
        let mut id: gl::types::GLuint = 0;
        let vao = VAO::new(drawing_type);
        unsafe {
            gl::GenBuffers(1, &mut id);
            let mut out = MultiVBO {
                id,
                vao: Arc::new(vao),
                indices,
                max_floats_per_index,
                floats_at_index: vec![0; indices],
            };
            if out.total_bytes() > MultiVBO::MAX_BYTES {
                panic!("A buffer with {} indices with {} floats each would be {}. Maximum allowed is 2147483648.", indices, max_floats_per_index, MultiVBO::MAX_BYTES)
            }
            out.set_vao();
            out.alloc();
            out
        }
    }

    fn total_bytes(&self) -> usize {
        self.indices * self.index_bytes()
    }

    fn index_bytes(&self) -> usize {
        get_bytes::<f32>(self.max_floats_per_index)
    }

    fn offset_bytes(&self, index: usize) -> usize { // TODO test this stuff
        if index >= self.indices {
            panic!("Trying to get offset of index {} of MultiVBO with {} indices", index, self.indices);
        }
        index * self.index_bytes()
    }

    fn offset_floats(&self, index: usize) -> usize {
        if index >= self.indices {
            panic!("Trying to get offset of index {} of MultiVBO with {} indices", index, self.indices);
        }
        index * self.max_floats_per_index
    }

    unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    fn alloc(&mut self) {
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                self.total_bytes() as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
            self.unbind();
        }
    }

    fn clear_index(&mut self, index: usize) {
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                self.offset_bytes(index) as gl::types::GLsizeiptr,
                self.index_bytes() as gl::types::GLsizeiptr,
                std::ptr::null(),
            );
        }
    }

    pub fn load(&mut self, index: usize, floats: Vec<f32>) {
        if index >= self.indices {
            panic!("Tried to load to index {} of MultiVBO with {} indices");
        }
        if floats.len() > self.max_floats_per_index {
            panic!("Tried to load {} floats into index of MultiVBO where max floats per index is {}", floats.len(), self.max_floats_per_index);
        }
        unsafe {
            self.bind();
            self.clear_index(index);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                self.offset_bytes(index) as gl::types::GLsizeiptr,
                get_bytes::<f32>(floats.len()) as gl::types::GLsizeiptr,
                floats.as_ptr() as *const gl::types::GLvoid
            );
            self.unbind();
        }
        self.floats_at_index[index] = floats.len();
    }

    pub unsafe fn set_vao(&self) {
        self.bind();
        self.vao.set();
        self.unbind();
    }

    pub fn draw(&self) {
        unsafe {
            self.vao.bind();
            for i in 0..self.indices {
                let floats = self.floats_at_index[i];
                if floats > 0 {
                    gl::DrawArrays(
                        get_draw_mode(&self.drawing_type()),
                        (self.offset_floats(i) / self.vao.stride()) as i32, // TODO functions for these?
                        (floats / self.vao.stride()) as i32,
                    );     
                }
            }
            
            self.vao.unbind();
        }
    }

    pub fn drawing_type(&self) -> &DrawingType {
        &self.vao.drawing_type
    }
}


#[derive(Clone)]
pub struct VAO {
    id: gl::types::GLuint,
    drawing_type: DrawingType,
}

impl VAO {
    pub fn new(drawing_type: DrawingType) -> VAO {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO {
            id,
            drawing_type: drawing_type,
        }
    }

    pub fn stride(&self) -> usize {
        match self.drawing_type {
            DrawingType::Plain => 6,
            _ => 7,
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
        setup_vao(&self.drawing_type);
        self.unbind();
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.id);
        }
    }
}

fn setup_vao_for_plain_drawing() { //TODO why are these not part of VAO?
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

fn setup_vao_for_sprite_drawing() {
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (7 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (7 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            (7 * std::mem::size_of::<f32>()) as gl::types::GLint,
            (5 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
        );
    }
}

fn setup_vao(drawing_type: &DrawingType) {
    match drawing_type {
        DrawingType::Plain => setup_vao_for_plain_drawing(),
        DrawingType::Text => setup_vao_for_sprite_drawing(),
        DrawingType::Billboard => setup_vao_for_sprite_drawing(),
    }
}

fn get_draw_mode(drawing_type: &DrawingType) -> gl::types::GLenum {
    match drawing_type {
        _ => gl::TRIANGLES,
    }
}
