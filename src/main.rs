use neexrs::{cli::Cli, runner::Runner, types::RunOptions};
use clap::Parser;
use std::sync::Arc;

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
        
        neexrs::cli::Commands::Servers { commands, port: _port, restart: _restart, env_file: _env_file } => {
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
            let runner_clone = Arc::clone(&runner.get_active_processes());
            tokio::spawn(async move {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
                println!("\n🛑 Stopping all servers...");
                // Kill all active processes
                let mut processes = runner_clone.lock().await;
                for (_, mut child) in processes.drain() {
                    let _ = child.kill().await;
                }
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