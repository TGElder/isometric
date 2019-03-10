use super::super::engine::Color;
use utils::float_ordering;
use std::f32;

pub trait SquareColoring {
    fn get_colors(&self, points: [na::Vector3<f32>; 4]) -> [Color; 4];
}

pub struct AltitudeColoring {
    max_height: f32,
}

impl AltitudeColoring {
    pub fn new(heights: & na::DMatrix<f32>) -> AltitudeColoring {
        let max_height = heights.iter().max_by(float_ordering).unwrap();
        AltitudeColoring{max_height: *max_height}
    }
}

impl SquareColoring for AltitudeColoring {
    fn get_colors(&self, points: [na::Vector3<f32>; 4]) -> [Color; 4] {
        let get_color = |point: na::Vector3<f32>| {
            let color = (point.z / (self.max_height * 2.0)) + 0.5;
            Color::new(color, color, color, 1.0)
        };
        [
            get_color(points[0]),
            get_color(points[1]),
            get_color(points[2]),
            get_color(points[3]),
        ]
    }
}

pub struct AngleColoring {
    light_direction: na::Vector3<f32>,
}

impl AngleColoring {
    pub fn new(light_direction: na::Vector3<f32>) -> AngleColoring {
        AngleColoring{light_direction}
    }
}

impl SquareColoring for AngleColoring {
    fn get_colors(&self, points: [na::Vector3<f32>; 4]) -> [Color; 4] {
        let u = points[0] - points[2];
        let v = points[1] - points[3];
        let normal = u.cross(&v);
        let angle: f32 = na::Matrix::angle(&normal, &self.light_direction);
        let color = angle / f32::consts::PI;
        let color = Color::new(color, color, color, 1.0);
        [color; 4]
    }
}

