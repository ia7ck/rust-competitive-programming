use anyhow::Result;
use clap::Parser;

use oj_test::{OjTestArgs, OjTestRunner};

#[derive(Parser)]
#[clap(name = "oj_test")]
#[clap(about = "Run online judge tests for Rust examples")]
struct Cli {
    /// Path pattern to search for example files
    #[clap(long, default_value = "**/examples/*.rs")]
    pattern: String,

    /// Dry run - show what would be tested without actually running
    #[clap(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    let args = OjTestArgs {
        pattern: cli.pattern,
        dry_run: cli.dry_run,
    };

    let runner = OjTestRunner::new()?;
    runner.run(args)?;

    Ok(())
}
