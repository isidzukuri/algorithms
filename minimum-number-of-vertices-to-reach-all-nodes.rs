// https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vertices = vec![false; n as usize];

        for edge in edges.iter() {
            vertices[edge[1] as usize] = true;
        }

        (0..n as usize)
            .filter(|&i| !vertices[i])
            .map(|i| i as i32)
            .collect()
    }
}
