use math;
use na::Vec2;
use sprite::Sprite;
use animation::State;
use timer::{Ms, Timer};

pub fn rotate(total: Ms) -> State<Sprite> {
    function!(Timer::new(total), move |sprite, timer| {
        if_out!(timer,
            { sprite.transform.rotation = math::rotation(0.0) }
            { sprite.transform.rotation = math::rotation(timer.ratio()) }
        )
    })
}


pub fn fade(ms: Ms, from: f32, to: f32) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        if_out!(timer,
            { sprite.color_multiply.a = to }
            { sprite.color_multiply.a = math::linear(from, to, timer.ratio()) }
        )
    })
}


pub fn move_(ms: Ms, a: Vec2<f32>, b: Vec2<f32>) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        if_out!(timer,
            { sprite.transform.position = b }
            { sprite.transform.position = math::linear(a, b, timer.ratio()) }
        )
    })
}


pub fn curve(ms: Ms, control: [Vec2<f32>; 4]) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        if_out!(timer,
            { sprite.transform.position = control[3] }
            { sprite.transform.position = math::curve(control, timer.ratio()) }
        )
    })
}


pub fn fade_in(ms: Ms) -> State<Sprite> { fade(ms, 0.0, 1.0) }


pub fn fade_out(ms: Ms) -> State<Sprite> { fade(ms, 1.0, 0.0) }
