// https://leetcode.com/problems/top-k-frequent-elements

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug)]
struct Item {
    num: i32,
    count: usize,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut stats = HashMap::new();

        for num in nums {
            *stats.entry(num).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::with_capacity(k as usize + 1);

        for (num, cnt) in stats {
            heap.push(Item { num: num, count: cnt });
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        while let Some(item) = heap.pop() {
            result.push(item.num);
        }

        result
    }
}


////////////////////////////////////////////////// AI

use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts = HashMap::new();
        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Rust compares tuples by first element (count) automatically
        let mut heap: BinaryHeap<(usize, i32)> = counts.into_iter().map(|(num, cnt)| (cnt, num)).collect();
        
        (0..k).map(|_| heap.pop().unwrap().1).collect()
    }
}




use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        
        let mut counts = HashMap::with_capacity(n);
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        // Buckets size must be `n + 1` because max possible frequency is `n`
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (num, count) in counts {
            buckets[count].push(num);
        }

        let mut res = Vec::with_capacity(k);
        for bucket in buckets.into_iter().rev() {
            for num in bucket {
                res.push(num);
                if res.len() == k {
                    return res;
                }
            }
        }

        res
    }
}