// https://leetcode.com/problems/design-graph-with-shortest-path-calculator

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    pub nodes: Vec<Vec<(usize,i32)>>
}

impl Graph {

    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut nodes = vec![vec![]; n as usize];
        for edge in edges.iter(){
            nodes[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }
        Self { nodes: nodes }
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        self.nodes[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let mut distances: Vec<_> = (0..self.nodes.len()).map(|_| i32::MAX).collect();
        let mut visited = vec![false; self.nodes.len()];
        let mut priority_q = BinaryHeap::new();

        priority_q.push(State { position: node1 as usize, cost: 0 });

        while let Some(state) = priority_q.pop(){
            visited[state.position] = true;

            if distances[state.position] < state.cost { continue }
            if state.position == node2 as usize { return state.cost }

            for (neighbor, cost) in self.nodes[state.position].iter() {
                let new_cost =  cost + state.cost;
                
                if new_cost < distances[*neighbor] {
                    distances[*neighbor] = new_cost;
                    priority_q.push(State { position: *neighbor, cost: new_cost });
                }
            } 
        }
        
        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */
