// https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let lbytes = labels.as_bytes();

        let mut neighbors = vec![vec![]; n];

        for edge_n in 0..edges.len() {
            let edge = &edges[edge_n];
            neighbors[edge[0] as usize].push((edge[2], edge[1] as usize));
        }

        let mut costs = vec![vec![i32::MAX; k+1]; n];
        costs[0][1] = 0;

        let mut que = BinaryHeap::new();
        que.push(Reverse((0, 0, 1)));

        while let Some(Reverse((total, at, consecutive))) = que.pop() {
            if at == n-1 { return total }
            if total > costs[at][consecutive] {
                continue;
            }

            for (cost, to) in &neighbors[at] {
                let cur_consecutive = if &lbytes[*to] == &lbytes[at] {
                    consecutive + 1
                } else {
                    1
                };
                if cur_consecutive > k {
                    continue;
                }
                
                let new_total = total + cost;

                if costs[*to][cur_consecutive] > new_total {
                    costs[*to][cur_consecutive] = new_total;
                    que.push(Reverse((new_total, *to, cur_consecutive)));
                }    
            }
        }
        -1
    }
}