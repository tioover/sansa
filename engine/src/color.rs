use glium::texture::{PixelValue, ClientFormat};
use std::ops::Add;
use std::cmp::{PartialEq, Eq};


#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32, pub g: f32, pub b: f32, pub a: f32
}


impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r: r, g: g, b: b, a: a }
    }

    pub fn white() -> Color {
        Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }

    pub fn black() -> Color {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }

    pub fn alpha(self, alpha: f32) -> Color {
        Color { a: alpha, ..self }
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }
}


unsafe impl PixelValue for Color {
    #[inline]
    fn get_format() -> ClientFormat {
        ClientFormat::F32F32F32F32
    }
}


impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        let a = rhs.a;
        let rhs = rhs ; 
        macro_rules! make {
            ( $( $x:ident ),* ) => {
                Color {
                    $($x: self.$x * (1.0-a) + rhs.$x,)*
                }
            }
        }
        make!(r, g, b, a)
    }
}


impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r == other.r && self.g == other.g &&
        self.b == other.b && self.a == other.a
    }
}


impl Eq for Color {}
