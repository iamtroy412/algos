use percolation::UnionFind;

pub struct Percolation {
    n: usize,
    uf: UnionFind,
    opened: Vec<bool>,
}

impl Percolation {
    pub fn new(n: usize) -> Self {
        Percolation {
            n,
            uf: UnionFind::new(n * n),
            opened: vec![false; n * n],
        }
    }

    pub fn open(&mut self, row: usize, col: usize) {}

    pub fn is_open(&self, row: usize, col: usize) -> bool {}

    pub fn is_full(&self, row: usize, col: usize) -> bool {}

    pub fn num_open_sites(&self) -> usize {}

    // Does the system percolate?
    pub fn percolates(&self) -> bool {}
}
