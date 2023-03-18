use crate::percolation::Percolation;
use indicatif::ParallelProgressIterator;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub struct PercolationStats {
    t: usize,
    xs: Vec<f64>,
}

impl PercolationStats {
    pub fn new(n: usize, t: usize) -> Self {
        let xs = Arc::new(Mutex::new(Vec::new()));
        let n_site = (n as f64) * (n as f64);

        (0..t).into_par_iter().progress().for_each(|_| {
            let clone = Arc::clone(&xs);
            let mut sites = Percolation::new(n);
            let mut count = 0_f64;

            loop {
                let mut random = thread_rng();
                let r = random.gen_range(0..n * n);
                let i = r / n + 1;
                let j = r % n + 1;

                if sites.is_open(i, j) {
                    continue;
                }
                sites.open(i, j);
                count += 1.0;
                if sites.percolates() {
                    break;
                }
            }
            let mut v = clone.lock().unwrap();
            v.push(count / n_site);
        });
        // This works but just doesn't look/feel right...
        // Will revisit arcs and mutex and threads in more detail
        // at a later date.
        let inner = Arc::try_unwrap(xs).unwrap().into_inner().unwrap();
        PercolationStats { t, xs: inner }
    }

    // sample mean of percolation threshold
    pub fn mean(&self) -> f64 {
        self.xs.iter().sum::<f64>() / self.xs.len() as f64
    }

    pub fn std_dev(&self) -> f64 {
        match (self.mean(), self.xs.len()) {
            (data_mean, count) if count > 0 => {
                let variance = self
                    .xs
                    .iter()
                    .map(|value| {
                        let diff = data_mean - *value;
                        diff * diff
                    })
                    .sum::<f64>()
                    / count as f64;
                variance.sqrt()
            }
            _ => 0.0,
        }
    }

    // high endpoint of 95% confidence interval
    pub fn confidence_high(&self) -> f64 {
        self.mean() + 1.96 * self.std_dev() / (self.t as f64).sqrt()
    }

    // low endpoint of 95% confidence interval
    pub fn confidence_low(&self) -> f64 {
        self.mean() - 1.96 * self.std_dev() / (self.t as f64).sqrt()
    }
}
