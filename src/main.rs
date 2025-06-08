use neexrs::{cli::Cli, runner::Runner, types::RunOptions};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        neexrs::cli::Commands::Run { commands, no_color, quiet, continue_on_error } => {
            let options = RunOptions {
                parallel: false,
                max_parallel: None,
                print_output: !quiet,
                color: !no_color,
                show_timing: true,
                prefix: true,
                stop_on_error: !continue_on_error,
                minimal_output: quiet,
                group_output: false,
                is_server_mode: false,
            };

            let runner = Runner::new(options);
            let results = runner.run(commands).await?;
            
            // Exit with error if any command failed
            let failed = results.iter().any(|r| !r.success);
            if failed {
                std::process::exit(1);
            }
        }
        
        neexrs::cli::Commands::Runx { commands, sequential, max_parallel, no_color, quiet, group } => {
            let options = RunOptions {
                parallel: !sequential,
                max_parallel,
                print_output: !quiet,
                color: !no_color,
                show_timing: true,
                prefix: true,
                stop_on_error: false,
                minimal_output: quiet,
                group_output: group,
                is_server_mode: false,
            };

            let runner = Runner::new(options);
            let results = runner.run(commands).await?;
            
            // Print final status
            let failed_count = results.iter().filter(|r| !r.success).count();
            if failed_count > 0 {
                eprintln!("\n❌ {} command(s) failed", failed_count);
                std::process::exit(1);
            } else {
                println!("\n✅ All commands completed successfully");
            }
        }
        
        neexrs::cli::Commands::Servers { commands, port, restart, env_file } => {
            let options = RunOptions {
                parallel: true,
                max_parallel: None,
                print_output: true,
                color: true,
                show_timing: false,
                prefix: true,
                stop_on_error: false,
                minimal_output: false,
                group_output: true,
                is_server_mode: true,
            };

            let runner = Runner::new(options);
            
            // For server mode, we want to keep running
            println!("🚀 Starting servers...");
            println!("Press Ctrl+C to stop all servers");
            
            // Setup signal handler
            let runner_clone = runner.clone_for_signal();
            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
                println!("\n🛑 Stopping all servers...");
                let _ = runner_clone.kill_all().await;
                std::process::exit(0);
            });
            
            let _results = runner.run(commands).await?;
            
            // Keep the main thread alive for servers
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

    Ok(())
}

// Extension trait for Runner to add signal handling
impl Runner {
    pub fn clone_for_signal(&self) -> Self {
        Self {
            options: self.options.clone(),
            active_processes: std::sync::Arc::clone(&self.active_processes),
            logger: std::sync::Arc::clone(&self.logger),
        }
    }
}