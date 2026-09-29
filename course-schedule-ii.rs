// https://leetcode.com/problems/course-schedule-ii

use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visited = vec![false; num_courses as usize];
        let mut order = Vec::from_iter(0..num_courses);
        let mut nodes_parents: HashMap<i32, Vec<i32>> = HashMap::new();

        for item in prerequisites {
            nodes_parents.entry(item[1]).or_insert(vec![]).push(item[0]);
        }

        let mut order_idx = (num_courses - 1) as usize;

        for from in 0..num_courses  {
            if visited[from as usize] { continue }
            let mut reccurent = vec![false; num_courses as usize];
            let mut traversed = vec![];

            if !dfs(from, &mut visited, &mut traversed, &nodes_parents, &mut reccurent) {
                return vec![]
            }

            for node in traversed {
                order[order_idx] = node;
                order_idx -=1;
            }
        }
        order
    }
}

pub fn dfs(from: i32, visited:  &mut Vec<bool>, traversed: &mut Vec<i32>, nodes_parents: &HashMap<i32, Vec<i32>>, reccurent: &mut Vec<bool>) -> bool{
    visited[from as usize] = true;
    reccurent[from as usize] = true;
    
    if let Some(parents) = nodes_parents.get(&from) {
        for parent in parents {
            if reccurent[*parent as usize] { return false }
            if visited[*parent as usize] { continue }
            if !dfs(*parent, visited, traversed, nodes_parents, reccurent) {
                return false
            }
        }
    }
    reccurent[from as usize] = false;
    traversed.push(from);
    true
}




//////////////////////////////////////////////////////// AI


use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

        // Build adjacency list: prerequisite -> course
        for item in prerequisites {
            adj.entry(item[1]).or_default().push(item[0]);
        }

        // State vector: 0 = Unvisited, 1 = Visiting (in recursion stack), 2 = Visited
        let mut state = vec![0u8; num_courses];
        let mut order = Vec::with_capacity(num_courses);

        for course in 0..num_courses as i32 {
            if state[course as usize] == 0 {
                if !dfs(course, &adj, &mut state, &mut order) {
                    return vec![]; // Cycle detected
                }
            }
        }

        // Nodes pushed post-order during DFS need to be reversed to get the standard topological order
        order.reverse();
        order
    }
}

pub fn dfs(
    curr: i32,
    adj: &HashMap<i32, Vec<i32>>,
    state: &mut Vec<u8>,
    order: &mut Vec<i32>,
) -> bool {
    state[curr as usize] = 1; // Mark as currently visiting

    if let Some(neighbors) = adj.get(&curr) {
        for &next in neighbors {
            if state[next as usize] == 1 {
                return false; // Cycle detected: visited a node in the current path
            }
            if state[next as usize] == 0 {
                if !dfs(next, adj, state, order) {
                    return false;
                }
            }
        }
    }

    state[curr as usize] = 2; // Mark as completely processed
    order.push(curr); // Post-order placement
    true
}




///////////////////////////////////////////////////////////////// AI 

use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        
        // 1. Build adjacency list and calculate indegrees
        let mut adj = vec![vec![]; n];
        let mut indegree = vec![0; n];

        for req in prerequisites {
            let course = req[0] as usize;
            let prereq = req[1] as usize;
            
            adj[prereq].push(course);
            indegree[course] += 1;
        }

        // 2. Queue all nodes with 0 prerequisites (ready to take)
        let mut queue = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        // 3. Process the queue
        let mut order = Vec::with_capacity(n);

        while let Some(curr) = queue.pop_front() {
            order.push(curr);

            for &neighbor in &adj[curr as usize] {
                indegree[neighbor] -= 1;
                // If all prerequisites for neighbor are fulfilled, add it to queue
                if indegree[neighbor] == 0 {
                    queue.push_back(neighbor as i32);
                }
            }
        }

        // 4. If order contains all courses, return it; otherwise a cycle exists
        if order.len() == n {
            order
        } else {
            vec![]
        }
    }
}