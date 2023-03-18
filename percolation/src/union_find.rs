use std::fmt;

#[derive(Debug)]
pub struct UnionFind {
    id: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            id: vec![0; n],
            sz: vec![0; n],
        }
    }

    fn root(&mut self, n: usize) -> usize {
        let mut i = self.id[n];
        while i != self.id[i] {
            self.id[i] = self.id[self.id[i]];
            i = self.id[i];
        }
        i
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);

        if i == j {
            return;
        }

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
