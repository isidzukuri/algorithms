// https://leetcode.com/problems/find-if-path-exists-in-graph/

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut visited = vec![false; n as usize];
        let mut neighbors = vec![vec![]; n as usize];
        let mut stack = vec![source];

        for edge in edges {
            neighbors[edge[0] as usize].push(edge[1]);
            neighbors[edge[1] as usize].push(edge[0]);
        }

        while let Some(node) = stack.pop() {
            if node == destination { return true }
            if visited[node as usize] { continue }

            visited[node as usize] = true;
            for neighbor in &neighbors[node as usize] {
                stack.push(*neighbor)
            }
        }
        false
    }
}