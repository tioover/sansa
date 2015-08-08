use na::Vec2;
use std::string::ToString;
use std::sync::mpsc::{Receiver, channel};
use threadpool::ThreadPool;
use glium::Display;
use sprite::Sprite;
use canvas::Canvas;
use widget::WidgetBuilder;
use text::{TextStyle, draw};


#[derive(Clone)]
pub struct Label {
    style: TextStyle,
    position: Vec2<f32>,
    hidpi_factor: f32,
    anchor: Vec2<f32>,
    text: String,
}


impl Label {
    pub fn new<T>(style: TextStyle, hidpi_factor: f32, x: T)
            -> Label where T: ToString {
        Label {
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
        let (tx, rx) = channel();
        let style = self.style.clone();
        let f = self.hidpi_factor;
        let text = self.text.clone();
        pool.execute(
            move || {
                tx.send(draw(style, f, text)).unwrap();
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

