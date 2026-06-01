/// Agente Autónomo Local para Ollama en Rust
/// Sistema inteligente con memoria, skills y RAG para Termux/Linux
/// Interfaz mejorada con guardado automático de memoria
mod commands;
mod filesystem;
mod memory;
mod ollama;
mod rag;
mod skills;
mod ui;

use anyhow::Result;
use chrono::Local;
use commands::CommandProcessor;
use filesystem::FileSystemManager;
use memory::MemoryManager;
use ollama::OllamaClient;
use rag::RAGEngine;
use skills::SkillsManager;
use ui::TerminalUI;
use toml::Table;

/// Configuración del agente
struct Config {
    ollama_url: String,
    ollama_model: String,
    ollama_timeout: u64,
    memory_limit: usize,
    enable_rag: bool,
    enable_auto_save: bool,
}

impl Config {
    /// Cargar configuración desde archivo
    async fn load() -> Result<Self> {
        let config_path = "./config/config.toml";
        
        let config_str = std::fs::read_to_string(config_path)
            .unwrap_or_else(|_| DEFAULT_CONFIG.to_string());

        let table: Table = config_str.parse()?;

        let ollama = table.get("ollama").and_then(|t| t.as_table()).unwrap();
        let system = table.get("system").and_then(|t| t.as_table()).ok();

        Ok(Self {
            ollama_url: ollama
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("http://localhost:11434")
                .to_string(),
            ollama_model: ollama
                .get("model")
                .and_then(|v| v.as_str())
                .unwrap_or("qwen2.5:1.5b")
                .to_string(),
            ollama_timeout: ollama
                .get("timeout")
                .and_then(|v| v.as_integer())
                .unwrap_or(300) as u64,
            memory_limit: table
                .get("memory")
                .and_then(|t| t.as_table())
                .and_then(|m| m.get("limit").and_then(|v| v.as_integer()))
                .unwrap_or(20000) as usize,
            enable_rag: table
                .get("rag")
                .and_then(|t| t.as_table())
                .and_then(|r| r.get("enable").and_then(|v| v.as_bool()))
                .unwrap_or(true),
            enable_auto_save: system
                .and_then(|s| s.get("auto_save").and_then(|v| v.as_bool()))
                .unwrap_or(true),
        })
    }
}

const DEFAULT_CONFIG: &str = r#"
[ollama]
url = "http://localhost:11434"
model = "qwen2.5:1.5b"
timeout = 300

[memory]
limit = 20000
auto_save = true

[rag]
enable = true
max_docs = 5
relevance_threshold = 0.3

[system]
base_path = "./agente"
max_logs = 100
save_interval = 300
auto_save = true
"#;

/// Estructura principal del agente
struct Agent {
    config: Config,
    ollama: OllamaClient,
    memory: MemoryManager,
    skills: SkillsManager,
    rag: RAGEngine,
    fs: FileSystemManager,
    commands: CommandProcessor,
    ui: TerminalUI,
}

impl Agent {
    /// Crear nueva instancia del agente
    async fn new() -> Result<Self> {
        let config = Config::load().await?;
        
        let ollama = OllamaClient::new(config.ollama_url.clone(), config.ollama_timeout);
        let memory = MemoryManager::new("./agente");
        let skills = SkillsManager::new("./agente");
        let rag = RAGEngine::new("./agente");
        let fs = FileSystemManager::new("./agente");
        let commands = CommandProcessor::new("./agente");
        let ui = TerminalUI::new();

        Ok(Self {
            config,
            ollama,
            memory,
            skills,
            rag,
            fs,
            commands,
            ui,
        })
    }

    /// Inicializar estructura del agente
    async fn init(&self) -> Result<()> {
        self.ui.show_header("🤖 Agente Ollama Local - Inicialización");
        
        self.ui.show_loading("Inicializando memoria");
        self.memory.init().await?;
        
        self.ui.show_loading("Inicializando skills");
        self.skills.init().await?;
        
        self.ui.show_loading("Inicializando RAG");
        self.rag.init().await?;
        
        self.ui.show_success("Sistema listo para usar");
        Ok(())
    }

    /// Verificar disponibilidad de Ollama
    async fn check_ollama(&self) -> Result<()> {
        self.ui.show_loading("Verificando conexión con Ollama");
        
        match self.ollama.health_check().await {
            Ok(true) => {
                self.ui.show_success("Conexión con Ollama establecida");
                Ok(())
            }
            Ok(false) => Err(anyhow::anyhow!(
                "Ollama no responde. Asegúrate de que Ollama está corriendo: ollama serve"
            )),
            Err(e) => Err(anyhow::anyhow!("Error verificando Ollama: {}", e)),
        }
    }

