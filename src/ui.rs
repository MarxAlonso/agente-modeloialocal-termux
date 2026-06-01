/// Módulo de UI mejorada para terminal
/// Interfaz visual moderna similar a OpenCode/OpenClaw
use colored::*;
use std::io::{self, Write};

/// Estructura para gestionar la UI
pub struct TerminalUI {
    width: usize,
    show_status: bool,
}

impl TerminalUI {
    /// Crear nueva instancia de UI
    pub fn new() -> Self {
        Self {
            width: 80,
            show_status: true,
        }
    }

    /// Mostrar header decorativo
    pub fn show_header(&self, title: &str) {
        self.clear_screen();
        println!("{}", "═".repeat(self.width).bright_cyan());
        println!();
        println!("  {} {} {}", "▶".bright_cyan(), title.bright_white().bold(), "◀".bright_cyan());
        println!();
        println!("{}", "═".repeat(self.width).bright_cyan());
        println!();
    }

    /// Mostrar panel de información
    pub fn show_info_panel(&self, title: &str, content: &str) {
        println!("\n{} {}", "┌─".bright_cyan(), title.bright_white().bold());
        println!("{}", "│".bright_cyan());
        
        for line in content.lines() {
            println!("{} {}", "│".bright_cyan(), line);
        }
        
        println!("{}", "│".bright_cyan());
        println!("{}\n", "└─".bright_cyan());
    }

    /// Mostrar opciones en formato menú
    pub fn show_menu(&self, options: Vec<(&str, &str)>) {
        println!("\n{} Opciones disponibles:\n", "⚙".bright_yellow());
        
        for (i, (cmd, desc)) in options.iter().enumerate() {
            println!(
                "  {} {} {} - {}",
                (i + 1).to_string().bright_yellow(),
                "│".bright_cyan(),
                cmd.bright_white().bold(),
                desc.bright_black()
            );
        }
        println!();
    }

    /// Leer input del usuario con prompt mejorado
    pub fn read_input(&self, status: &str) -> io::Result<String> {
        self.print_prompt(status);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }

    /// Imprimir prompt personalizado
    pub fn print_prompt(&self, status: &str) {
        print!(
            "{}",
            format!("\n┌─ {} ──\n│ 🤖 > ", status).bright_cyan()
        );
    }

    /// Mostrar resultado del agente
    pub fn show_response(&self, response: &str) {
        println!("│");
        println!("{} {}", "└─ Respuesta:".bright_cyan(), "┐".bright_cyan());
        
        for line in response.lines() {
            println!("  {} {}", "║".bright_cyan(), line);
        }
        
        println!("{}\n", "  └─".bright_cyan());
    }

    /// Mostrar mensaje de error
    pub fn show_error(&self, error: &str) {
        println!(
            "\n{} {}\n",
            "✗ Error:".bright_red().bold(),
            error.bright_red()
        );
    }

    /// Mostrar mensaje de éxito
    pub fn show_success(&self, message: &str) {
        println!(
            "\n{} {}\n",
            "✓".bright_green().bold(),
            message.bright_green()
        );
    }

    /// Mostrar mensaje de información
    pub fn show_info(&self, message: &str) {
        println!(
            "\n{} {}\n",
            "ℹ".bright_blue(),
            message.bright_white()
        );
    }

    /// Mostrar barra de progreso
    pub fn show_progress(&self, label: &str, percent: usize) {
        let filled = (self.width - 20) * percent / 100;
        let empty = self.width - 20 - filled;
        
        print!(
            "\r{} [{}{}] {}%",
            label.bright_yellow(),
            "█".bright_green().repeat(filled),
            "░".bright_black().repeat(empty),
            percent
        );
        io::stdout().flush().ok();
    }

    /// Limpiar pantalla
    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().ok();
    }

    /// Mostrar tabla de datos
    pub fn show_table(&self, headers: Vec<&str>, rows: Vec<Vec<&str>>) {
        println!("\n{}", "┌".bright_cyan());
        
        // Headers
        print!("{} ", "│".bright_cyan());
        for (i, header) in headers.iter().enumerate() {
            if i < headers.len() - 1 {
                print!("{} {} ", header.bright_white().bold(), "│".bright_cyan());
            } else {
                print!("{} {}\n", header.bright_white().bold(), "│".bright_cyan());
            }
        }
        
        println!("{}", "├".bright_cyan());
        
        // Rows
        for row in rows {
            print!("{} ", "│".bright_cyan());
            for (i, cell) in row.iter().enumerate() {
                if i < row.len() - 1 {
                    print!("{} {} ", cell, "│".bright_cyan());
                } else {
                    print!("{} {}\n", cell, "│".bright_cyan());
                }
            }
        }
        
        println!("{}\n", "└".bright_cyan());
    }

    /// Mostrar contexto activo
    pub fn show_context_info(
        &self,
        memory_size: usize,
        skills_count: usize,
        docs_count: usize,
    ) {
        println!(
            "\n{} {} │ {} {} │ {} {}",
            "📝".bright_yellow(),
            format!("Memoria: {}B", memory_size).bright_white(),
            "🔧".bright_cyan(),
            format!("Skills: {}", skills_count).bright_white(),
            "📚".bright_green(),
            format!("Docs: {}", docs_count).bright_white()
        );
    }

    /// Mostrar separador
    pub fn show_separator(&self) {
        println!("{}\n", "─".repeat(self.width).bright_black());
    }

    /// Mostrar loading animation
    pub fn show_loading(&self, message: &str) {
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        
        for i in 0..20 {
            print!(
                "\r{} {}...",
                frames[i % frames.len()].bright_yellow(),
                message.bright_white()
            );
            io::stdout().flush().ok();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        println!("\r{} {} ✓", "✓".bright_green(), message.bright_white());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_creation() {
        let ui = TerminalUI::new();
        assert_eq!(ui.width, 80);
    }
}
