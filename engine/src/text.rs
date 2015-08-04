use std::path::PathBuf;
use std::collections::HashMap;
use std::rc::Rc;
use std::ffi::OsStr;
use std::cell::RefCell;
use freetype as ft;
use freetype::{GlyphSlot, ffi};
use nalgebra::Vec2;
use unicode_normalization::UnicodeNormalization;
use color::Color;
use canvas::Canvas;



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

    pub fn face<P: AsRef<OsStr>>(&self, path: P) -> Face {
        let face = self.library.new_face(path, 0).unwrap();
        Face {
            load_flag: ft::face::RENDER,
            height: face.height() as i32,
            ascender: face.ascender() as i32,
            ft_face: face,
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
    pub fn load(&self, c: char) {
        self.ft_face.load_char(c as usize, self.load_flag).unwrap();
    }

    pub fn glyph(&self) -> Glyph {
        macro_rules! cast (
            ($x:expr, $y:expr) => (
                na![($x >> 6) as i32, ($y >> 6) as i32]
            )
        );
        let glyph = self.ft_face.glyph();
        let metrics = glyph.metrics();
        let advance = cast![metrics.horiAdvance, metrics.vertAdvance];
        let bearing = cast![metrics.horiBearingX, metrics.horiBearingY];
        let bounding = cast![metrics.width, metrics.height];
        let bitmap = glyph.bitmap();
        let row = bitmap.rows() as usize;
        let width = bitmap.width() as usize;
        let buffer = bitmap.buffer();
        let mut data = Vec::with_capacity(row);
        for i in 0..row {
            let mut line = Vec::with_capacity(width);
            for j in 0..width {
                line[j] = buffer[i*width+j] as f32 / u8::max_value() as f32;
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

    fn set_size(&self, size: u32) {
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


pub struct TextStyle {
    pub font: PathBuf,
    pub font_size: u32,
    pub underline: bool,
    pub box_size: Option<(u32, u32)>,
    pub linegap: i32,
}


pub fn draw<T>(system: &System, style: TextStyle, text: T) -> Canvas
        where T: ToString {
    let text = text.to_string();
    let face = system.face(style.font.clone());
    let mut cache = system.cache.borrow_mut();
    face.set_size(style.font_size);
    let chars: Vec<char> = text.nfc().collect();

    // Load
    let keys: Vec<Key> = chars.iter()
        .map(|c| (style.font.clone(), style.font_size, *c)).collect();
    for key in keys.iter() {
        if !cache.contains_key(key) {
            face.load(key.2);
            cache.insert(key.clone(), face.glyph());
        }
    }
    let glyphs: Vec<_> = keys.iter().map(|key| cache.get(&key).unwrap()).collect();

    // Typesetting
    let padding = style.font_size as i32/2;
    let (width, height) = match style.box_size {
        Some(s) => s,
        None => {
            let mut max_width = 0;
            let mut width = padding*2;
            let mut height = padding*2 + face.height;
            for index in 0..chars.len() {
                let c = chars[index];
                let glyph = glyphs[index];
                width += glyph.advance.x;
                if width > max_width { max_width = width }
                if c == '\n' {
                    height += face.height + style.linegap;
                }
            }
            (max_width as u32, height as u32)
        }
    };

    let mut canvas = Canvas::new(width as usize, height as usize);
    let mut pen = na![padding, padding + face.ascender];

    for index in 0..chars.len() {
        let c = chars[index];
        let glyph = glyphs[index];
    }

    return canvas;
}
