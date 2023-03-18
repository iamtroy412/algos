use crate::percolation::Percolation;
use rand::{thread_rng, Rng};

pub struct PercolationStats {
    t: usize,
    xs: Vec<f64>,
}

impl PercolationStats {
    pub fn new(n: usize, t: usize) -> Self {
        let mut random = thread_rng();
        let mut xs = Vec::new();
        let n_site = (n as f64) * (n as f64);

        for _ in 0..t {
            let mut sites = Percolation::new(n);
            let mut count = 0_f64;

            loop {
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
            xs.push(count / n_site);
        }
        PercolationStats { t, xs }
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