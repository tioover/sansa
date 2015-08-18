use na::Vec2;
use std::string::ToString;
use glium::Display;
use sprite::Sprite;
use canvas::Canvas;
use widget::WidgetBuilder;
use text;
use text::{TextStyle, GlyphCache};
use event::EventStream;


#[derive(Clone)]
pub struct Label {
    cache: GlyphCache,
    style: TextStyle,
    position: Vec2<f32>,
    anchor: Vec2<f32>,
    text: String,
}


impl Label {
    pub fn new<T>(cache: GlyphCache, style: TextStyle, x: T)
            -> Label where T: ToString {
        Label {
            cache: cache,
            style: style,
            position: ::na::zero(),
            anchor: ::na::zero(),
            text: x.to_string(),
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
    fn render(&self) -> Canvas {
        let glyphs = {
            let mut cache = self.cache.lock().unwrap();
            text::load(&mut *cache, &self.style, &self.text)
        };
        let xs = glyphs.iter().map(|&(c, ref g)| (c, &**g)).collect();
        text::draw(&self.style, xs)
    }

    fn event_respond(&self, stream: EventStream, _: &mut Sprite)
        -> (EventStream, Option<Label>)
    {
        (stream, None)
    }

    fn sprite(&self, display: &Display, canvas: Canvas) -> Sprite {
        canvas.into_sprite(display)
            .position(self.position)
            .anchor(self.anchor)
    }
}
