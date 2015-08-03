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
        let current = self.offset;
        let state: State<Camera> = function!(Timer::new(time), move |camera, timer| {
            camera.offset = current + linear(zero(), offset, timer.ratio());
            Return::Remain
        });
        self.state = state;
    }

    pub fn reset(&mut self) {
        self.offset = zero();
    }

    pub fn right_top(&self) -> Vec2<f32> {
        let hidpi_factor = self.display.get_window().unwrap().hidpi_factor();
        let (width, height) = self.display.get_framebuffer_dimensions();
        let size = na![width as f32, height as f32];
        size / hidpi_factor / 2.0
    }

    pub fn right_bottom(&self) -> Vec2<f32> {
        let rt = self.right_top();
        na![rt.x, -rt.y]
    }

    pub fn left_top(&self) -> Vec2<f32> {
        -self.right_bottom()
    }

    pub fn left_bottom(&self) -> Vec2<f32> {
        -self.right_top()
    }


    pub fn matrix(&self) -> Mat {
        let factor = self.display.get_window().unwrap().hidpi_factor();
        let (w, h) = self.display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0;
        Mat::from_diag(&na![f/w, f/h, -1.0, 1.0]) * translation(-self.offset)
    }

    pub fn update(&mut self, delta: Ms) {
        let mut state = self.state.clone();
        if let  Return::Become(x) = state.transition(self, delta) {
            self.state = x;
        }
    }
}
