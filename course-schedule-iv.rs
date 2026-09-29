// https://leetcode.com/problems/course-schedule-iv

use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let num = num_courses as usize;
        let mut dependencies = vec![vec![false; num]; num];
        let mut dependents = vec![vec![]; num];
        let mut indegree = vec![0; num];

        for item in prerequisites {
            dependents[item[1] as usize].push(item[0] as usize);
            indegree[item[0] as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for idx in 0..num {
            if indegree[idx] == 0 {
                queue.push_back(idx)
            }
        }

        while let Some(node) = queue.pop_front() {
            for dep in dependents[node].iter() {
                unsafe {
                    // Get raw mutable pointers to both elements directly
                    let cur_ptr = dependencies.as_mut_ptr().add(*dep);
                    let node_ptr = dependencies.as_ptr().add(node);
                    
                    // Dereference safely because we asserted they don't overlap
                    let cur_dependencies = &mut *cur_ptr;
                    let node_dependencies = &*node_ptr;
                    
                    cur_dependencies[node] = true;
                    cur_dependencies.iter_mut()
                                    .zip(node_dependencies.iter())
                                    .for_each(|(a, &b)| *a |= b);
                }

                indegree[*dep] -= 1;

                if indegree[*dep] == 0 {
                    queue.push_back(*dep);
                }
            }
        }

        let mut result = vec![];
        for query in queries {
            result.push(dependencies[query[0] as usize][query[1] as usize]);
        }

        result
    }
}






//////////////////////////////////////////////////////////////// AI 


impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut reach = vec![0u128; n];

        for req in prerequisites {
            reach[req[0] as usize] |= 1 << req[1] as usize;
        }

        // Warshall's Algorithm using bitwise OR
        for k in 0..n {
            let k_reach = reach[k];
            for i in 0..n {
                // If i can reach k, then i can also reach everything k can reach
                if (reach[i] & (1 << k)) != 0 {
                    reach[i] |= k_reach;
                }
            }
        }

        queries
            .into_iter()
            .map(|q| (reach[q[0] as usize] & (1 << q[1] as usize)) != 0)
            .collect()
    }
}