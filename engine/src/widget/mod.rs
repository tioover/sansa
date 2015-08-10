use std::cell::RefCell;
use std::sync::mpsc::Receiver;
use threadpool::ThreadPool;
use glium::{Display, Frame};
use sprite::Sprite;
use renderable::{Renderer, Renderable};
use event::{Update, EventStream};
use timer::Ms;
use canvas::Canvas;
use math::Mat;

pub mod label;

pub use self::label::Label;


pub enum Thunk {
    Just (Sprite),
    Wait (Receiver<Canvas>),
}


pub trait WidgetBuilder: Clone {
    fn sprite(&self, &Display, Canvas) -> Sprite;
    fn render(&self, &ThreadPool) -> Receiver<Canvas>;

    fn build(self, pool: &ThreadPool) -> Widget<Self> {
        Widget {
            thunk: RefCell::new(Thunk::Wait(self.render(pool))),
            builder: self,
        }
    }
}


pub struct Widget<B: WidgetBuilder> {
    thunk: RefCell<Thunk>,
    pub builder: B,
}

impl<B: WidgetBuilder> Widget<B> {
}

impl<B: WidgetBuilder> Renderable for Widget<B> {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        {
            if let &Thunk::Just(ref sprite) = &*self.thunk.borrow() {
                sprite.draw(renderer, target, parent);
                return
            }
        }
        let thunk = &mut *self.thunk.borrow_mut();
        let canvas = match thunk {
            &mut Thunk::Wait(ref x) => x.recv().unwrap(),
            _ => unreachable!(),
        };
        let sprite = self.builder.sprite(renderer.display, canvas);
        sprite.draw(renderer, target, parent);
        *thunk = Thunk::Just(sprite);
    }
}


impl<B: WidgetBuilder> Update for Widget<B> {
    fn update(&mut self, delta: Ms, stream: EventStream) -> EventStream {
        //self.sprite.update(delta, stream)
        stream
    }
}
