// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut itr = nums.len();
        let mut result = 0;

        loop {
            itr -=1;
            if itr > 1 && nums[itr] == nums[itr-2]{
                nums.remove(itr);
            } else {
                result +=1;
            }          
            if itr == 0 { break }
        }

        result
    }
}


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut itr = nums.len();
        let mut current = i32::MIN;
        let mut count = 0;
        let mut result = 0;

        loop {
            itr -=1;

            let num = nums[itr];
            if num == current {
                count +=1;
            } else {
                current = num;
                count = 1;
            }

            if count > 2 {
                nums.remove(itr);
            } else {
                result +=1;
            }
        
            if itr == 0 { break }
        }

        result
    }
}
