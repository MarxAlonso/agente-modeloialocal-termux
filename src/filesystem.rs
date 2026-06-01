/// Módulo de Sistema de Archivos
/// Gestiona lectura y escritura de archivos
use anyhow::Result;
use chrono::Local;
use std::path::PathBuf;
use tokio::fs;

/// Gestor del sistema de archivos
pub struct FileSystemManager {
    base_path: PathBuf,
}

impl FileSystemManager {
    /// Crear nuevo gestor de archivos
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }

    /// Leer archivo
    pub async fn read_file(&self, relative_path: &str) -> Result<String> {
        let path = self.base_path.join(relative_path);
        Ok(fs::read_to_string(&path).await?)
    }

    /// Escribir archivo
    pub async fn write_file(&self, relative_path: &str, content: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        
        // Crear directorios si no existen
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&path, content).await?;
        Ok(())
    }

    /// Crear directorio
    pub async fn create_dir(&self, relative_path: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        fs::create_dir_all(&path).await?;
        Ok(())
    }

    /// Listar contenido de directorio
    pub async fn list_dir(&self, relative_path: &str) -> Result<Vec<String>> {
        let path = self.base_path.join(relative_path);
        let mut items = Vec::new();

        if !path.exists() {
            return Ok(items);
        }

        let mut dir = fs::read_dir(&path).await?;

        while let Some(entry) = dir.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                items.push(name.to_string());
            }
        }

        items.sort();
        Ok(items)
    }

    /// Guardar logs de conversación
    pub async fn log_conversation(&self, user_msg: &str, assistant_msg: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_entry = format!(
            "\n## {}\n\n**Usuario:**\n{}\n\n**Asistente:**\n{}\n",
            timestamp, user_msg, assistant_msg
        );

        let log_path = "logs/conversaciones.md";
        let current = fs::read_to_string(self.base_path.join(log_path))
            .await
            .unwrap_or_else(|_| "# Conversaciones\n".to_string());

        let updated = format!("{}{}", current, log_entry);
        self.write_file(log_path, &updated).await?;

        Ok(())
    }

    /// Exportar backup
    pub async fn export_backup(&self, backup_name: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let backup_dir = format!("backups/backup_{}", timestamp);

        self.create_dir(&backup_dir).await?;

        // Copiar memoria
        if let Ok(memory) = self.read_file("memoria/memoria.md").await {
            self.write_file(&format!("{}/memoria.md", backup_dir), &memory)
                .await?;
        }

        // Copiar skills
        let skills = self.list_dir("skills").await.unwrap_or_default();
        for skill in skills {
            if let Ok(content) = self.read_file(&format!("skills/{}", skill)).await {
                self.write_file(&format!("{}/skills_{}", backup_dir, skill), &content)
                    .await?;
            }
        }

        Ok(())
    }

    /// Obtener estadísticas del sistema
    pub async fn get_stats(&self) -> Result<String> {
        let memory_size = self.calculate_dir_size("memoria").await.unwrap_or(0);
        let knowledge_size = self.calculate_dir_size("conocimiento").await.unwrap_or(0);
        let logs_size = self.calculate_dir_size("logs").await.unwrap_or(0);

        Ok(format!(
            "📊 Estadísticas del Agente\n\
             ├─ Memoria: {} KB\n\
             ├─ Conocimiento: {} KB\n\
             ├─ Logs: {} KB\n\
             └─ Total: {} KB",
            memory_size / 1024,
            knowledge_size / 1024,
            logs_size / 1024,
            (memory_size + knowledge_size + logs_size) / 1024
        ))
    }

    /// Calcular tamaño de directorio
    async fn calculate_dir_size(&self, relative_path: &str) -> Result<u64> {
        let path = self.base_path.join(relative_path);
        let mut total = 0u64;

        if !path.exists() {
            return Ok(0);
        }

        self.sum_dir_size(&path, &mut total).await?;
        Ok(total)
    }

    /// Helper recursivo para calcular tamaño
    async fn sum_dir_size(&self, path: &PathBuf, total: &mut u64) -> Result<()> {
        let mut dir = fs::read_dir(path).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path).await {
                    *total += metadata.len();
                }
            } else if path.is_dir() {
                self.sum_dir_size(&path, total).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_manager_creation() {
        let manager = FileSystemManager::new("./agente");
        assert_eq!(manager.base_path, PathBuf::from("./agente"));
    }
}
