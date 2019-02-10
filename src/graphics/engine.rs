use super::program::Program;
use super::shader::Shader;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct GraphicsEngine {
    terrain_triangles: VBO<ColoredVertex>,
    terrain_lines: VBO<Vertex>,
    viewport_size: na::Point2<u32>,
    program: Program,
    scale: na::Point2<f32>,
    translation: na::Point2<f32>,
    rotation: usize,
}

impl GraphicsEngine {

    const ISOMETRIC_COEFFS: [(f32, f32); 4] = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];

    pub fn new(viewport_size: na::Point2<u32>) -> GraphicsEngine {
        let mut terrain_triangles: VBO<ColoredVertex> = VBO::new(gl::TRIANGLES);
        let mut terrain_lines: VBO<Vertex> = VBO::new(gl::LINES);

        let program = GraphicsEngine::load_program();

        let mut out = GraphicsEngine {
            terrain_triangles,
            terrain_lines,
            viewport_size,
            program,
            scale: na::Point2::new(1.0, 1.0),
            translation: na::Point2::new(0.0, 0.0),
            rotation: 0,
        };
        out.set_viewport(viewport_size);
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
            self.scale.x, 0.0, 0.0, self.translation.x,
            0.0, self.scale.y, 0.0, self.translation.y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = GraphicsEngine::compute_isometric_matrix(self.rotation, z_adjustment);

        let composite_matrix = scale_matrix * isometric_matrix;
        self.program.load_matrix("transform", composite_matrix);
    }

    pub fn set_viewport(&mut self, viewport_size: na::Point2<u32>) {
        let scale = na::Point2::new(
            self.scale.x * ((self.viewport_size.x as f32) / (viewport_size.x as f32)),
            self.scale.y * ((self.viewport_size.y as f32) / (viewport_size.y as f32)));

        self.viewport_size = viewport_size;
        self.scale = scale;
        unsafe {
            gl::Viewport(0, 0, viewport_size.x as i32, viewport_size.y as i32);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }
    }

    pub fn scale(&mut self, delta: f32) {
        self.scale = self.scale * delta;
    }

    pub fn translate(&mut self, delta: na::Point2<f32>) {
        self.translation = na::Point2::new(self.translation.x - delta.x, self.translation.y + delta.y);
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
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.compute_transform_matrix(0.0);
            self.terrain_triangles.draw();
            self.compute_transform_matrix(-0.001);
            self.terrain_lines.draw();
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

    pub fn set_terrain_lines(&mut self, vertices: Vec<f32>) {
        self.terrain_triangles.load(vertices);
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