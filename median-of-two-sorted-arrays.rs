// https://leetcode.com/problems/median-of-two-sorted-arrays

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = vec![];
        let mut left_index = 0;
        let mut right_index = 0;
        while left_index < nums1.len() && right_index < nums2.len() {
            if nums1[left_index] < nums2[right_index] {
                merged.push(nums1[left_index]);
                left_index += 1;
            } else {
                merged.push(nums2[right_index]);
                right_index += 1;
            }
        }

        merged.extend(&nums1[left_index..]);
        merged.extend(&nums2[right_index..]);
        
        let mid = merged.len() / 2;
        if merged.len() % 2 == 0 {
            (merged[mid - 1] + merged[mid]) as f64 / 2.0
        } else {
            merged[mid] as f64
        }
    }
    
}
