use glium::{Display, Program, DrawParameters, Frame, Surface};
use glium::uniforms::Uniforms;
use mesh::Mesh;
pub use math::Mat;


pub trait Renderable {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat);
}


impl<'a> Renderable for Vec<&'a Renderable> {
    fn draw(&self, renderer: &Renderer, target: &mut Frame, parent: Mat) {
        for x in self {
            x.draw(renderer, target, parent);
        }
    }
}


pub struct Renderer<'display> {
    pub display: &'display Display,
    program: Program,
    params: DrawParameters<'display>,
}



impl<'display> Renderer<'display> {
    pub fn new(display: &'display Display) -> Renderer<'display> {
        Renderer::with_shader(display,
                              include_str!("shader/140/default.vert"),
                              include_str!("shader/140/default.frag"))
    }

    pub fn with_shader(display: &'display Display, vertex: &str, fragment: &str)
        -> Renderer<'display>
    {
        let program = program!(display,
            140 => {
                vertex: vertex,
                fragment: fragment,
            },
        ).unwrap();
        Renderer {
            display: display,
            program: program,
            params: Renderer::build_params(),
        }
    }

    pub fn draw<U>(&self, target: &mut Frame, mesh: &Mesh, uniforms: &U)
            where U: Uniforms {
        target.draw(
            &mesh.vertex,
            &mesh.index,
            &self.program,
            uniforms,
            &self.params
        ).unwrap();
    }

    pub fn render<T: Renderable>(&self, target: &mut Frame, renderable: &T, matrix: Mat) {
        renderable.draw(self, target, matrix);
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


pub trait DisplayExt {
    fn hidpi_factor(&self) -> f32;
}


impl DisplayExt for Display {
    fn hidpi_factor(&self) -> f32 {
        self.get_window().unwrap().hidpi_factor()
    }
}
