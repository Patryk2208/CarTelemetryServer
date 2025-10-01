use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct RefreshRate {
    pub rate: Duration
}
impl RefreshRate {
    const SLOW: Duration = Duration::from_nanos(66_666_666);   // 15 Hz
    const MEDIUM: Duration = Duration::from_nanos(33_333_333); // 30 Hz
    const FAST: Duration = Duration::from_nanos(16_666_666);   // 60 Hz
    pub fn speed_up(&mut self) {
        match self.rate {
            Self::SLOW => self.rate = Self::MEDIUM,
            Self::MEDIUM => self.rate = Self::FAST,
            Self::FAST => self.rate = Self::FAST,
            _ => self.rate = Self::SLOW
        }
    }
    pub fn slow_down(&mut self) {
        match self.rate { 
            Self::SLOW => self.rate = Self::SLOW,
            Self::MEDIUM => self.rate = Self::SLOW,
            Self::FAST => self.rate = Self::MEDIUM,
            _ => self.rate = Self::SLOW
        }
    }
}

pub struct FlowControl {
    pub iteration_start: Instant,
    pub send_duration: Duration,
    pub refresh_rate: RefreshRate
}

impl FlowControl {
    pub fn new() -> Self {
        Self {
            iteration_start: Instant::now(),
            send_duration: Duration::from_nanos(0),
            refresh_rate: RefreshRate { rate: RefreshRate::FAST }
        }
    }
    pub fn start_iteration(&mut self) {
        self.iteration_start = Instant::now();
    }

    pub fn complete_iteration(&mut self) {
        self.send_duration = self.iteration_start.elapsed();
        if self.send_duration >= self.refresh_rate.rate {
            self.refresh_rate.slow_down();
        } else if self.send_duration < self.refresh_rate.rate / 2 {
            self.refresh_rate.speed_up();
        } else {
            sleep(self.refresh_rate.rate - self.send_duration);
        }
    }
}