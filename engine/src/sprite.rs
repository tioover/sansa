use std::rc::Rc;
use glium::{Frame, Display, Surface};
use na;
use na::Vec2;
use color::Color;
use image::Image;
use renderable::Renderable;
use ::Context;
use texture::Texture;
use mesh::{Mesh, Vertex};
use event::{Update, Event};
use transform::Transform;
use math::Mat;
use timer::Ms;
use animation;
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
        let &[w, h] = image.size.as_array();
        let &[i, j] = image.offset.as_array();
        let &[a, b] = (size / 2.0).as_array();

        macro_rules! m {
            ($x: expr, $y: expr) =>
                (*transform.compute(na![$x, $y]).as_array())
        }
        macro_rules! n { ($x: expr, $y: expr) => ([$x+i, $y+j]) }

        [
            Vertex { position: m![-a,  b], tex_coords: n![0.0, 0.0] },
            Vertex { position: m![ a,  b], tex_coords: n![  w, 0.0] },
            Vertex { position: m![-a, -b], tex_coords: n![0.0,   h] },
            Vertex { position: m![ a, -b], tex_coords: n![  w,   h] },
        ]
    }

    #[inline]
    fn mesh(&self, display: &Display) -> Mesh {
        Mesh::rectangle(display,
            Sprite::vertices(self.size, &self.transform,
                             &self.image))
    }

    #[inline]
    pub fn transform(self, transform: Transform) -> Sprite {
        Sprite { transform: transform, ..self }
    }

    #[inline]
    pub fn anchor(self, center: Vec2<f32>) -> Sprite {
        let transform = self.transform.offset(-self.size * center);
        Sprite { transform: transform, ..self }
    }

    #[inline]
    pub fn position(self, position: Vec2<f32>) -> Sprite {
        let transform = self.transform.position(position);
        Sprite { transform: transform, ..self }
    }

    fn similar(&self, other: &Sprite) -> bool {
        self.image.texture == other.image.texture &&
        self.color_multiply == other.color_multiply
    }
}




impl Update for Sprite {
    fn update(&mut self, delta: Ms, event: Box<Event>)
            -> Box<Event> {
        animation::next(self, delta);
        return event;
    }
}


impl Renderable for Sprite {
    fn draw(&self, context: &Context, target: &mut Frame, parent: Mat) {
        context.draw(target, &self.mesh(&context.display),
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply.as_array(),
                tex_offset: na![0.0, 0.0],
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
    fn draw(&self, context: &Context, target: &mut Frame, parent: Mat) {
        context.draw(target, &self.mesh,
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply.as_array(),
                tex_offset: na![0.0, 0.0],
                tex: &self.texture.data
            }
        );
    }
}


pub fn render(context: &Context, sprites: Vec<&Sprite>) {
    let len = sprites.len();
    let camera = context.camera.matrix;
    let mut target = context.display.draw();
    let mut head = 0;

    target.clear_color(0.75, 0.75, 1.0, 1.0);

    for i in 1..len+1 {
        if i == len || !sprites[head].similar(sprites[i]) {
            if i-head == 1 {
                sprites[head].draw(context, &mut target, camera);
            }
            else {
                Batch::new(context.display, &sprites[head..i])
                    .draw(context, &mut target, camera);
            }
            head = i;
        }
    }
    target.finish().unwrap();
}


