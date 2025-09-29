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

    /// Get the most recently added item (newest)
    pub fn peek_newest(&self) -> Option<&T> {
        self.buffer.back()
    }

    /// Get the oldest item in the buffer
    pub fn peek_oldest(&self) -> Option<&T> {
        self.buffer.front()
    }

    /// Remove and return the oldest item
    pub fn pop_oldest(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    /// Get item by index (0 = oldest, len-1 = newest)
    pub fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    /// Get mutable reference by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.buffer.get_mut(index)
    }

    /// Number of items currently in buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }

    /// Buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Convert to Vec (oldest first)
    pub fn to_vec(&self) -> Vec<&T> {
        self.buffer.iter().collect()
    }

    /// Iterate from oldest to newest
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter()
    }

    /// Iterate from newest to oldest
    pub fn iter_rev(&self) -> impl Iterator<Item = &T> {
        self.buffer.iter().rev()
    }

    /// Drain items from the buffer
    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.buffer.drain(..)
    }
}