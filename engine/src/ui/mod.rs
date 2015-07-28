use glium::Display;
use sprite::Sprite;
use renderable::*;
use event::{Update, Event};
use timer::Ms;

pub mod text;

pub use self::text::{Text, GlyphCache, Glyph};


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
    pub sprite: Sprite,
    pub builder: B,
}

impl<B: UIBuilder> UI<B> {
    pub fn re_build(&mut self, display: &Display) {
        self.sprite = self.builder.sprite(display);
    }
}

impl<B: UIBuilder> Renderable for UI<B> {
    fn draw(&self, ctx: &Context, target: &mut Frame, parent: Mat) {
        self.sprite.draw(ctx, target, parent);
    }
}


impl<B: UIBuilder> Update for UI<B> {
    fn update(&mut self, delta: Ms, event: Box<Event>) -> Box<Event> {
        self.sprite.update(delta, event)
    }
}
