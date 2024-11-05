// https://leetcode.com/problems/number-of-provinces

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![];
        let mut clusters_count = 0;

        for (node, connections) in is_connected.iter().enumerate() {
            if visited.contains(&node) { 
                continue 
            } else { 
                clusters_count += 1; 
            }
            let mut stack = vec![node];

            while let Some(current_node) = stack.pop(){
                if visited.contains(&current_node) { continue }
                visited.push(current_node);   

                for (neighbor, connected) in is_connected[current_node].iter().enumerate(){
                    if *connected == 1 { 
                        stack.push(neighbor);
                    }
                }
            }
            
        }
        clusters_count
    }
}
