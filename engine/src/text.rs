use std::path::PathBuf;
use std::sync::Arc;
use std::cell::RefCell;
use std::collections::HashMap;
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



pub struct System {
    library: ft::Library,
    cache: RefCell<GlyphCache>,
}


impl System {
    pub fn new() -> System {
        System {
            library: ft::Library::init().unwrap(),
            cache: RefCell::new(GlyphCache::new()),
        }
    }
}


#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Key {
    character: char,
    font_size: u32,
    font_path: PathBuf,
}


pub type GlyphCache = HashMap<Key, Arc<Glyph>>;


pub struct Face<'a> {
    ft_face: ft::Face<'a>,
    load_flag: ft::face::LoadFlag,
}


impl<'a> Face<'a> {
    pub fn new(system: &'a System, font: PathBuf) -> Face<'a> {
        let face = system.library.new_face(font, 0).unwrap();
        Face {
            load_flag: ft::face::RENDER,
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
    pub width: u32,
    pub height: u32,
    pub linegap: i32,
    pub padding: i32,
}


impl TextStyle {
    pub fn new(font: PathBuf) -> TextStyle {
        TextStyle {
            font: font,
            color: Color::black(),
            font_size: 18,
            underline: false,
            width: 0,
            height: 0,
            linegap: 0,
            padding: 10,
        }
    }
}



pub fn load(system: &System, style: &TextStyle, hidpi_factor: f32, text: &String)
    -> Vec<(char, Arc<Glyph>)>
{
    let mut cache = system.cache.borrow_mut();
    let face = Face::new(&system, style.font.clone());
    let font_size = (style.font_size as f32 * hidpi_factor) as u32;
    face.set_size(font_size);
    let mut result = Vec::with_capacity(text.nfc().count());
    for c in text.nfc() {
        let key = Key {
            character: c,
            font_path: style.font.clone(),
            font_size: font_size,
        };
        if !cache.contains_key(&key) {
            let glyph = Arc::new(Glyph::new(&face, c));
            cache.insert(key.clone(), glyph.clone());
            result.push((c, glyph))
        }
        else {
            result.push((c, cache[&key].clone()))
        }
    }
    return result;
}


pub fn draw(style: TextStyle, hidpi_factor: f32, glyphs: Vec<(char, Arc<Glyph>)>)
    -> Canvas
{
    macro_rules! scale (
        ($x:expr) => (
            ($x as f32 * hidpi_factor) as i32
        )
    );
    let mut ascent = 0;
    let mut descent = 0;

    for &(_, ref glyph) in &glyphs {
        let now_ascent = glyph.bearing.y;
        if now_ascent > ascent { ascent = now_ascent }
        let now_descent = -(glyph.bounding.y - now_ascent);
        if now_descent < descent { descent = now_descent }
    }
    let glyph_height = ascent - descent;

    let linegap = scale!(style.linegap);
    let padding = scale!(style.padding);
    let mut width = scale!(style.width);
    let mut height = scale!(style.height);
    let auto_width = width == 0;
    let auto_height = height == 0;

    // Compute canvas size
    if auto_width || auto_height {
        let mut w = padding;
        let mut h = ascent+padding;

        for &(c, ref glyph) in &glyphs {
            let out_box = !auto_width && (w + glyph.advance.x + padding >= width);
            if out_box || c == '\n' {
                if auto_width && w >= width { width = w+padding+1 }
                w = padding;
                h += glyph_height + linegap;
            }
            w += glyph.advance.x;
        }
        h += -descent + padding;
        if auto_width && w >= width { width = w+padding+1 }
        if auto_height { height = h+1 }
    }

    // Typesetting
    let mut canvas = Canvas::new(width as usize, height as usize);
    let mut pen = na![padding, ascent+padding];

    for (c, glyph) in glyphs {
        // Wrap
        let out_box = pen.x + padding + glyph.advance.x >= width as i32;
        if out_box || c == '\n' {
            pen.x = padding;
            pen.y += glyph_height + linegap;
        }

        // ASCII control character
        if (c as usize) < 32 { continue }

        // Draw
        pen.x += glyph.bearing.x;
        for i in 0..glyph.data.len() {
            let line = &glyph.data[i];
            for j in 0..line.len() {
                let x = pen.x+j as i32;
                let y = pen.y-glyph.bearing.y+i as i32;

                let value = line[j];
                let color = style.color.alpha(value);

                if x < width as i32 || y < height as i32 || value > 0.1 {
                    canvas[(x, y)] = color;
                }
            }
        }
        pen.x += glyph.advance.x - glyph.bearing.x;
    }

    return canvas.factor(hidpi_factor);
}
