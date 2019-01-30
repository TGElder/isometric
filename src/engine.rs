extern crate glutin;

use self::glutin::GlContext;
use std::ffi::CString;
use shader::Shader;
use program::Program;
use std::marker::PhantomData;

pub struct IsometricEngine {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    vao: VAO<TriangleBuffer>,
}

impl IsometricEngine {

    const GL_VERSION: glutin::GlRequest = glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3));

    pub fn new(title: &str, width: u32, height: u32) -> IsometricEngine {
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

        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::Viewport(0, 0, width as i32, height as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
        
        let vao: VAO<TriangleBuffer> = VAO::new();
        let vbo: VBO = VBO::new(&vao);
        
        let vertices: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            -0.5, 0.5, 0.0,
            -0.5, 0.5, 0.0,
            0.5, -0.5, 0.0,
            0.5, 0.5, 0.0
        ];

        vbo.load(vertices);

        let program = IsometricEngine::load_program();
        
        unsafe {
            let mvp_location = gl::GetUniformLocation(program.id(), CString::new("MVP").unwrap().as_ptr() as *const gl::types::GLchar);
            let proj_matrix: na::Matrix3<f32> = na::Matrix3::new(
                1.0, -1.0, 0.0,
                0.5, 0.5, -1.0,
                0.5, 0.5, -1.0
            );
            let proj_ptr = proj_matrix.as_slice().as_ptr();
            gl::UniformMatrix3fv(mvp_location, 1, gl::FALSE, proj_ptr);
        }

        IsometricEngine{
            events_loop,
            window: gl_window,
            vao,
        }
    }

    pub fn run(&mut self) {
        let mut events_loop = &mut self.events_loop;
        let window = &self.window;
        let mut running = true;
        while running {
            events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                        },
                        _ => ()
                    },
                    _ => ()
                }
            });

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                self.vao.bind();
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // starting index in the enabled arrays
                    6 // number of indices to be rendered
                );
                self.vao.unbind();
            }

            self.window.swap_buffers().unwrap();
        }
    }

    pub fn load_program() -> Program {
        let vertex_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.vert")).unwrap(), //TODO don't like exposing CString
            gl::VERTEX_SHADER
        ).unwrap();

        let fragment_shader = Shader::from_source(
            &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
            gl::FRAGMENT_SHADER
        ).unwrap();

        let shader_program = Program::from_shaders(
            &[vertex_shader, fragment_shader]
        ).unwrap();

        shader_program.set_used();

        return shader_program;
    }

}

struct VBO {
    id: gl::types::GLuint,
}

impl VBO {
    
    pub fn new <T: BufferType> (vao: &VAO<T>) -> VBO {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            let out = VBO {
                id
            };
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

    pub unsafe fn set_vao <T: BufferType> (&self, vao: &VAO<T>) {
        self.bind();
        vao.set();
        self.unbind();
    }

}

struct VAO<T: BufferType> {
    id: gl::types::GLuint,
    buffer_type: PhantomData<T>,
}

impl <T: BufferType> VAO<T> {
    
    pub fn new() -> VAO<T> {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO {
            id,
            buffer_type: PhantomData
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
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );
        }
    }
}

