// https://leetcode.com/problems/kth-largest-element-in-an-array/

use std::collections::HashMap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut stats = HashMap::new();

        for num in nums {
            *stats.entry(num).or_insert(0) += 1;
        }

        let mut keys: Vec<&i32> = stats.keys().collect();
        keys.sort_unstable_by(|aa, bb| bb.cmp(aa));

        let mut position = 0;
        let mut keys_iterator = keys.iter();
        while let Some(key) = keys_iterator.next() {
            position += stats[key];

            if k <= position {
                return **key
            }
        }    
        0
    }
}


//////////////////////////////////////////////////////


use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut min_heap = BinaryHeap::with_capacity(k + 1);

        for num in nums {
            min_heap.push(Reverse(num));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        min_heap.pop().unwrap().0
    }
}

////////////////////////////////////////////////// AI

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let target_idx = nums.len() - k as usize;
        let (_, &mut val, _) = nums.select_nth_unstable(target_idx);
        val
    }
}