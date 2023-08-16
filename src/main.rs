//! sleep-interactive - Like sleep but with a visual progress indicator and more explicit param names.
use clap::Parser;
use core::time::Duration;
use indicatif::ProgressBar;
use std::thread::sleep;

/// Allowed arguments to be parsed by `clap`
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Seconds to sleep for
    #[arg(short, long)]
    seconds: Option<u64>,

    /// Minutes to sleep for
    #[arg(short, long)]
    minutes: Option<u64>,
}

fn main() {
    let args = Args::parse();
    let seconds = args
        .seconds
        .unwrap_or_default()
        .saturating_add(args.minutes.unwrap_or_default().saturating_mul(60));

    let pb = ProgressBar::new(seconds);
    for _ in 0..seconds {
        pb.inc(1);
        sleep(Duration::from_secs(1));
    }
    pb.finish();
}
