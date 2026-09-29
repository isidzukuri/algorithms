// https://leetcode.com/problems/find-missing-elements

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums;
        sorted.sort_unstable();
        let mut missing_count = (&sorted[&sorted.len()-1] - &sorted[0]) as usize - sorted.len() + 1;
        let mut res = vec![];
        let mut idx = 0;

        while missing_count > 0 {
            let num = sorted[idx];
            let mut diff = sorted[idx+1] - num;
            for add in 1..diff {
                res.push(num + add);
                missing_count -= 1;
            }
            idx +=1;
        }

        res
    }
}





/////////////////////////////////// AI

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return Vec::new();
        }

        nums.sort_unstable();

        // Calculate missing count to allocate exact memory needed upfront
        let min_val = nums[0];
        let max_val = nums[nums.len() - 1];
        let estimated_missing = ((max_val - min_val) as usize + 1).saturating_sub(nums.len());

        let mut res = Vec::with_capacity(estimated_missing);

        // Process pairwise adjacent numbers safely using windows
        for w in nums.windows(2) {
            let (curr, next) = (w[0], w[1]);
            if next > curr + 1 {
                res.extend((curr + 1)..next);
            }
        }

        res
    }
}

use std::collections::HashSet;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return Vec::new();
        }

        let mut min_val = i32::MAX;
        let mut max_val = i32::MIN;
        let mut set = HashSet::with_capacity(nums.len());

        for &num in &nums {
            min_val = min_val.min(num);
            max_val = max_val.max(num);
            set.insert(num);
        }

        let total_range = (max_val as i64 - min_val as i64 + 1) as usize;
        let missing_count = total_range.saturating_sub(set.len());
        let mut res = Vec::with_capacity(missing_count);

        for val in (min_val + 1)..max_val {
            if !set.contains(&val) {
                res.push(val);
            }
        }

        res
    }
}



impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return Vec::new();
        }

        let mut min_val = i32::MAX;
        let mut max_val = i32::MIN;

        for &num in &nums {
            min_val = min_val.min(num);
            max_val = max_val.max(num);
        }

        let range = (max_val - min_val + 1) as usize;
        let mut present = vec![false; range];

        for &num in &nums {
            present[(num - min_val) as usize] = true;
        }

        let mut res = Vec::with_capacity(range.saturating_sub(nums.len()));

        for i in 0..range {
            if !present[i] {
                res.push(min_val + i as i32);
            }
        }

        res
    }
}