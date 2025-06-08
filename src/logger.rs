use std::collections::HashMap;
use std::time::Duration;
use crossterm::style::Color;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;

pub struct Logger {
    colors: HashMap<String, Color>,
    spinners: HashMap<String, ProgressBar>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            colors: HashMap::new(),
            spinners: HashMap::new(),
        }
    }

    pub fn setup_commands(&mut self, commands: &[String]) {
        self.print_banner();
        
        // Assign colors to commands
        let vibrant_colors = [
            Color::Cyan, Color::Green, Color::Yellow, 
            Color::Magenta, Color::Blue, Color::Red,
        ];
        
        for (i, cmd) in commands.iter().enumerate() {
            let color = vibrant_colors[i % vibrant_colors.len()];
            self.colors.insert(cmd.clone(), color);
        }
    }

    pub fn start_spinner(&mut self, command: &str) {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.blue} {msg}")
                .unwrap()
        );
        pb.set_message(format!("{}: Running...", command));
        pb.enable_steady_tick(Duration::from_millis(80));
        
        self.spinners.insert(command.to_string(), pb);
    }

    pub fn stop_spinner(&mut self, command: &str, success: bool) {
        if let Some(pb) = self.spinners.remove(command) {
            let status = if success { "✓ Completed" } else { "✗ Failed" };
            pb.finish_with_message(format!("{}: {}", command, status));
        }
    }

    pub fn print_banner(&self) {
        println!("\n🚀 neex - Modern Build System");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }

    pub fn print_command_output(&self, command: &str, output: &str, is_error: bool) {
        let _color = self.colors.get(command).unwrap_or(&Color::White);
        let prefix = format!("[{}]", command).cyan();
        
        if is_error {
            eprintln!("{} {}", prefix, output.red());
        } else {
            println!("{} {}", prefix, output);
        }
    }

    pub fn print_summary(&self, results: &[crate::types::RunResult]) {
        println!("\n📊 Execution Summary");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        for result in results {
            let status = if result.success { "✓".green() } else { "✗".red() };
            let duration = result.duration
                .map(|d| format!("{}ms", d.as_millis()))
                .unwrap_or_else(|| "N/A".to_string());
            
            println!("{} {} ({})", status, result.command, duration);
        }
    }
}