/// Módulo de Skills
/// Gestiona la creación, actualización y ejecución de skills
/// Soporta skills en Markdown (conocimiento) y TypeScript (ejecutables via Deno)
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Gestor de Skills del agente
pub struct SkillsManager {
    skills_dir: PathBuf,
    source_dir: PathBuf,
}

impl SkillsManager {
    /// Crear nuevo gestor de skills
    pub fn new(base_path: &str) -> Self {
        let skills_dir = PathBuf::from(format!("{}/skills", base_path));
        let source_dir = PathBuf::from("skills");
        Self { skills_dir, source_dir }
    }

    /// Inicializar directorio de skills
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.skills_dir).await?;

        // Copiar skills TypeScript del directorio fuente al runtime
        self.copy_ts_skills().await?;

        // Crear skills por defecto (solo si no existen)
        self.create_skill_if_not_exists("ingeniero-software", "Ingeniería de Software").await?;
        self.create_skill_if_not_exists("psicologo-apoyo", "Apoyo Emocional").await?;
        self.create_skill_if_not_exists("estadistica-economica", "Estadística y Economía").await?;
        self.create_skill_if_not_exists("vida", "Habilidades para la Vida").await?;

        Ok(())
    }

    /// Copiar skills .ts del directorio fuente al runtime
    async fn copy_ts_skills(&self) -> Result<()> {
        if !self.source_dir.exists() {
            return Ok(());
        }
        let mut dir = fs::read_dir(&self.source_dir).await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ts") {
                let dest = self.skills_dir.join(entry.file_name());
                if !dest.exists() {
                    fs::copy(&path, &dest).await?;
                }
            }
        }
        Ok(())
    }

    /// Crear nueva skill solo si no existe
    async fn create_skill_if_not_exists(&self, name: &str, description: &str) -> Result<()> {
        let path = self.skills_dir.join(format!("{}.md", name));
        if path.exists() {
            return Ok(());
        }
        let content = format!(
            "# {}\n\n## Descripción\n\n{}\n\n## Conocimientos\n\n",
            name.to_uppercase(),
            description
        );
        fs::write(&path, content).await?;
        Ok(())
    }

    /// Crear nueva skill
    pub async fn create_skill(&self, name: &str, description: &str) -> Result<()> {
        let path = self.skills_dir.join(format!("{}.md", name));
        
        if path.exists() {
            return Ok(());
        }

        let content = format!(
            "# {}\n\n## Descripción\n\n{}\n\n## Conocimientos\n\n",
            name.to_uppercase(),
            description
        );

        fs::write(&path, content).await?;
        Ok(())
    }

    #[allow(dead_code)]
    /// Agregar contenido a un skill
    pub async fn add_to_skill(&self, skill_name: &str, content: &str) -> Result<()> {
        let path = self.skills_dir.join(format!("{}.md", skill_name));

        if !path.exists() {
            self.create_skill(skill_name, "Skill personalizada").await?;
        }

        let mut current = fs::read_to_string(&path).await.unwrap_or_default();
        current.push_str(&format!("\n- {}\n", content));

        fs::write(&path, current).await?;
        Ok(())
    }

    /// Listar todos los skills (md y ts)
    pub async fn list_skills(&self) -> Result<Vec<String>> {
        let mut skills = Vec::new();
        
        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        let mut dir = fs::read_dir(&self.skills_dir).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".md") || name.ends_with(".ts") {
                    let skill_name = name
                        .trim_end_matches(".md")
                        .trim_end_matches(".ts")
                        .to_string();
                    if !skills.contains(&skill_name) {
                        skills.push(skill_name);
                    }
                }
            }
        }

        skills.sort();
        Ok(skills)
    }

    /// Listar solo skills ejecutables (.ts)
    pub async fn list_ts_skills(&self) -> Result<Vec<String>> {
        let mut skills = Vec::new();
        
        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        let mut dir = fs::read_dir(&self.skills_dir).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".ts") {
                    let skill_name = name.trim_end_matches(".ts").to_string();
                    skills.push(skill_name);
                }
            }
        }

        skills.sort();
        Ok(skills)
    }

    /// Leer skill completo (md)
    pub async fn read_skill(&self, skill_name: &str) -> Result<String> {
        let path = self.skills_dir.join(format!("{}.md", skill_name));
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    /// Leer fuente de un skill TypeScript
    pub async fn read_ts_source(&self, skill_name: &str) -> Result<String> {
        let path = self.skills_dir.join(format!("{}.ts", skill_name));
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    /// Verificar si existe un skill TypeScript ejecutable
    pub async fn ts_skill_exists(&self, skill_name: &str) -> bool {
        let path = self.skills_dir.join(format!("{}.ts", skill_name));
        path.exists()
    }

    /// Obtener todos los skills como contexto (solo md)
    pub async fn get_skills_context(&self) -> Result<String> {
        let skills = self.list_skills().await?;
        let mut context = String::from("## SKILLS DISPONIBLES\n\n");

        for skill in skills {
            let content = self.read_skill(&skill).await?;
            if !content.is_empty() {
                context.push_str(&format!("### {}\n{}\n\n", skill, content));
            }
        }

        // Agregar lista de skills ejecutables
        let ts_skills = self.list_ts_skills().await?;
        if !ts_skills.is_empty() {
            context.push_str("### SKILLS EJECUTABLES (TypeScript)\n\n");
            for s in &ts_skills {
                let source = self.read_ts_source(s).await.unwrap_or_default();
                let desc = source.lines()
                    .find(|l| l.starts_with("// desc:"))
                    .map(|l| l.trim_start_matches("// desc:").trim())
                    .unwrap_or("Skill ejecutable");
                context.push_str(&format!("- **{}**: {}\n", s, desc));
            }
            context.push('\n');
        }

        Ok(context)
    }

    /// Ejecutar un skill TypeScript via Deno
    /// Recibe el nombre del skill y un string de input
    /// Retorna stdout del script
    pub async fn run_ts_skill(&self, skill_name: &str, input: &str) -> Result<String> {
        let path = self.skills_dir.join(format!("{}.ts", skill_name));
        
        if !path.exists() {
            return Err(anyhow!("Skill '{}' no encontrado. Creado como .ts en skills/", skill_name));
        }

        // Verificar que Deno esté instalado
        let deno_check = Command::new("deno")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;

        match deno_check {
            Err(_) => {
                return Err(anyhow!(
                    "Deno no está instalado. Instálalo con:\n  curl -fsSL https://deno.land/install.sh | sh\n  (o en Termux: pkg install deno)"
                ));
            }
            Ok(s) if !s.success() => {
                return Err(anyhow!(
                    "Deno no está disponible. Instálalo con:\n  curl -fsSL https://deno.land/install.sh | sh\n  (o en Termux: pkg install deno)"
                ));
            }
            _ => {}
        }

        let mut child = Command::new("deno")
            .arg("run")
            .arg("--allow-read")   // acceso a leer archivos
            .arg("--allow-write")  // acceso a escribir archivos
            .arg("--allow-net")    // acceso a red si necesita
            .arg("--allow-env")    // variables de entorno
            .arg("--quiet")        // sin warnings de Deno
            .arg(&path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // Pasar input por stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes()).await?;
            stdin.flush().await?;
            // Cerrar stdin para que el script sepa que terminó
            drop(stdin);
        }

        let output = child.wait_with_output().await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Error ejecutando skill '{}':\n{}", skill_name, stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skills_manager_creation() {
        let manager = SkillsManager::new("./agente");
        assert!(manager.skills_dir.to_string_lossy().contains("skills"));
    }
}
