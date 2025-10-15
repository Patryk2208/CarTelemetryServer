use tokio::time::sleep;
use std::time::{Duration, Instant};

pub struct RefreshRate {
    pub rate: Duration
}
impl RefreshRate {
    pub const SLOW: Duration = Duration::from_nanos(66_666_666);   // 15 Hz
    pub const MEDIUM: Duration = Duration::from_nanos(33_333_333); // 30 Hz
    pub const FAST: Duration = Duration::from_nanos(16_666_666);   // 60 Hz
    pub const ULTRAFAST: Duration = Duration::from_nanos(5_000_000); //testable
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
    pub last_iteration_start: Option<Instant>,
    pub iteration_start: Instant,
    pub send_duration: Duration,
    pub refresh_rate: RefreshRate
}

impl FlowControl {
    pub fn new(refresh_rate: RefreshRate) -> Self {
        Self {
            last_iteration_start: None,
            iteration_start: Instant::now(),
            send_duration: Duration::from_nanos(0),
            refresh_rate: refresh_rate
        }
    }
    pub fn start_iteration(&mut self) {
        if self.last_iteration_start.is_none() {
            self.iteration_start = Instant::now();
        } else {
            self.iteration_start = self.last_iteration_start.unwrap() + self.refresh_rate.rate;
        }
    }

    pub async fn complete_iteration(&mut self) {
        self.send_duration = self.iteration_start.elapsed();
        self.last_iteration_start = Some(self.iteration_start);
        if self.send_duration >= self.refresh_rate.rate {
            self.refresh_rate.slow_down();
            return;
        } else if self.send_duration < self.refresh_rate.rate / 2 {
            self.refresh_rate.speed_up();
        }
        sleep(self.refresh_rate.rate - self.send_duration).await;
    }
}