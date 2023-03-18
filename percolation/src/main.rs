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

    // Given a site (row, col) return a list of indexes in the `uf`
    // that represent it's neighbor sites.
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        if row != 1 {
            neighbors.push(self.index_of(row - 1, col));
        }
        if row != self.n {
            neighbors.push(self.index_of(row + 1, col));
        }
        if col != 1 {
            neighbors.push(self.index_of(row, col - 1));
        }
        if col != self.n {
            neighbors.push(self.index_of(row, col + 1));
        }
        neighbors
    }

    // opens the site (row, col) if it is not already open
    pub fn open(&mut self, row: usize, col: usize) {
        // Open the give site
        let index = self.index_of(row, col);
        self.opened[index] = true;

        // Open the sites neighbors
        for i in self.get_neighbors(row, col) {
            if self.opened[i] {
                self.uf.union(index, i);
            }
        }
    }

    // is the site (row, col) open?
    pub fn is_open(&self, row: usize, col: usize) -> bool {
        self.opened[self.index_of(row, col)]
    }

    // is the site (row, col) full?
    pub fn is_full(&mut self, row: usize, col: usize) -> bool {
        let index = self.index_of(row, col);
        for column in 1..self.n + 1 {
            if self.is_open(1, column) && self.uf.connected(self.index_of(1, column), index) {
                return true;
            }
        }
        false
    }

    // returns the number of open sites
    pub fn num_open_sites(&self) -> usize {
        self.opened.iter().filter(|&x| *x == true).count()
    }

    // Does the system percolate?
    pub fn percolates(&mut self) -> bool {
        for col in 1..=self.n {
            if self.is_open(self.n, col) && self.is_full(self.n, col) {
                return true;
            }
        }
        false
    }
}
