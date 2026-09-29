// https://leetcode.com/problems/relative-ranks/

const GOLD: &str = "Gold Medal";
const SILVER: &str = "Silver Medal";
const BRONZE: &str = "Bronze Medal";

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        score
        .iter()
        .map(|&item| {
            let position = sorted.binary_search_by(|&probe| item.cmp(&probe)).unwrap() + 1;

            match position {
                1 => GOLD.to_string(),
                2 => SILVER.to_string(),
                3 => BRONZE.to_string(),
                th => th.to_string(),
            }
        })
        .collect()
    }
}


////////////////////////////////////////////// AI


use std::collections::BinaryHeap;

const GOLD: &str = "Gold Medal";
const SILVER: &str = "Silver Medal";
const BRONZE: &str = "Bronze Medal";

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let n = score.len();
        // Push (score, original_index) into Max-Heap
        let mut heap: BinaryHeap<(i32, usize)> = score
            .iter()
            .enumerate()
            .map(|(idx, &s)| (s, idx))
            .collect();

        let mut res = vec![String::new(); n];
        let mut rank = 1;

        // Pop elements in descending order of score
        while let Some((_, original_idx)) = heap.pop() {
            res[original_idx] = match rank {
                1 => GOLD.to_string(),
                2 => SILVER.to_string(),
                3 => BRONZE.to_string(),
                th => th.to_string(),
            };
            rank += 1;
        }

        res
    }
}