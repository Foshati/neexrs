use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neex")]
#[command(about = "🚀 The Modern Build System for Polyrepo-in-Monorepo Architecture")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(long_about = "
neex is a powerful command runner inspired by npm-run-all with modern features:
- Run commands sequentially or in parallel
- Beautiful colored output with spinners
- Server mode for development
- Smart error handling and reporting
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run commands sequentially
    #[command(alias = "s")]
    #[command(long_about = "Run commands one after another. Stops on first error.")]
    Run {
        /// Commands to run
        commands: Vec<String>,
        
        #[arg(short = 'c', long, help = "Disable colored output")]
        no_color: bool,
        
        #[arg(short = 'q', long, help = "Minimal output")]
        quiet: bool,
        
        #[arg(long, help = "Continue even if a command fails")]
        continue_on_error: bool,
    },
    
    /// Run commands in parallel (default) or sequentially with -s
    #[command(alias = "p")]  
    #[command(long_about = "Run commands in parallel for faster execution. Use -s for sequential.")]
    Runx {
        /// Commands to run
        commands: Vec<String>,
        
        #[arg(short = 's', long, help = "Run sequentially instead of parallel")]
        sequential: bool,
        
        #[arg(short = 'j', long, value_name = "N", help = "Maximum parallel jobs")]
        max_parallel: Option<usize>,
        
        #[arg(short = 'c', long, help = "Disable colored output")]
        no_color: bool,
        
        #[arg(short = 'q', long, help = "Minimal output")]
        quiet: bool,
        
        #[arg(long, help = "Group output by command")]
        group: bool,
    },
    
    /// Run multiple servers in parallel
    #[command(alias = "srv")]
    #[command(long_about = "Run multiple servers in parallel. Perfect for development environments.")]
    Servers {
        /// Server commands to run
        commands: Vec<String>,
        
        #[arg(short = 'p', long, value_name = "PORT", help = "Base port number")]
        port: Option<u16>,
        
        #[arg(long, help = "Auto-restart servers on failure")]
        restart: bool,
        
        #[arg(short = 'e', long, value_name = "FILE", help = "Environment file to load")]
        env_file: Option<String>,
    },
}