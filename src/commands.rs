/// Módulo de Comandos
/// Implementa el sistema de comandos del agente
use crate::{
    filesystem::FileSystemManager, memory::MemoryManager, skills::SkillsManager,
    rag::RAGEngine, ollama::OllamaClient,
};
use anyhow::Result;
use regex::Regex;

/// Tipos de comandos disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Memoria,
    Skills,
    Conocimiento,
    Buscar,
    Stats,
    Exportar,
    Ayuda,
    Desconocido,
}

/// Estructura de un comando
pub struct Command {
    pub cmd_type: CommandType,
    pub args: Vec<String>,
}

/// Procesador de comandos
pub struct CommandProcessor {
    memory: MemoryManager,
    skills: SkillsManager,
    rag: RAGEngine,
    fs: FileSystemManager,
}

impl CommandProcessor {
    /// Crear nuevo procesador de comandos
    pub fn new(base_path: &str) -> Self {
        Self {
            memory: MemoryManager::new(base_path),
            skills: SkillsManager::new(base_path),
            rag: RAGEngine::new(base_path),
            fs: FileSystemManager::new(base_path),
        }
    }

    /// Parsear entrada de usuario
    pub fn parse(&self, input: &str) -> Command {
        let input = input.trim();

        let cmd_type = if input.starts_with("/memoria") {
            CommandType::Memoria
        } else if input.starts_with("/skills") {
            CommandType::Skills
        } else if input.starts_with("/conocimiento") {
            CommandType::Conocimiento
        } else if input.starts_with("/buscar") {
            CommandType::Buscar
        } else if input.starts_with("/stats") {
            CommandType::Stats
        } else if input.starts_with("/exportar") {
            CommandType::Exportar
        } else if input.starts_with("/ayuda") {
            CommandType::Ayuda
        } else {
            CommandType::Desconocido
        };

        let args = input
            .split_whitespace()
            .skip(1)
            .map(|s| s.to_string())
            .collect();

        Command { cmd_type, args }
    }

    /// Ejecutar comando
    pub async fn execute(&self, cmd: &Command) -> Result<String> {
        match cmd.cmd_type {
            CommandType::Memoria => self.cmd_memoria().await,
            CommandType::Skills => self.cmd_skills().await,
            CommandType::Conocimiento => self.cmd_conocimiento().await,
            CommandType::Buscar => {
                let query = cmd.args.join(" ");
                self.cmd_buscar(&query).await
            }
            CommandType::Stats => self.cmd_stats().await,
            CommandType::Exportar => self.cmd_exportar().await,
            CommandType::Ayuda => Ok(self.cmd_ayuda()),
            CommandType::Desconocido => Ok("Comando desconocido. Escribe /ayuda para ver opciones.".to_string()),
        }
    }

    // Implementar comandos individuales

    async fn cmd_memoria(&self) -> Result<String> {
        let memory = self.memory.read().await?;
        if memory.is_empty() {
            return Ok("📝 Memoria vacía".to_string());
        }
        Ok(format!("📝 Memoria:\n\n{}", memory))
    }

    async fn cmd_skills(&self) -> Result<String> {
        let skills = self.skills.list_skills().await?;
        if skills.is_empty() {
            return Ok("🔧 No hay skills disponibles".to_string());
        }
        Ok(format!("🔧 Skills:\n\n{}", skills.join("\n  - ")))
    }

    async fn cmd_conocimiento(&self) -> Result<String> {
        let mut output = "📚 Conocimiento:\n\n".to_string();

        for category in &["proyectos", "personas", "ideas", "investigaciones"] {
            let items = self.rag.list_knowledge(category).await?;
            if !items.is_empty() {
                output.push_str(&format!(
                    "**{}:**\n  - {}\n\n",
                    category,
                    items.join("\n  - ")
                ));
            }
        }

        if output == "📚 Conocimiento:\n\n" {
            output = "📚 No hay conocimientos registrados".to_string();
        }

        Ok(output)
    }

    async fn cmd_buscar(&self, query: &str) -> Result<String> {
        let docs = self.rag.retrieve(query, 5).await?;
        
        if docs.is_empty() {
            return Ok("🔍 No se encontraron resultados".to_string());
        }

        let mut output = format!("🔍 Resultados para '{}'\n\n", query);
        for (i, doc) in docs.iter().enumerate() {
            output.push_str(&format!(
                "{}. {} (relevancia: {:.0}%)\n",
                i + 1,
                doc.path,
                doc.relevance * 100.0
            ));
        }

        Ok(output)
    }

    async fn cmd_stats(&self) -> Result<String> {
        Ok(self.fs.get_stats().await?)
    }

    async fn cmd_exportar(&self) -> Result<String> {
        self.fs.export_backup("auto").await?;
        Ok("💾 Backup exportado correctamente".to_string())
    }

    fn cmd_ayuda(&self) -> String {
        "🆘 Comandos disponibles:\n\n\
         /memoria     - Mostrar memoria\n\
         /skills      - Listar skills\n\
         /conocimiento - Listar conocimientos\n\
         /buscar TEXTO - Buscar en conocimiento\n\
         /stats       - Mostrar estadísticas\n\
         /exportar    - Exportar backup\n\
         /ayuda       - Mostrar esta ayuda"
            .to_string()
    }

    /// Detectar intención especial en texto
    pub fn detect_memory_intent(&self, text: &str) -> bool {
        let re = Regex::new(r"(guarda|recuerda|memoriza|actualiza).*memoria").unwrap();
        re.is_match(&text.to_lowercase())
    }

    /// Detectar creación de skills
    pub fn detect_skill_creation(&self, text: &str) -> Option<String> {
        let re = Regex::new(r"crea.*skill.*llamada\s+(\w+)").unwrap();
        re.captures(&text.to_lowercase())
            .map(|cap| cap[1].to_string())
    }

    /// Detectar creación de conocimiento
    pub fn detect_knowledge_creation(&self, text: &str) -> Option<(String, String)> {
        let re = Regex::new(r"crea.*conocimiento\s+(\w+)\s+(\w+)").unwrap();
        re.captures(&text.to_lowercase()).map(|cap| {
            (cap[1].to_string(), cap[2].to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_memory_command() {
        let processor = CommandProcessor::new("./agente");
        let cmd = processor.parse("/memoria");
        assert_eq!(cmd.cmd_type, CommandType::Memoria);
    }

    #[test]
    fn test_detect_memory_intent() {
        let processor = CommandProcessor::new("./agente");
        assert!(processor.detect_memory_intent("guarda esto en memoria"));
        assert!(processor.detect_memory_intent("recuerda mi nombre"));
    }

    #[test]
    fn test_detect_skill_creation() {
        let processor = CommandProcessor::new("./agente");
        let skill = processor.detect_skill_creation("crea una skill llamada python");
        assert_eq!(skill, Some("python".to_string()));
    }
}
