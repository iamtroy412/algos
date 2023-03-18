use percolation::percolation::Percolation;
use rand::{thread_rng, Rng};

struct PercolationStats {
    t: usize,
    xs: Vec<f64>,
}

impl PercolationStats {
    fn new(n: usize, t: usize) -> Self {
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
}

fn main() {
    println!("Hello");
}
