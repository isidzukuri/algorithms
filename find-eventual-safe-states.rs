// https://leetcode.com/problems/find-eventual-safe-states

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let nc = graph.len();
        let mut safe = vec![false; nc];
        let mut visited = vec![false; nc];
        let mut result = vec![];

        for idx in 0..nc {
            if dfs(idx, &graph, &mut visited, &mut safe) {
                result.push(idx as i32);
            }
        }
        result
    }
}

pub fn dfs(node: usize, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, safe: &mut Vec<bool>) -> bool {
    if visited[node] { return safe[node] }

    visited[node] = true;

    if graph[node].len() > 0 {
        for neighbor in &graph[node] {
            if dfs(*neighbor as usize, graph, visited, safe) == false { 
                return false 
            }
        }
    }
    safe[node] = true;
    true
}

// but it has a subtle bug/inefficiency in cycle detection: it does not track nodes currently in the recursion stack.
// Because visited[node] remains true after a path completes, a back-edge encountered during traversal might return safe[node], 
// which happens to default to false. While this accidentally gives the correct result for cycle detection, 
// it degrades performance or breaks down on certain graph structures because visited and safe aren't properly 
// reflecting "currently visiting" vs. "fully processed" states.



///////////////////////////////////////////////////////////////////////

use std::collections::VecDeque;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let nc = graph.len();
        let mut result = vec![];

        let mut in_degree = vec![0; nc];
        let mut adj = vec![vec![]; nc];
        let mut que = VecDeque::new();

        for node in 0..nc {
            in_degree[node] += graph[node].len();

            for neighbor in &graph[node]{
                adj[*neighbor as usize].push(node);
            }
        }

        for node in 0..nc {
            if in_degree[node] == 0 { que.push_back(node) }
        }

        while let Some(node) = que.pop_front(){
            result.push(node as i32);

            for neighbor in &adj[node]{
                in_degree[*neighbor] -=1;
                if in_degree[*neighbor] == 0 { que.push_back(*neighbor) }
            }
        }
        result.sort_unstable();
        result
    }
}

//////////////////////////////////////////////////////////////// AI


impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        // 0 = unvisited, 1 = visiting (in current DFS path), 2 = safe node
        let mut state = vec![0u8; n];

        fn dfs(node: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<u8>) -> bool {
            if state[node] > 0 {
                return state[node] == 2;
            }

            state[node] = 1; // Mark as visiting

            for &neighbor in &graph[node] {
                if !dfs(neighbor as usize, graph, state) {
                    return false; // Found a cycle or unsafe path
                }
            }

            state[node] = 2; // Mark as safe
            true
        }

        (0..n)
            .filter(|&node| dfs(node, &graph, &mut state))
            .map(|node| node as i32)
            .collect()
    }
}