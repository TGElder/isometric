extern crate glutin;

use self::glutin::GlContext;
use program::Program;
use shader::Shader;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    graphics: GraphicsEngine,
    drag_controller: DragController,
}

impl IsometricEngine {
    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32, triangle_vertices: Vec<f32>, line_vertices: Vec<f32>) -> IsometricEngine {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width as f64, height as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(IsometricEngine::GL_VERSION)
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }

        let graphics = GraphicsEngine::new(&gl_window, triangle_vertices, line_vertices);

        IsometricEngine {
            events_loop,
            window: gl_window,
            graphics,
            drag_controller: DragController::new(),
        }
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            let graphics = &mut self.graphics;
            let events_loop = &mut self.events_loop;
            let window = &self.window;
            let drag_controller = &mut self.drag_controller;
            let window_size = self.window.window().get_inner_size().unwrap();
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                        }
                        glutin::WindowEvent::MouseWheel { delta, .. } => match delta {
                            glutin::MouseScrollDelta::LineDelta(_, d) if d > 0.0 => {
                                graphics.scale(2.0)
                            }
                            glutin::MouseScrollDelta::LineDelta(_, d) if d < 0.0 => {
                                graphics.scale(0.5)
                            }
                            _ => (),
                        },
                        glutin::WindowEvent::KeyboardInput{ input, .. } => match input {
                            glutin::KeyboardInput{
                                virtual_keycode: Some(glutin::VirtualKeyCode::Space), 
                                state: glutin::ElementState::Pressed,
                                modifiers, .. } => match modifiers {
                                    glutin::ModifiersState{ shift: true, .. } => graphics.rotate(3),
                                    _ => graphics.rotate(1),
                            },
                            _ => (),
                        }
                        _ => (),
                    };
                    if let Some((x, y)) = drag_controller.handle(event) {
                        graphics.translate((
                            (x / (window_size.width / 2.0)) as f32,
                            (y / (window_size.height / 2.0)) as f32,
                        ));
                    }
                }
                _ => (),
            });

            graphics.draw();

            self.window.swap_buffers().unwrap();
        }
    }
}

pub struct DragController {
    dragging: bool,
    last_pos: Option<glutin::dpi::LogicalPosition>,
}

impl DragController {
    fn new() -> DragController {
        DragController {
            dragging: false,
            last_pos: None,
        }
    }

    fn handle(&mut self, event: glutin::WindowEvent) -> Option<(f64, f64)> {
        match event {
            glutin::WindowEvent::MouseInput {
                state,
                button: glutin::MouseButton::Left,
                ..
            } => {
                match state {
                    glutin::ElementState::Pressed => self.dragging = true,
                    glutin::ElementState::Released => self.dragging = false,
                };
                None
            }
            glutin::WindowEvent::CursorMoved { position, .. } => {
                let out = if self.dragging {
                    if let Some(last_pos) = self.last_pos {
                        Some((last_pos.x - position.x, last_pos.y - position.y))
                    } else {
                        None
                    }
                } else {
                    None
                };
                self.last_pos = Some(position);
                out
            }
            _ => None,
        }
    }
}

pub struct GraphicsEngine {
    triangle_vao: VAO<TriangleBuffer>,
    triangle_vertex_count: i32,
    line_vao: VAO<TriangleBuffer>,
    line_vertex_count: i32,
    program: Program,
    scale: f32,
    translation: (f32, f32),
    rotation: usize,
}

impl GraphicsEngine {

    const ISOMETRIC_COEFFS: [(f32, f32); 4] = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];

    pub fn new(gl_window: &glutin::GlWindow, triangle_vertices: Vec<f32>, line_vertices: Vec<f32>) -> GraphicsEngine {
        let window_size: (u32, u32) = gl_window.window().get_inner_size().unwrap().into();

        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        }

        let triangle_vao: VAO<TriangleBuffer> = VAO::new();
        let triangle_vbo: VBO = VBO::new(&triangle_vao);
        let triangle_vertex_count = triangle_vertices.len();
        triangle_vbo.load(triangle_vertices);

        let line_vao: VAO<TriangleBuffer> = VAO::new();
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
        out.compute_transform_matrix();
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

    fn compute_transform_matrix(&self) {
        let scale_matrix: na::Matrix4<f32> = na::Matrix4::new(
            self.scale, 0.0, 0.0, self.translation.0,
            0.0, self.scale, 0.0, self.translation.1,
            0.0, 0.0, self.scale, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        let isometric_matrix = GraphicsEngine::compute_isometric_matrix(self.rotation);

        let composite_matrix = scale_matrix * isometric_matrix;
        self.program.load_matrix("transform", composite_matrix);
    }

    pub fn scale(&mut self, delta: f32) {
        self.scale = self.scale * delta;
        self.compute_transform_matrix();
    }

    pub fn translate(&mut self, delta: (f32, f32)) {
        self.translation = (self.translation.0 - delta.0, self.translation.1 + delta.1);
        self.compute_transform_matrix();
    }

    fn compute_isometric_matrix(angle: usize) -> na::Matrix4<f32> {
        let c = GraphicsEngine::ISOMETRIC_COEFFS[angle].0;
        let s = GraphicsEngine::ISOMETRIC_COEFFS[angle].1;
        na::Matrix4::new(
            c, -s, 0.0, 0.0,
            -s / 2.0, -c / 2.0, 16.0, 0.0,
            0.0, 0.0, -1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotate(&mut self, rotations: usize) {
        self.rotation = (self.rotation + rotations) % GraphicsEngine::ISOMETRIC_COEFFS.len();
        self.compute_transform_matrix();
    }

    pub fn draw(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.triangle_vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,     // mode
                0,                 // starting index in the enabled arrays
                self.triangle_vertex_count, // number of indices to be rendered
            );
            self.triangle_vao.unbind();
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

struct TriangleBuffer {}

impl BufferType for TriangleBuffer {
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