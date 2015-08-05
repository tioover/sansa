use std::rc::Rc;
use glium::Display;
use ui::Label;
use context::Context;
use timer::ProgramTimer;
use text;

pub struct Engine<'display> {
    pub timer: ProgramTimer,
    pub display: &'display Display,
    pub context: Context<'display>,
    pub text_system: Rc<text::System>,
}


impl<'display> Engine<'display> {
    pub fn new(display: &'display Display) -> Engine<'display> {
        Engine {
            timer: ProgramTimer::new(),
            display: display,
            context: Context::new(display),
            text_system: Rc::new(text::System::new()),
        }
    }

    pub fn label<T: ToString>(&self, style: text::TextStyle, x: T) -> Label {
        let hidpi_factor = self.display.get_window().unwrap().hidpi_factor();
        Label::new(self.text_system.clone(), style, hidpi_factor, x)
    }

    pub fn update(&mut self) {
        self.timer.update();
    }

}

