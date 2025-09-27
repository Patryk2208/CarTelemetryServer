use std::time::{Duration, Instant};

const ITERATION_TIME: Duration = Duration::from_nanos(1_000_000_000 / 60);

pub struct FlowControl {
    pub iteration_start: Instant,
    pub desired_iteration_time: Duration,
    pub delta_iteration_time: Duration,
    pub estimated_iteration_time_no_send: Duration,
    iteration_count: u32,
    pub sending_start: Instant,
    pub estimated_sending_time: Duration
}

impl FlowControl {
    pub fn confirm_concat_decide_send(&mut self) -> bool {
        let elapsed = Instant::now().duration_since(self.iteration_start);
        self.estimated_iteration_time_no_send = ((self.estimated_iteration_time_no_send * self.iteration_count) + elapsed) / (self.iteration_count + 1);
        let under = self.desired_iteration_time.abs_diff(elapsed + self.estimated_sending_time);
        let over = self.desired_iteration_time.abs_diff(elapsed + 2 * self.estimated_sending_time + self.estimated_iteration_time_no_send);
        self.sending_start = Instant::now();
        if under < over {
            true
        } else {
            false
        }
    }

    pub fn confirm_message_sent(&mut self) {
        let elapsed = Instant::now().duration_since(self.sending_start);
        self.estimated_sending_time = ((self.estimated_sending_time * self.iteration_count) + elapsed) / (self.iteration_count + 1);
        self.desired_iteration_time -= elapsed;
        self.delta_iteration_time = ((self.delta_iteration_time * self.iteration_count) + elapsed) / (self.iteration_count + 1);

        self.iteration_start = Instant::now();
        self.desired_iteration_time = ITERATION_TIME;
        self.iteration_count += 1;
    }
}