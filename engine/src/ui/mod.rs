use glium::Display;
use sprite::Sprite;
use renderable::*;
use event::{Update, EventStream};
use timer::Ms;

pub mod text;

pub use self::text::{Label, GlyphCache, Glyph};


pub trait UIBuilder: Clone {
    fn sprite(&self, &Display) -> Sprite;
    fn build(self, display: &Display) -> UI<Self> {
        UI {
            sprite: self.sprite(display),
            builder: self,
        }
    }
}


pub struct UI<B: UIBuilder> {
    sprite: Sprite,
    pub builder: B,
}

impl<B: UIBuilder> UI<B> {
    pub fn rebuild(&mut self, display: &Display) {
        self.sprite = self.builder.sprite(display);
    }
}

impl<B: UIBuilder> Renderable for UI<B> {
    fn draw(&self, ctx: &Context, parent: Mat) {
        self.sprite.draw(ctx, parent);
    }
}


impl<B: UIBuilder> Update for UI<B> {
    fn update(&mut self, delta: Ms, stream: EventStream) -> EventStream {
        self.sprite.update(delta, stream)
    }
}
