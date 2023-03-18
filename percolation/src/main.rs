use clap::Parser;
use percolation::stats::PercolationStats;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Dimension of grid to create (SIZE * SIZE)
    size: usize,

    /// Number of experiments to run
    exp: usize,
}

fn main() {
    let cli = Cli::parse();
    println!("Hello!");

    let stats = PercolationStats::new(cli.size, cli.exp);
    println!("{:23} = {}", "mean", stats.mean());
    println!("{:23} = {}", "stddev", stats.std_dev());
    println!(
        "{:23} = {}, {}",
        "95% confidence interval",
        stats.confidence_low(),
        stats.confidence_high()
    );
}
