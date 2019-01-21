mod engine;

use engine::IsometricEngine;

fn main() {
    let mut engine = IsometricEngine::new("Isometric", 512.0, 512.0);
    engine.run();
}