https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array

// O(n) ?

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let max_i = nums.len() - 1;
        let mut first = -1;
        let mut last = -1;

        if nums.len() == 0 { return vec![first, last]}

        while i <= max_i {
            if first < 0 {
                if nums[i] == target { first = i as i32 }
            }
            if last < 0 {
                if nums[max_i-i] == target { last = (max_i-i) as i32 }
            }
            if last >= 0 && first >=0 { break }
            
            i+=1;
        }

        vec![first, last]
    }
}
