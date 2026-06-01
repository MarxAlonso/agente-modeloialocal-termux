/// Módulo para comunicación con Ollama
/// Maneja las peticiones HTTP a la API de Ollama
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub response: String,
    pub model: String,
    pub created_at: String,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Vec<u32>>,
}

/// Cliente para comunicarse con Ollama
pub struct OllamaClient {
    client: Client,
    url: String,
    timeout: Duration,
}

impl OllamaClient {
    /// Crear nueva instancia de OllamaClient
    pub fn new(url: String, timeout_secs: u64) -> Self {
        let client = Client::new();
        Self {
            client,
            url,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Generar respuesta desde Ollama
    pub async fn generate(&self, model: &str, prompt: &str) -> Result<String> {
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            temperature: Some(0.7),
            top_k: Some(40),
            top_p: Some(0.9),
        };

        let url = format!("{}/api/generate", self.url);
        
        let response = self
            .client
            .post(&url)
            .timeout(self.timeout)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Error conectando con Ollama: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow!("Error HTTP {}: {}", status, response.text().await?));
        }

        let body = response
            .text()
            .await
            .map_err(|e| anyhow!("Error leyendo respuesta: {}", e))?;

        // Procesar líneas de respuesta (streaming)
        let mut full_response = String::new();
        for line in body.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            match serde_json::from_str::<OllamaResponse>(line) {
                Ok(chunk) => {
                    full_response.push_str(&chunk.response);
                }
                Err(_) => {
                    // Log silencioso de errores de parsing
                    continue;
                }
            }
        }

        if full_response.is_empty() {
            return Err(anyhow!("Respuesta vacía de Ollama"));
        }

        Ok(full_response)
    }

    /// Verificar disponibilidad de Ollama
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.url);
        
        match self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// Listar modelos disponibles
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.url);
        
        #[derive(Deserialize)]
        struct ModelsList {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| anyhow!("Error obteniendo lista de modelos: {}", e))?;

        let body: ModelsList = response
            .json()
            .await
            .map_err(|e| anyhow!("Error parseando modelos: {}", e))?;

        Ok(body.models.into_iter().map(|m| m.name).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_client_creation() {
        let client = OllamaClient::new("http://localhost:11434".to_string(), 300);
        assert_eq!(client.url, "http://localhost:11434");
        assert_eq!(client.timeout, Duration::from_secs(300));
    }
}
