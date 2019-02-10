use super::program::Program;
use super::shader::Shader;
use std::ffi::CString;
use std::marker::PhantomData;

use super::transformer::Transformer;

pub struct GraphicsEngine {
    terrain_triangles: VBO<ColoredVertex>,
    terrain_lines: VBO<Vertex>,
    program: Program,
    transformer: Transformer,
}

impl GraphicsEngine {

    pub fn new(viewport_size: na::Point2<u32>) -> GraphicsEngine {
        let program = GraphicsEngine::load_program();

        GraphicsEngine {
            terrain_triangles: VBO::new(gl::TRIANGLES),
            terrain_lines: VBO::new(gl::LINES),
            program,
            transformer: Transformer::new(viewport_size),
        }
    }

    pub fn get_transformer(&mut self) -> &mut Transformer {
        &mut self.transformer
    }

    fn load_program() -> Program {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.vert")).unwrap(), //TODO don't like exposing CString
            gl::VERTEX_SHADER,
        )
        .unwrap();

        let fragment_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
            gl::FRAGMENT_SHADER,
        )
        .unwrap();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        let shader_program = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

        shader_program.set_used();

        return shader_program;
    }

    fn load_transform_matrix(&self, transform_matrix: na::Matrix4<f32>) {
        self.program.load_matrix("transform", transform_matrix);
    }

    pub fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.load_transform_matrix(self.transformer.compute_transform_matrix(0.0));
            self.terrain_triangles.draw();
            self.load_transform_matrix(self.transformer.compute_transform_matrix(-0.001));
            self.terrain_lines.draw();
        }
    }

    pub fn set_viewport_size(&mut self, viewport_size: na::Point2<u32>) {
        self.transformer.set_viewport_size(viewport_size);
        unsafe {
            gl::Viewport(0, 0, viewport_size.x as i32, viewport_size.y as i32);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }
    }

    pub fn load_terrain(&mut self, heights: na::DMatrix<f32>) {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut triangle_vertices: Vec<f32> = Vec::with_capacity(width * height * 36);
        let mut line_vertices: Vec<f32> = Vec::with_capacity(width * height * 24);
        for x in 0..(width - 1) {
            for y in 0..(height - 1) {
                let a = (x as f32, y as f32, heights[(x, y)]);
                let b = (x as f32 + 1.0, y as f32, heights[(x + 1, y)]);
                let c = (x as f32 + 1.0, y as f32 + 1.0, heights[(x + 1, y + 1)]);
                let d = (x as f32, y as f32 + 1.0, heights[(x, y + 1)]);
                triangle_vertices.extend([
                    a.0, a.1, a.2, a.2, a.2, a.2,
                    d.0, d.1, d.2, d.2, d.2, d.2,
                    c.0, c.1, c.2, c.2, c.2, c.2,
                    a.0, a.1, a.2, a.2, a.2, a.2,
                    c.0, c.1, c.2, c.2, c.2, c.2,
                    b.0, b.1, b.2, b.2, b.2, b.2
                ].iter().cloned());
                line_vertices.extend([
                    a.0, a.1, a.2,
                    b.0, b.1, b.2,
                    b.0, b.1, b.2,
                    c.0, c.1, c.2,
                    c.0, c.1, c.2,
                    d.0, d.1, d.2,
                    d.0, d.1, d.2,
                    a.0, a.1, a.2   
                ].iter().cloned());
            }
        }
        self.terrain_triangles.load(triangle_vertices);
        self.terrain_lines.load(line_vertices);
    }
}

struct VBO<T: BufferType> {
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

struct VAO<T: BufferType> {
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

trait BufferType {
    fn setup_vao();
}

struct Vertex {}

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

struct ColoredVertex {}

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