use glium::Display;
use widget::Label;
use timer::ProgramTimer;
use text::{TextStyle, GlyphCache};

pub struct Engine<'display> {
    pub timer: ProgramTimer,
    pub display: &'display Display,
    pub glyph_cache: GlyphCache,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: ProgramTimer::new(),
            display: display,
            glyph_cache: GlyphCache::new(),
        }
    }

    pub fn label<T: ToString>(&self, style: TextStyle, x: T) -> Label {
        let style = style.factor(self.display.get_window().unwrap().hidpi_factor());
        Label::new(self.glyph_cache.clone(), style, x)
    }

    pub fn update(&mut self) {
        self.timer.update();
    }
}


