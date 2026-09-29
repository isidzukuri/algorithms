// https://leetcode.com/problems/game-of-life

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        let state = board.clone();

        let max_y = board.len();
        let max_x = board[0].len();
        for y in 0..max_y {
            for x in 0..max_x {
                let cell = state[y][x];
                let mut nei_alive = 0;

                for dn in 0..dirs.len(){
                    let nei_y = (y as i32 + dirs[dn].0) as usize;
                    let nei_x = (x as i32 + dirs[dn].1) as usize;
                    if nei_y == usize::MAX || nei_x == usize::MAX ||
                       nei_y >= max_y || nei_x >= max_x {
                        continue
                    }
                    nei_alive += state[nei_y][nei_x];
                }

                board[y][x] = match nei_alive {
                    alive if cell == 1 && alive < 2 => 0,
                    alive if cell == 1 && (alive == 2 || alive == 3) => 1,
                    alive if cell == 1 && alive > 3 => 0,
                    alive if cell == 0 && alive == 3 => 1,
                   _ => board[y][x]
                }
            }
        }
    }
}




///////////////////////////////////////////////// AI

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dirs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        let max_y = board.len();
        let max_x = board[0].len();

        // Pass 1: Determine next states and mark cells with 2 or 3
        for y in 0..max_y {
            for x in 0..max_x {
                let mut nei_alive = 0;

                for &(dy, dx) in &dirs {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;

                    if ny >= 0 && ny < max_y as i32 && nx >= 0 && nx < max_x as i32 {
                        // Check if neighbor WAS alive in the current generation (1 or 3)
                        let val = board[ny as usize][nx as usize];
                        if val == 1 || val == 3 {
                            nei_alive += 1;
                        }
                    }
                }

                let current = board[y][x];
                // Apply Game of Life rules
                if current == 1 && (nei_alive < 2 || nei_alive > 3) {
                    board[y][x] = 3; // Alive -> Dead
                } else if current == 0 && nei_alive == 3 {
                    board[y][x] = 2; // Dead -> Alive
                }
            }
        }

        // Pass 2: Map temporary states to final 0s and 1s
        for y in 0..max_y {
            for x in 0..max_x {
                board[y][x] = match board[y][x] {
                    2 | 1 => 1,
                    _ => 0,
                };
            }
        }
    }
}