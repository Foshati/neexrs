use neex::{cli::Cli, runner::Runner, types::RunOptions};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // match based on cli.command
    // ساخت Runner و اجرای دستورات بر اساس نوع اجرا
    Ok(())
}
