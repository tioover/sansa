use na;
use na::Vec2;
use std::rc::Rc;
use std::ops::{Index, IndexMut};
use std::borrow::Cow;
use glium::Display;
use color::Color;
use ::{Image, Texture};
use sprite::Sprite;
use glium::texture::{RawImage2d, PixelValue};


pub type Buffer = Vec<Color>;


#[derive(Clone)]
pub struct Canvas {
    pub width: i32,
    pub height: i32,
    buffer: Buffer,
    fake: Color,
    hidpi_factor: f32,
}


impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas::with_color(width, height, Color::new(1.0, 1.0, 1.0, 0.0))
    }

    pub fn with_color(width: usize, height: usize, color: Color) -> Canvas {
        let len = height * width;
        let mut buffer = Vec::with_capacity(len);
        for _ in 0..len {
            buffer.push(color);
        }

        Canvas {
            width: width as i32,
            height: height as i32,
            buffer: buffer,
            fake: Color::black(),
            hidpi_factor: 1.0,
        }
    }

    pub fn resize(&mut self, (width, height): (i32, i32)) {
        assert!(width <= self.width && height <= self.height);
        self.width = width;
        self.height = height;
    }

    pub fn factor(self, f: f32) -> Canvas {
        Canvas { hidpi_factor: f, ..self }
    }

    pub fn rect(&mut self, a: Vec2<i32>, b: Vec2<i32>, color: Color) {
        let d = b - a;
        for y in 0..d.y {
            for x in 0..d.x {
                let i = (a.x+x, a.y+y);
                self[i] = self[i] + color;
            }
        }
    }

    pub fn into_sprite(self, display: &Display) -> Sprite {
        let size = Vec2::new(self.width as i32, self.height as i32);
        let texture = Texture::new(display, RawImage2d {
            data: Cow::Owned(self.buffer),
            width: self.width as u32,
            height: self.height as u32,
            format: Color::get_format(),
        });
        let image = Image::new(Rc::new(texture), size);
        let size: Vec2<f32> = na::cast(size);
        let size = na::cast(size/self.hidpi_factor);
        Sprite::new(size, image)
    }
}


pub type Pos = (i32, i32);


impl Index<Pos> for Canvas {
    type Output = Color;

    fn index<'a>(&'a self, (x, y): Pos) -> &'a Color {
        let y = self.height - y - 1;
        let index = (y * self.width + x) as usize;

        if index < self.buffer.len() {
            &self.buffer[index]
        } else {
            &self.fake
        }
    }
}



impl IndexMut<Pos> for Canvas {
    fn index_mut<'a>(&'a mut self, (x, y): Pos) -> &'a mut Color {
        let y = self.height - y - 1;
        let index = (y * self.width + x) as usize;

        if index < self.buffer.len() {
            &mut self.buffer[index]
        } else {
            &mut self.fake
        }
    }
}
