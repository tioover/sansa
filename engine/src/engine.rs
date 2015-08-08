use glium::Display;
use widget::Label;
use context::Context;
use timer::ProgramTimer;
use text;

pub struct Engine<'display> {
    pub timer: ProgramTimer,
    pub display: &'display Display,
    pub context: Context<'display>,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: ProgramTimer::new(),
            display: display,
            context: Context::new(display),
        }
    }

    pub fn label<T: ToString>(&self, style: text::TextStyle, x: T) -> Label {
        let hidpi_factor = self.display.get_window().unwrap().hidpi_factor();
        Label::new(style, hidpi_factor, x)
    }

    pub fn update(&mut self) {
        self.timer.update();
    }
}

