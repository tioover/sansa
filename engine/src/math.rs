use na::{Vec1, Vec2, Vec4, Mat4, Rot2, Diag};

pub type Mat = Mat4<f32>;

#[inline]
pub fn scaling(factor: f32) -> Mat {
    Mat4::from_diag(&Vec4::new(factor, factor, factor, 1.0))
}

#[inline]
pub fn translation(vec: Vec2<f32>) -> Mat {
    Mat4::new(
        1.0, 0.0, 0.0, vec.x,
        0.0, 1.0, 0.0, vec.y,
        0.0, 0.0, 1.0,   0.0,
        0.0, 0.0, 0.0,   1.0)
}

#[inline]
pub fn rotation(angle: f32) -> Rot2<f32> {
    Rot2::new(Vec1::new(angle))
}
