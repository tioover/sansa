use glium::{Display, Program, DrawParameters, Frame, Surface};
use glium::uniforms::Uniforms;
use mesh::Mesh;
pub use math::Mat;
pub use context::Context;


pub trait Renderable {
    fn draw(&self, context: &Context, parent: Mat);
}


impl<'a> Renderable for Vec<&'a Renderable> {
    fn draw(&self, context: &Context, parent: Mat) {
        for x in self {
            x.draw(context, parent);
        }
    }
}


macro_rules! params {
    () => (
        let blending_function = ::glium::BlendingFunction::Addition {
            source: ::glium::LinearBlendingFactor::SourceAlpha,
            destination: ::glium::LinearBlendingFactor::OneMinusSourceAlpha,
        };

        DrawParameters::DrawParameters {
            blending_function: Some (blending_function),
            .. ::std::default::Default::default()
        }
    )
}


struct Renderer<'display> {
    display: &'display Display,
    program: Program,
    params: DrawParameters<'display>,
}


impl<'display> Renderer<'display> {
    pub fn new(display: &'display Display, shader: (&str, &str)) -> Renderer<'display> {
        let (vertex, fragment) = shader;
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
