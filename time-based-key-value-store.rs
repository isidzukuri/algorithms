https://leetcode.com/problems/time-based-key-value-store/

use std::collections::HashMap;
use std::cmp::Ordering;

struct TimeMap {
    store: HashMap<String, Vec<(i32, String)>>
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
        match self.store.get_mut(&key) {
            Some(val_snaps) => { 
                let pos = val_snaps.binary_search_by(|item| item.0.cmp(&timestamp))
                                   .unwrap_or_else(|elm| elm);
                val_snaps.insert(pos, (timestamp.clone(), value));
            },
            _ => {
               let mut first_snap = Vec::new();
                first_snap.push((timestamp.clone(), value));
                self.store.insert(key, first_snap);
            }
        };
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        match self.store.get(&key) {
            Some(val_snaps) => { 
                if val_snaps.first().unwrap().0 > timestamp { return Self::default_value() }

                let mut low = 0 as i32;
                let mut high = (val_snaps.len() - 1) as i32;
                let mut max = val_snaps.first().unwrap();
                while low <= high {
                    let mid = (low + high) / 2;
                    let item = &val_snaps[mid as usize];
                    match item.0.cmp(&timestamp) {
                        Ordering::Less => {
                            if max.0 < item.0 { max = &item }
                            low = mid + 1
                        },
                        Ordering::Greater => {
                            high = mid - 1},
                        Ordering::Equal => return item.1.clone(),
                    }
                }
                return max.1.clone();           
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
