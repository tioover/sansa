use std::rc::Rc;
use na;
use na::Vec2;
use texture::Texture;


#[derive(Clone)]
pub struct Image {
    pub texture: Rc<Texture>,
    pub size: Vec2<f32>,
    pub offset: Vec2<f32>,
}


impl Image {
    pub fn new<'a>(texture: Rc<Texture>,
                   size: Vec2<i32>) -> Image {
        Image {
            texture: texture,
            size: na::cast(size),
            offset: na::zero(),
        }
    }

    pub fn offset(self, offset: Vec2<i32>) -> Image {
        Image { offset: na::cast(offset), ..self }
    }
}

