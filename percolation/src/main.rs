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
}
