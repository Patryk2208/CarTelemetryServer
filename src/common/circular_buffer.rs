use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) -> Option<T> {
        if self.buffer.len() == self.capacity {
            let removed = self.buffer.pop_front();
            self.buffer.push_back(item);
            removed
        } else {
            self.buffer.push_back(item);
            None
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}