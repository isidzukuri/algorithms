// https://leetcode.com/problems/min-cost-to-connect-all-points




use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let points_count = points.len();
        let mut edges = BinaryHeap::new();


        for at in 0..points_count {
            for to in (at+1)..points_count {
                let cost = cost(&points[at], &points[to]);
                edges.push(Reverse((cost, at, to)));
            }
        }

        let mut uf = UnionFind::new(points_count);

        let mut edges_count = 0;
        while let Some(Reverse(edge)) = edges.pop() {
            if uf.union(edge.1, edge.2, edge.0) {
                edges_count += 1;
                if edges_count == points_count - 1 {
                    break;
                }
            }
        }

        uf.cost
    }
}


//////////////////////////////////////////////////////////////////

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let points_count = points.len();
        let mut edges = vec![];

        for at in 0..points_count {
            for to in (at+1)..points_count {
                let cost = cost(&points[at], &points[to]);
                edges.push((cost, at, to));
            }
        }

        edges.sort_unstable();

        let mut uf = UnionFind::new(points_count);

        for edge in edges {
            uf.union(edge.1, edge.2, edge.0);
        }

        uf.cost
    }
}

pub fn cost(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
    (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}


pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    cost: i32
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            // groups: n,
            cost: 0,
        }
    }

    // Find the representative root of element i with path compression
    pub fn find(&mut self, mut i: usize) -> usize {
        // Find the root
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        
        // Path compression: point all visited nodes directly to the root
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        
        root
    }

    // Unify the sets containing element i and element j
    // Returns true if a merge happened, false if they were already in the same set
    pub fn union(&mut self, i: usize, j: usize, cost: i32) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        // Union by rank: attach smaller depth tree under root of deeper tree
        match self.rank[root_i].cmp(&self.rank[root_j]) {
            std::cmp::Ordering::Less => self.parent[root_i] = root_j,
            std::cmp::Ordering::Greater => self.parent[root_j] = root_i,
            std::cmp::Ordering::Equal => {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
            }
        }
        self.cost += cost;

        true
    }
}
