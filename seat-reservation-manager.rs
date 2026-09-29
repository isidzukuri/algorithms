// https://leetcode.com/problems/seat-reservation-manager

use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Default)]
struct SeatManager {
    cur: i32,
    heap: BinaryHeap<Reverse<i32>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        Self::default()
    }
    
    fn reserve(&mut self) -> i32 {
        if self.heap.is_empty() {
            self.cur += 1;
            self.cur
        } else {
            self.heap.pop().unwrap().0
        }
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(Reverse(seat_number))
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */