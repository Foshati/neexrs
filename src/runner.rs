use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::future::try_join_all;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use crate::types::{RunOptions, RunResult, CommandOutput, OutputType};
use crate::logger::Logger;

pub struct Runner {
    pub(crate) options: RunOptions,
    pub(crate) active_processes: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
    pub(crate) logger: Arc<Mutex<Logger>>,
}

impl Runner {
    pub fn new(options: RunOptions) -> Self {
        Self {
            options,
            active_processes: Arc::new(Mutex::new(HashMap::new())),
            logger: Arc::new(Mutex::new(Logger::new())),
        }
    }

    pub fn get_active_processes(&self) -> &Arc<Mutex<HashMap<String, tokio::process::Child>>> {
        &self.active_processes
    }

    pub async fn run(&self, commands: Vec<String>) -> anyhow::Result<Vec<RunResult>> {
        // Setup logger
        {
            let mut logger = self.logger.lock().await;
            logger.setup_commands(&commands);
        }

        if self.options.parallel {
            self.run_parallel(commands).await
        } else {
            self.run_sequential(commands).await
        }
    }

    async fn run_parallel(&self, commands: Vec<String>) -> anyhow::Result<Vec<RunResult>> {
        let futures = commands.into_iter().map(|cmd| {
            let runner = self.clone_for_parallel();
            async move { runner.run_command(cmd).await }
        });

        let results = try_join_all(futures).await?;
        
        // Print summary
        {
            let logger = self.logger.lock().await;
            logger.print_summary(&results);
        }

        Ok(results)
    }

    async fn run_sequential(&self, commands: Vec<String>) -> anyhow::Result<Vec<RunResult>> {
        let mut results = Vec::new();
        
        for command in commands {
            let result = self.run_command(command).await?;
            
            if !result.success && self.options.stop_on_error {
                results.push(result);
                break;
            }
            
            results.push(result);
        }

        // Print summary
        {
            let logger = self.logger.lock().await;
            logger.print_summary(&results);
        }

        Ok(results)
    }

    async fn run_command(&self, command: String) -> anyhow::Result<RunResult> {
        let start_time = SystemTime::now();
        
        // Start spinner
        {
            let mut logger = self.logger.lock().await;
            logger.start_spinner(&command);
        }

        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&command);
        
        // Configure stdio
        cmd.stdout(std::process::Stdio::piped())
           .stderr(std::process::Stdio::piped());

        let child = cmd.spawn()?;
        
        // Store the child process
        {
            let mut processes = self.active_processes.lock().await;
            processes.insert(command.clone(), child);
        }
        
        // Retrieve the child process for execution
        let mut child = {
            let mut processes = self.active_processes.lock().await;
            processes.remove(&command).unwrap()
        };
        
        let mut outputs = Vec::new();
        
        // Handle stdout
        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            
            while reader.read_line(&mut line).await? > 0 {
                let output = CommandOutput {
                    command: command.clone(),
                    output_type: OutputType::Stdout,
                    data: line.trim().to_string(),
                    timestamp: SystemTime::now(),
                };
                
                if self.options.print_output {
                    let logger = self.logger.lock().await;
                    logger.print_command_output(&command, &line.trim(), false);
                }
                
                outputs.push(output);
                line.clear();
            }
        }
        
        let status = child.wait().await?;
        let end_time = SystemTime::now();
        
        let success = status.success();
        
        // Stop spinner
        {
            let mut logger = self.logger.lock().await;
            logger.stop_spinner(&command, success);
        }
        
        Ok(RunResult {
            command,
            success,
            exit_code: status.code(),
            start_time,
            end_time: Some(end_time),
            duration: end_time.duration_since(start_time).ok(),
            error: if success { None } else { Some("Command failed".to_string()) },
            output: outputs,
        })
    }

    fn clone_for_parallel(&self) -> Self {
        Self {
            options: self.options.clone(),
            active_processes: Arc::clone(&self.active_processes),
            logger: Arc::clone(&self.logger),
        }
    }

    pub async fn kill_all(&self) -> anyhow::Result<()> {
        let mut processes = self.active_processes.lock().await;
        for (_, mut child) in processes.drain() {
            let _ = child.kill().await;
        }
        Ok(())
    }
}