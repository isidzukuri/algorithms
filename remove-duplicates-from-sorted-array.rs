// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut counter = 1;
        let mut i = nums.len() -1;
        loop {
            if i == 0 { break } 
            if nums[i] == nums[i-1] { 
                nums.remove(i);
            } else {
                counter += 1 
            }
            i -= 1;
        }
        counter
    }
}
