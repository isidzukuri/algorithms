// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![]; n as usize];
        let mut neighbors = vec![vec![]; n as usize];

        for edge in edges {
            neighbors[edge[1] as usize].push(edge[0]);
        }

        for root in 0..n {
            let mut stack = vec![root];
            let mut visited = vec![false; n as usize];

            while let Some(node) = stack.pop() {
                for neighbor in &neighbors[node as usize] {
                    if visited[*neighbor as usize] { continue }
                    visited[*neighbor as usize] = true;
                    stack.push(*neighbor)
                }
            }

            for (node, marked) in visited.iter().enumerate() {
                if !marked { continue } 
                result[root as usize].push(node as i32);
            }
        }
        result    
    }
}




/////////////////////////////////////////////// AI
// For large $N$, using topological ordering (Kahn's Algorithm) combined with BitSet set unions reduces 
//   set merges down to bitwise operations, running in $O(V + E + \frac{V^2}{64})$ time.

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut in_degree = vec![0; n];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            in_degree[v] += 1;
        }

        // Kahn's Algorithm setup
        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        // Use bitsets (represented as Vec<u64>) to represent ancestor sets efficiently
        let words = (n + 63) / 64;
        let mut ancestor_bits = vec![vec![0u64; words]; n];

        while let Some(node) = queue.pop_front() {
            for &child in &graph[node] {
                // Bitwise OR: child gets all ancestors of node + node itself
                for k in 0..words {
                    ancestor_bits[child][k] |= ancestor_bits[node][k];
                }
                ancestor_bits[child][node / 64] |= 1u64 << (node % 64);

                in_degree[child] -= 1;
                if in_degree[child] == 0 {
                    queue.push_back(child);
                }
            }
        }

        // Unpack bitsets into sorted output vectors
        let mut result = vec![vec![]; n];
        for i in 0..n {
            for bit_idx in 0..n {
                if (ancestor_bits[i][bit_idx / 64] & (1u64 << (bit_idx % 64))) != 0 {
                    result[i].push(bit_idx as i32);
                }
            }
        }

        result
    }
}