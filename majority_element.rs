// https://leetcode.com/problems/majority-element/description/



// Boyer-Moore Majority Vote Algorithm

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut cand = nums[0];

        for num in nums {
            if count == 0 {
                cand = num;
            }

            if cand == num {
                count +=1;
            } else {
                count -=1;
            }
        }
        cand
    }
}