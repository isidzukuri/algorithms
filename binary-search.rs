// https://leetcode.com/problems/binary-search

use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0 as i32;
        let mut high = (nums.len() - 1) as i32;

        while low <= high {
            let mid = (low + high) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid - 1,
                Ordering::Equal => return mid,
            }
        }
        
        -1
    }
}
