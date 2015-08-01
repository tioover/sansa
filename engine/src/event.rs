use na::Vec2;
use glium::Display;
use glium::glutin;
use timer::Ms;
use engine::Engine;


pub enum Event {
    WindowEvent (glutin::Event),
    Message (String),
}


pub struct EventStream (Vec<Event>);


impl EventStream {
    pub fn new(display: &Display) -> EventStream {
        use glium::glutin::Event::MouseMoved;
        use glium::glutin::ElementState::*;
        let f = display.get_window().unwrap().hidpi_factor();
        let (w, h) = display.get_framebuffer_dimensions();
        let (w, h) = (w as f32, h as f32);

        let events: Vec<_> = display.poll_events().map(|event| match event {
            MouseMoved ((x, y)) => {
                let (x, y) = (x as f32, y as f32);
                MouseMoved((((x - w/2.0)/f) as i32, (-(y - h/2.0)/f)) as i32)
            }
            x => x
        }).collect();
        EventStream (events)
    }
}


pub trait Update {
    fn update(&mut self, delta: Ms, stream: EventStream) -> EventStream;
}



impl<'a> Update for Vec<&'a mut Update> {
    fn update(&mut self, delta: Ms, mut stream: EventStream) -> EventStream {
        for item in self {
            stream = item.update(delta, stream);
        }
        return stream;
    }
}

