// https://leetcode.com/problems/redundant-connection

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut uf = UnionFind::new(edges.len());

        for edge in edges {
            if !uf.union((edge[0]-1) as usize, (edge[1]-1) as usize) {
                return edge;
            }
        }

        vec![]
    }
}


pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
    pub fn union(&mut self, i: usize, j: usize) -> bool {
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
        true
    }
}