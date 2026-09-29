// https://leetcode.com/problems/network-delay-time

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut dist = vec![i32::MAX; n as usize];

        dist[(k-1) as usize] = 0;
        let mut que = BinaryHeap::new();

        que.push(Reverse((0, (k-1) as usize)));

        let mut edges = vec![vec![]; n as usize];
        for (idx, item) in times.iter().enumerate() {
            edges[(item[0]-1) as usize].push(&times[idx]);
        }

        while let Some(Reverse((min_value, idx))) = que.pop() {
            if min_value > dist[idx] { continue }
            
            for edge in edges[idx].iter() {
                let to = (edge[1] - 1) as usize;
                let new_dist = dist[idx] + edge[2];

                if new_dist < dist[to] {
                    dist[to] = new_dist;
                    que.push(Reverse((new_dist, to)));
                }
            }
        }

        let result = *dist.iter().max().unwrap();
        if result == i32::MAX {-1} else {result}
    }
}