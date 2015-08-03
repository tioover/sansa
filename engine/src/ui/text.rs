use freetype;
use freetype::{Library, GlyphSlot};
use unicode_normalization::UnicodeNormalization;
use na::Vec2;
use std::path::PathBuf;
use std::collections::HashMap;
use std::string::ToString;
use std::rc::Rc;
use std::cell::RefCell;
use glium::Display;
use sprite::Sprite;
use color::Color;
use canvas::Canvas;
use ui::WidgetBuilder;


#[derive(Clone)]
pub struct Label {
    underline: u8,
    size: u16,
    line_spacing: u16,
    padding: i32,
    hidpi_factor: f32,
    cache: GlyphCache,
    font: PathBuf,
    content: String,
    box_size: (u32, u32),
    color: Color,
    position: Vec2<f32>,
    anchor: Vec2<f32>,
}


impl Label {
    pub fn new(font_path: PathBuf, cache: GlyphCache, box_size: (u32, u32)) -> Label {
        Label {
            cache: cache,
            font: font_path,
            size: 18,
            line_spacing: 4,
            color: Color::black(),
            underline: 0,
            box_size: box_size,
            hidpi_factor: 1.0,
            padding: 10,
            content: String::new(),
            position: ::na::zero(),
            anchor: ::na::zero(),
        }
    }

    pub fn draw(&self) -> Canvas {
        draw(self)
    }

    pub fn content<T: ToString>(self, data: T) -> Label {
        Label { content: data.to_string(), ..self }
    }

    pub fn size(self, size: u16) -> Label {
        Label { size: size, ..self }
    }

    pub fn line_spacing(self, v: u16) -> Label {
        Label { line_spacing: v, ..self }
    }

    pub fn padding(self, padding: i32) -> Label {
        Label { padding: padding, ..self }
    }

    pub fn hidpi_factor(self, f: f32) -> Label {
        Label { hidpi_factor: f, ..self }
    }

    pub fn box_size(self, box_size: (u32, u32)) -> Label {
        Label { box_size: box_size, ..self }
    }

    pub fn anchor(self, center: Vec2<f32>) -> Label {
        Label { anchor: center, ..self }
    }

    pub fn position(self, position: Vec2<f32>) -> Label {
        Label { position: position, ..self }
    }

    pub fn underline(self, underline: u8) -> Label {
        Label { underline: underline, ..self }
    }

    pub fn color(self, color: Color) -> Label {
        Label { color: color, ..self }
    }
}


impl WidgetBuilder for Label {
    fn sprite(&self, display: &Display) -> Sprite {
        self.draw().into_sprite(display)
            .position(self.position)
            .anchor(self.anchor)
    }
}



#[derive(Clone)]
pub struct GlyphCache {
    map: Rc<RefCell<HashMap<(PathBuf, i32, char), Glyph>>>,
}


impl GlyphCache {
    pub fn new() -> GlyphCache {
        GlyphCache {
            map: Rc::new(RefCell::new(HashMap::new()))
        }
    }
}


#[derive(Clone)]
pub struct Glyph {
    bitmap: Vec<f32>,
    bitmap_size: (i32, i32),
    glyph_size: (i32, i32),
    left: i32,
    top: i32,
    right: i32,
}


impl Glyph {
    pub fn new(ft_glyph: &GlyphSlot) -> Glyph {
        let ft_bitmap = ft_glyph.bitmap();
        let ft_buffer = ft_bitmap.buffer();
        let mut bitmap: Vec<f32> = Vec::with_capacity(ft_buffer.len());
        for v in ft_buffer { bitmap.push(*v as f32 / 255.0) }

        let a = ft_bitmap.width();
        let b = ft_bitmap.rows();
        let advance = ft_glyph.advance();
        let w = (advance.x as i32) / 64;
        let h = (advance.y as i32) / 64;
        let left = ft_glyph.bitmap_left();
        let right = w - left - a;

        Glyph {
            bitmap: bitmap,
            bitmap_size: (a, b),
            glyph_size: (w, h),
            left: left,
            top: ft_glyph.bitmap_top(),
            right: right,
        }
    }
}

pub fn draw(style: &Label) -> Canvas {
    let library = Library::init().unwrap();
    let face = library.new_face(&style.font, 0).unwrap();
    let mut cache = style.cache.map.borrow_mut();

    let factor = style.hidpi_factor;
    macro_rules! scale {
        ($x:expr) => (($x as f32 * factor) as i32)
    }

    // font size
    let font_size = scale!(style.size);
    face.set_pixel_sizes(font_size as u32, 0)
        .unwrap();
    let chars: Vec<_> = style.content.nfc().collect();

    // load
    for c in chars.iter() {
        let key = (style.font.clone(), font_size, *c);
        if !cache.contains_key(&key) {
            face.load_char(key.2 as usize, freetype::face::RENDER).unwrap();
            let glyph = Glyph::new(face.glyph());
            cache.insert(key, glyph);
        }
    }

    let glyphs: Vec<_> = chars.iter().map(|c| {
        let key = (style.font.clone(), font_size, *c);
        cache.get(&key).unwrap()
    }).collect();


    // typesetting
    let line_spacing = scale!(style.line_spacing);
    let padding = scale!(style.padding);
    let (width, height) = (scale!(style.box_size.0), scale!(style.box_size.1));

    let mut canvas = Canvas::new(width as usize, height as usize)
            .factor(factor);
    let mut x: i32 = padding;
    let mut baseline = padding + font_size;

    macro_rules! draw_underline {() => (
        if style.underline > 0 {
            let size = scale!(style.underline);
            let shift = 0;
            canvas.rect(
                Vec2::new(padding, baseline+shift),
                Vec2::new(x, baseline+size+shift),
                Color::black(),
            );
        }
    )}

    for i in 0..chars.len() {
        let c = chars[i];
        let glyph = glyphs[i];
        let (w, h) = glyph.bitmap_size;

        // wrap
        if glyph.glyph_size.0 + x + padding >= width || c == '\n' {
            draw_underline!();
            x = padding;
            baseline += font_size + line_spacing * 2;
        }
        // space
        if c == ' ' { x += glyph.left + glyph.right }
        // skip
        if w == 0 || h == 0  { continue }

        // draw
        x += glyph.left;
        let y = baseline - glyph.top;
        for i in 0..h {
            for j in 0..w {
                let index = (i*w+j) as usize;
                let alpha = glyph.bitmap[index];
                let line = y+i;
                let  row = x+j;
                if line >= height || row >= width || alpha < 0.1 { continue }
                let index = (row, line);
                let origin = canvas[index];
                canvas[index] = origin + Color::black().alpha(alpha);
            }
        }
        x += w + glyph.right;
    }
    draw_underline!();
    return canvas;
}


