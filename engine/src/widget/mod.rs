use glium::{Display, Frame};
use sprite::Sprite;
use render::{Renderer, Renderable};
use event::{Update, EventStream};
use timer::Ms;
use canvas::Canvas;
use math::Mat;
use id::Id;

pub mod label;

pub use self::label::Label;


pub trait WidgetBuilder: Sized {
    fn sprite(&self, &Display, Canvas) -> Sprite;
    fn render(&self) -> Canvas;
    fn event_respond(&self, EventStream, &mut Sprite) -> (EventStream, Option<Self>);

    fn build(self, display: &Display) -> Widget<Self> {
        Widget::new(display, self)
    }
}


pub struct Widget<B: WidgetBuilder> {
    pub id: Id,
    visible: bool,
    sprite: Sprite,
    pub builder: B,
}


impl<B: WidgetBuilder> Widget<B> {
    pub fn new(display: &Display, builder: B) -> Widget<B> {
        let canvas = builder.render();
        Widget::with_canvas(display, builder, canvas)
    }

    pub fn with_canvas(display: &Display, builder: B, canvas: Canvas) -> Widget<B> {
        Widget {
            id: Id::new(),
            visible: true,
            sprite: builder.sprite(display, canvas),
            builder: builder,
        }
    }
}

impl<B: WidgetBuilder> Renderable for Widget<B> {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: &Mat) {
        if self.visible {
            self.sprite.draw(renderer, target, parent);
        }
    }
}


impl<B: WidgetBuilder> Update for Widget<B> {
    fn update(&mut self, renderer: &Renderer, delta: Ms, stream: EventStream)
        -> EventStream
    {
        let (mut stream, rebuilder) = self.builder.event_respond(stream, &mut self.sprite);
        if let Some(new) = rebuilder {
            *self = Widget::new(renderer.display, new)
        }
        else {
            stream = self.sprite.update(renderer, delta, stream);
        }
        return stream;
    }
}
