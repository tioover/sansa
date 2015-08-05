use std::path::PathBuf;
use std::collections::HashMap;
use std::cell::RefCell;
use freetype as ft;
use nalgebra::Vec2;
use unicode_normalization::UnicodeNormalization;
use color::Color;
use canvas::Canvas;

macro_rules! cast (
    ($x:expr) => (
        ($x >> 6) as i32
    )
);


pub type Key = (PathBuf, u32, char);

pub struct System {
    library: ft::Library,
    pub cache: RefCell<HashMap<Key, Glyph>>,
}


impl System {
    pub fn new() -> System {
        System {
            library: ft::Library::init().unwrap(),
            cache: RefCell::new(HashMap::new()),
        }
    }
}


pub struct Face<'a> {
    ft_face: ft::Face<'a>,
    load_flag: ft::face::LoadFlag,
    height: i32,
    ascender: i32,
}


impl<'a> Face<'a> {
    pub fn new(system: &'a System, font: PathBuf) -> Face<'a> {
        let face = system.library.new_face(font, 0).unwrap();
        Face {
            load_flag: ft::face::RENDER,
            height: cast!(face.height()),
            ascender: cast!(face.ascender()),
            ft_face: face,
        }
    }

    pub fn set_size(&self, size: u32) {
        self.ft_face.set_pixel_sizes(size, 0).unwrap();
    }
}


#[derive(Clone)]
pub struct Glyph {
    data: Vec<Vec<f32>>,
    advance: Vec2<i32>,
    bearing: Vec2<i32>,
    bounding: Vec2<i32>,
}


impl Glyph {
    pub fn new<'a>(face: &Face<'a>, c: char) -> Glyph {
        face.ft_face.load_char(c as usize, face.load_flag).unwrap();
        let glyph = face.ft_face.glyph();
        let metrics = glyph.metrics();
        let advance = na![cast!(metrics.horiAdvance), cast!(metrics.vertAdvance)];
        let bearing = na![cast!(metrics.horiBearingX), cast!(metrics.horiBearingY)];
        let bounding = na![cast!(metrics.width), cast!(metrics.height)];
        let bitmap = glyph.bitmap();
        let row = bitmap.rows() as usize;
        let width = bitmap.width() as usize;
        let buffer = bitmap.buffer();
        let mut data = Vec::with_capacity(row);
        for i in 0..row {
            let mut line = Vec::with_capacity(width);
            for j in 0..width {
                line.push(buffer[i*width+j] as f32 / u8::max_value() as f32)
            }
            data.push(line);
        }
        Glyph {
            data: data,
            advance: advance,
            bearing: bearing,
            bounding: bounding,
        }
    }
}


#[derive(Clone)]
pub struct TextStyle {
    pub font: PathBuf,
    pub color: Color,
    pub font_size: u32,
    pub underline: bool,
    pub box_size: (u32, u32),
    pub linegap: i32,
    pub padding: i32,
}


impl TextStyle {
    pub fn new(font: PathBuf, box_size: (u32, u32)) -> TextStyle {
        TextStyle {
            font: font,
            color: Color::black(),
            font_size: 18,
            underline: false,
            box_size: box_size,
            linegap: 4,
            padding: 10,
        }
    }
}


pub fn draw(system: &System, style: &TextStyle, hidpi_factor: f32, text: &String)
        -> Canvas {
    macro_rules! scale (
        ($x:expr) => (
            ($x as f32 * hidpi_factor) as i32
        )
    );

    let face = Face::new(system, style.font.clone());
    let mut cache = system.cache.borrow_mut();
    let font_size = scale!(style.font_size) as u32;
    face.set_size(font_size);
    let chars: Vec<char> = text.nfc().collect();

    // Load
    let keys: Vec<Key> = chars.iter()
        .map(|c| (style.font.clone(), font_size, *c)).collect();
    for key in keys.iter() {
        let c = key.2;
        if !cache.contains_key(key) {
            let glyph = Glyph::new(&face, c);
            cache.insert(key.clone(), glyph);
        }
    }
    let glyphs: Vec<_> = keys.iter().map(|key| cache.get(&key).unwrap()).collect();

    // Typesetting
    let padding = scale!(style.padding);
    let (width, height) = (scale!(style.box_size.0), scale!(style.box_size.1));

    let mut canvas = Canvas::new(width as usize, height as usize);
    let mut pen = na![padding, padding + face.ascender];

    for index in 0..chars.len() {
        let c = chars[index];
        let glyph = glyphs[index];

        // Wrap
        let out_box = pen.x + glyph.advance.x + padding >= width as i32;
        if out_box || c == '\n' {
            pen.x = padding;
            pen.y += face.height + scale!(style.linegap);
        }

        // Draw
        pen.x += glyph.bearing.x;
        for i in 0..glyph.data.len() {
            let line = &glyph.data[i];
            for j in 0..line.len() {
                let value = line[j];
                let color = style.color.alpha(value);
                let x = pen.x+j as i32;
                let y = pen.y-glyph.bearing.y+i as i32;
                if x < width as i32 || y < height as i32 || value > 0.1 {
                    canvas[(x, y)] = color;
                }
            }
        }
        pen.x += glyph.advance.x - glyph.bearing.x;
    }

    return canvas.factor(hidpi_factor);
}
