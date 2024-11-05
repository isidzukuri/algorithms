// https://leetcode.com/problems/all-paths-from-source-to-target

use std::collections::VecDeque;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let destination = (graph.len() - 1) as i32;  
        let mut paths = vec![];
        let mut stack = vec![(0, vec![])];
        while let Some((node, mut path)) = stack.pop(){
            path.push(node);
            if node == destination {
                paths.push(path);
                continue;
            }

            for neighbor in graph[node as usize].iter(){
                stack.push((*neighbor, path.clone()));
            }
        }
        paths
    }
}
