use std::path::PathBuf;
use glium::{Display, Frame};
use ui::{GlyphCache, Text};
use context::Context;
use sprite::Sprite;
use math::Mat;
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
    }

    pub fn render_sprites(&self, target: &mut Frame, sprites: Vec<&Sprite>, parent: Mat) {
        ::sprite::render(&self.context, target, sprites, parent)
    }
}

