use time;
use std::collections::VecDeque;

pub type Ns = u64;
pub type Ms = u64;

const SECOND_NS: Ns = 1_000_000_000;
const MS_NS: Ns = SECOND_NS / 1000;

pub fn now() -> Ms {
    time::precise_time_ns() / (SECOND_NS/1000)
}


pub struct GlobalTimer {
    updates: VecDeque<Ns>,
    now: Ns,
    pub delta: Ms,
}


impl GlobalTimer {
    pub fn new() -> GlobalTimer {
        let now = time::precise_time_ns();
        GlobalTimer {
            updates: VecDeque::with_capacity(128),
            now: now,
            delta: 0,
        }
    }

    // Thanks https://github.com/PistonDevelopers/fps_counter/
    pub fn update(&mut self) {
        let now = time::precise_time_ns();
        let a_second_ago = now - SECOND_NS;
        self.delta = (now - self.now) / MS_NS; // microsecond
        self.now = now;
        while self.updates.front().map_or(false, |t| *t < a_second_ago) {
            self.updates.pop_front();
        }
        self.updates.push_back(now);
    }

    pub fn fps(&self) -> usize {
        self.updates.len()
    }

    pub fn now(&self) -> Ms {
        self.now / MS_NS
    }
}


#[derive(Clone)]
pub struct LocalTimer {
    pub total: Ms,
    pub delta: Ms,
    pub now  : Ms,
}


impl LocalTimer {
    pub fn new(time: Ms) -> LocalTimer {
        LocalTimer { total: time, now: 0, delta: 0 }
    }

    pub fn empty() -> LocalTimer {
        LocalTimer::new(0)
    }

    pub fn ratio(&self) -> f32 {
        self.now as f32 / self.total as f32
    }

    pub fn is_out(&self) -> bool {
        self.now > self.total
    }

    pub fn update(&mut self, delta: Ms) {
        self.delta = delta;
        self.now += delta;
    }
}
