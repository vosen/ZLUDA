//! A ring allocator for the short-lived objects of each launch: uniforms, pipeline words, control streams and
//! parameter blocks. Each allocation belongs to the queue point of the command that reads it and is freed once that
//! point has signalled. Points complete in order, so the ring is freed from its oldest end.

use std::collections::VecDeque;

#[derive(Debug)]
pub struct RingAlloc {
    size: u64,
    /// Next free offset.
    head: u64,
    /// (point, start, end) of the live allocations, oldest first.
    live: VecDeque<(u64, u64, u64)>,
}

impl RingAlloc {
    pub fn new(size: u64) -> RingAlloc {
        RingAlloc { size, head: 0, live: VecDeque::new() }
    }

    /// `len` bytes aligned to `align`, owned by `point`; `None` if they do not fit until older points retire.
    pub fn alloc(&mut self, len: u64, align: u64, point: u64) -> Option<u64> {
        let len = len.max(1);
        if len > self.size {
            return None;
        }
        if self.live.is_empty() {
            self.head = 0;
        }
        let at = match self.live.front() {
            None => 0,
            Some(&(_, tail, _)) => {
                let a = self.head.next_multiple_of(align);
                if self.head >= tail && !(self.head == tail && !self.live.is_empty()) {
                    // free space is [head, size) and [0, tail)
                    if a + len <= self.size {
                        a
                    } else if len <= tail {
                        0
                    } else {
                        return None;
                    }
                } else if self.head < tail && a + len <= tail {
                    a
                } else {
                    return None;
                }
            }
        };
        self.head = at + len;
        self.live.push_back((point, at, at + len));
        Some(at)
    }

    /// Frees the allocations of every point up to `done`.
    pub fn retire(&mut self, done: u64) {
        while self.live.front().is_some_and(|&(p, _, _)| p <= done) {
            self.live.pop_front();
        }
    }

    /// Drops the allocations of a point whose command was never submitted.
    pub fn rollback(&mut self, point: u64) {
        while let Some(&(p, start, _)) = self.live.back() {
            if p != point {
                break;
            }
            self.live.pop_back();
            self.head = start;
        }
    }

    /// The oldest point that still holds memory.
    pub fn oldest(&self) -> Option<u64> {
        self.live.front().map(|&(p, _, _)| p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_and_waits_for_the_oldest_point() {
        let mut r = RingAlloc::new(100);
        assert_eq!(r.alloc(40, 8, 1), Some(0));
        assert_eq!(r.alloc(40, 8, 2), Some(40));
        // 20 left at the end, 0 at the start until point 1 retires
        assert_eq!(r.alloc(30, 8, 3), None);
        assert_eq!(r.oldest(), Some(1));
        r.retire(1);
        assert_eq!(r.alloc(30, 8, 3), Some(0));
        // between head (30) and tail (40)
        assert_eq!(r.alloc(8, 8, 3), Some(32));
        assert_eq!(r.alloc(8, 8, 3), None);
        r.retire(3);
        assert_eq!(r.alloc(100, 8, 4), Some(0));
        assert_eq!(r.alloc(1, 1, 5), None);
        assert_eq!(r.alloc(101, 1, 5), None);
    }

    #[test]
    fn rollback_frees_an_unsubmitted_point() {
        let mut r = RingAlloc::new(64);
        assert_eq!(r.alloc(16, 16, 1), Some(0));
        assert_eq!(r.alloc(16, 16, 2), Some(16));
        assert_eq!(r.alloc(16, 16, 2), Some(32));
        r.rollback(2);
        assert_eq!(r.alloc(32, 16, 2), Some(16));
        assert_eq!(r.alloc(16, 16, 2), Some(48));
        assert_eq!(r.alloc(1, 1, 2), None);
    }

    #[test]
    fn many_points_cycle_through_the_ring() {
        let mut r = RingAlloc::new(4096);
        let mut done = 0;
        for point in 1..10_000u64 {
            loop {
                if let Some(a) = r.alloc(100 + point % 300, 16, point) {
                    assert!(a % 16 == 0 && a + 100 + point % 300 <= 4096);
                    break;
                }
                done = r.oldest().unwrap();
                r.retire(done);
            }
        }
        assert!(done > 9000);
    }
}
