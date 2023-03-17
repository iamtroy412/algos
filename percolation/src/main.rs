use percolation::UnionFind;

struct Percolation {
    n: usize,
    uf: UnionFind,
    opened: Vec<bool>,
}
