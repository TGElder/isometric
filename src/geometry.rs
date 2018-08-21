#[derive(Debug)]
pub struct Coord2D {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub struct Coord3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Coord2D {
    pub fn new(x: f64, y: f64) -> Coord2D {
        return Coord2D{x, y};
    }
}

impl Coord3D {

    pub fn new(x: f64, y: f64, z: f64) -> Coord3D {
        return Coord3D{x, y, z};
    }

    pub fn to_isometric(self: Coord3D) -> Coord2D {
        let iso_x = self.x - self.y;
        let iso_y = ((self.x + self.y) / 2.0) - (self.z / 10.0);
        //let iso_x = self.x;
        //let iso_y = self.y;
        Coord2D::new(iso_x, iso_y)
    }

}