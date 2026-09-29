// https://leetcode.com/problems/count-servers-that-communicate

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        if rows == 0 { return 0 }
        let cols = grid[0].len();

        let mut uf = UnionFind::new(rows * cols);

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 1 {
                    let cur = row * cols + col;

                    let mut next_col = col+1;
                    while next_col < cols {
                        if next_col < cols && grid[row][next_col] == 1 {
                            uf.union(cur as usize, row * cols + next_col  as usize);
                            break;
                        }
                        next_col+=1;
                    }

                    let mut next_row = row+1;
                    while next_row < rows {
                        if next_row < rows && grid[next_row][col] == 1 {
                            uf.union(cur as usize, (next_row * cols + col) as usize);
                            break;
                        }
                        next_row +=1;
                    }
                }
            }
        }

        let total: usize = uf.group_count.iter().filter(|&&x| x > 1).sum();
        total as i32
    }
}

#[derive(Debug)]
pub struct UnionFind {
    parent: Vec<usize>,
    group_count: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            group_count: vec![1; n],
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        
        let mut curr = i;
        while curr != root {
            let nxt = self.parent[curr];
            self.parent[curr] = root;
            curr = nxt;
        }
        
        root
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }
        self.parent[root_j] = root_i;
        self.group_count[root_i] += self.group_count[root_j];
        self.group_count[root_j] = 0;

        true
    }

}


/////////////////////////////////////// AI


impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        
        let mut row_counts = vec![0; rows];
        let mut col_counts = vec![0; cols];

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    row_counts[r] += 1;
                    col_counts[c] += 1;
                }
            }
        }

        let mut connected_servers = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 && (row_counts[r] > 1 || col_counts[c] > 1) {
                    connected_servers += 1;
                }
            }
        }

        connected_servers
    }
}