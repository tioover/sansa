use na::Diag;
use math::Mat;
use glium::Display;

pub struct Camera<'display> {
    display: &'display Display,
    pub matrix: Mat,
    dimensions: (u32, u32),
    factor: f32,
}


impl<'display> Camera<'display> {
    pub fn new(display: &'display Display) -> Camera<'display> {
        let f = display.get_window().unwrap().hidpi_factor();
        let dim = display.get_framebuffer_dimensions();
        let mat = Camera::build_matrix(dim, f);
        Camera {
            display: display,
            matrix: mat,
            factor: f,
            dimensions: dim,
        }
    }

    fn build_matrix((w, h): (u32, u32), factor: f32) -> Mat {
        let (w, h) = (w as f32, h as f32);
        let f = factor * 2.0; 
        Mat::from_diag(&na![f/w, f/h, -1.0, 1.0])
    }

    pub fn update(&mut self) {
        let dim = self.display.get_framebuffer_dimensions();
        if dim != self.dimensions {
            self.dimensions = dim;
            self.matrix = Camera::build_matrix(dim, self.factor);
        }
    }
}
