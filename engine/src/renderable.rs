pub use glium::{Frame, Surface};
pub use math::Mat;
pub use context::Context;


pub trait Renderable {
    fn draw(&self, context: &Context, parent: Mat);
}


pub fn render(context: &Context, xs: Vec<&Renderable>, parent: Mat) {
    for x in xs { x.draw(context, parent) }
}

