https://leetcode.com/problems/plus-one/

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut last_i = digits.len() - 1;
        let mut result = digits.clone();
        let mut sum = digits[last_i] + 1;
        loop {
            if sum < 10 {
                result[last_i] = sum;
                break 
            } else {
                result[last_i] = sum - 10;
                if last_i == 0 {
                    result.insert(0, 1);
                    break
                }
                sum = result[last_i - 1] + 1
            }
            last_i -=1;
        }
        result
    }
}
