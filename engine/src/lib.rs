#![feature(box_syntax, box_patterns, slice_patterns, rc_weak)]
#[macro_use]
extern crate glium;
extern crate image as img;
extern crate num;
extern crate nalgebra as na;
extern crate time;
extern crate uuid;
extern crate freetype;
extern crate unicode_normalization;


pub mod canvas;
pub mod color;
pub mod event;
pub mod math;
pub mod resources;
pub mod sprite;
pub mod timer;
pub mod ui;
pub mod animation;
pub mod transform;
pub mod mesh;
pub mod context;
pub mod image;
pub mod camera;
pub mod renderable;
pub mod texture;

pub use glium::{Frame, Display};
pub use timer::{GlobalTimer, LocalTimer, Ms};
pub use context::Context;
pub use image::Image;
pub use renderable::{Renderable, render};
pub use sprite::Sprite;
pub use event::{Event, Update, update};
pub use ui::{GlyphCache, Glyph, Text, UI, UIBuilder};
pub use resources::Manager;
pub use transform::Transform;
pub use texture::Texture;

use std::path::PathBuf;
use glium::glutin::WindowBuilder;
use glium::DisplayBuild;



pub struct Engine<'display> {
    pub timer: GlobalTimer,
    pub display: &'display Display,
    pub context: Context<'display>,
    pub glyph_cache: GlyphCache,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: GlobalTimer::new(),
            display: display,
            context: Context::new(display),
            glyph_cache: GlyphCache::new(),
        }
    }

    pub fn text(&self, font: PathBuf) -> Text {
        Text::new(font, self.glyph_cache.clone())
            .hidpi_factor(self.context.hidpi_factor)
    }

    pub fn update(&mut self) {
        self.timer.update();
        self.context.update();
    }

    pub fn render_sprites(&self, sprites: Vec<&Sprite>) {
        sprite::render(&self.context, sprites)
    }
}


pub fn build_display(title: String, (width, height): (u32, u32)) -> Display {
    WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height)
        .with_vsync()
        .build_glium()
        .unwrap()
}


