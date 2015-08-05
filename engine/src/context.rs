use std::cell::RefCell;
use glium::{Display, Program, DrawParameters, Frame, Surface};
use glium::uniforms::Uniforms;
use mesh::Mesh;


pub struct Context<'display> {
    pub display: &'display Display,
    pub program: Program,
    pub params: DrawParameters<'display>,
    frame: Option<RefCell<Frame>>,
}


impl<'display> Context<'display> {
    pub fn new(display: &'display Display) -> Context<'display> {
        let program = Context::build_program(display);
        Context {
            display: display,
            program: program,
            params: Context::build_params(),
            frame: None,
        }
    }

    pub fn frame(&mut self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        self.frame = Some(RefCell::new(target));
    }

    pub fn finish(&mut self) {
        let target = self.frame.take().unwrap().into_inner();
        target.finish().unwrap();
    }

    fn build_program(display: &Display) -> Program {
        program!(display,
            140 => {
                vertex: &include_str!("shader/140/vertex.glsl"),
                fragment: &include_str!("shader/140/fragment.glsl"),
            },
        ).unwrap()
    }

    pub fn draw<U>(&self, mesh: &Mesh, uniforms: &U)
            where U: Uniforms {
        let cell = self.frame.as_ref().unwrap();
        cell.borrow_mut().draw(
            &mesh.vertex,
            &mesh.index,
            &self.program,
            uniforms,
            &self.params).unwrap();
    }

    fn build_params<'a>() -> DrawParameters<'a> {
        use glium::BlendingFunction::Addition;
        use glium::LinearBlendingFactor::*;
        use std::default::Default;

        let blending_function = Addition {
            source: SourceAlpha,
            destination: OneMinusSourceAlpha
        };

        DrawParameters {
            blending_function: Some (blending_function),
            .. Default::default()
        }
    }
}
