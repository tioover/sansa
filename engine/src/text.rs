use std::path::PathBuf;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
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
}


impl System {
    pub fn new() -> System {
        System {
            library: ft::Library::init().unwrap(),
        }
    }
}


#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Key {
    character: char,
    font_size: u32,
    font_path: PathBuf,
}

pub type CacheMap = HashMap<Key, Arc<Glyph>>;


#[derive(Clone)]
pub struct GlyphCache(Arc<Mutex<CacheMap>>);


impl GlyphCache {
    pub fn new() -> GlyphCache {
        GlyphCache(Arc::new(Mutex::new(HashMap::new())))
    }
}


impl Deref for GlyphCache {
    type Target = Arc<Mutex<CacheMap>>;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        let &GlyphCache(ref x) = self;
        x
    }
}


unsafe impl Sync for GlyphCache {}
unsafe impl Send for GlyphCache {}


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
        let size = size as isize * 64;
        let dpi = 72;
        self.ft_face.set_char_size(size, size, dpi, dpi).unwrap();
    }
}


#[derive(Clone)]
pub struct Glyph {
    buffer: Vec<f32>,
    pub bitmap_width: usize,
    pub bitmap_rows: usize,
    pub advance: Vec2<i32>,
    pub bearing: Vec2<i32>,
    pub bounding: Vec2<i32>,
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
        let buffer = bitmap.buffer();
        let mut data = Vec::with_capacity(buffer.len());
        let max = u8::max_value() as f32;
        for v in buffer {
            data.push(*v as f32 / max);
        }
        Glyph {
            buffer: data,
            bitmap_width: bitmap.width() as usize,
            bitmap_rows: bitmap.rows() as usize,
            advance: advance,
            bearing: bearing,
            bounding: bounding,
        }
    }

    pub fn get_line(&self, i: usize) -> &[f32] {
        let w = self.bitmap_width;
        &self.buffer[i*w..(i+1)*w]
    }
}


#[derive(Clone)]
pub struct TextStyle {
    pub font: PathBuf,
    pub color: Color,
    pub font_size: u32,
    pub underline: bool,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub linegap: i32,
    pub padding: i32,
    pub hidpi_factor: f32,
}


impl TextStyle {
    pub fn new(font: PathBuf) -> TextStyle {
        TextStyle {
            font: font,
            color: Color::black(),
            font_size: 18,
            underline: false,
            width: None,
            height: None,
            linegap: 0,
            padding: 10,
            hidpi_factor: 1.0,
        }
    }

    pub fn factor(self, hidpi_factor: f32) -> TextStyle {
        macro_rules! scale (($x:expr) => (
            $x as f32 * hidpi_factor
        ));
        if self.hidpi_factor != 1.0 {
            return self.inverse_factor().factor(hidpi_factor);
        }

        TextStyle {
            font: self.font,
            color: self.color,
            font_size: scale!(self.font_size) as u32,
            underline: self.underline,
            width: self.width.map(|x| scale!(x) as usize),
            height: self.height.map(|x| scale!(x) as usize),
            linegap: scale!(self.linegap) as i32,
            padding: scale!(self.padding) as i32,
            hidpi_factor: hidpi_factor
        }
    }

    fn inverse_factor(self) -> TextStyle {
        macro_rules! scale (($x:expr) => (
            $x as f32 / self.hidpi_factor
        ));

        TextStyle {
            width: self.width.map(|x| scale!(x) as usize),
            height: self.height.map(|x| scale!(x) as usize),
            font: self.font,
            color: self.color,
            font_size: scale!(self.font_size) as u32,
            underline: self.underline,
            linegap: scale!(self.linegap) as i32,
            padding: scale!(self.padding) as i32,
            hidpi_factor: 1.0,
        }
    }
}



pub fn load(cache: &mut CacheMap, style: &TextStyle, text: &String)
    -> Vec<(char, Arc<Glyph>)>
{
    let system = System::new();
    let face = Face::new(&system, style.font.clone());
    face.set_size(style.font_size);
    let mut result = Vec::with_capacity(text.nfc().count());
    for c in text.nfc() {
        let key = Key {
            character: c,
            font_path: style.font.clone(),
            font_size: style.font_size,
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


pub fn draw(style: &TextStyle, glyphs: Vec<(char, &Glyph)>)
    -> Canvas
{
    let mut ascent = 0;
    let mut descent = 0;

    for &(_, ref glyph) in &glyphs {
        let now_ascent = glyph.bearing.y;
        if now_ascent > ascent { ascent = now_ascent }
        let now_descent = -(glyph.bounding.y - now_ascent);
        if now_descent < descent { descent = now_descent }
    }
    let glyph_height = ascent - descent;

    let padding = style.padding;
    let auto_width = style.width.is_none();
    let auto_height = style.height.is_none();

    // Compute canvas size
    let (width, height) = if auto_width || auto_height {
        let mut width = if auto_width { 0 } else { style.width.unwrap() } as i32;
        let mut height = if auto_height { 0 } else { style.height.unwrap() } as i32;
        let mut w = padding;
        let mut h = ascent+padding;

        for &(c, ref glyph) in &glyphs {
            let out_box = !auto_width && (w + glyph.advance.x + padding >= width);
            if out_box || c == '\n' {
                if auto_width && w >= width { width = w+padding+1 }
                w = padding;
                h += glyph_height + style.linegap;
            }
            w += glyph.advance.x;
        }
        h += -descent + padding;
        if auto_width && w >= width { width = w+padding+1 }
        if auto_height { height = h+1 }
        (width, height)
    } else {
        (style.width.unwrap() as i32, style.height.unwrap() as i32)
    };

    // Typesetting
    let mut canvas = Canvas::new(width as usize, height as usize);
    let mut pen = na![padding, ascent+padding];

    for (c, glyph) in glyphs {
        // Wrap
        let out_box = pen.x + padding + glyph.advance.x >= width as i32;
        if out_box || c == '\n' {
            pen.x = padding;
            pen.y += glyph_height + style.linegap;
        }

        // ASCII control character
        if (c as usize) < 32 { continue }

        // Draw
        pen.x += glyph.bearing.x;
        for i in 0..glyph.bitmap_rows {
            let glyph_line = glyph.get_line(i);
            let draw_y = pen.y-glyph.bearing.y+i as i32;

            if draw_y >= canvas.height as i32 || draw_y < 0 {
                continue
            }

            let mut canvas_line = canvas.line_mut(draw_y as usize);

            for j in 0..glyph_line.len() {
                let value = glyph_line[j];
                let n = pen.x + j as i32;

                if n < width && n >= 0 && value > 0.025 {
                    canvas_line[n as usize] = style.color.alpha(value)
                }
            }
        }
        pen.x += glyph.advance.x - glyph.bearing.x;
    }

    return canvas.factor(style.hidpi_factor);
}
