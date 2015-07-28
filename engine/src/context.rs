use glium::{Display, Program, DrawParameters, Frame, Surface};
use glium::uniforms::Uniforms;
use na::Vec2;
use mesh::Mesh;
use camera::Camera;


pub struct Context<'display> {
    pub display: &'display Display,
    pub hidpi_factor: f32,
    pub program: Program,
    pub params: DrawParameters<'display>,
    pub camera: Camera<'display>,
}

impl<'display> Context<'display> {
    pub fn new(display: &'display Display) -> Context<'display> {
        let hidpi_factor = display.get_window().unwrap().hidpi_factor(); 
        let program = Context::build_program(display);
        Context {
            display: display,
            hidpi_factor: hidpi_factor,
            program: program,
            params: Context::build_params(),
            camera: Camera::new(display),
        }
    }

    pub fn update(&mut self) {
        self.camera.update()
    }

    #[inline]
    pub fn right_top(&self) -> Vec2<f32> {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let size = Vec2::new(width as f32, height as f32);
        size / self.hidpi_factor / 2.0
    }

    #[inline]
    pub fn left_bottom(&self) -> Vec2<f32> {
        -self.right_top()
    }

    fn build_program(display: &Display) -> Program {
        program!(display,
            140 => {
                vertex: &include_str!("shader/140/vertex.glsl"),
                fragment: &include_str!("shader/140/fragment.glsl"), 
            },
        ).unwrap()
    }

    pub fn draw<U>(&self, target: &mut Frame, mesh: &Mesh, uniforms: &U)
            where U: Uniforms {
        target.draw(
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
