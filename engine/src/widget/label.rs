use na::Vec2;
use std::string::ToString;
use std::sync::mpsc::{Receiver, channel};
use threadpool::ThreadPool;
use glium::Display;
use std::rc::Rc;
use sprite::Sprite;
use canvas::Canvas;
use widget::WidgetBuilder;
use text;
use text::{TextStyle, GlyphCache, System};


#[derive(Clone)]
pub struct Label {
    system: Rc<System>,
    style: TextStyle,
    position: Vec2<f32>,
    hidpi_factor: f32,
    anchor: Vec2<f32>,
    text: String,
}


impl Label {
    pub fn new<T>(system: Rc<System>, style: TextStyle, hidpi_factor: f32, x: T)
            -> Label where T: ToString {
        Label {
            system: system,
            style: style,
            position: ::na::zero(),
            anchor: ::na::zero(),
            text: x.to_string(),
            hidpi_factor: hidpi_factor,
        }
    }


    pub fn anchor(self, center: Vec2<f32>) -> Label {
        Label { anchor: center, ..self }
    }

    pub fn position(self, position: Vec2<f32>) -> Label {
        Label { position: position, ..self }
    }
}


impl WidgetBuilder for Label {
    fn render(&self, pool: &ThreadPool) -> Receiver<Canvas> {
        let glyphs = text::load(&*self.system,
                                &self.style,
                                self.hidpi_factor,
                                &self.text);
        let (tx, rx) = channel();
        let style = self.style.clone();
        let f = self.hidpi_factor;
        pool.execute(
            move || {
                tx.send(text::draw(style, f, glyphs)).unwrap();
            }
        );
        return rx;
    }

    fn sprite(&self, display: &Display, canvas: Canvas) -> Sprite {
        canvas.into_sprite(display)
            .position(self.position)
            .anchor(self.anchor)
    }
}

