use anyhow::Result;
use clap::Parser as _;
use vizshhist::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    let exitcode = vizshhist::run(args)?;
    std::process::exit(exitcode);
}
