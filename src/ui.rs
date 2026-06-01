/// Módulo de UI mejorada para terminal
/// Interfaz limpia tipo chat, optimizada para Termux
use colored::Colorize;
use std::io::{self, Write};

pub struct TerminalUI {}

impl TerminalUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show_header(&self, title: &str) {
        self.clear_screen();
        println!("{}", "╭──────────────────────────────────────────────╮".bright_cyan());
        println!("{} {:^44} {}",
            "│".bright_cyan(),
            title.bright_white().bold(),
            "│".bright_cyan(),
        );
        println!("{}", "╰──────────────────────────────────────────────╯".bright_cyan());
        println!();
    }

    pub fn show_info_panel(&self, title: &str, content: &str) {
        println!("  {} {}", "┌".bright_cyan(), title.bright_white().bold());
        for line in content.lines() {
            println!("  {}  {}", "│".bright_cyan(), line);
        }
        println!("  {}\n", "└".bright_cyan());
    }

    pub fn read_input(&self, _status: &str) -> io::Result<String> {
        let prompt = format!("{} {} ", "┃".bright_cyan(), "🤖 >".bright_yellow().bold());
        print!("{}", prompt);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    pub fn show_response(&self, response: &str) {
        println!("{}", "┃".bright_cyan());
        for line in response.lines() {
            println!(" {} {}", "║".bright_cyan(), line);
        }
        println!("{}", "┃".bright_cyan());
    }

    pub fn show_error(&self, error: &str) {
        println!(" {} {}\n", "●".bright_red().bold(), error.bright_red());
    }

    pub fn show_success(&self, message: &str) {
        println!(" {} {}", "●".bright_green().bold(), message.bright_green());
    }

    pub fn show_info(&self, message: &str) {
        println!(" {} {}", "●".bright_blue().bold(), message.bright_white());
    }

    /// Spinner inline: imprime el mensaje con animación, devuelve un
    /// handle que se debe detener llamando a `.done()` al terminar.
    pub fn start_loading(&self, message: &str) -> LoadingSpinner {
        LoadingSpinner::new(message)
    }

    /// Línea de estado compacta
    pub fn show_context_info(&self, memory_size: usize, skills_count: usize, docs_count: usize) {
        let mem_str = format!("🧠 {}B", memory_size);
        let skill_str = format!("🔧 {}", skills_count);
        let doc_str = format!("📚 {}", docs_count);
        println!(
            "{} {} | {} | {}",
            "┃".bright_cyan(),
            mem_str.bright_black(),
            skill_str.bright_black(),
            doc_str.bright_black(),
        );
    }

    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().ok();
    }
}

/// Spinner que corre en un hilo separado.
/// Llamar `.done()` para detenerlo y mostrar el check.
pub struct LoadingSpinner {
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
    message: String,
}

impl LoadingSpinner {
    pub fn new(message: &str) -> Self {
        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let r = running.clone();
        let msg = message.to_string();

        let handle = std::thread::spawn(move || {
            let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;
            while r.load(std::sync::atomic::Ordering::Relaxed) {
                print!("\r {} {}... ", frames[i % frames.len()].bright_yellow(), msg.bright_white());
                io::stdout().flush().ok();
                std::thread::sleep(std::time::Duration::from_millis(80));
                i += 1;
            }
            print!("\r");
            io::stdout().flush().ok();
        });

        Self {
            running,
            handle: Some(handle),
            message: message.to_string(),
        }
    }

    pub fn done(self) {
        self.running.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(h) = self.handle {
            h.join().ok();
        }
        println!(" {} {} {}", "●".bright_green(), "✔".bright_green(), self.message.bright_white());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        let _ui = TerminalUI::new();
    }
}
