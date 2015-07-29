use std::rc::Rc;
use timer::Ms;
use timer::Timer;


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
        $crate::animation::State::Function($crate::timer::Timer::empty(),
                                           ::std::rc::Rc::new($function))
    };
    ($timer: expr, $function: expr) => {
        $crate::animation::State::Function($timer, ::std::rc::Rc::new($function))
    };
);



#[macro_export]
macro_rules! if_out {
    ($timer: expr, $a: block $b: block) => (
        if $timer.is_out()
        { $a; $crate::animation::Return::Become(State::Nil) }
        else { $b; $crate::animation::Return::Remain }
    )
}



impl<T> State<T> {
    pub fn next(&mut self, data: &mut T, delta: Ms) -> Return<T> {
        use self::State::*;

        macro_rules! next (
            ($s: expr) => (
                if let Return::Become(x) = $s.next(data, delta) {
                    $s = x;
                }
            )
        );

        let stop = Return::Become(Nil);

        match *self {
            Nil => {}

            Series(ref mut list) => {
                loop {
                    if list.is_empty() { return stop }
                    let mut front = list.pop().unwrap();
                    if let Nil = front { continue }
                    next!(front);
                    list.push(front);
                    break
                }
            }

            Parallel(ref mut list) => {
                let mut flag = true;
                for state in list {
                    next!(*state);
                    if let Nil = *state {}
                    else { flag = false }
                }
                if flag { return stop }
            }

            Repeat(ref mut index, ref template, ref mut state) => {
                if let Some(0) = *index { return stop }
                if let Nil = **state {
                    *state = template.clone();
                }
                next!(**state);
                if let Some (ref mut i) = *index { *i -= 1 }
            },

            Function(ref mut timer, ref function) => {
                timer.update(delta);
                return function(data, timer)
            }
        }
        Return::Remain
    }
}


macro_rules! state_next_fn (
    ($T: ty) => (
        pub fn next(x: &mut $T, delta: ::timer::Ms) {
            use std::mem::swap;
            let mut state = ::animation::State::Nil;
            swap(&mut state, &mut x.state);
            let new_state = state.next(x, delta);
            x.state = match new_state {
                $crate::animation::Return::Become (new) => new,
                $crate::animation::Return::Remain => state,
            }
        }
    )
);

