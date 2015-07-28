use na;
use na::Vec2;
use rand;
use object::{Block, RoleKind, Role};
use engine::{Sprite, Texture, Image, Manager};


const TILE_SIZE: i32 = 256;
pub type Offset = Vec2<i32>;


pub trait Tile {
    fn offset(&self) -> Offset;
}



impl Tile for Block {
    #[inline]
    fn offset(&self) -> Offset {
        use object::Block::*;

        match *self {
            Land   => v![0, 0],
            River  => v![if rand::random() {0} else {1}, 1],
            Nil    => v![0, 0],
        }
    }
}


impl Tile for RoleKind {
    #[inline]
    fn offset(&self) -> Offset {
        use object::RoleKind::*;

        match *self {
            Player => v![0, 2],
            Enemy  => v![1, 2],
        }
    }
}


impl Tile for Role {
    fn offset(&self) -> Offset {
        self.kind.offset()
    }
}



pub struct TileGen {
    pub tile_size: i32,
    pub display_size: i32,
    pub margin: i32,
    image: Image,
}


impl TileGen {
    #[inline]
    pub fn horizontal(&self) -> i32 {
        self.display_size + self.margin
    }

    #[inline]
    pub fn vertical(&self) -> i32 {
        (self.display_size + self.margin) / 2
    }

    pub fn new(manager: &Manager<Texture>) -> TileGen {
        let tex = manager.get("block.png");
        TileGen {
            image: Image::new(tex.clone(), v![TILE_SIZE, TILE_SIZE]),
            tile_size: TILE_SIZE,
            display_size: 64,
            margin: 8,
        }
    }

    pub fn sprite(&self, offset: Vec2<i32>, (i, j): (i32, i32)) -> Sprite {
        let size = v![self.display_size, self.display_size] + self.margin;
        let image = self.image.clone()
            .offset(offset * self.tile_size);
        let (a, b) = (size.x/2, size.y/4);
        let x = v![a, b]*i;
        let y = v![a, -b]*-j;
        Sprite::new(size-self.margin, image).position(na::cast(x+y))
    }
}







