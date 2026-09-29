// https://leetcode.com/problems/jump-game

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last_idx = nums.len()-1;
        let mut max_reach = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i > max_reach {
                return false
            }

            max_reach = max_reach.max(i + num as usize);
            if max_reach >= last_idx { return true }
        }
        true
    }
}


use std::collections::HashMap;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let last_idx = nums.len()-1;
        let mut stack = vec![0];
        let mut no_path = HashMap::new();

        while let Some(cur) = stack.pop() {
            if cur == last_idx { return true }
            if cur > last_idx { continue }
            if no_path.contains_key(&cur) { continue }

            let max_jumps = nums[cur] as usize;
            if max_jumps == 0 { continue }

            let mut jumps = 0;
            while jumps < max_jumps {
                jumps += 1;
                let next = cur + jumps;
                stack.push(next);
            }
            no_path.insert(cur, true);
        }        
        false
    }
}