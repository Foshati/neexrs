use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use futures::future::try_join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Runner {
    options: RunOptions,
    active_processes: Arc<Mutex<HashMap<String, tokio::process::Child>>>,
}

impl Runner {
    pub fn new(options: RunOptions) -> Self {
        Self {
            options,
            active_processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&self, commands: Vec<String>) -> anyhow::Result<Vec<RunResult>> {
        if self.options.parallel {
            self.run_parallel(commands).await
        } else {
            self.run_sequential(commands).await
        }
    }

    async fn run_command(&self, command: String) -> anyhow::Result<RunResult> {
        let start_time = SystemTime::now();
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(&command);
        
        // Configure stdio
        cmd.stdout(std::process::Stdio::piped())
           .stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn()?;
        
        // Handle output streams
        // ... implementation for capturing and displaying output
        
        let status = child.wait().await?;
        let end_time = SystemTime::now();
        
        Ok(RunResult {
            command,
            success: status.success(),
            exit_code: status.code(),
            start_time,
            end_time: Some(end_time),
            duration: end_time.duration_since(start_time).ok(),
            error: None,
            output: vec![], // populated during execution
        })
    }
}