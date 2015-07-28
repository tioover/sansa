use na::Vec2;
use timer::Ms;
use ::{Engine, Context};
pub use glium::glutin::{MouseScrollDelta, ElementState, ScanCode,
                        VirtualKeyCode, MouseButton};


#[derive(Debug)]
pub struct Event {
    pub mouse: Vec2<f32>,
    pub mouse_press: Option<MouseButton>,
    pub mouse_release: Option<MouseButton>,
    pub scroll: Option<MouseScrollDelta>,
    pub key_press: Vec<(ScanCode, Option<VirtualKeyCode>)>,
    pub key_release: Vec<(ScanCode, Option<VirtualKeyCode>)>,
    pub string: String,
    pub closed: bool,
}


impl Event {
    pub fn new(context: &Context) -> Event {
        use glium::glutin::Event::*;
        use glium::glutin::ElementState::*;

        let display = context.display;
        let f = context.hidpi_factor;
        let size = {
            let (w, h) = context.display.get_framebuffer_dimensions();
            Vec2::new(w as f32, h as f32)
        };

        let mut string = String::new();
        let mut mouse_position = Vec2::new(0.0, 0.0);
        let mut mouse_press = None;
        let mut mouse_release = None;
        let mut scroll = None;
        let mut key_press = Vec::new();
        let mut key_release = Vec::new();
        let mut closed = false;
        for event in display.poll_events() {
            match event {
                MouseMoved ((x, y)) => {
                    let position = Vec2::new(x as f32, y as f32);
                    let mut a = (position - size/2.0) / f;
                    a.y = -a.y;
                    mouse_position = a;
                }
                MouseWheel (x) => scroll = Some (x),
                MouseInput (Released, x) => mouse_release = Some (x),
                MouseInput (Pressed, x) => mouse_press = Some (x),
                KeyboardInput (Released, x, y) => key_release.push((x, y)),
                KeyboardInput (Pressed, x, y) => key_press.push((x, y)),
                ReceivedCharacter (c) => string.push(c),
                Closed => closed = true,
                _ => {}
            }
        }

        Event {
            mouse: mouse_position,
            mouse_press: mouse_press,
            mouse_release: mouse_release,
            scroll: scroll,
            key_press: key_press,
            key_release: key_release,
            string: string,
            closed: closed,
        }
    }
}


pub trait Update {
    fn update(&mut self, delta: Ms, event: Box<Event>) -> Box<Event>;
}




pub fn update(engine: &Engine, xs: Vec<&mut Update>) -> Box<Event> {
    let mut event = box Event::new(&engine.context);
    let delta = engine.timer.delta;

    for x in xs {
        event = x.update(delta, event);
    }
    return event;
}

