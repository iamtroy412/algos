use clap::Parser;
use percolation::stats::PercolationStats;
use std::ops::RangeFrom;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Dimension of grid to create (SIZE * SIZE)
    size: usize,

    /// Number of experiments to run
    #[arg(value_parser = exp_in_range)]
    exp: usize,
}

const MIN_EXP: RangeFrom<usize> = 30..;

fn exp_in_range(s: &str) -> Result<usize, String> {
    let exp: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if MIN_EXP.contains(&exp) {
        Ok(exp)
    } else {
        Err(format!(
            "Number of experiments needs to be greater than {}!",
            MIN_EXP.start
        ))
    }
}

fn main() {
    let cli = Cli::parse();
    println!("Hello!");
    println!("Running experiments...");

    let stats = PercolationStats::new(cli.size, cli.exp);
    println!("Results:");
    println!("{:23} = {}", "mean", stats.mean());
    println!("{:23} = {}", "stddev", stats.std_dev());
    println!(
        "{:23} = {}, {}",
        "95% confidence interval",
        stats.confidence_low(),
        stats.confidence_high()
    );
}
