// https://leetcode.com/problems/find-peak-element


impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 { return 0 }
        if nums[0] > nums[1] { return 0 }
        if nums[nums.len()-1] > nums[nums.len()-2] {return (nums.len()-1) as i32}

        let mut cur = 0;
        while cur < nums.len(){
            cur +=1;
            if nums[cur - 1] < nums[cur] && nums[cur] > nums[cur + 1] {
                break
            }
        }
        cur as i32
    }
}


//////////////////////////////////////////////////////////

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left)/2;

            if nums[mid] < nums[mid+1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}