use glium::Display;
use sprite::Sprite;
use renderable::*;
use event::{Update, EventStream};
use timer::Ms;

pub mod text;

pub use self::text::{Label, GlyphCache, Glyph};


pub trait WidgetBuilder: Clone {
    fn sprite(&self, &Display) -> Sprite;
    fn build(self, display: &Display) -> Widget<Self> {
        Widget {
            sprite: self.sprite(display),
            builder: self,
        }
    }
}


pub struct Widget<B: WidgetBuilder> {
    sprite: Sprite,
    pub builder: B,
}

impl<B: WidgetBuilder> Widget<B> {
    pub fn rebuild(&mut self, display: &Display) {
        self.sprite = self.builder.sprite(display);
    }
}

impl<B: WidgetBuilder> Renderable for Widget<B> {
    fn draw(&self, ctx: &Context, parent: Mat) {
        self.sprite.draw(ctx, parent);
    }
}


impl<B: WidgetBuilder> Update for Widget<B> {
    fn update(&mut self, delta: Ms, stream: EventStream) -> EventStream {
        self.sprite.update(delta, stream)
    }
}
