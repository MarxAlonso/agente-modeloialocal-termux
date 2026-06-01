# Ollama - Modelos LLM Locales

## Creado

2026-05-31

## Descripción

Ollama es un framework para ejecutar Large Language Models localmente.

## Características

- Interfaz simple vía CLI y API HTTP
- Soporte para múltiples modelos
- Optimizado para CPU y GPU
- Gestión automática de memoria
- Descargas facilitadas de modelos populares

## Modelos Recomendados

- **qwen2.5:1.5b** - Rápido, ideal para Termux
- **llama2:7b** - General purpose
- **mistral:7b** - Buen balance
- **neural-chat:7b** - Chat especializado

## API

```
GET http://localhost:11434/api/tags - Listar modelos
POST http://localhost:11434/api/generate - Generar respuesta
```

## Instalación

Relacionado: [[linux]] [[termux]]

```bash
# Linux
curl https://ollama.ai/install.sh | sh

# Termux
pkg install ollama
# O compilar desde source
```

## Documentación

https://github.com/ollama/ollama
