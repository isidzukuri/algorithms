// https://leetcode.com/problems/time-based-key-value-store

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct TimeMap {
    store: HashMap<String, BinaryHeap<Item>>
}

#[derive(PartialEq, Eq, Clone)]
struct Item {
    pub id: i32,
    pub value: String
}

impl Item {
    fn new(id: i32, value: String) -> Self {
        Self { id: id, value: value }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let item = Item::new(timestamp, value);
        match self.store.get_mut(&key) {
            Some(val_snaps) => { val_snaps.push(item); },
            _ => {
                let mut first_snap = BinaryHeap::new();
                first_snap.push(item);
                self.store.insert(key, first_snap);
            }
        };
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        match self.store.get(&key) {
            Some(heap) => { 
                let val_snaps = heap.clone().into_sorted_vec();

                if val_snaps[0].id > timestamp { return Self::default_value() }

                let mut low = 0 as i32;
                let mut high = (val_snaps.len() - 1) as i32;
                let mut max = val_snaps.first().unwrap();
                while low <= high {
                    let mid = (low + high) / 2;
                    let item = &val_snaps[mid as usize];
                    match item.id.cmp(&timestamp) {
                        Ordering::Less => {
                            if max.id < item.id { max = &item }
                            low = mid + 1
                        },
                        Ordering::Greater => {
                            high = mid - 1},
                        Ordering::Equal => return item.value.clone(),
                    }
                }
                return max.value.clone();           
            },
            _ => Self::default_value(),
        }
    }

    fn default_value() -> String{
        "".to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
