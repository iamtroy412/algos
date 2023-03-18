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

    fn index_of(&self, row: usize, col: usize) -> usize {
        (row - 1) * self.n + col - 1
    }

    // opens the site (row, col) if it is not already open
    pub fn open(&mut self, row: usize, col: usize) {}

    // is the site (row, col) open?
    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.opened[self.index_of(row, col)]
    }

    // is the site (row, col) full?
    pub fn is_full(&self, row: usize, col: usize) -> bool {}

    // returns the number of open sites
    pub fn num_open_sites(&self) -> usize {
        self.opened.iter().filter(|&x| *x == true).count()
    }

    // Does the system percolate?
    pub fn percolates(&self) -> bool {}
}
