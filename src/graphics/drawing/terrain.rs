use super::super::engine::Drawing;
use super::super::vertex_objects::{VBO, Vertex, ColoredVertex};

pub struct TerrainDrawing {
    terrain_triangles: VBO<ColoredVertex>,
    terrain_lines: VBO<Vertex>,
}

impl Drawing for TerrainDrawing {
    fn draw(&self) {
        self.terrain_triangles.draw();
        self.terrain_lines.draw();
    }
}

impl TerrainDrawing {
    pub fn from_heights(heights: &na::DMatrix<f32>) -> TerrainDrawing {
        let width = heights.shape().0;
        let height = heights.shape().1;
        let mut triangle_vertices: Vec<f32> = Vec::with_capacity(width * height * 36);
        let mut line_vertices: Vec<f32> = Vec::with_capacity(width * height * 24);
        for x in 0..(width - 1) {
            for y in 0..(height - 1) {
                
                let a = (x as f32, y as f32, heights[(x, y)]);
                let a = (a.0, a.1, a.2, (a.2 / 2.0) + 0.5);
                let b = (x as f32 + 1.0, y as f32, heights[(x + 1, y)]);
                let b = (b.0, b.1, b.2, (b.2 / 2.0) + 0.5);
                let c = (x as f32 + 1.0, y as f32 + 1.0, heights[(x + 1, y + 1)]);
                let c = (c.0, c.1, c.2, (c.2 / 2.0) + 0.5);
                let d = (x as f32, y as f32 + 1.0, heights[(x, y + 1)]);
                let d = (d.0, d.1, d.2, (d.2 / 2.0) + 0.5);

                triangle_vertices.extend([
                    a.0, a.1, a.2, a.3, a.3, a.3,
                    d.0, d.1, d.2, d.3, d.3, d.3,
                    c.0, c.1, c.2, c.3, c.3, c.3,
                    a.0, a.1, a.2, a.3, a.3, a.3,
                    c.0, c.1, c.2, c.3, c.3, c.3,
                    b.0, b.1, b.2, b.3, b.3, b.3
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

        let mut out = TerrainDrawing{
            terrain_triangles: VBO::new(gl::TRIANGLES),
            terrain_lines: VBO::new(gl::LINES),
        };

        out.terrain_triangles.load(triangle_vertices);
        out.terrain_lines.load(line_vertices);

        out
    }
}
