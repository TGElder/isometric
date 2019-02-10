
use program::Program;
use shader::Shader;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct GraphicsEngine {
    triangle_vao: VAO<ColoredVertex>,
    triangle_vertex_count: i32,
    line_vao: VAO<Vertex>,
    line_vertex_count: i32,
    program: Program,
    scale: f32,
    translation: (f32, f32),
    rotation: usize,
}

impl GraphicsEngine {

    const ISOMETRIC_COEFFS: [(f32, f32); 4] = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];

    pub fn new(triangle_vertices: Vec<f32>, line_vertices: Vec<f32>) -> GraphicsEngine {
        let triangle_vao: VAO<ColoredVertex> = VAO::new();
        let triangle_vbo: VBO = VBO::new(&triangle_vao);
        let triangle_vertex_count = triangle_vertices.len();
        triangle_vbo.load(triangle_vertices);

        let line_vao: VAO<Vertex> = VAO::new();
        let line_vbo: VBO = VBO::new(&line_vao);
        let line_vertex_count = line_vertices.len();
        line_vbo.load(line_vertices);


        let program = GraphicsEngine::load_program();

        let out = GraphicsEngine {
            triangle_vao,
            triangle_vertex_count: triangle_vertex_count as i32,
            line_vao,
            line_vertex_count: line_vertex_count as i32,
            program,
            scale: 1.0,
            translation: (0.0, 0.0),
            rotation: 0,
        };
        out
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

    fn compute_transform_matrix(&self, z_adjustment: f32) {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::new(
            self.scale, 0.0, 0.0, self.translation.0,
            0.0, self.scale, 0.0, self.translation.1,
            0.0, 0.0, self.scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = GraphicsEngine::compute_isometric_matrix(self.rotation, z_adjustment);

        let composite_matrix = scale_matrix * isometric_matrix;
        self.program.load_matrix("transform", composite_matrix);
    }

    pub fn scale(&mut self, delta: f32) {
        self.scale = self.scale * delta;
    }

    pub fn translate(&mut self, delta: (f32, f32)) {
        self.translation = (self.translation.0 - delta.0, self.translation.1 + delta.1);
    }

    fn compute_isometric_matrix(angle: usize, z_adjustment: f32) -> na::Matrix4<f32> {
        let c = GraphicsEngine::ISOMETRIC_COEFFS[angle].0;
        let s = GraphicsEngine::ISOMETRIC_COEFFS[angle].1;
        na::Matrix4::new(
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 16.0, 0.0,
            0.0, 0.0, -1.0 + z_adjustment, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate(&mut self, rotations: usize) {
        self.rotation = (self.rotation + rotations) % GraphicsEngine::ISOMETRIC_COEFFS.len();
    }

    pub fn draw(&self) {
        unsafe {
            self.compute_transform_matrix(0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.triangle_vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,     // mode
                0,                 // starting index in the enabled arrays
                self.triangle_vertex_count, // number of indices to be rendered
            );
            self.triangle_vao.unbind();
            self.compute_transform_matrix(-0.001);
            self.line_vao.bind();
            gl::DrawArrays(
                gl::LINES,              // mode
                0,                      // starting index in the enabled arrays
                self.line_vertex_count, // number of indices to be rendered
            );
            self.line_vao.unbind();
        }
    }
}

struct VBO {
    id: gl::types::GLuint,
}

impl VBO {
    pub fn new<T: BufferType>(vao: &VAO<T>) -> VBO {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            let out = VBO { id };
            out.set_vao(vao);
            out
        }
    }

    unsafe fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
    }

    unsafe fn unbind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    pub fn load(&self, vertices: Vec<f32>) {
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

    pub unsafe fn set_vao<T: BufferType>(&self, vao: &VAO<T>) {
        self.bind();
        vao.set();
        self.unbind();
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