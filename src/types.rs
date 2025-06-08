use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunOptions {
    pub parallel: bool,
    pub max_parallel: Option<usize>,
    pub print_output: bool,
    pub color: bool,
    pub show_timing: bool,
    pub prefix: bool,
    pub stop_on_error: bool,
    pub minimal_output: bool,
    pub group_output: bool,
    pub is_server_mode: bool,
}

#[derive(Debug, Clone)]
pub struct RunResult {
    pub command: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub duration: Option<Duration>,
    pub error: Option<String>,
    pub output: Vec<CommandOutput>,
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub command: String,
    pub output_type: OutputType,
    pub data: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone)]
pub enum OutputType {
    Stdout,
    Stderr,
}