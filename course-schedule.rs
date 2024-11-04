// https://leetcode.com/problems/course-schedule

use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.len() <= 1 { return true }

        let mut dependants_count = HashMap::new();
        let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new();

        for edge in prerequisites.iter() {
            if let Some(node) = neighbors.get_mut(&edge[1]) {
                node.push(edge[0]);
            } else {
                neighbors.insert(edge[1], vec![edge[0]]);
            }
            dependants_count.entry(&edge[1]).or_insert(0);
            *dependants_count.entry(&edge[0]).or_insert(0) += 1;
        }

        let mut queue = VecDeque::new();
        for (id, count) in dependants_count.iter() {
            if *count == 0 { queue.push_back(*id) }
        }

        let mut sorted = vec![];
        while let Some(item_id) = queue.pop_front(){
            sorted.push(item_id);
            if let Some(item_neighbors) = neighbors.get(item_id) {
                for n_id in item_neighbors.iter(){
                    dependants_count.entry(&n_id).and_modify(|val| *val -= 1);
                    if dependants_count[n_id] == 0 { queue.push_back(n_id) }
                }
            }
        }
        return dependants_count.values().all(|x| *x == 0 );

        true
    }
}
