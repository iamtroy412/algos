use percolation::UnionFind;

pub struct Percolation {
    n: usize,
    uf: UnionFind,
    opened: Vec<bool>,
}

impl Percolation {
    // creates an n-by-n grid, will all sites initiall blocked
    pub fn new(n: usize) -> Self {
        Percolation {
            n,
            uf: UnionFind::new(n * n),
            opened: vec![false; n * n],
        }
    }

    // opens the site (row, col) if it is not already open
    pub fn open(&mut self, row: usize, col: usize) {}

    // is the site (row, col) open?
    pub fn is_open(&self, row: usize, col: usize) -> bool {}

    // is the site (row, col) full?
    pub fn is_full(&self, row: usize, col: usize) -> bool {}

    // returns the number of open sites
    pub fn num_open_sites(&self) -> usize {}

    // Does the system percolate?
    pub fn percolates(&self) -> bool {}
}
