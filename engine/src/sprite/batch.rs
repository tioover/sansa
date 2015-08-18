use std::rc::Rc;
use glium::{Display, Surface, Frame};
use color::Color;
use render::{Renderable, Renderer};
use texture::Texture;
use mesh::Mesh;
use math::Mat;
use sprite::Sprite;



pub struct Batch {
    texture: Rc<Texture>,
    mesh: Mesh,
    color_multiply: Color,
}


impl Batch {
    pub fn from_sprites(display: &Display, sprites: &[&Sprite]) -> Batch {
        use mesh::{VertexBuffer, IndexBuffer};
        use glium::index::PrimitiveType;

        let len = sprites.len();

        if len == 0 { panic!() }

        let first = sprites[0];

        let mut vb = VertexBuffer::empty_dynamic(display, len * 4).unwrap();
        let mut ib = Vec::with_capacity(len * 6);

        for (i, chunk) in vb.map().chunks_mut(4).enumerate() {
            let sprite = sprites[i];
            assert!(first.batchable(sprite));
            let vertices = sprite.rectangle();

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
            texture: first.texture.clone(),
            mesh: mesh,
            color_multiply: first.color_multiply,
        }
    }
}


impl Renderable for Batch {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        renderer.draw(target, &self.mesh,
            &uniform! {
                matrix: parent,
                color_multiply: self.color_multiply,
                tex: &self.texture.data
            }
        );
    }
}


impl<'a> Renderable for Vec<&'a Sprite> {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        let len = self.len();

        if len == 0 { return }

        let mut i = 0;

        for j in 1..len+1 {
            // utill last element or next unbatchable sprite.
            if j != len && self[i].batchable(self[j]) { continue }

            // else cutoff and render.
            if j-i > 1 {
                // multiple sprite should batch ender.
                let sprites = &self[i..j];
                let batch = Batch::from_sprites(renderer.display, sprites);
                batch.draw(renderer, target, parent);
            }
            else {
                // only one sprite.
                self[i].draw(renderer, target, parent);
            }
            i = j;
        }
    }
}

