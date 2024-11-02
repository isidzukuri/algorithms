// https://leetcode.com/problems/time-based-key-value-store

use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    fn new() -> Self {
        Self::default()
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.store.entry(key).or_default().push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        match self.store.get(&key) {
            Some(val_snaps) => { 
                return match val_snaps.partition_point(|&(ts, _)| ts <= timestamp) {
                    0 => Self::default_value(),
                    partition_point => val_snaps[partition_point - 1].1.clone(),
                }
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
