# Termux - Guía de Compilación y Despliegue

## ¿Qué es Termux?

Termux es un emulador de terminal que proporciona un entorno Linux completo en Android sin necesidad de root.

## Instalación en Android

### 1. Descargar Termux

Descarga desde:
- **F-Droid**: https://f-droid.org/packages/com.termux/ (Recomendado)
- **Google Play**: Busca "Termux"

### 2. Instalar dependencias básicas

```bash
# Actualizar repositorios
pkg update && pkg upgrade

# Instalar Rust
pkg install rust

# Instalar herramientas de compilación
pkg install clang llvm openssl-dev
```

## Compilación en Termux

### 1. Clonar o descargar proyecto

```bash
# Si tienes git
git clone <tu-repo> agente-ollama
cd agente-ollama

# O descarga el ZIP manualmente y extrae
```

### 2. Compilar automático (recomendado)

```bash
bash install.sh
```

### 3. Compilación manual

```bash
# Desarrollo
cargo build

# Optimizado para móvil
cargo build --release
```

## Uso en Termux

### 1. Instalar Ollama (si disponible)

```bash
pkg install ollama
```

Si Ollama no está en repositorio, instalar desde source o usar versión pre-compilada.

### 2. Ejecutar Ollama

```bash
# Terminal 1: Iniciar servidor
ollama serve

# Terminal 2: Descargar modelo
ollama pull qwen2.5:1.5b
```

### 3. Ejecutar el agente

```bash
# En Terminal 3
bash run.sh

# O directamente
./target/release/agente
```

## Notas para Termux

- 📱 Usa `$HOME` para paths en lugar de rutas locales
- 🔋 Considera usar tmux para ejecutar Ollama en background
- 💾 Termux almacena datos en `/data/data/com.termux/files/`
- 🌐 La red local es accesible en `localhost:11434`
- ⏱️ Los procesos pueden ser detenidos si Android mata la app

## Optimización para dispositivos móviles

### RAM limitada

Editar `config/config.toml`:

```toml
[memory]
limit = 10000          # Reducir de 20000

[rag]
max_docs = 2           # Reducir de 5
```

### Almacenamiento limitado

```bash
# Ver espacio disponible
df -h

# Limpiar logs antiguos
rm agente/logs/conversaciones.md

# Crear backup antes de limpiar
/exportar
```

### Rendimiento

- Usar modelos más pequeños (qwen2.5:1.5b es ideal)
- Ejecutar con modelo cuantizado si disponible
- Reducir timeout de Ollama

## Trucos Termux

```bash
# Crear alias para ejecución rápida
echo "alias agente='cd ~/agente-ollama && ./target/release/agente'" >> ~/.bashrc
source ~/.bashrc

# Ejecutar en background con tmux
tmux new-session -d -s ollama "ollama serve"
tmux new-window -t ollama -n agente
tmux send-keys -t ollama:agente "bash run.sh" Enter

# Ver procesos
ps aux | grep ollama
```

## Troubleshooting Termux

### Error: "failed to find clang"
```bash
pkg install clang
```

### Error: "Ollama connection refused"
```bash
# Verificar que Ollama está corriendo
netstat -tlnp | grep 11434

# Si no aparece, iniciar
ollama serve
```

### Error: "Out of memory"
```bash
# Reducir límites en config.toml
# Ver memoria disponible
free -h
```

### Agente muy lento
```bash
# Optimizar compilación
cargo build --release -j 2

# Usar modelo más rápido o cuantizado
```

---

**Termux es perfecto para desarrollar y probar IA local en tu teléfono!** 📱🤖
