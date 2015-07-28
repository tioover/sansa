use na;
use na::Vec2;
use std::rc::Rc;
use std::mem;
use timer::Ms;
use timer::LocalTimer as Timer;
use sprite::Sprite;
use math;

pub type List<T> = Vec<State<T>>;


pub enum Return<T> {
    Become(State<T>),
    Remain,
}


pub enum State<T> {
    Nil,
    Repeat(Option<usize>, Box<State<T>>, Box<State<T>>),
    Function(Timer, Rc<Fn(&mut T, &mut Timer) -> Return<T>>),
    Series(List<T>),
    Parallel(List<T>),
}


impl<T> Clone for State<T> {
    fn clone(&self) -> State<T> {
        use self::State::*;

        match *self {
            Nil => Nil,
            Repeat(ref a, ref b, ref c) => Repeat(a.clone(), b.clone(), c.clone()),
            Function(ref a, ref b) => Function(a.clone(), b.clone()),
            Series(ref a) => Series(a.clone()),
            Parallel(ref a) => Parallel(a.clone()),
        }
    }
}


pub fn parallel<T>(list: List<T>) -> State<T> {
    State::Parallel(list)
}


pub fn series<T>(mut list: List<T>) -> State<T> {
    list.reverse();
    State::Series(list)
}

#[macro_export]
macro_rules! repeat (
    ($state: expr) => {
        $crate::animation::State::Repeat(None, box $state.clone(), box $state)
    };
    ($n: expr, $f: expr) => {
        $crate::animation::State::Repeat(Some($n), box $state.clone(), box $state)
    };
);

#[macro_export]
macro_rules! function (
    ($function: expr) => {
        $crate::animation::State::Function($crate::timer::LocalTimer::empty(),
                                           ::std::rc::Rc::new($function))
    };
    ($timer: expr, $function: expr) => {
        $crate::animation::State::Function($timer, ::std::rc::Rc::new($function))
    };
);



pub fn rotate(total: Ms) -> State<Sprite> {
    function!(move |sprite, timer| {
        if timer.is_out() {
            Return::Become(State::Nil)
        }
        else {
            sprite.transform.rotation = math::rotation(timer.ratio());
            Return::Remain
        }
    })
}


#[macro_export]
macro_rules! is_out {
    ($timer: expr, $a: block $b: block) => (
        if $timer.is_out()
        { $a; Return::Become(State::Nil) }
        else { $b; Return::Remain }
    )
}


pub fn fade(ms: Ms, from: f32, to: f32) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        is_out!(timer,
            { sprite.color_multiply.a = to }
            { sprite.color_multiply.a = math::linear(from, to, timer.ratio()) }
        )
    })
}


pub fn move_(ms: Ms, a: Vec2<f32>, b: Vec2<f32>) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        is_out!(timer,
            { sprite.transform.position = b }
            { sprite.transform.position = math::linear(a, b, timer.ratio()) }
        )
    })
}


pub fn curve(ms: Ms, control: [Vec2<f32>; 4]) -> State<Sprite> {
    function!(Timer::new(ms), move |sprite, timer| {
        is_out!(timer,
            { sprite.transform.position = control[3] }
            { sprite.transform.position = math::curve(control, timer.ratio()) }
        )
    })
}


pub fn fade_in(ms: Ms) -> State<Sprite> { fade(ms, 0.0, 1.0) }


pub fn fade_out(ms: Ms) -> State<Sprite> { fade(ms, 1.0, 0.0) }


impl<T> State<T> {
    fn next(&mut self, data: &mut T, delta: Ms) -> Return<T> {
        use self::State::*;

        let stop = Return::Become(Nil);
        match *self {
            Nil => {}

            Series(ref mut list) => {
                loop {
                    if list.is_empty() { return stop }
                    let mut front = list.pop().unwrap();
                    if let Nil = front {}
                    else {
                        front.next(data, delta);
                        list.push(front);
                        break
                    }
                }
            }

            Parallel(ref mut list) => {
                let mut flag = true;
                for state in list {
                    state.next(data, delta);
                    if let Nil = *state {}
                    else { flag = false }
                }
                if flag { return stop }
            }

            Repeat(ref mut index, ref template, ref mut state) => {
                if let Some(0) = *index { return stop }
                else {
                    if let Nil = **state {
                        *state = template.clone();
                    }
                    else {
                        state.next(data, delta);
                    }
                }
                if let Some (ref mut i) = *index { *i -= 1 }
            },

            Function(ref mut timer, ref function) => {
                timer.update(delta);
                if let Return::Become(state) = function(data, timer) {
                    return Return::Become(state)
                }
            }
        }
        Return::Remain
    }
}


// pub fn next(sprite: &mut Sprite, delta: Ms) {
//     let mut state = State::Nil;
//     mem::swap(&mut state, &mut sprite.state);
//     let new_state = state.next(sprite, delta);
//     sprite.state = match new_state {
//         Return::Become (new) => new,
//         Return::Remain => state,
//     }
// }

