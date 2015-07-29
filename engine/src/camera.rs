use na::{Diag, Vec2, zero};
use math::{Mat, translation, linear};
use timer::Ms;
use animation::{Return, State};
use timer::Timer;

use glium::Display;

pub struct Camera<'display> {
    display: &'display Display,
    offset: Vec2<f32>,
    state: State<Camera<'display>>,
}



impl<'display> Camera<'display> {
    pub fn new(display: &'display Display) -> Camera<'display> {
        Camera {
            display: display,
            offset: ::na::zero(),
            state: State::Nil,
        }
    }

    pub fn move_(&mut self, time: Ms, offset: Vec2<f32>) {
        self.offset = zero();
        let state: State<Camera> = function!(Timer::new(time), move |camera, timer| {
            camera.offset = linear(zero(), offset, timer.ratio());
            Return::Remain
        });
        self.state = state;
    }


    pub fn matrix(&self) -> Mat {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Mat::from_diag(&na![f/w, f/h, -1.0, 1.0]) * translation(-self.offset)
    }

    pub fn update(&mut self, delta: Ms) {
        next(self, delta)
    }
}


state_next_fn! { Camera }
