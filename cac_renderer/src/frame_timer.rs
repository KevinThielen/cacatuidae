use std::time::Instant;

pub struct FrameTimer {
    pub delta_time: f64,
    pub timer: f64,
    repeat: bool,
    current: f64,
    previous_time: Instant,
}

impl FrameTimer {
    pub fn with_repeated(timer: f64) -> Self {
        Self {
            delta_time: 0.0,
            timer,
            repeat: true,
            current: 0.0,
            previous_time: Instant::now(),
        }
    }

    pub fn done(&mut self) -> bool {
        if self.current >= self.timer {
            if self.repeat {
                self.current = 0.0;
            }
            true
        } else {
            false
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.previous_time).as_secs_f64();
        self.current += self.delta_time;
        self.previous_time = now;
    }

    pub fn tick_done(&mut self) -> bool {
        self.tick();
        self.done()
    }
}
