mod engine;
mod shader;

use engine::IsometricEngine;

use std::ffi::{CString, CStr};
use shader::Shader;

fn main() {
    
    let mut engine = IsometricEngine::new("Isometric", 512, 512);
    let shader = Shader::from_source(
        &CString::new(include_str!("shaders/triangle.vert")).unwrap(),
        gl::VERTEX_SHADER
    ).unwrap();

    
    // engine.run();
}