# Proyecto: Agente Autónomo Local para Ollama en Rust

## Objetivo

Crear un agente local en Rust que funcione sobre Ollama utilizando el modelo qwen2.5:1.5b.

El agente debe actuar como una capa inteligente entre el usuario y Ollama, permitiendo:

* Memoria persistente.
* Creación automática de conocimientos.
* Sistema de skills.
* Base de conocimiento en Markdown.
* Lectura y escritura de archivos.
* Recuperación contextual de información.
* Evolución autónoma del conocimiento.

Todo debe funcionar localmente en Linux y Termux Android.

---

# Arquitectura

Usuario
↓
Agente Rust
↓
Motor de Memoria
↓
Motor de Skills
↓
Motor de Conocimiento
↓
Ollama API
↓
Respuesta

---

# Comunicación con Ollama

Utilizar la API local:

http://localhost:11434/api/generate

Modelo por defecto:

qwen2.5:1.5b

No utilizar "ollama run".

El agente debe comunicarse mediante HTTP.

---

# Estructura de Carpetas

agente/

memoria/

* memoria.md
* usuario.md
* preferencias.md

skills/

* rust.md
* linux.md
* termux.md

conocimiento/

* proyectos/
* personas/
* ideas/
* investigaciones/

logs/

* conversaciones.md

config/

* config.toml

src/

* main.rs
* ollama.rs
* memory.rs
* skills.rs
* rag.rs
* commands.rs
* filesystem.rs

---

# Sistema de Memoria

El agente debe reconocer frases como:

"guarda esto en memoria"

"recuerda esto"

"memoriza esto"

"actualiza memoria"

Cuando detecte estas frases:

* Extraer la información.
* Guardarla en memoria.md.
* Evitar duplicados.
* Añadir fecha y hora.

Ejemplo:

## Memoria

Fecha: 2026-05-31

El usuario se llama Carlos.

---

# Sistema de Skills

Cuando el usuario escriba:

"crea una skill llamada linux"

El agente debe crear:

skills/linux.md

Contenido inicial:

# Linux

Descripción:

Skill creada automáticamente.

Conocimientos:

---

Cuando el usuario diga:

"agrega esto a la skill linux"

Debe actualizar el archivo.

---

# Sistema de Conocimiento

Comando:

"crea conocimiento proyecto tienda"

Resultado:

conocimiento/proyectos/tienda.md

Formato:

# Proyecto Tienda

Creado:

Fecha

Contenido:

---

# Enlaces Bidireccionales

Soportar sintaxis:

[[rust]]
[[linux]]
[[termux]]

El motor debe:

* Detectar enlaces.
* Resolver relaciones.
* Cargar archivos relacionados.

Ejemplo:

proyecto.md

Relacionado:
[[rust]]
[[termux]]

Al consultar proyecto.md:

También cargar rust.md y termux.md.

---

# Motor RAG Local

Implementar recuperación contextual.

Proceso:

1. Buscar palabras clave.
2. Buscar archivos relacionados.
3. Calcular relevancia.
4. Seleccionar mejores documentos.
5. Inyectar contexto al prompt.

No usar bases de datos externas.

Todo debe funcionar con archivos Markdown.

---

# Constructor de Prompt

Antes de enviar al modelo:

Leer:

* memoria.md
* skills relevantes
* conocimientos relevantes

Construir:

=== MEMORIA ===

(contenido)

=== SKILLS ===

(contenido)

=== CONOCIMIENTO ===

(contenido)

=== USUARIO ===

(mensaje)

=== RESPONDE ===

Enviar a Ollama.

---

# Sistema de Comandos

Implementar:

/memoria

Mostrar memoria.

/skills

Listar skills.

/conocimiento

Listar conocimientos.

/buscar texto

Buscar en todos los archivos.

/stats

Mostrar estadísticas.

/exportar

Generar backup.

/ayuda

Mostrar comandos.

---

# Logs

Guardar todas las conversaciones.

Archivo:

logs/conversaciones.md

Formato:

Fecha

Usuario:
...

Asistente:
...

---

# Configuración

config.toml

model = "qwen2.5:1.5b"

memory_limit = 20000

auto_save = true

enable_rag = true

---

# Dependencias Rust

tokio

reqwest

serde

serde_json

chrono

walkdir

regex

toml

anyhow

---

# Requisitos

* Código modular.
* Código comentado.
* Compatible con Linux.
* Compatible con Termux Android.
* Manejo de errores.
* Sistema extensible.
* Sin dependencias de nube.
* Todo local.
* Persistencia automática.

Generar el proyecto completo con todos los módulos, structs, funciones, documentación y código listo para compilar con cargo build --release.
