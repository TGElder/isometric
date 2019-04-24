use super::engine::DrawingType;

pub struct VBO {
    id: gl::types::GLuint,
    vao: VAO,
    vertex_count: usize,
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
        println!("Binding {}", self.id);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    unsafe fn unbind(&self) {
        println!("Unbinding {}", self.id);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    pub fn clear(&mut self, vertex_count: usize) {
        self.vertex_count = vertex_count;
        unsafe {
            self.bind();
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertex_count * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                std::ptr::null(),
                gl::STATIC_DRAW,
            );
            self.unbind();
        }
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

    pub fn load_at_offset(&self, offset: isize, vertices: Vec<f32>) {
        if offset < 0 {
            panic!("Tried to load at negative offset {} - not supported.", offset);
        }
        if offset as usize + vertices.len() > self.vertex_count {
            panic!("Tried to load {} vertices into buffer of size {} starting from offset {}", vertices.len(), self.vertex_count, offset);
        }
        unsafe {
            self.bind();
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                offset,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid
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
            println!("Drawing {} vertices {}/{}", (self.vertex_count / self.vao.stride()) as i32, self.vertex_count, self.vao.stride());
            gl::DrawArrays(
                get_draw_mode(&self.drawing_type()),
                0,
                (self.vertex_count / self.vao.stride()) as i32,
            );
            self.vao.unbind();
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

fn setup_vao_for_plain_drawing() {
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
