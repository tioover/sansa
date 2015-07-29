#![feature(box_syntax, box_patterns, slice_patterns)]
#[macro_use]
extern crate glium;
extern crate image;
extern crate num;
extern crate nalgebra as na;
extern crate rand;
extern crate noise;
extern crate uuid;
extern crate time;
#[macro_use]
extern crate engine;

macro_rules! na {
    ($x: expr) =>
        (::na::Vec1::new($x));
    ($x: expr, $y: expr) =>
        (::na::Vec2::new($x, $y));
    ($x: expr, $y: expr, $z: expr) =>
        (::na::Vec3::new($x, $y, $z));
    ($x: expr, $y: expr, $z: expr, $w: expr) =>
        (::na::Vec4::new($x, $y, $z, $w));
}

mod object;
mod game;
mod tile;

use std::path::PathBuf;
use na::Vec2;
use glium::Display;
use engine::{Texture, Manager, UIBuilder, Sprite, Update, Text,
             Engine, build_display, update};
use engine::timer::Ms;
use object::Block;
use tile::{Tile, TileGen};
use game::{Game, Input, Output};

const TILE: &'static str = "assets/tile";
const FONT: &'static str = "assets/font.otf";



fn game_path() -> PathBuf {
    PathBuf::new()
}


struct Env<'a> {
    #[allow(dead_code)]
    display: &'a Display,
    font: PathBuf,
    engine: Engine<'a>,
    textures: Manager<'a, Texture>,

}


impl<'a> Env<'a> {
    fn new(display: &'a Display) -> Env<'a> {
        let path = game_path();
        Env {
            display: display,
            engine: Engine::new(display),
            textures: Manager::new(display, path.join(TILE)),
            font: path.join(FONT),
        }
    }

    fn update(&mut self) {
        self.engine.update();
        std::thread::sleep_ms(5);
    }

    #[inline]
    fn text(&self) -> Text {
        self.engine.text(self.font.clone())
    }

    #[inline]
    fn now(&self) -> Ms {
        self.engine.timer.now()
    }
}


fn main() {
    let turn_time = 250;
    let display = build_display("sansa".to_string(), (800, 600));
    let mut env = Env::new(&display);

    let mut game = game::Game::new((1000, 1000));
    let tile = TileGen::new(&env.textures);
    let mut last_turn = env.now();
    let mut ground = make_tiles(&game, &tile);
    let mut offset: Vec2<i32> = na::zero();

    let mut text = env.text()
            .size(18)
            .line_spacing(4)
            .underline(1)
            .box_size((500, 100))
            .content("Answer to the Ultimate Question of Life, the Universe, and Everything".to_string())
            .build(&display);

    // {
    //     use engine::sprite::animate;
    //     use engine::animation::State;
    //     let state = animate::rotate(1000);
    //     text.sprite.set_state(repeat!(state));
    // }

    'main: loop {
        let event = { // update
            let mut queue: Vec<&mut Update> = Vec::new();
            queue.push(&mut text);
            update(&env.engine, queue)
        };
        if event.closed { break 'main }
        for e in event.key_press.iter() {
            use engine::event::VirtualKeyCode::*;

            if let &(_, Some (x)) = e {
                offset = match x {
                    W => na![ 1,  1],
                    S => na![-1, -1],
                    A => na![-1,  1],
                    D => na![ 1, -1],
                    _ => na![ 0,  0],
                }
            }
        }
        let now = env.now();
        if now - last_turn >= turn_time {
            let mut camera = &mut env.engine.context.camera;
            ground = make_tiles(&game, &tile);
            let output = game.next(Input::Move ((offset.x, offset.y)));
            last_turn = now;
            match output {
                Output::Move(offset) => {
                    let v = tile.vertical();
                    let h = tile.horizontal();
                    camera.move_(turn_time, na::cast(match offset {
                        ( 0,  0) => na![ 0,  0],
                        ( 1,  1) => na![ 0,  v],
                        (-1, -1) => na![ 0, -v],
                        (-1,  1) => na![-h,  0],
                        ( 1, -1) => na![ h,  0],
                        _ => unreachable!()
                    }));
                }
            }
            offset = na::zero();
        }
        // render
        let mut queue: Vec<_> = ground.iter().collect();
        queue.push(&text.sprite);
        env.engine.render_sprites(queue);
        env.update();
        println!("FPS: {}", env.engine.timer.fps());
    }
}


fn make_tiles(game: &Game, tile: &TileGen) -> Vec<Sprite> {
    let mut sprites = Vec::new();
    for k in 0..game::LAYER {
        for j in -10..10 {
            for i in -10..10 {
                let unit = game.get([i, j, k]);
                let block = unit.block;
                let role = unit.role;
                let pos = (-i+k, -j+k);
                if let Block::Nil = block {} else {
                    sprites.push(tile.sprite(block.offset(), pos));
                }
                if let Some(role) = role {
                    sprites.push(tile.sprite(role.offset(), pos));
                }
            }
        }
    }
    return sprites;
}
