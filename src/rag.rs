/// Módulo RAG (Retrieval-Augmented Generation)
/// Implementa recuperación contextual local de conocimiento
use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio::fs;

/// Documento recuperado
#[derive(Debug, Clone)]
pub struct Document {
    pub path: String,
    pub content: String,
    pub relevance: f32,
}

/// Motor RAG para recuperación local
pub struct RAGEngine {
    base_path: PathBuf,
    knowledge_dir: PathBuf,
}

impl RAGEngine {
    /// Crear nuevo motor RAG
    pub fn new(base_path: &str) -> Self {
        let base = PathBuf::from(base_path);
        let knowledge_dir = base.join("conocimiento");
        
        Self {
            base_path: base,
            knowledge_dir,
        }
    }

    /// Inicializar estructura de conocimiento
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.knowledge_dir).await?;
        fs::create_dir_all(self.knowledge_dir.join("proyectos")).await?;
        fs::create_dir_all(self.knowledge_dir.join("personas")).await?;
        fs::create_dir_all(self.knowledge_dir.join("ideas")).await?;
        fs::create_dir_all(self.knowledge_dir.join("investigaciones")).await?;
        Ok(())
    }

    /// Recuperar documentos relevantes basado en palabras clave
    pub async fn retrieve(&self, query: &str, max_docs: usize) -> Result<Vec<Document>> {
        let keywords = self.extract_keywords(query);
        let mut documents = Vec::new();

        // Buscar en todos los archivos de conocimiento
        self.search_directory(&self.knowledge_dir, &keywords, &mut documents)
            .await?;

        // Calcular relevancia y ordenar
        for doc in &mut documents {
            doc.relevance = self.calculate_relevance(query, &doc.content);
        }

        documents.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
        documents.truncate(max_docs);

        Ok(documents)
    }

    /// Buscar recursivamente en directorio
    async fn search_directory(
        &self,
        dir: &PathBuf,
        keywords: &[String],
        documents: &mut Vec<Document>,
    ) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_dir() {
                self.search_directory(&path, keywords, documents).await?;
            } else if path.extension().map_or(false, |ext| ext == "md") {
                if let Ok(content) = fs::read_to_string(&path).await {
                    // Verificar si contiene palabras clave
                    if keywords.iter().any(|kw| content.to_lowercase().contains(&kw.to_lowercase())) {
                        documents.push(Document {
                            path: path.to_string_lossy().to_string(),
                            content,
                            relevance: 0.0,
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Extraer palabras clave de la consulta
    fn extract_keywords(&self, query: &str) -> Vec<String> {
        query
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.to_lowercase())
            .collect()
    }

    /// Calcular relevancia de un documento
    fn calculate_relevance(&self, query: &str, content: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let content_lower = content.to_lowercase();

        let mut score = 0.0f32;
        let keywords = self.extract_keywords(query);

        // Contar coincidencias de palabras clave
        for keyword in keywords {
            let count = content_lower.matches(&keyword).count() as f32;
            score += count * 0.5;
        }

        // Boost si aparece en título
        if content_lower.lines().next().map_or(false, |line| 
            keywords.iter().any(|kw| line.contains(kw))
        ) {
            score *= 1.5;
        }

        // Normalizar entre 0 y 1
        (score / 100.0).min(1.0)
    }

    /// Detectar enlaces bidireccionales [[referencia]]
    pub fn extract_links(&self, content: &str) -> Vec<String> {
        let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        re.captures_iter(content)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    /// Crear nuevo conocimiento
    pub async fn create_knowledge(
        &self,
        category: &str,
        name: &str,
        content: &str,
    ) -> Result<()> {
        let dir = self.knowledge_dir.join(category);
        fs::create_dir_all(&dir).await?;

        let path = dir.join(format!("{}.md", name));
        let full_content = format!(
            "# {}\n\n## Creado\n\n{}\n\n## Contenido\n\n{}\n",
            name,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            content
        );

        fs::write(&path, full_content).await?;
        Ok(())
    }

    /// Listar conocimientos
    pub async fn list_knowledge(&self, category: &str) -> Result<Vec<String>> {
        let dir = self.knowledge_dir.join(category);
        let mut items = Vec::new();

        if !dir.exists() {
            return Ok(items);
        }

        let mut entries = fs::read_dir(&dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".md") {
                    items.push(name.trim_end_matches(".md").to_string());
                }
            }
        }

        items.sort();
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let engine = RAGEngine::new("./agente");
        let keywords = engine.extract_keywords("crear un proyecto en Rust");
        assert!(keywords.contains(&"crear".to_string()));
        assert!(keywords.contains(&"proyecto".to_string()));
        assert!(keywords.contains(&"rust".to_string()));
    }

    #[test]
    fn test_extract_links() {
        let engine = RAGEngine::new("./agente");
        let content = "Ver [[rust]] y [[termux]]";
        let links = engine.extract_links(content);
        assert_eq!(links.len(), 2);
        assert!(links.contains(&"rust".to_string()));
    }

    #[test]
    fn test_calculate_relevance() {
        let engine = RAGEngine::new("./agente");
        let score = engine.calculate_relevance("rust", "Rust es un lenguaje");
        assert!(score > 0.0);
    }
}
