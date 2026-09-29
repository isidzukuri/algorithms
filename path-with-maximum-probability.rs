// https://leetcode.com/problems/path-with-maximum-probability

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, PartialEq)]
struct State {
    prob: f64,
    node: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.prob.partial_cmp(&other.prob)
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let start_node = start_node as usize;
        let end_node = end_node as usize;
        let mut neighbors = vec![vec![]; n];

        for edge in 0..edges.len() {
            let node_a = edges[edge][0] as usize;
            let node_b = edges[edge][1] as usize;
            neighbors[node_b].push((node_a, succ_prob[edge]));
            neighbors[node_a].push((node_b, succ_prob[edge]));
        }

        let mut que = BinaryHeap::new();
        que.push(State {
            prob: 1.0,
            node: start_node,
        });

        let mut probs = vec![0.0; n];
        probs[start_node] = 1.0;

        while let Some(State { prob, node }) = que.pop() {
            if node == end_node {
                return prob
            }

            if prob < probs[node] {
                continue
            }

            for neighbor in &neighbors[node] {
                let new_prob = prob * neighbor.1;
                if new_prob > probs[neighbor.0] {
                    probs[neighbor.0] = new_prob;
                    que.push(State {
                        prob: new_prob,
                        node: neighbor.0,
                    });
                }
            }

        }

        0.0
    }
}



/////////////////////////////////////////////////// AI 


use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Wrapper for ordered standard float comparison in BinaryHeap
#[derive(PartialEq)]
struct State(f64, usize);

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Direct partial_cmp unwrap: faster than custom struct implementations
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        let n = n as usize;
        let start = start_node as usize;
        let end = end_node as usize;

        // 1. Pre-allocate adjacency list capacity to avoid reallocation overhead
        let mut degree = vec![0; n];
        for edge in &edges {
            degree[edge[0] as usize] += 1;
            degree[edge[1] as usize] += 1;
        }

        let mut adj = vec![vec![]; n];
        for i in 0..n {
            adj[i].reserve(degree[i]);
        }

        for (i, edge) in edges.iter().enumerate() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let p = succ_prob[i];
            // Skip 0-probability edges early to avoid expanding useless paths
            if p > 0.0 {
                adj[u].push((v, p));
                adj[v].push((u, p));
            }
        }

        let mut max_prob = vec![0.0; n];
        max_prob[start] = 1.0;

        let mut heap = BinaryHeap::new();
        heap.push(State(1.0, start));

        while let Some(State(prob, u)) = heap.pop() {
            // Early exit: First time end_node is popped, it is optimal
            if u == end {
                return prob;
            }

            // Skip stale heap entries
            if prob < max_prob[u] {
                continue;
            }

            for &(v, edge_prob) in &adj[u] {
                let next_prob = prob * edge_prob;
                if next_prob > max_prob[v] {
                    max_prob[v] = next_prob;
                    heap.push(State(next_prob, v));
                }
            }
        }

        0.0
    }
}