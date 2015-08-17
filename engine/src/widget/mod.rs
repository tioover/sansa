use glium::{Display, Frame};
use sprite::Sprite;
use render::{Renderer, Renderable};
use event::{Update, EventStream};
use timer::Ms;
use canvas::Canvas;
use math::Mat;

pub mod label;

pub use self::label::Label;


pub trait WidgetBuilder: Clone {
    fn sprite(&self, &Display, Canvas) -> Sprite;
    fn render(&self) -> Canvas;

    fn build(self, display: &Display) -> Widget<Self> {
        Widget::new(display, self)
    }
}


pub struct Widget<B: WidgetBuilder> {
    sprite: Box<Sprite>,
    pub builder: B,
}

impl<B: WidgetBuilder> Widget<B> {
    pub fn new(display: &Display, builder: B) -> Widget<B> {
        let canvas = builder.render();
        Widget::with_canvas(display, builder, canvas)
    }

    pub fn with_canvas(display: &Display, builder: B, canvas: Canvas) -> Widget<B> {
        Widget {
            sprite: box builder.sprite(display, canvas),
            builder: builder,
        }
    }
}

impl<B: WidgetBuilder> Renderable for Widget<B> {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        self.sprite.draw(renderer, target, parent);
    }
}


impl<B: WidgetBuilder> Update for Widget<B> {
    fn update(&mut self, delta: Ms, mut stream: EventStream) -> EventStream {
        stream = self.sprite.update(delta, stream);
        return stream;
    }
}