    /// Mostrar información del agente
    fn show_info(&self) {
        self.ui.show_header("🤖 Agente Autónomo Local para Ollama");
        
        self.ui.show_info_panel(
            "Información del Sistema",
            &format!(
                "Modelo: {}\nURL Ollama: {}\nRAG: {}\nAuto-guardado: {}\n\n\
                 Escribe /ayuda para ver comandos disponibles\n\
                 Escribe 'salir' para terminar",
                self.config.ollama_model,
                self.config.ollama_url,
                if self.config.enable_rag { "✓ Activado" } else { "✗ Desactivado" },
                if self.config.enable_auto_save { "✓ Activado" } else { "✗ Desactivado" }
            ),
        );
    }

    /// Mostrar contexto activo
    async fn show_context(&self) -> Result<()> {
        let memory_size = self.memory.get_size().await.unwrap_or(0);
        let skills = self.skills.list_skills().await.unwrap_or_default();
        let docs = self.rag.list_knowledge("proyectos").await.unwrap_or_default();

        self.ui.show_context_info(
            memory_size as usize,
            skills.len(),
            docs.len(),
        );

        Ok(())
    }

    /// Construir prompt con contexto
    async fn build_context(&self, user_input: &str) -> Result<String> {
        let mut context = String::new();

        // Agregar memoria
        let memory_content = self.memory.read().await.unwrap_or_default();
        if !memory_content.is_empty() {
            context.push_str(&format!("=== MEMORIA ===\n{}\n\n", memory_content));
        }

        // Agregar skills
        let skills_context = self.skills.get_skills_context().await.unwrap_or_default();
        if !skills_context.is_empty() {
            context.push_str(&format!("{}\n\n", skills_context));
        }

        // Agregar RAG si está habilitado
        if self.config.enable_rag {
            let docs = self.rag.retrieve(user_input, 3).await.unwrap_or_default();
            if !docs.is_empty() {
                context.push_str("=== DOCUMENTOS RELEVANTES ===\n");
                for doc in docs {
                    context.push_str(&format!("Fuente: {}\n{}\n\n", doc.path, doc.content));
                }
            }
        }

        // Agregar input del usuario
        context.push_str(&format!("=== USUARIO ===\n{}\n\n=== RESPONDE ===\n", user_input));

        Ok(context)
    }

    /// Procesar entrada del usuario
    async fn process_input(&self, input: &str) -> Result<String> {
        // Auto-guardar información importante automáticamente (sin necesidad de comando)
        if self.config.enable_auto_save {
            if let Ok(Some(save_msg)) = self.commands.auto_save_if_needed(input).await {
                self.ui.show_success(&save_msg.replace("💾 Auto-guardado: ", ""));
            }
        }

        // Detectar intenciones especiales
        if self.commands.detect_memory_intent(input) {
            self.memory.save(input).await?;
            return Ok("💾 Guardado en memoria".to_string());
        }

        if let Some(skill_name) = self.commands.detect_skill_creation(input) {
            self.skills.create_skill(&skill_name, "Skill personalizada").await?;
            return Ok(format!("✓ Skill '{}' creada", skill_name));
        }

        if let Some((category, name)) = self.commands.detect_knowledge_creation(input) {
            self.rag
                .create_knowledge(&category, &name, "Contenido inicial")
                .await?;
            return Ok(format!(
                "✓ Conocimiento '{}/{}' creado",
                category, name
            ));
        }

        // Procesar comandos normales
        let cmd = self.commands.parse(input);
        if cmd.cmd_type != commands::CommandType::Desconocido {
            return self.commands.execute(&cmd).await;
        }

        // Si no es comando, enviar a Ollama
        let context = self.build_context(input).await?;
        let response = self.ollama.generate(&self.config.ollama_model, &context).await?;

        // Guardar en logs
        self.fs.log_conversation(input, &response).await.ok();

        Ok(response)
    }

    /// Loop principal del agente
    async fn run(&self) -> Result<()> {
        self.show_info();

        loop {
            let _ = self.show_context().await;
            
            match self.ui.read_input("Esperando entrada") {
                Ok(input) => {
                    if input.is_empty() {
                        continue;
                    }

                    if input.to_lowercase() == "salir" {
                        self.ui.show_info("👋 ¡Hasta luego!");
                        break;
                    }

                    match self.process_input(&input).await {
                        Ok(response) => {
                            self.ui.show_response(&response);
                        }
                        Err(e) => {
                            self.ui.show_error(&format!("{}", e));
                        }
                    }
                }
                Err(e) => {
                    self.ui.show_error(&format!("Error leyendo entrada: {}", e));
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    let agent = Agent::new().await?;
    agent.init().await?;
    
    // Verificar Ollama
    if let Err(e) = agent.check_ollama().await {
        agent.ui.show_error(&format!("⚠️  Advertencia: {}", e));
        eprintln!("\nPara usar el agente necesitas Ollama. Instrucciones:");
        eprintln!("  Linux: https://ollama.ai");
        eprintln!("  Termux Android:");
        eprintln!("    1. pkg install ollama");
        eprintln!("    2. ollama serve");
        eprintln!("    3. En otra terminal: ollama pull qwen2.5:1.5b");
        std::process::exit(1);
    }

    agent.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let agent = Agent::new().await;
        assert!(agent.is_ok());
    }
}
