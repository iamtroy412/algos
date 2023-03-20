use std::fmt;

#[derive(Debug)]
// Our UnionFind struct contains an `id` vector where the items are the
// tree roots for the `id` indexes. ex id[4] = 8 is the root of 4
// The `sz` vector keeps track of the number of objects rooted at `i`.
// ex sz[8 root] = 6 item tree
pub struct UnionFind {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            id: (0..n).collect(),
            sz: vec![0; n],
        }
    }

    // Chase parent pointers until you reach the root
    // of i = id[i]
    fn root(&mut self, n: usize) -> usize {
        let mut i = self.id[n];
        while i != self.id[i] {
            // Path compression - make every other node in path
            // point to its grandparent, thereby halving the path
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }
        i
    }

    // Two items are considered connected if they share the same root.
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    // Join two items by changing one of their roots.
    // If they already have the same root, they are already joined
    // so return early. Else, use the size of their trees as weights
    // to decide which tree root gets linked to the other.
    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);

        if i == j {
            return;
        }

        // Weighted quick union - use the extra sz[i] array to count the
        // number of objects in the tree rooted at i.
        // Compare the size of the trees, link root of smaller tree
        // to larger one.
        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }
}

// Print out our UnionFind
impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.id.iter() {
            write!(f, "{}", i)?;
        }

        Ok(())
    }
}
