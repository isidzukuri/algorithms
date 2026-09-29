// https://leetcode.com/problems/number-of-operations-to-make-network-connected

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);

        for node in 0..connections.len() {
            uf.union(connections[node][0] as usize, connections[node][1] as usize);
        }

        if (uf.redundant_connections_total as i32 - uf.groups as i32 + 1) < 0 {
            -1
        } else {
            (uf.groups - 1) as i32
        } 
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    redundant_connections_total: usize,
    groups: usize,
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            redundant_connections_total: 0,
            groups: n
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
            self.redundant_connections_total += 1;
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
        self.groups -= 1;

        true
    }
}


///////////////////////////////////////////////////////////// AI


pub struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
    pub components: usize,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    // Path halving: compresses paths during a single traversal pass
    pub fn find(&mut self, mut x: usize) -> usize {
        while x != self.parent[x] {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            std::cmp::Ordering::Less => self.parent[root_x] = root_y,
            std::cmp::Ordering::Greater => self.parent[root_y] = root_x,
            std::cmp::Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
        self.components -= 1;
        true
    }
}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // Quick check: need at least n - 1 edges to connect n nodes
        if connections.len() < (n - 1) as usize {
            return -1;
        }

        let mut dsu = Dsu::new(n as usize);

        for conn in &connections {
            dsu.union(conn[0] as usize, conn[1] as usize);
        }

        // Operations needed = number of connected components - 1
        (dsu.components - 1) as i32
    }
}