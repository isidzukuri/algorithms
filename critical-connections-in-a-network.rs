impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut solver = BridgeFinder::new(n as usize, connections);
        solver.find_bridges()
    }
}

struct BridgeFinder {
    neighbors: Vec<Vec<usize>>,
    ids: Vec<usize>,
    low: Vec<usize>,
    visited: Vec<bool>,
    bridges: Vec<Vec<i32>>,
    iteration: usize,
}

impl BridgeFinder {
    pub fn new(nodes_count: usize, connections: Vec<Vec<i32>>) -> Self {
        let mut neighbors = vec![vec![]; nodes_count];
        for con in connections {
            neighbors[con[0] as usize].push(con[1] as usize);
            neighbors[con[1] as usize].push(con[0] as usize);
        }

        Self {
            neighbors,
            ids: vec![usize::MAX; nodes_count],
            low: vec![usize::MAX; nodes_count],
            visited: vec![false; nodes_count],
            bridges: Vec::new(),
            iteration: 0,
        }
    }

    pub fn find_bridges(&mut self) -> Vec<Vec<i32>> {
        for at in 0..self.visited.len() {
            if !self.visited[at] {
                self.dfs(at, None);
            }
        }  
        self.bridges.clone()
    }

    fn dfs(&mut self, at: usize, parent: Option<usize>) {
        self.visited[at] = true;
        self.low[at] = self.iteration;
        self.ids[at] = self.iteration;
        self.iteration +=1;

        let len = self.neighbors[at].len();
        for i in 0..len {
            let to = self.neighbors[at][i];

            if Some(to) == parent {
                continue;
            }

            if self.visited[to] {
                self.low[at] = self.low[at].min(self.ids[to]);
            } else {
                self.dfs(to, Some(at));
                self.low[at] = self.low[at].min(self.low[to]);
                if self.ids[at] < self.low[to] {
                    self.bridges.push(vec![at as i32, to as i32]);
                }
            }
        }
    }
}


///////////////////////////////////////////

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut low = vec![usize::MAX; n];
        let mut ids = vec![usize::MAX; n];
        let mut visited = vec![false; n];
        let mut bridges = vec![];

        let mut neighbors = vec![vec![]; n];
        for con in connections {
            neighbors[con[0] as usize].push(con[1] as usize);
            neighbors[con[1] as usize].push(con[0] as usize);
        }

        for at in 0..n {
            if visited[at] { continue }
            dfs(at, None, 0, &mut ids, &mut visited, &mut low, &neighbors, &mut bridges);
        }  
        bridges
    }
}

pub fn dfs(
    at: usize, 
    parent_opt: Option<usize>, 
    id: usize, 
    ids: &mut Vec<usize>, 
    visited: &mut Vec<bool>, 
    low: &mut Vec<usize>, 
    neighbors: &Vec<Vec<usize>>, 
    bridges: &mut Vec<Vec<i32>> // extract it from here
) {
    
    
    visited[at] = true;
    low[at] = id;
    ids[at] = id;

    for to in neighbors[at].iter() {
        let to = *to;
        if let Some(parent) = parent_opt {
            if to == parent { continue }
        }
        if visited[to] {
            low[at] = low[at].min(ids[to]);
        } else {
            dfs(to, Some(at), id+1, ids, visited, low, neighbors, bridges);
            low[at] = low[at].min(low[to]);
            if ids[at] < low[to] {
                bridges.push(vec![at as i32, to as i32]);
            }
        }
    }
}



////////////////////////////////////////// AI


// considered better separation of concerns:

//     Separation of Graph vs. Algorithm:

//         - Graph (Data): Immutable topology that never changes during a query.

//         - TraversalState (Transient Execution): Scratchpad memory that only lives for the duration of a single search run.

//     Reusability & Thread Safety:
//          - Because neighbors is separated from the scratchpad state, multiple threads can safely run read-only DFS 
//            searches over the same shared graph concurrently (&Graph) by each instantiating their own lightweight TraversalState.


impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut solver = BridgeFinder::new(n as usize, connections);
        solver.find_bridges()
    }
}

// Public interface: Entirely encapsulated single struct
pub struct BridgeFinder {
    neighbors: Vec<Vec<usize>>,
}

// Private execution context hidden inside the module/file
struct Scratchpad {
    ids: Vec<usize>,
    low: Vec<usize>,
    visited: Vec<bool>,
    bridges: Vec<Vec<i32>>,
    timer: usize,
}

impl BridgeFinder {
    pub fn new(n: usize, connections: Vec<Vec<i32>>) -> Self {
        let mut neighbors = vec![vec![]; n];
        for con in connections {
            let u = con[0] as usize;
            let v = con[1] as usize;
            neighbors[u].push(v);
            neighbors[v].push(u);
        }
        Self { neighbors }
    }

    pub fn find_bridges(&mut self) -> Vec<Vec<i32>> {
        let n = self.neighbors.len();
        let mut state = Scratchpad {
            ids: vec![usize::MAX; n],
            low: vec![usize::MAX; n],
            visited: vec![false; n],
            bridges: Vec::new(),
            timer: 0,
        };

        for at in 0..n {
            if !state.visited[at] {
                self.dfs(at, None, &mut state);
            }
        }

        state.bridges
    }

    // Fully encapsulated private helper method
    fn dfs(&self, at: usize, parent: Option<usize>, state: &mut Scratchpad) {
        state.visited[at] = true;
        state.ids[at] = state.timer;
        state.low[at] = state.timer;
        state.timer += 1;

        for &to in &self.neighbors[at] {
            if Some(to) == parent {
                continue;
            }

            if state.visited[to] {
                state.low[at] = state.low[at].min(state.ids[to]);
            } else {
                self.dfs(to, Some(at), state);
                state.low[at] = state.low[at].min(state.low[to]);

                if state.ids[at] < state.low[to] {
                    state.bridges.push(vec![at as i32, to as i32]);
                }
            }
        }
    }
}