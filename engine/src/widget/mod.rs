use glium::Display;
use sprite::Sprite;
use renderable::*;
use event::{Update, EventStream};
use timer::Ms;

pub mod label;

pub use self::label::Label;


pub trait WidgetBuilder: Clone {
    fn sprite(&self, &Display) -> Sprite;
    fn build(self, display: &Display) -> Widget<Self> {
        Widget {
            sprite: box self.sprite(display),
            builder: box self,
        }
    }
}


pub struct Widget<B: WidgetBuilder> {
    sprite: Box<Sprite>,
    pub builder: Box<B>,
}

impl<B: WidgetBuilder> Widget<B> {
    pub fn rebuild(&mut self, display: &Display) {
        self.sprite = box self.builder.sprite(display);
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
