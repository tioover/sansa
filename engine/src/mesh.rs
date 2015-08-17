use glium;
use glium::Display;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);


impl Vertex {
    pub fn in_screen(&self) -> bool {
        let [a, b] = self.position;
        (a <= 1.0 || a >= -1.0) && (b <= 1.0 || b >= -1.0)
    }
}


pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;


pub struct Mesh {
    pub vertex: VertexBuffer,
    pub index: IndexBuffer,
}


impl Mesh {
    pub fn new(vertex: VertexBuffer, index: IndexBuffer) -> Mesh {
        Mesh {
            vertex: vertex,
            index: index,
        }
    }

    pub fn rectangle(display: &Display, vertices: [Vertex; 4]) -> Mesh {
        let vertex = VertexBuffer::new(display, &vertices).unwrap();
        let index_type = glium::index::PrimitiveType::TriangleStrip;
        let index = IndexBuffer::new(display, index_type, &[0, 1, 2, 3]).unwrap();
        Mesh::new(vertex, index)
    }
}
