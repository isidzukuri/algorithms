// https://leetcode.com/problems/unique-paths-iii

#[derive(Clone, Debug)]
struct Node {
    pub position: (i32, i32),
    pub parent: Option<Box<Node>>
}

impl Node {
  fn new(position: (i32, i32), parent: Option<Box<Node>>) -> Self {
    Node {
        position: position,
        parent: parent
    }
  }

  pub fn is_visited(&self, position: &(i32, i32)) -> bool{
    if self.position == *position { return true }
    
    match self.parent {
        Some(ref parent) => return parent.is_visited(position),
        _ => return false
    }
  }

  pub fn visited_count(&self) -> i32 {
    match self.parent {
        Some(ref parent) => return parent.visited_count() + 1,
        _ => return 1
    }
  }

  pub fn print_path(&self) {
    if let Some(ref parent) = self.parent {
        parent.print_path();
        println!("|");
        println!("v");
    }
    println!("{:?}", self.position);
  }
}

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let grid_size = (grid.len() * grid[0].len()) as i32;
        let mut position: (i32, i32) = (0,0); 
        let mut finish: (i32, i32) = (0,0);
        let mut blocked_count = 0;

        let right: (i32, i32) = (0, 1);
        let down: (i32, i32) = (1, 0);
        let left: (i32, i32) = (0, -1);
        let up: (i32, i32) = (-1, 0);
        let directions = [right, down, left, up];

        for (r, row) in grid.iter().enumerate() {
            for (l, cell) in row.iter().enumerate() {
                if 1.eq(cell) { position = (r as i32, l as i32) }
                if 2.eq(cell) { finish = (r as i32, l as i32) }
                if (-1).eq(cell) { blocked_count +=1 }
            }
        }

        let mut ways = vec![Node::new(position, None)];

        while let Some(way) = ways.pop(){
            if way.position == finish { 
                if grid_size == (blocked_count + way.visited_count()){
                    result +=1;

                    continue
                }
            }

            for direction in directions.iter(){
                let next = Self::go(&way.position, &direction);
                if Self::is_cell_available(&next, &grid){
                    if !way.is_visited(&next){
                        ways.push(Node::new(next, Some(Box::new(way.clone()))));
                    }
                }
            }
        }

        result
    }

    pub fn go(from: &(i32, i32), to:&(i32, i32)) -> (i32, i32) {
        (from.0 + to.0, from.1 + to.1)
    }

    pub fn is_cell_available(coords: &(i32, i32), grid: &Vec<Vec<i32>>) -> bool {
        if let Some(row) = grid.get(coords.0 as usize){
            if let Some(cell) = row.get(coords.1 as usize){
                return *cell == 0 || *cell == 2
            }
        }
        false
    }
}
