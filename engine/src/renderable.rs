pub use glium::{Frame, Surface};
pub use math::Mat;
pub use context::Context;


pub trait Renderable {
    fn draw(&self, context: &Context, target: &mut Frame, parent: Mat);
}


pub fn render(context: &Context, xs: Vec<&Renderable>, parent: Mat) {
    let mut target = context.display.draw();
    target.clear_color(0.75, 0.75, 1.0, 1.0);
    for x in xs { x.draw(context, &mut target, parent) }
    target.finish().unwrap();
}

