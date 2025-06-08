use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neex")]
#[command(about = "The Modern Build System for Polyrepo-in-Monorepo Architecture")]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run commands sequentially
    #[command(alias = "s")]
    Run {
        commands: Vec<String>,
        #[arg(short = 'c', long, help = "Disable colored output")]
        no_color: bool,
        // ... other options
    },
    /// Run commands in parallel (default) or sequentially with -q
    #[command(alias = "p")]  
    Runx {
        commands: Vec<String>,
        #[arg(short = 'q', long, help = "Run sequentially")]
        sequential: bool,
        // ... other options
    },
    /// Run multiple servers
    #[command(alias = "srv")]
    Servers {
        commands: Vec<String>,
        // ... server specific options
    },
}