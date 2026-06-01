/// Módulo de Skills
/// Gestiona la creación y actualización de skills
use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

/// Gestor de Skills del agente
pub struct SkillsManager {
    skills_dir: PathBuf,
}

impl SkillsManager {
    /// Crear nuevo gestor de skills
    pub fn new(base_path: &str) -> Self {
        let skills_dir = PathBuf::from(format!("{}/skills", base_path));
        Self { skills_dir }
    }

    /// Inicializar directorio de skills
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.skills_dir).await?;
        
        // Crear skills por defecto
        self.create_skill("rust", "Programación en Rust").await?;
        self.create_skill("linux", "Administración de Linux").await?;
        self.create_skill("termux", "Desarrollo en Termux Android").await?;
        
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

    /// Listar todos los skills
    pub async fn list_skills(&self) -> Result<Vec<String>> {
        let mut skills = Vec::new();
        
        if !self.skills_dir.exists() {
            return Ok(skills);
        }

        let mut dir = fs::read_dir(&self.skills_dir).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".md") {
                    let skill_name = name.trim_end_matches(".md").to_string();
                    skills.push(skill_name);
                }
            }
        }

        skills.sort();
        Ok(skills)
    }

    /// Leer skill completo
    pub async fn read_skill(&self, skill_name: &str) -> Result<String> {
        let path = self.skills_dir.join(format!("{}.md", skill_name));
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    /// Verificar si skill existe
    pub async fn skill_exists(&self, skill_name: &str) -> bool {
        let path = self.skills_dir.join(format!("{}.md", skill_name));
        path.exists()
    }

    /// Obtener todos los skills como contexto
    pub async fn get_skills_context(&self) -> Result<String> {
        let skills = self.list_skills().await?;
        let mut context = String::from("## SKILLS DISPONIBLES\n\n");

        for skill in skills {
            let content = self.read_skill(&skill).await?;
            context.push_str(&format!("### {}\n{}\n\n", skill, content));
        }

        Ok(context)
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
