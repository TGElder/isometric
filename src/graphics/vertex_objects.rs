use super::engine::DrawingType;
use std::sync::Arc;

fn get_bytes<T> (floats: usize) -> usize {
    floats * std::mem::size_of::<T>()
}

pub struct VBO {
    id: gl::types::GLuint,
    vao: VAO,
}

impl VBO {
    fn new(drawing_type: DrawingType) -> VBO {
        let mut id: gl::types::GLuint = 0;
        let vao = VAO::new(drawing_type);
        unsafe {
            gl::GenBuffers(1, &mut id);
            let out = VBO {
                id,
                vao,
            };
            out.set_vao();
            out
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn set_vao(&self) {
        self.bind();
        self.vao.set();
        self.unbind();
    }

    fn drawing_type(&self) -> &DrawingType {
        &self.vao.drawing_type
    }

    fn count_verticies(&self, floats: usize) -> usize {
        floats / self.vao.stride()
    }

    fn alloc(&self, bytes: usize) {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                bytes as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
        }
        self.unbind();
    }

    fn clear_index(&self, offset_bytes: usize, clear_bytes: usize) {
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                offset_bytes as gl::types::GLsizeiptr,
                clear_bytes as gl::types::GLsizeiptr,
                std::ptr::null(),
            );
        }
    }

    fn draw(&self, float_offset: usize, floats: usize) {
        if floats > 0 {
            self.vao.bind();
            unsafe {
                gl::DrawArrays(
                    get_draw_mode(&self.drawing_type()),
                    self.count_verticies(float_offset) as i32,
                    self.count_verticies(floats) as i32,
                );     
            }
            self.vao.unbind();
        }
    }

    fn draw_many(&self, float_offset_increment: usize, floats_vec: &Vec<usize>) {
        self.vao.bind();
        let mut float_offset = 0;
        for floats in floats_vec {
            if *floats > 0 {
                unsafe {
                    gl::DrawArrays(
                        get_draw_mode(&self.drawing_type()),
                        self.count_verticies(float_offset) as i32,
                        self.count_verticies(*floats) as i32,
                    );
                }
            }
            float_offset += float_offset_increment;
        }
        self.vao.unbind();
    }
}

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

pub struct SimpleVBO {
    vbo: VBO,
    floats: usize,
}

impl SimpleVBO {
    pub fn new(drawing_type: DrawingType) -> SimpleVBO {
        SimpleVBO{
            vbo: VBO::new(drawing_type),
            floats: 0
        }
    }

    pub fn load(&mut self, vertices: Vec<f32>) {
        self.floats = vertices.len();
        unsafe {
            self.vbo.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            self.vbo.unbind();
        }
    }

    pub fn draw(&self) {
        self.vbo.draw(0, self.floats);
    }

    pub fn drawing_type(&self) -> &DrawingType {
        self.vbo.drawing_type()
    }

}

#[derive(Clone)]
pub struct MultiVBO {
    vbo: Arc<VBO>,
    indices: usize,
    max_floats_per_index: usize,
    floats_at_index: Vec<usize>,
}

impl MultiVBO {
    const MAX_BYTES: usize = 2147483648;
                             
    pub fn new(drawing_type: DrawingType, indices: usize, max_floats_per_index: usize) -> MultiVBO {
        let out = MultiVBO {
            vbo: Arc::new(VBO::new(drawing_type)),
            indices,
            max_floats_per_index,
            floats_at_index: vec![0; indices],
        };
        if out.total_bytes() > MultiVBO::MAX_BYTES {
            panic!("A buffer with {} indices with {} floats each would be {}. Maximum allowed is 2147483648.", indices, max_floats_per_index, MultiVBO::MAX_BYTES)
        }
        out.vbo.set_vao();
        out.vbo.alloc(out.total_bytes());
        out
        
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


    
    pub fn load(&mut self, index: usize, floats: Vec<f32>) {
        if index >= self.indices {
            panic!("Tried to load to index {} of MultiVBO with {} indices");
        }
        if floats.len() > self.max_floats_per_index {
            panic!("Tried to load {} floats into index of MultiVBO where max floats per index is {}", floats.len(), self.max_floats_per_index);
        }
        unsafe {
            self.vbo.bind();
            self.vbo.clear_index(self.offset_bytes(index), self.index_bytes());
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                self.offset_bytes(index) as gl::types::GLsizeiptr,
                get_bytes::<f32>(floats.len()) as gl::types::GLsizeiptr,
                floats.as_ptr() as *const gl::types::GLvoid
            );
            self.vbo.unbind();
        }
        self.floats_at_index[index] = floats.len();
    }

    pub fn draw(&self) {
        self.vbo.draw_many(self.max_floats_per_index, &self.floats_at_index);
    }

    pub fn drawing_type(&self) -> &DrawingType {
        &self.vbo.vao.drawing_type
    }
}

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

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn set(&self) {
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
