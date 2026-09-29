// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        
        for road in roads {
            uf.union((road[0] -1) as usize, (road[1]-1) as usize, road[2]);
        }

        let root_0 = uf.find(0);
        uf.scores[root_0]
    }
}


pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    scores: Vec<i32>,
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            scores: vec![i32::MAX; n],
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
            self.scores[root_i] = self.scores[root_i].min(cost);
            return false;
        }

        let new_min = self.scores[root_i].min(self.scores[root_j]).min(cost);

        // Union by rank: attach smaller depth tree under root of deeper tree
       match self.rank[root_i].cmp(&self.rank[root_j]) {
            std::cmp::Ordering::Less => {
                self.parent[root_i] = root_j;
                self.scores[root_j] = new_min;
            }
            std::cmp::Ordering::Greater => {
                self.parent[root_j] = root_i;
                self.scores[root_i] = new_min;
            }
            std::cmp::Ordering::Equal => {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
                self.scores[root_i] = new_min;
            }
        }

        true
    }
}



///////////////////////////////////////////////////////////////////////////////

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n + 1];

        // Build adjacency list (O(V + E) space)
        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2];
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        let mut visited = vec![false; n + 1];
        let mut stack = vec![1];
        visited[1] = true;
        let mut min_score = i32::MAX;

        // Traverse the connected component containing node 1
        while let Some(node) = stack.pop() {
            for &(neighbor, weight) in &adj[node] {
                // Check every edge connected to a node in this component
                min_score = min_score.min(weight);
                
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }

        min_score
    }
}