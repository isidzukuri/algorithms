// https://leetcode.com/problems/cheapest-flights-within-k-stops/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = (k+1) as usize;
        let src = src as usize;
        let dst = dst as usize;

        let mut neighbors = vec![vec![]; n];

        for edge_n in 0..flights.len() {
            let edge = &flights[edge_n];
            neighbors[edge[0] as usize].push((edge[2], edge[1] as usize));
        }

        let mut costs = vec![vec![i32::MAX; k]; n];
        let mut stack = BinaryHeap::new();
        stack.push(Reverse((0, src, 0)));

        while let Some(Reverse((total, at, stops))) = stack.pop() {
            if at == dst { return total }
            if stops == k { continue }

            for neighbor in &neighbors[at] {
                let new_total = total + neighbor.0;
                if costs[neighbor.1][stops] > new_total {
                    costs[neighbor.1][stops] = new_total;
                    stack.push(Reverse((new_total, neighbor.1, (stops + 1))));
                }
            }
        }
        -1
    }
}



////////////////////////////////////////////////// AI

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let k = k as usize;

        // dist[i] holds the minimum cost to reach node i
        let mut dist = vec![i32::MAX; n];
        dist[src] = 0;

        // Perform at most k + 1 edge relaxations (k stops allows k + 1 flights)
        for _ in 0..=k {
            let mut temp = dist.clone();
            let mut updated = false;

            for flight in &flights {
                let u = flight[0] as usize;
                let v = flight[1] as usize;
                let price = flight[2];

                if dist[u] != i32::MAX && dist[u] + price < temp[v] {
                    temp[v] = dist[u] + price;
                    updated = true;
                }
            }

            dist = temp;
            // Early exit if no distances improved during this step
            if !updated {
                break;
            }
        }

        if dist[dst] == i32::MAX { -1 } else { dist[dst] }
    }
}