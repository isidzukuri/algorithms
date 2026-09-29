// https://leetcode.com/problems/design-event-manager

use std::collections::{HashMap, BTreeSet};
use std::cmp::Reverse;

struct Event {
    active: bool,
    priority: i32
}

#[derive(Default)]
struct EventManager {
    btree: BTreeSet<(i32, Reverse<i32>)>, // fix comparison
    items: HashMap<i32, Event>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl EventManager {

    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut mngr = Self::default();
        for item in events {
            let event = Event{ active: true, priority: item[1] };
            mngr.items.insert(item[0], event);
            mngr.btree.insert((item[1], Reverse(item[0])));
        }
        mngr
    }
    
    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        if let Some(event) = self.items.get_mut(&event_id) {
            self.btree.remove(&(event.priority, Reverse(event_id)));
            event.priority = new_priority;
            self.btree.insert((new_priority, Reverse(event_id)));
        }
    }
    
    fn poll_highest(&mut self) -> i32 {
        if let Some((_, Reverse(event_id))) = self.btree.pop_last() {
            if let Some(event) = self.items.get_mut(&event_id){
                event.active = false;
            }
            event_id
        } else {
            -1
        }
    }
}

/**
 * Your EventManager object will be instantiated and called as such:
 * let obj = EventManager::new(events);
 * obj.update_priority(eventId, newPriority);
 * let ret_2: i32 = obj.poll_highest();
 */

 ////////////////////////////////////////////////////////////////////////// AI

 use std::collections::HashMap;

struct HeapItem {
    priority: i32,
    event_id: i32,
}

struct EventManager {
    heap: Vec<HeapItem>,
    pos_map: HashMap<i32, usize>, // maps event_id -> index in heap vector
}

impl EventManager {
    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut heap = Vec::with_capacity(events.len());
        for item in events {
            heap.push(HeapItem {
                priority: item[1],
                event_id: item[0],
            });
        }

        let mut pos_map = HashMap::with_capacity(heap.len());
        for i in 0..heap.len() {
            pos_map.insert(heap[i].event_id, i);
        }

        let mut mngr = Self { heap, pos_map };

        // Floyd's heap construction (O(N) build time)
        if !mngr.heap.is_empty() {
            for i in (0..mngr.heap.len() / 2).rev() {
                mngr.sift_down(i);
            }
        }

        mngr
    }
    
    fn update_priority(&mut self, event_id: i32, new_priority: i32) {
        if let Some(&idx) = self.pos_map.get(&event_id) {
            let old_priority = self.heap[idx].priority;
            self.heap[idx].priority = new_priority;

            if new_priority > old_priority {
                self.sift_up(idx);
            } else if new_priority < old_priority {
                self.sift_down(idx);
            }
        }
    }
    
    fn poll_highest(&mut self) -> i32 {
        if self.heap.is_empty() {
            return -1;
        }

        let top_event_id = self.heap[0].event_id;
        self.pos_map.remove(&top_event_id);

        if self.heap.len() == 1 {
            self.heap.pop();
            return top_event_id;
        }

        // Move the last element to the root and sift down
        let last = self.heap.pop().unwrap();
        self.heap[0] = last;
        self.pos_map.insert(self.heap[0].event_id, 0);
        self.sift_down(0);

        top_event_id
    }

    // Helper: returns true if 'a' has higher precedence than 'b'
    // Rule: Higher priority first, then smaller event_id first.
    fn greater(a: &HeapItem, b: &HeapItem) -> bool {
        if a.priority != b.priority {
            a.priority > b.priority
        } else {
            a.event_id < b.event_id
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.pos_map.insert(self.heap[i].event_id, j);
        self.pos_map.insert(self.heap[j].event_id, i);
        self.heap.swap(i, j);
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if Self::greater(&self.heap[idx], &self.heap[parent]) {
                self.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let len = self.heap.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && Self::greater(&self.heap[left], &self.heap[largest]) {
                largest = left;
            }
            if right < len && Self::greater(&self.heap[right], &self.heap[largest]) {
                largest = right;
            }

            if largest != idx {
                self.swap(idx, largest);
                idx = largest;
            } else {
                break;
            }
        }
    }
}