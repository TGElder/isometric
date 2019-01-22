mod utils;
mod engine;
mod shader;
mod program;

use engine::IsometricEngine;

fn main() {
    
    let mut engine = IsometricEngine::new("Isometric", 512, 512);

    
    engine.run();
}

