use std::rc::Rc;
use glium::{Display, Surface, Frame};
use na;
use na::Vec2;
use color::Color;
use render::{Renderable, Renderer};
use texture::Texture;
use mesh::{Mesh, Vertex};
use event::{Update, EventStream};
use transform::Transform;
use math::Mat;
use timer::Ms;
use animation::State;


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

    pub fn rectangle(&self) -> [Vertex; 4] {
        let tex_w = self.texture.width as f32;
        let tex_h = self.texture.height as f32;
        let &[w, h] = self.texture_clip_size.as_array();
        let &[i, j] = self.texture_offset.as_array();
        let &[a, b] = (self.size / 2.0).as_array();

        macro_rules! vertex {
            ([$a:expr, $b:expr] [$c:expr, $d:expr]) => (
                Vertex {
                    position: *self.transform.compute(na![$a, $b]).as_array(),
                    tex_coords: [($c+i)/tex_w, 1.0-($d+j)/tex_h],
                }
            )
        }
        [
            vertex!([-a,  b] [0.0, 0.0]),
            vertex!([ a,  b] [  w, 0.0]),
            vertex!([-a, -b] [0.0,   h]),
            vertex!([ a, -b] [  w,   h]),
        ]
    }

    fn batchable(&self, other: &Sprite) -> bool {
        self.texture == other.texture && self.color_multiply == other.color_multiply
    }

    fn mesh(&self, display: &Display) -> Option<Mesh> {
        Some(Mesh::rectangle(display, self.rectangle()))
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
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        let mesh = self.mesh(&renderer.display);
        if mesh.is_none() { return }
        renderer.draw(target, &mesh.unwrap(),
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply.as_array(),
                tex: &self.texture.data,
            }
        );
    }
}



impl Update for Sprite {
    fn update(&mut self, delta: Ms, stream: EventStream) -> EventStream {
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


pub mod animate;

pub mod batch;
