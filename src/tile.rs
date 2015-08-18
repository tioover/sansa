use std::rc::Rc;
use na;
use na::Vec2;
use rand;
use object::{Block, RoleKind, Role};
use engine::{Sprite, Texture, Manager};


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
            Land   => na![0, 0],
            River  => na![if rand::random() {0} else {1}, 1],
            Nil    => na![0, 0],
        }
    }
}


impl Tile for RoleKind {
    #[inline]
    fn offset(&self) -> Offset {
        use object::RoleKind::*;

        match *self {
            Player => na![0, 2],
            Enemy  => na![1, 2],
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
    pub texture: Rc<Texture>
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
            texture: tex.clone(),
            tile_size: TILE_SIZE,
            display_size: 64,
            margin: 8,
        }
    }

    pub fn sprite(&self, offset: Vec2<i32>, (i, j): (i32, i32)) -> Sprite {
        let size = na![self.display_size, self.display_size] + self.margin;
        let (a, b) = (size.x/2, size.y/4);
        let x = na![a, b]*i;
        let y = na![a, -b]*-j;
        let tile_size = na![self.tile_size, self.tile_size];
        Sprite::new(size-self.margin, tile_size, self.texture.clone())
            .position(na::cast(x+y))
            .offset(offset * self.tile_size)
    }
}







