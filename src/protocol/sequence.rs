//! Sequence number management for packet headers

use std::cell::Cell;

/// Sequence counter for packet headers (wrapping around 255)
pub struct SequenceCounter {
    counter: Cell<u8>,
}

#[allow(dead_code)]
impl SequenceCounter {
    /// Create a new sequence counter
    pub fn new() -> Self {
        Self {
            counter: Cell::new(0),
        }
    }

    /// Get the next sequence number
    pub fn next(&self) -> u8 {
        let current = self.counter.get();
        let next = if current == 0 {
            1
        } else {
            let n = current.wrapping_add(1);
            if n == 0 {
                1 // Skip 0, wrap to 1
            } else {
                n
            }
        };
        self.counter.set(next);
        next
    }

    /// Increment counter
    pub fn increment(&self) {
        let current = self.counter.get();
        let next = if current == 0 {
            1
        } else {
            let n = current.wrapping_add(1);
            if n == 0 { 1 } else { n }
        };
        self.counter.set(next);
    }
}

impl Default for SequenceCounter {
    fn default() -> Self {
        Self::new()
    }
}
