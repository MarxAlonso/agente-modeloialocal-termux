/// Módulo de Memoria Persistente
/// Gestiona el almacenamiento y recuperación de memoria
use anyhow::Result;
use chrono::Local;
use std::path::PathBuf;
use tokio::fs;

/// Estructura para gestionar memoria
pub struct MemoryManager {
    memory_dir: PathBuf,
}

impl MemoryManager {
    /// Crear nuevo gestor de memoria
    pub fn new(base_path: &str) -> Self {
        let memory_dir = PathBuf::from(format!("{}/memoria", base_path));
        Self { memory_dir }
    }

    /// Inicializar estructura de memoria
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.memory_dir).await?;
        
        // Crear archivos base si no existen
        self.ensure_file("memoria.md", "# Memoria\n\n").await?;
        self.ensure_file("usuario.md", "# Usuario\n\n").await?;
        self.ensure_file("preferencias.md", "# Preferencias\n\n").await?;
        
        Ok(())
    }

    /// Guardar información en memoria
    pub async fn save(&self, content: &str) -> Result<()> {
        let path = self.memory_dir.join("memoria.md");
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let entry = format!("\n## {}\n\n{}\n", timestamp, content);
        
        let mut current = fs::read_to_string(&path).await.unwrap_or_default();
        current.push_str(&entry);
        
        fs::write(&path, current).await?;
        Ok(())
    }

    /// Leer memoria completa
    pub async fn read(&self) -> Result<String> {
        let path = self.memory_dir.join("memoria.md");
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    #[allow(dead_code)]
    /// Guardar preferencia del usuario
    pub async fn save_preference(&self, key: &str, value: &str) -> Result<()> {
        let path = self.memory_dir.join("preferencias.md");
        let entry = format!("{}: {}\n", key, value);
        
        let mut current = fs::read_to_string(&path).await.unwrap_or_default();
        current.push_str(&entry);
        
        fs::write(&path, current).await?;
        Ok(())
    }

    #[allow(dead_code)]
    /// Leer preferencias
    pub async fn read_preferences(&self) -> Result<String> {
        let path = self.memory_dir.join("preferencias.md");
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    #[allow(dead_code)]
    /// Guardar información del usuario
    pub async fn save_user_info(&self, info: &str) -> Result<()> {
        let path = self.memory_dir.join("usuario.md");
        let entry = format!("\n## {}\n\n{}\n", Local::now().format("%Y-%m-%d"), info);
        
        let mut current = fs::read_to_string(&path).await.unwrap_or_default();
        current.push_str(&entry);
        
        fs::write(&path, current).await?;
        Ok(())
    }

    #[allow(dead_code)]
    /// Leer información del usuario
    pub async fn read_user_info(&self) -> Result<String> {
        let path = self.memory_dir.join("usuario.md");
        Ok(fs::read_to_string(&path).await.unwrap_or_default())
    }

    #[allow(dead_code)]
    /// Verificar si memoria existe
    pub async fn exists(&self) -> bool {
        self.memory_dir.exists()
    }

    /// Helper: asegurar que un archivo existe
    async fn ensure_file(&self, filename: &str, default_content: &str) -> Result<()> {
        let path = self.memory_dir.join(filename);
        if !path.exists() {
            fs::write(&path, default_content).await?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    /// Obtener tamaño total de memoria
    pub async fn get_size(&self) -> Result<u64> {
        let mut total = 0u64;
        let mut dir = fs::read_dir(&self.memory_dir).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            if let Ok(metadata) = entry.metadata().await {
                total += metadata.len();
            }
        }
        
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new("./agente");
        assert!(manager.memory_dir.to_string_lossy().contains("memoria"));
    }
}
