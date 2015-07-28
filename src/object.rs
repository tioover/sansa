use na::Vec3;

use self::Block::*;


#[derive(Clone, Copy, Debug)]
pub enum Block {
    Nil,
    Land,
    River,
}


#[derive(Clone, Copy, Debug)]
pub enum RoleKind {
    Player,
    Enemy,
}


#[derive(Clone, Debug)]
pub struct Role {
    pub health: u32,
    pub kind: RoleKind,
    pub position: Vec3<i32>,
}


impl Role {
    pub fn new(kind: RoleKind, position: Vec3<i32>) -> Role {
        Role {
            kind: kind,
            health: 100,
            position: position,
        }
    }
}


