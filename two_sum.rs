// https://leetcode.com/problems/two-sum/

// V1 [memory]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut parts = vec![];

        for (idx, num) in nums.iter().enumerate() {
            let new_part = target - num;

            match parts.iter().position(|&item| item == new_part){
                Some(part_idx) => { return vec![idx as i32, part_idx as i32]; },
                None => { parts.push(*num); }
            }
        }

        vec![]
    }
}



// V2 [speed]
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut available_parts:  HashMap<i32, usize> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let new_part = target - num;

            match available_parts.get(&num) {
                Some(part_idx) => { return vec![idx as i32, *part_idx as i32]; }
                None => { available_parts.insert(new_part, idx); }
            }
        }

        vec![]
    }
}


// V3
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let max_n = nums.len() -1 ;
        let mut n = 0;

        while n < max_n {
            for (i, num) in nums.iter().enumerate() {
                if i == n { continue } 
                let sum = nums[n] + num;
                if(sum == target) { return vec![n as i32, i as i32] }
            }
            n += 1;
        } 

        return vec![]  
    }
}




      
