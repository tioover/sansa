use na::Vec2;
use std::string::ToString;
use std::rc::Rc;
use glium::Display;
use sprite::Sprite;
use canvas::Canvas;
use widget::WidgetBuilder;
use text::{TextStyle, System, draw};


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

    pub fn draw(&self) -> Canvas {
        draw(&*self.system, &self.style, self.hidpi_factor, &self.text)
    }

    pub fn anchor(self, center: Vec2<f32>) -> Label {
        Label { anchor: center, ..self }
    }

    pub fn position(self, position: Vec2<f32>) -> Label {
        Label { position: position, ..self }
    }
}


impl WidgetBuilder for Label {
    fn sprite(&self, display: &Display) -> Sprite {
        self.draw().into_sprite(display)
            .position(self.position)
            .anchor(self.anchor)
    }
}

