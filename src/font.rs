use std::fs::File;
use std::io::Read;
use ::graphics::texture::Texture;

#[derive(Clone, Copy)]
pub struct Glyph {
    pub character: char,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: i32,
}

impl Glyph {
    fn from_line(line: &str) -> Glyph {
        let mut columns: Vec<&str> = line.split(",").collect();
        let mut id: usize = columns[0].parse().unwrap();
        Glyph {
            character: columns[0].parse().unwrap(),
            x: columns[1].parse().unwrap(),
            y: columns[2].parse().unwrap(),
            width: columns[3].parse().unwrap(),
            height: columns[4].parse().unwrap(),
            xoffset: columns[5].parse().unwrap(),
            yoffset: columns[6].parse().unwrap(),
            xadvance: columns[7].parse().unwrap(),
        }
    }

    pub fn from_csv(file_name: &str) -> [Option<Glyph>; 256] {
        let mut file = File::open(file_name).expect(&format!("Font file {} not found", file_name));
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect(&format!("Failed to read font file {}", file_name));

        let mut glyphs = [None; 256];

        for line in contents.split("\n") {
            let glyph = Glyph::from_line(line);
            glyphs[glyph.character as usize] = Some(glyph);
        };

        glyphs
    }
}

pub struct Font {
    glyphs: [Option<Glyph>; 256],
    texture: Texture,
}

impl Font {

    pub fn from_csv_and_texture(csv_file_name: &str, texture: Texture) -> Font {
        Font {
            glyphs: Glyph::from_csv(csv_file_name),
            texture
        }
    }
    
}