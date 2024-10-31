https://leetcode.com/problems/snapshot-array/

use std::collections::HashMap;

struct SnapshotArray {
    store: HashMap<i32, HashMap<i32, i32>>,
    snap_index: i32
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    const DEFAULT_VALUE: i32 = 0;

    fn new(length: i32) -> Self {
        Self {
            store: HashMap::new(),
            snap_index: 0
        }
    }
    
    fn set(&mut self, index: i32, val: i32) {
        match self.store.get_mut(&index) {
            Some(val_snaps) => { 
                val_snaps.insert(self.snap_index.clone(), val); 
            },
            _ => {
               let mut first_snap = HashMap::new();
                first_snap.insert(self.snap_index, val);
                self.store.insert(index, first_snap);
            }
        };
    }
    
    fn snap(&mut self) -> i32 {
        self.snap_index += 1;
        self.snap_index - 1
    }
    
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.store.get(&index) {
            Some(val_snaps) => { 
                match val_snaps.get(&snap_id) {
                    Some(val_snap) => { return *val_snap },
                    _ => {
                        if snap_id < *val_snaps.keys().min().unwrap() {
                            return Self::DEFAULT_VALUE
                        } 
                        
                        let last_snap_id = *val_snaps.keys().max().unwrap();
                        if snap_id > last_snap_id{
                           return *val_snaps.get(&last_snap_id).unwrap()
                        }

                        let mut max = 0;
                        let mut keys = val_snaps.keys();
                        while let Some(s_id) = keys.next() {
                            if *s_id < snap_id && max < *s_id { max = *s_id }; 
                        }
                        return *val_snaps.get(&max).unwrap()
                    }
                }
            },
            _ => Self::DEFAULT_VALUE,
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
