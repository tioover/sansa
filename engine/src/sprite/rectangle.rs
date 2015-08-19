use glium::Display;
use mesh::{Vertex, Polygon, VertexBuffer, IndexBuffer, Mesh};
use sprite::Sprite;


pub struct Rectangle {
    top_left: Vertex,
    top_right: Vertex,
    bottom_left: Vertex,
    bottom_right: Vertex,
}


impl Rectangle {
    pub fn new(sprite: &Sprite) -> Rectangle {
        let tex_w = sprite.texture.width as f32;
        let tex_h = sprite.texture.height as f32;
        let &[w, h] = sprite.texture_clip_size.as_array();
        let &[i, j] = sprite.texture_offset.as_array();
        let &[a, b] = (sprite.size / 2.0).as_array();

        macro_rules! vertex {
            ([$a:expr, $b:expr] [$c:expr, $d:expr]) => (
                Vertex {
                    position: sprite.transform.compute(na![$a, $b]),
                    tex_coords: [($c+i)/tex_w, 1.0-($d+j)/tex_h],
                }
            )
        }
        Rectangle {
                top_left: vertex!([-a,  b] [0.0, 0.0]),
               top_right: vertex!([ a,  b] [  w, 0.0]),
             bottom_left: vertex!([-a, -b] [0.0,   h]),
            bottom_right: vertex!([ a, -b] [  w,   h]),
        }
    }

    pub fn as_array(&self) -> [Vertex; 4] {
        [
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right,
        ]
    }
}

impl Polygon for Rectangle {
    fn mesh(&self, display: &Display) -> Mesh {
        let vb = VertexBuffer::new(display, &[
            self.top_left,
            self.top_right,
            self.bottom_left,
            self.bottom_right
        ]).unwrap();
        let index_type = ::glium::index::PrimitiveType::TriangleStrip;
        let ib = IndexBuffer::new(display, index_type, &[0, 1, 2, 3]).unwrap();
        Mesh {
            vertex_buffer: vb,
            index_buffer: ib,
        }
    }
}

