use glium;
use glium::Display;
use na::Vec2;


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec2<f32>,
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);


pub type VertexBuffer = glium::VertexBuffer<Vertex>;
pub type IndexBuffer = glium::IndexBuffer<u16>;


pub trait Polygon {
    fn mesh(&self, &Display) -> Mesh;
}



pub struct Mesh {
    pub vertex_buffer: VertexBuffer,
    pub index_buffer: IndexBuffer,
}
