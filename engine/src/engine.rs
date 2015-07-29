use std::path::PathBuf;
use glium::Display;
use ui::{GlyphCache, Text};
use context::Context;
use sprite::{Sprite, render};
use timer::ProgramTimer;

pub struct Engine<'display> {
    pub timer: ProgramTimer,
    pub display: &'display Display,
    pub context: Context<'display>,
    pub glyph_cache: GlyphCache,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: ProgramTimer::new(),
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
        render(&self.context, sprites)
    }
}

