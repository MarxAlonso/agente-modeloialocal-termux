# Proyecto: Agente Autónomo Local para Ollama en Rust

**Estado**: ✅ Proyecto completado

## ¿Qué contiene?

Este es un proyecto Rust completo listo para compilar con `cargo build --release`.

### Módulos implementados:

- ✅ **ollama.rs** - Cliente HTTP para comunicación con Ollama
- ✅ **memory.rs** - Sistema de memoria persistente
- ✅ **skills.rs** - Gestor de skills especializados  
- ✅ **rag.rs** - Motor RAG para recuperación contextual
- ✅ **commands.rs** - Procesador de comandos
- ✅ **filesystem.rs** - Gestor del sistema de archivos
- ✅ **main.rs** - Aplicación principal con REPL interactivo

### Características completadas:

- ✅ Comunicación HTTP con Ollama (async)
- ✅ Sistema de memoria con persistencia
- ✅ Creación y gestión de skills
- ✅ Motor RAG con búsqueda local
- ✅ Detección de intenciones especiales
- ✅ Comandos del sistema (/, memoria, skills, etc.)
- ✅ Logs automáticos de conversaciones
- ✅ Exportación de backups
- ✅ Manejo de errores robusto
- ✅ Código comentado y documentado
- ✅ Estructura modular y extensible

## 🚀 Para compilar en Termux:

```bash
# Instalar dependencias (una sola vez)
bash install.sh

# Compilar y ejecutar
bash run.sh
```

## 📝 Configuración:

Editar `config/config.toml` para cambiar:
- URL de Ollama
- Modelo a usar
- Límites de memoria
- Parámetros RAG

## 💬 Uso:

```
📌 > Hola
🤖 [Respuesta de Ollama]

📌 > guarda que me llamo Carlos
💾 Guardado en memoria

📌 > /memoria
📝 [Muestra memoria]

📌 > /ayuda
🆘 [Muestra comandos]
```

## 📦 Dependencias Rust:

- tokio (async runtime)
- reqwest (HTTP client)
- serde/serde_json (serialización)
- chrono (timestamps)
- walkdir (recorrido de directorios)
- regex (expresiones regulares)
- toml (parsing de config)
- anyhow (manejo de errores)
- colored (output coloreado)
- dirs (paths del sistema)

## 🎯 Todo lo especificado en construccion.md está implementado:

✅ Arquitectura completa (Usuario → Agente → Ollama)
✅ Comunicación HTTP con Ollama  
✅ Estructura de carpetas (memoria/, skills/, conocimiento/, logs/)
✅ Sistema de Memoria (detección de frases especiales)
✅ Sistema de Skills (crear y actualizar)
✅ Sistema de Conocimiento (crear y listar)
✅ Enlaces Bidireccionales (detectar [[referencias]])
✅ Motor RAG Local (recuperación con palabras clave)
✅ Constructor de Prompt (inyectar contexto)
✅ Sistema de Comandos (/, memoria, skills, etc.)
✅ Logs de Conversaciones (guardar en archivo)
✅ Configuración (config.toml)
✅ Manejo de errores robusto
✅ Sistema extensible
✅ Sin dependencias de nube
✅ Compatible Termux/Linux
✅ Persistencia automática

## 📄 Documentación:

- **README.md** - Guía completa de uso
- **Código comentado** - Cada función tiene documentación
- **Tests** - Incluye tests unitarios
- **Scripts** - install.sh y run.sh para facilitar uso

¡Listo para usar en Termux! 🎉
