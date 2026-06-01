/// Módulo de Comandos
/// Implementa el sistema de comandos del agente
use crate::{
    filesystem::FileSystemManager, memory::MemoryManager, skills::SkillsManager,
    rag::RAGEngine,
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
    Script,
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
        } else if input.starts_with("/script") || input.starts_with("/ejecutar") {
            CommandType::Script
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
            CommandType::Script => self.cmd_script(&cmd.args).await,
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

    async fn cmd_script(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            let ts_skills = self.skills.list_ts_skills().await?;
            if ts_skills.is_empty() {
                return Ok("📜 No hay skills TypeScript disponibles. Crea un archivo .ts en skills/".to_string());
            }
            let mut msg = "📜 Skills TypeScript disponibles:\n\n".to_string();
            for s in &ts_skills {
                let source = self.skills.read_ts_source(s).await.unwrap_or_default();
                let desc = source.lines()
                    .find(|l| l.starts_with("// desc:"))
                    .map(|l| l.trim_start_matches("// desc:").trim())
                    .unwrap_or("Sin descripción");
                msg.push_str(&format!("  /script {} — {}\n", s, desc));
            }
            msg.push_str("\nUsa: /script <skill> [argumentos...]");
            return Ok(msg);
        }

        let skill_name = &args[0];
        if !self.skills.ts_skill_exists(skill_name).await {
            return Ok(format!(
                "❌ Skill '{}' no encontrado. Skills disponibles:\n{}",
                skill_name,
                self.skills.list_ts_skills().await?
                    .iter()
                    .map(|s| format!("  /script {}", s))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }

        let input = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            String::new()
        };

        match self.skills.run_ts_skill(skill_name, &input).await {
            Ok(output) => {
                if output.trim().is_empty() {
                    Ok(format!("✓ Skill '{}' ejecutado (sin output)", skill_name))
                } else {
                    Ok(output.trim().to_string())
                }
            }
            Err(e) => Ok(format!("❌ Error: {}", e)),
        }
    }

    fn cmd_ayuda(&self) -> String {
        "🆘 Comandos disponibles:\n\n\
         /memoria      - Mostrar memoria\n\
         /skills       - Listar skills\n\
         /conocimiento - Listar conocimientos\n\
         /buscar TEXTO - Buscar en conocimiento\n\
         /script NOMBRE - Ejecutar skill TypeScript via Deno\n\
         /stats        - Mostrar estadísticas\n\
         /exportar     - Exportar backup\n\
         /ayuda        - Mostrar esta ayuda"
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
        let text_lower = text.to_lowercase();
        let re = Regex::new(
            r"(?i)crea\s+conocimiento\s+(?:llamado|llamada|sobre)\s+(\w+)\s+(?:en|de|en\s+la\s+categoría)\s+(\w+)"
        ).unwrap();
        if let Some(cap) = re.captures(&text_lower) {
            Some((cap[2].to_string(), cap[1].to_string()))
        } else {
            None
        }
    }

    /// Detectar información importante automáticamente
    pub fn detect_auto_save_content(&self, text: &str) -> Option<String> {
        let _text_lower = text.to_lowercase();
        
        // Patrones para información valiosa
        let patterns = [
            // Información personal
            (r"(?i)(me llamo|mi nombre es|soy|nombre:\s*)([^.,\n]+)", "Información personal: "),
            // Habilidades
            (r"(?i)(sé de|conozco|domino|especialista en)([^.,\n]+)", "Habilidad: "),
            // Proyectos
            (r"(?i)(estoy (trabajando|desarrollando) en|proyecto de)([^.,\n]+)", "Proyecto: "),
            // URLs y referencias
            (r"(https?://[^\s]+)", "Referencia: "),
            // Decisiones importantes
            (r"(?i)(decidí|voy a|planeo|plan)([^.,\n]+)", "Plan/Decisión: "),
        ];
        
        for (pattern, prefix) in &patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(cap) = re.captures(text) {
                    let content = if cap.len() > 2 {
                        cap[2].trim().to_string()
                    } else {
                        cap[0].to_string()
                    };
                    
                    if !content.is_empty() && content.len() > 3 {
                        return Some(format!("{}{}", prefix, content));
                    }
                }
            }
        }
        
        None
    }

    /// Detectar si hay información valiosa que guardar automáticamente
    pub async fn auto_save_if_needed(&self, text: &str) -> Result<Option<String>> {
        // No guardar si es muy corto o es un comando
        if text.len() < 10 || text.starts_with('/') || text.starts_with('!') {
            return Ok(None);
        }
        
        // Detectar patrones de información importante
        if let Some(content) = self.detect_auto_save_content(text) {
            self.memory.save(&content).await?;
            return Ok(Some(format!("💾 Auto-guardado: {}", content)));
        }
        
        Ok(None)
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
        assert!(processor.detect_memory_intent("recuerda mi nombre en memoria"));
    }

    #[test]
    fn test_detect_skill_creation() {
        let processor = CommandProcessor::new("./agente");
        let skill = processor.detect_skill_creation("crea una skill llamada python");
        assert_eq!(skill, Some("python".to_string()));
    }
}
