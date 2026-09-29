// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut dist = vec![vec![None; n]; n];
        // let mut path = vec![vec![None; n]; n];

        for edge in edges {
            dist[edge[0] as usize][edge[1] as usize] = Some(edge[2]);
            dist[edge[1] as usize][edge[0] as usize] = Some(edge[2]);
            // path[edge[0] as usize][edge[1] as usize] = Some(edge[1] as usize);
            // path[edge[1] as usize][edge[0] as usize] = Some(edge[0] as usize);
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if i == j { continue } 

                    match (dist[i][j], dist[i][k], dist[k][j]) {
                        (Some(curr), Some(left), Some(right)) =>{
                            if curr > left + right {
                                dist[i][j] = Some(left + right);
                                // path[i][j] = Some(k);
                            }
                        },
                        (None, Some(left), Some(right)) =>{
                            dist[i][j] = Some(left + right);
                            // path[i][j] = Some(k);
                        },
                        (_, _, _) => {}
                    }
                }
            }
        }
        // println!("dist: {:?}", &dist);
        // println!("path: {:?}", &path);
        let mut result = 0;
        let mut min_reach_count = usize::MAX;
        for (idx, item) in dist.iter().enumerate() {
            let count = item.iter()
                            .filter(|itm| itm.is_some_and(|val| val <= distance_threshold))
                            .count();

            if count <= min_reach_count {
                min_reach_count = count;
                result = idx;
            }
        }
        result as i32 
    }
}




////////////////////////////////////////////////////////// AI


impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        // Large sentinel value that won't overflow when added
        const INF: i32 = 100_000_000;
        let mut dist = vec![vec![INF; n]; n];

        // Distance to self is 0
        for i in 0..n {
            dist[i][i] = 0;
        }

        // Fill direct edges
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            dist[u][v] = w;
            dist[v][u] = w;
        }

        // Floyd-Warshall Algorithm
        for k in 0..n {
            for i in 0..n {
                // Optimization: Skip if path from i to k is already out of bounds
                if dist[i][k] > distance_threshold {
                    continue;
                }
                for j in 0..n {
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist < dist[i][j] {
                        dist[i][j] = new_dist;
                    }
                }
            }
        }

        let mut result = 0;
        let mut min_reach = usize::MAX;

        for i in 0..n {
            // Count reachable cities within distance_threshold (excluding self)
            let reach = dist[i]
                .iter()
                .enumerate()
                .filter(|&(j, &d)| i != j && d <= distance_threshold)
                .count();

            // Tie-breaker: If reach counts are equal, pick the larger index
            if reach <= min_reach {
                min_reach = reach;
                result = i;
            }
        }

        result as i32
    }
}



///////////////////////////////////////////////////////// AI 

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![]; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        let mut result = 0;
        let mut min_reach = usize::MAX;

        for start in 0..n {
            let reach = dijkstra_count(start, n, &adj, distance_threshold);
            if reach <= min_reach {
                min_reach = reach;
                result = start;
            }
        }

        result as i32
    }
}

fn dijkstra_count(
    start: usize,
    n: usize,
    adj: &Vec<Vec<(usize, i32)>>,
    threshold: i32,
) -> usize {
    let mut dist = vec![i32::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0, start)));

    let mut reachable = 0;

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }

        if u != start {
            reachable += 1;
        }

        for &(v, w) in &adj[u] {
            let next_d = d + w;
            if next_d <= threshold && next_d < dist[v] {
                dist[v] = next_d;
                heap.push(Reverse((next_d, v)));
            }
        }
    }

    reachable
}