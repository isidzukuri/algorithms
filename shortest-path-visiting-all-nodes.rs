// https://leetcode.com/problems/shortest-path-visiting-all-nodes/

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let nodes_count = graph.len();
        let target = (1<<nodes_count) - 1;
        let mut visited = vec![vec![0; 1<<nodes_count]; nodes_count];
        let mut queue = VecDeque::new();

        for (position, neighbors) in graph.iter().enumerate(){
            let mask = 1<<position;
            queue.push_back((position, mask));
            visited[position][mask] = 0;
        }

        while let Some((position, mask)) = queue.pop_front(){
            if mask == target {
               return visited[position][mask];
            }

            for n_position in graph[position].iter() {
                let n_mask = mask | 1<<n_position;
                let n_position_usize = *n_position as usize;
                if visited[n_position_usize][n_mask] > 0 { continue }

                visited[n_position_usize][n_mask] = visited[position][mask] + 1;
                queue.push_back((n_position_usize, n_mask));
            }
        }
        unreachable!()
    }
}
