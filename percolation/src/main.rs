use percolation::stats::PercolationStats;

fn main() {
    println!("Hello!");

    let stats = PercolationStats::new(200, 100);
    println!("{:23} = {}", "mean", stats.mean());
    println!("{:23} = {}", "stddev", stats.std_dev());
    println!(
        "{:23} = {}, {}",
        "95% confidence interval",
        stats.confidence_low(),
        stats.confidence_high()
    );
}
