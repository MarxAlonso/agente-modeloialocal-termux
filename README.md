# 🤖 Agente Autónomo Local para Ollama en Rust

Agente inteligente que funciona localmente en **Rust**, diseñado para **Termux (Android)** y **Linux**, con capacidades de memoria persistente, sistema de skills, recuperación contextual RAG y evolución autónoma de conocimiento.

## 🎯 Características

- **Memoria Persistente**: Almacena y recupera información automáticamente
- **Sistema de Skills**: Crea y gestiona skills especializados
- **Motor RAG**: Recuperación contextual de documentos locales
- **Sin Dependencias Nube**: Todo funciona localmente
- **Compatible Termux**: Compilable y ejecutable en Android
- **Interfaz Interactiva**: REPL con comandos especiales

## 📋 Requisitos

### Linux
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Instalar Ollama
curl https://ollama.ai/install.sh | sh
```

### Termux (Android)
```bash
# Actualizar paquetes
pkg update && pkg upgrade

# Instalar Rust
pkg install rust

# Instalar Ollama (si está disponible) o alternativa
pkg install ollama
# O compilar desde fuente
```

## 🚀 Instalación y Compilación

### 1. Obtener el código
```bash
cd ~/proyectos
git clone <tu-repositorio> agente-ollama
cd agente-ollama
```

### 2. Compilar para Linux
```bash
# Compilación de desarrollo
cargo build

# Compilación optimizada para Termux/Android
cargo build --release --target aarch64-unknown-linux-android
```

### 3. Compilación para Termux
```bash
# En Termux, simplemente:
cargo build --release
```

El binario compilado estará en:
- Linux/Termux: `target/release/agente`

## ⚙️ Configuración

Editar `config/config.toml`:

```toml
[ollama]
url = "http://localhost:11434"    # URL de Ollama
model = "qwen2.5:1.5b"             # Modelo a usar
timeout = 300                      # Timeout en segundos

[memory]
limit = 20000                      # Límite de caracteres
auto_save = true                   # Auto-guardar cambios

[rag]
enable = true                      # Habilitar RAG
max_docs = 5                        # Máx. documentos a recuperar
relevance_threshold = 0.3           # Umbral de relevancia
```

## 🏃 Ejecución

### Paso 1: Iniciar Ollama
```bash
# En una terminal
ollama serve

# Nota: Ollama estará en http://localhost:11434
```

### Paso 2: Descargar modelo
```bash
# En otra terminal
ollama pull qwen2.5:1.5b
```

### Paso 3: Ejecutar agente
```bash
./target/release/agente

# O en desarrollo:
cargo run
```

## 💬 Uso

El agente proporciona una interfaz interactiva:

```
📌 > Hola, ¿cuál es tu nombre?
🤖 Mi nombre es el Agente Autónomo para Ollama...

📌 > guarda que me llamo Carlos en memoria
💾 Guardado en memoria

📌 > /memoria
📝 Memoria:

## 2026-05-31 14:30:45

guarda que me llamo Carlos en memoria

📌 > /ayuda
🆘 Comandos disponibles:
...
```

## 📝 Comandos

| Comando | Descripción |
|---------|-------------|
| `/memoria` | Mostrar contenido de memoria |
| `/skills` | Listar skills disponibles |
| `/conocimiento` | Listar conocimientos |
| `/buscar TEXTO` | Buscar en documentos |
| `/stats` | Mostrar estadísticas del sistema |
| `/exportar` | Crear backup automático |
| `/ayuda` | Mostrar ayuda |

## 🧠 Intenciones Especiales

El agente reconoce automáticamente:

- **Guardar en memoria**: "guarda esto", "recuerda", "memoriza"
- **Crear skills**: "crea una skill llamada NOMBRE"
- **Crear conocimiento**: "crea conocimiento CATEGORIA NOMBRE"

## 📁 Estructura de Carpetas

```
agente/
├── memoria/              # Archivos de memoria
│   ├── memoria.md       # Memoria principal
│   ├── usuario.md       # Información del usuario
│   └── preferencias.md  # Preferencias
├── skills/              # Skills especializados
│   ├── rust.md
│   ├── linux.md
│   └── termux.md
├── conocimiento/        # Base de conocimiento
│   ├── proyectos/
│   ├── personas/
│   ├── ideas/
│   └── investigaciones/
├── logs/                # Registros de conversaciones
├── config/              # Configuración
└── backups/             # Backups automáticos
```

## 🔗 Enlaces Bidireccionales

Los documentos pueden referenciar otros usando:

```markdown
# Mi Proyecto

Relacionado: [[rust]] [[linux]] [[termux]]

El proyecto utiliza tecnologías de [[base-datos]] y [[api-rest]].
```

## 🧪 Testing

```bash
# Ejecutar tests
cargo test

# Tests con salida verbose
cargo test -- --nocapture

# Test específico
cargo test test_name
```

## 📊 Motor RAG

El Motor de Recuperación Aumentada por Generación:

1. **Extrae palabras clave** de la consulta
2. **Busca en documentos** locales (Markdown)
3. **Calcula relevancia** basada en frecuencia y contexto
4. **Inyecta contexto** en el prompt para Ollama

Todo sin usar bases de datos externas.

## 🐛 Solución de Problemas

### "Ollama no responde"
```bash
# Verificar que Ollama está corriendo
curl http://localhost:11434/api/tags

# Iniciar Ollama si no está activo
ollama serve
```

### Error de compilación en Termux
```bash
# Instalar herramientas de compilación
pkg install clang llvm

# Compilar
cargo build --release
```

### Memoria llena
```bash
# Exportar backup
📌 > /exportar

# Limpiar logs
rm logs/conversaciones.md
```

## 📚 Documentación de Módulos

### `ollama.rs`
Cliente HTTP para comunicación con Ollama. Maneja requests/responses, health checks y listado de modelos.

### `memory.rs`
Gestor de memoria persistente. Almacena información del usuario, preferencias e historial.

### `skills.rs`
Sistema de skills. Permite crear, listar y actualizar conocimientos especializados.

### `rag.rs`
Motor RAG. Recupera documentos relevantes basados en búsqueda semántica y palabras clave.

### `commands.rs`
Procesador de comandos. Parsea y ejecuta comandos especiales (/, intenciones).

### `filesystem.rs`
Gestor del sistema de archivos. Lee/escribe archivos y gestiona estructura del proyecto.

## 🤝 Contribuir

Si encuentras bugs o tienes sugerencias:

1. Abre un issue
2. Haz un fork
3. Crea una rama (`git checkout -b feature/AmazingFeature`)
4. Commit cambios (`git commit -m 'Add AmazingFeature'`)
5. Push a la rama (`git push origin feature/AmazingFeature`)
6. Abre un Pull Request

## 📄 Licencia

Este proyecto está bajo licencia MIT. Ver archivo LICENSE para más detalles.

## 🎓 Aprendizaje

Este proyecto es una excelente introducción a:

- **Rust moderno**: Async/await, manejo de errores
- **APIs REST**: Comunicación HTTP asíncrona
- **Procesamiento de archivos**: Lectura/escritura en Markdown
- **Sistemas de IA locales**: Integración con modelos LLM
- **Desarrollo para mobile**: Compilación en Termux

## 📞 Contacto

Para soporte o consultas sobre el proyecto, puedes contactar a través de:

- GitHub Issues
- Email del mantenedor
- Comunidad de Rust

---

**Construido con ❤️ para la comunidad de Rust y Termux**
