use std::rc::Rc;
use glium::{Surface, Frame};
use na;
use na::Vec2;
use color::Color;
use render::{Renderable, Renderer};
use texture::Texture;
use mesh::Polygon;
use event::{Update, EventStream};
use transform::Transform;
use math::Mat;
use timer::Ms;
use animation::State;
use self::rectangle::Rectangle;

pub mod animate;
pub mod batch;
pub mod rectangle;

#[derive(Clone)]
pub struct Sprite {
    pub size: Vec2<f32>,
    pub transform: Transform,
    pub texture_offset: Vec2<f32>,
    pub color_multiply: Color,
    texture: Rc<Texture>,
    texture_clip_size: Vec2<f32>,
    state: State<Sprite>,
}


impl Sprite {
    pub fn new(size: Vec2<i32>, clip_size: Vec2<i32>, texture: Rc<Texture>)
        -> Sprite
    {
        Sprite {
            size: na::cast(size),
            transform: Transform::new(),
            state: State::Nil,
            texture_clip_size: na::cast(clip_size),
            texture: texture,
            color_multiply: Color::white(),
            texture_offset: na::zero(),
        }
    }

    pub fn offset(self, offset: Vec2<i32>) -> Sprite {
        Sprite { texture_offset: na::cast(offset), ..self }
    }

    fn batchable(&self, other: &Sprite) -> bool {
        self.texture == other.texture && self.color_multiply == other.color_multiply
    }

    #[inline]
    fn rectangle(&self) -> Rectangle {
        Rectangle::new(self)
    }

    pub fn transform(self, transform: Transform) -> Sprite {
        Sprite { transform: transform, ..self }
    }

    pub fn anchor(self, center: Vec2<f32>) -> Sprite {
        let transform = self.transform.offset(self.size * center / 2.0);
        Sprite { transform: transform, ..self }
    }

    pub fn position(self, position: Vec2<f32>) -> Sprite {
        let transform = self.transform.position(position);
        Sprite { transform: transform, ..self }
    }

    pub fn state(self, state: State<Sprite>) -> Sprite {
        Sprite { state: state, ..self }
    }

    pub fn set_state(&mut self, state: State<Sprite>) {
        self.state = state;
    }
}



impl Renderable for Sprite {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: &Mat) {
        let rect = self.rectangle();
        renderer.draw(target, &rect.mesh(renderer.display),
            &uniform! {
                matrix: *parent,
                color_multiply: self.color_multiply,
                tex: &self.texture.data,
            }
        );
    }
}



impl Update for Sprite {
    fn update(&mut self, _: &Renderer, delta: Ms, stream: EventStream)
        -> EventStream
    {
        use std::mem::swap;
        use animation::Return;

        let mut state = State::Nil;
        swap(&mut self.state, &mut state);
        if let Return::Become(x) = state.transition(self, delta) {
            state = x;
        }
        self.state = state;
        return stream;
    }
}


