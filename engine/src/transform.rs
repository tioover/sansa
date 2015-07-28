use na;
use na::{Vec2, Rot2};
use math;


#[derive(Clone)]
pub struct Transform {
    pub scale: f32,
    pub position: Vec2<f32>,
    pub rotation: Rot2<f32>,
    pub offset: Vec2<f32>,
}


impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: na::zero(),
            rotation: math::rotation(0.0),
            scale: 1.0,
            offset: Vec2::new(0.0, 0.0),
        }
    }

    pub fn position(self, position: Vec2<f32>) -> Transform {
        Transform { position: position, ..self }
    }

    pub fn scale(self, scale: f32) -> Transform {
        Transform { scale: scale, ..self }
    }

    pub fn offset(self, offset: Vec2<f32>) -> Transform {
        Transform { offset: offset, ..self }
    }

    #[inline]
    pub fn compute(&self, x: Vec2<f32>) -> Vec2<f32> {
        (x + self.offset) * self.scale * self.rotation + self.position
    }
}
