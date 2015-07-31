// use std::collections::LinkedList
use rand;
use noise::{Brownian2, perlin2};
use object::{Block, RoleKind, Role};

pub const LAYER: i32 = 8;


pub enum Input {
    Move ((i32, i32)),
}


pub enum Output {
    Move ((i32, i32)),
}

pub struct Game {
    turn: u64,
    player: Role,
    map: Map,
}


impl Game {
    pub fn new((x, y): (i32, i32)) -> Game {
        Game {
            turn: 0,
            player: Role::new(RoleKind::Player, na![x/2, y/2, 1]),
            map: Map::new((x, y)),
        }
    }

    #[inline]
    pub fn get(&self, i: [i32; 3]) -> Unit {
        let [x, y, z] = i;
        let position = self.player.position;
        Unit {
            block: self.map.get([x+position.x, y+position.y, z]),
            role: if [x, y, z] == [0, 0, position.z] {
                      Some (self.player.clone())
                  } else { None }
        }
    }

    pub fn next(&mut self, input: Input) -> Output {
        self.turn += 1;
        match input {
            Input::Move ((x, y)) => {
                self.player.position.x -= x;
                self.player.position.y -= y;
                Output::Move((x, y))
            }
        }
    }
}


pub struct Unit {
    pub block: Block,
    pub role: Option<Role>,
}


struct Map {
    size: (i32, i32),
    data: Vec<Block>,
}


impl Map {
    pub fn new((x, y): (i32, i32)) -> Map {
        let size = (x*y*LAYER) as usize;
        let seed = rand::random();
        let noise = Brownian2::new(perlin2, 4).wavelength(32.0);
        let mut data = Vec::with_capacity(size);
        for i in 0..y {
            for j in 0..x {
                let val = noise.apply(&seed, &[i as f32, j as f32]);
                data.push(
                    if val < 0.0 {
                        Block::River
                    }
                    else {
                        Block::Land
                    }
                );
            }
        }
        for _ in 1..LAYER {
            for _ in 0..y {
                for _ in 0..x {
                    data.push(Block::Nil);
                }
            }
        }
        Map {
            size: (x, y),
            data: data,
        }
    }

    #[inline]
    fn index(&self, i: [i32; 3]) -> usize {
        let (a, b) = self.size;
        let index = (i[2] * a * b + i[1] * a + i[0]) as usize;
        return index;
    }

    #[inline]
    pub fn get(&self, i: [i32; 3]) -> Block {
        if self.index(i) < self.data.len() {
            let index = self.index(i);
            self.data[index].clone()
        }
        else {
            Block::Nil
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn set(&mut self, i: [i32; 3], block: Block) {
        if self.index(i) < self.data.len() {
            let index = self.index(i);
            self.data[index] = block
        }
    }
}

