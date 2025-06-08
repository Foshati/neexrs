use colored::*;
use crossterm::style::{Color, SetForegroundColor};
use indicatif::{ProgressBar, ProgressStyle};

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
}