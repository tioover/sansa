use image;
use id::Id;
use glium::Display;
use glium::texture::{Texture2dDataSource, CompressedTexture2d};
use std::cmp::{PartialEq, Eq};
use std::path::Path;
use resources::Resource;

pub struct Texture {
    pub id: Id,
    pub height: u32,
    pub width: u32,
    pub data: CompressedTexture2d,
}


impl Texture {
    pub fn new<'a, T>(display: &Display, source: T) -> Texture
            where T: Texture2dDataSource<'a> {
        let tex = CompressedTexture2d::new(display, source).unwrap();
        Texture {
            id: Id::new(),
            width: tex.get_width(),
            height: tex.get_height().unwrap(),
            data: tex,
        }
    }
}


impl PartialEq<Texture> for Texture {
    fn eq(&self, other: &Texture) -> bool {
        self.id == other.id
    }
}


impl Eq for Texture {}


impl Resource for Texture {
    fn load(display: &Display, path: &Path) -> Texture {
        Texture::new(display, image::open(path).unwrap())
    }
}
