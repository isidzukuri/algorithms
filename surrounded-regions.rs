// https://leetcode.com/problems/surrounded-regions

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut uf = UnionFind::new(board.len() * board[0].len());

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                
                let cur = board[row][col];
                if cur == 'O' {
                    let mut border = row == 0 || 
                                (row == board.len() -1) || 
                                col == 0 || 
                                col == (board[0].len() - 1);
                    let cur_n = row * board[0].len() + col;

                    if border {
                        uf.border[cur_n] = true;
                    }

                    let next_row = row + 1;
                    let next_col = col + 1;

                    if next_row < board.len() {
                        let neighbor = board[next_row][col];
                        if neighbor == 'O' {
                            border = border || (next_row == board.len() -1);
                            let neighbor_n = next_row * board[0].len() + col;
                            uf.union(cur_n, neighbor_n, border);
                        }
                    }

                    if next_col < board[0].len() {
                        let neighbor = board[row][next_col];
                        if neighbor == 'O' {
                            border = border || (next_col == board[0].len() -1);
                            let neighbor_n = row * board[0].len() + next_col;
                            uf.union(cur_n, neighbor_n, border);
                        }
                    }
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                let cur_n = row * board[0].len() + col;
                if board[row][col] == 'O' {
                    let parent = uf.find(cur_n);

                    if !uf.border[parent] {
                        board[row][col] = 'X';
                    }
                }
            }
        }
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    border: Vec<bool>,
}

impl UnionFind {
    // Create a new universe of n isolated elements
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            border: vec![false; n],
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
    pub fn union(&mut self, i: usize, j: usize, border: bool) -> bool {
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
        if border {
            self.border[self.parent[root_i]] = true;
        }
        true
    }
}


/////////////////////////////////////////////////////// AI 

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        // Step 1: Traverse the boundary cells and run DFS on any 'O' found
        for r in 0..rows {
            if board[r][0] == 'O' {
                Self::dfs(board, r, 0);
            }
            if board[r][cols - 1] == 'O' {
                Self::dfs(board, r, cols - 1);
            }
        }

        for c in 0..cols {
            if board[0][c] == 'O' {
                Self::dfs(board, 0, c);
            }
            if board[rows - 1][c] == 'O' {
                Self::dfs(board, rows - 1, c);
            }
        }

        // Step 2: Flip captured internal 'O's to 'X', and restore border-connected 'E's to 'O'
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == 'E' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
        let rows = board.len();
        let cols = board[0].len();

        // Base checks: bounds and checking if cell is 'O'
        if r >= rows || c >= cols || board[r][c] != 'O' {
            return;
        }

        // Mark current boundary-connected cell as Escaped/Safe
        board[r][c] = 'E';

        // Explore neighbors (using wrapping subtraction prevention for 0 indexing)
        if r > 0 { Self::dfs(board, r - 1, c); }
        if r + 1 < rows { Self::dfs(board, r + 1, c); }
        if c > 0 { Self::dfs(board, r, c - 1); }
        if c + 1 < cols { Self::dfs(board, r, c + 1); }
    }
}