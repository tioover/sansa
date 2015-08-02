use std::rc::Rc;
use glium::{Display, Surface};
use na;
use na::Vec2;
use color::Color;
use image::Image;
use renderable::Renderable;
use ::Context;
use texture::Texture;
use mesh::{Mesh, Vertex};
use event::{Update, EventStream};
use transform::Transform;
use math::Mat;
use timer::Ms;
use animation::State;


#[derive(Clone)]
pub struct Sprite {
    pub image: Image,
    pub size: Vec2<f32>,
    pub transform: Transform,
    pub color_multiply: Color,
    pub state: State<Sprite>,
}


impl Sprite {
    pub fn new(size: Vec2<i32>, image: Image) -> Sprite {
        Sprite {
            image: image,
            size: na::cast(size),
            transform: Transform::new(),
            color_multiply: Color::white(),
            state: State::Nil,
        }
    }

    fn vertices(size: Vec2<f32>, transform: &Transform, image: &Image)
                -> [Vertex; 4] {
        let tex_w = image.texture.width as f32;
        let tex_h = image.texture.height as f32;
        let &[w, h] = image.size.as_array();
        let &[i, j] = image.offset.as_array();
        let &[a, b] = (size / 2.0).as_array();

        macro_rules! vertex {
            ([$a:expr, $b:expr] [$c:expr, $d:expr]) => (
                Vertex {
                    position: *transform.compute(na![$a, $b]).as_array(),
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

    fn mesh(&self, display: &Display) -> Mesh {
        let vertices = Sprite::vertices(self.size, &self.transform, &self.image);
        Mesh::rectangle(display, vertices)
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

    fn similar(&self, other: &Sprite) -> bool {
        self.image.texture == other.image.texture &&
        self.color_multiply == other.color_multiply
    }
}




impl Update for Sprite {
    fn update(&mut self, delta: Ms, stream: EventStream)
            -> EventStream {
        self::animate::next(self, delta);
        return stream;
    }
}


pub mod animate {
    use math;
    use na::Vec2;
    use sprite::Sprite;
    use animation::State;
    use timer::{Ms, Timer};

    state_next_fn! { Sprite }

    pub fn rotate(total: Ms) -> State<Sprite> {
        function!(Timer::new(total), move |sprite, timer| {
            if_out!(timer,
                { sprite.transform.rotation = math::rotation(0.0) }
                { sprite.transform.rotation = math::rotation(timer.ratio()) }
            )
        })
    }


    pub fn fade(ms: Ms, from: f32, to: f32) -> State<Sprite> {
        function!(Timer::new(ms), move |sprite, timer| {
            if_out!(timer,
                { sprite.color_multiply.a = to }
                { sprite.color_multiply.a = math::linear(from, to, timer.ratio()) }
            )
        })
    }


    pub fn move_(ms: Ms, a: Vec2<f32>, b: Vec2<f32>) -> State<Sprite> {
        function!(Timer::new(ms), move |sprite, timer| {
            if_out!(timer,
                { sprite.transform.position = b }
                { sprite.transform.position = math::linear(a, b, timer.ratio()) }
            )
        })
    }


    pub fn curve(ms: Ms, control: [Vec2<f32>; 4]) -> State<Sprite> {
        function!(Timer::new(ms), move |sprite, timer| {
            if_out!(timer,
                { sprite.transform.position = control[3] }
                { sprite.transform.position = math::curve(control, timer.ratio()) }
            )
        })
    }


    pub fn fade_in(ms: Ms) -> State<Sprite> { fade(ms, 0.0, 1.0) }


    pub fn fade_out(ms: Ms) -> State<Sprite> { fade(ms, 1.0, 0.0) }
}


impl Renderable for Sprite {
    fn draw(&self, context: &Context, parent: Mat) {
        context.draw(&self.mesh(&context.display),
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply.as_array(),
                tex: &self.image.texture.data,
            }
        );
    }
}


pub struct Batch {
    texture: Rc<Texture>,
    mesh: Mesh,
    color_multiply: Color,
}


impl Batch {
    pub fn new(display: &Display, sprites: &[&Sprite]) -> Batch {
        use mesh::{VertexBuffer, IndexBuffer};
        use glium::index::PrimitiveType;

        let len = sprites.len();

        if len == 0 { panic!("No sprite for batch.") }

        let mut vb = VertexBuffer::empty_dynamic(display, len * 4).unwrap();
        let mut ib = Vec::with_capacity(len * 6);

        for (i, chunk) in vb.map().chunks_mut(4).enumerate() {
            let sprite = sprites[i];
            let vertices = Sprite::vertices(sprite.size,
                                            &sprite.transform,
                                            &sprite.image);

            for i in 0..4 {
                chunk[i] = vertices[i];
            }
            let num = i as u16;
            ib.push(num * 4 + 0);
            ib.push(num * 4 + 1);
            ib.push(num * 4 + 2);
            ib.push(num * 4 + 1);
            ib.push(num * 4 + 3);
            ib.push(num * 4 + 2);
        }
        let index = IndexBuffer::new(display, PrimitiveType::TrianglesList, &ib[..]).unwrap();
        let mesh = Mesh::new(vb, index);
        Batch {
            texture: sprites[0].image.texture.clone(),
            mesh: mesh,
            color_multiply: Color::white(),
        }
    }
}


impl Renderable for Batch {
    fn draw(&self, context: &Context, parent: Mat) {
        context.draw(&self.mesh,
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply,
                tex: &self.texture.data
            }
        );
    }
}


impl<'a> Renderable for Vec<&'a Sprite> {
    fn draw(&self, context: &Context, parent: Mat) {
        let len = self.len();
        let mut head = 0;


        for i in 1..len+1 {
            if i == len || !self[head].similar(self[i]) {
                if i-head == 1 {
                    self[head].draw(context, parent);
                }
                else {
                    Batch::new(context.display, &self[head..i])
                        .draw(context, parent);
                }
                head = i;
            }
        }
    }
}

