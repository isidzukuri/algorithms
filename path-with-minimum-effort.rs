// https://leetcode.com/problems/path-with-minimum-effort/

use std::cmp::Ordering;
use std::collections::HashMap;
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

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut nodes = HashMap::new();

        let finish = heights.len() * heights[0].len() - 1;

        for (i, row) in heights.iter().enumerate(){
            for (j, cur_alt) in row.iter().enumerate(){
                let position = (i,j);
                let mut neighbors = vec![];
                for n_position in Self::neighbors(&position, &(heights.len() - 1, heights[0].len() - 1)).iter(){
                    let neighbor_alt = heights[n_position.0][n_position.1];
                    let min = cur_alt.min(&neighbor_alt);
                    let max = cur_alt.max(&neighbor_alt);
                    let diff = max - min;

                    let id = Self::id(n_position.0, n_position.1, heights[0].len());
                    neighbors.push((id, diff));
                }
                let id = Self::id(position.0, position.1, heights[0].len());
                nodes.insert(id, neighbors);
            }
        }

        let mut distances: Vec<_> = (0..nodes.len()).map(|_| i32::MAX).collect();

        let mut priority_q = BinaryHeap::new();

        priority_q.push(State { position: 0, cost: 0 });

        while let Some(state) = priority_q.pop(){
            if distances[state.position] < state.cost { continue }
            if state.position == finish { return state.cost }

            for (neighbor, cost) in nodes[&state.position].iter() {
                let new_cost = cost.max(&state.cost);
                
                if *new_cost < distances[*neighbor] {
                    distances[*neighbor] = *new_cost;
                    priority_q.push(State { position: *neighbor, cost: *new_cost });
                }
            } 
        }  
        unreachable!();    
    }

    fn id(row: usize, col: usize, row_len: usize) -> usize{
        row_len*row + col
    }

    fn neighbors(from: &(usize, usize), constraints: &(usize, usize)) -> Vec<(usize, usize)>{
        let right = (0, 1);
        let down = (1, 0);
        let left = (0, -1);
        let up = (-1, 0);
        let directions = [right, down, left, up];

        let mut result = vec![];

        for direction in directions.iter(){
            let coord1 = from.0 as i32 + direction.0;
            let coord2 = from.1 as i32 + direction.1;

            if coord1 < 0 || coord2 < 0 { continue }

            let candidate = (coord1 as usize, coord2 as usize);

            if candidate.0 > constraints.0 || candidate.1 > constraints.1 { continue }
            result.push(candidate);
        }
        result
    }
}
