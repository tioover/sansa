use na;
use na::Vec2;
use std::rc::Rc;
use std::borrow::Cow;
use glium::Display;
use color::Color;
use ::{Image, Texture};
use sprite::Sprite;
use glium::texture::{RawImage2d, PixelValue};


pub type Buffer = Vec<Color>;


#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
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
            width: width,
            height: height,
            buffer: buffer,
            fake: Color::black(),
            hidpi_factor: 1.0,
        }
    }

    pub fn factor(self, f: f32) -> Canvas {
        Canvas { hidpi_factor: f, ..self }
    }

    pub fn into_sprite(self, display: &Display) -> Sprite {
        let texture = Texture::new(display, RawImage2d {
            data: Cow::Owned(self.buffer),
            width: self.width as u32,
            height: self.height as u32,
            format: Color::get_format(),
        });
        let size = Vec2::new(self.width as i32, self.height as i32);
        let image = Image::new(Rc::new(texture), size);
        let size: Vec2<f32> = na::cast(size);
        let size = na::cast(size/self.hidpi_factor);
        Sprite::new(size, image)
    }

    pub fn line<'a>(&'a mut self, n: usize) -> &'a [Color] {
        let n = self.height - n - 1;
        let w = self.width;
        &self.buffer[n*w..n*(w+1)]
    }

    pub fn line_mut<'a>(&'a mut self, n: usize) -> &'a mut [Color] {
        let n = self.height - n - 1;
        let w = self.width;
        &mut self.buffer[n*w..n*(w+1)]
    }
}
