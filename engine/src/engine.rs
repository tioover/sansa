use std::sync::{Arc, Mutex};
use glium::Display;
use widget::Label;
use context::Context;
use timer::ProgramTimer;
use text;

pub struct Engine<'display> {
    pub timer: ProgramTimer,
    pub display: &'display Display,
    pub context: Context<'display>,
    pub glyph_cache: Arc<Mutex<text::GlyphCache>>,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: ProgramTimer::new(),
            display: display,
            context: Context::new(display),
            glyph_cache: Arc::new(Mutex::new(text::GlyphCache::new())),
        }
    }

    pub fn label<T: ToString>(&self, style: text::TextStyle, x: T) -> Label {
        let hidpi_factor = self.display.get_window().unwrap().hidpi_factor();
        Label::new(self.glyph_cache.clone(), style, hidpi_factor, x)
    }

    pub fn update(&mut self) {
        self.timer.update();
    }
}

