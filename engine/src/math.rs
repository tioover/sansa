use std::ops::{Add, Sub, Mul};
use num;
use num::Float;
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
pub fn rotation(t: f32) -> Rot2<f32> {
    const ROUND: f32 = 6.28318530717958647692528676655900576;

    Rot2::new(Vec1::new(t * ROUND))
}


pub fn curve<F: Float>(control: [Vec2<F>; 4], t: F) -> Vec2<F> {
    // Cubic Bézier curves

    macro_rules! cast (
        ($x: expr) => (
            num::cast::<_, F>($x).unwrap()
        )
    );
    let p = control;
    let r = cast!(1.0) - t;

    let a = r.powi(3);
    let b = cast!(3.0)*t*r.powi(2);
    let c = cast!(3.0)*t.powi(2)*r;
    let d = t.powi(3);

    p[0] * a + p[1] * b + p[2] * c + p[3] * d
}


pub fn linear<T, F: Float>(a: T, b: T, t: F) -> T
        where T: Copy + Add<T, Output=T> + Sub<T, Output=T> + Mul<F, Output=T> {
    (b-a) * t + a
}


