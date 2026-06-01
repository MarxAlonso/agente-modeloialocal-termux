# ⚡ QUICK START - Agente Ollama en 5 minutos

## En tu PC (Windows)

```bash
# El proyecto ya está creado en:
C:\Users\GamingWorld\OneDrive\Desktop\utp proyects\agente-modeloialocal-termux\
```

**Archivos clave:**
- `src/main.rs` - Aplicación principal
- `config/config.toml` - Configuración
- `README.md` - Documentación completa

## En Termux (Android)

### Opción 1: Rápida (automática)
```bash
# 1. Copiar proyecto a Termux
cd ~/storage/downloads
unzip agente-ollama.zip
cd agente-ollama

# 2. Instalación automática
bash install.sh

# 3. En otra terminal, iniciar Ollama
ollama serve

# 4. En otra terminal, ejecutar agente
bash run.sh
```

### Opción 2: Manual (si prefieres ver cada paso)
```bash
# Actualizar e instalar
pkg update && pkg upgrade -y
pkg install rust clang llvm openssl-dev -y

# Compilar
cargo build --release

# Crear directorios
mkdir -p agente/{memoria,skills,conocimiento/{proyectos,personas,ideas,investigaciones},logs,config}

# Copiar config
cp config/config.toml agente/

# Ejecutar
./target/release/agente
```

## Prueba rápida

```
📌 > Hola, soy Carlos
🤖 [respuesta del agente]

📌 > guarda que tengo un proyecto en Rust
💾 Guardado en memoria

📌 > /memoria
📝 Memoria:
[muestra lo guardado]

📌 > /ayuda
🆘 [muestra comandos]

📌 > salir
👋 ¡Hasta luego!
```

## Troubleshooting rápido

| Problema | Solución |
|----------|----------|
| `error: could not compile` | `pkg install clang llvm` |
| `Ollama connection refused` | `ollama serve` en otra terminal |
| `Model not found` | `ollama pull qwen2.5:1.5b` |
| `Out of memory` | Reducir `memory.limit` en config.toml |

## Archivos importantes

```
📁 agente-ollama/
├── 📄 README.md           👈 Lee esto primero
├── 📄 TERMUX_GUIDE.md     👈 Para Termux
├── 📄 COMPLETADO.md       👈 Resumen del proyecto
├── 📁 src/
│   └── 📄 main.rs         👈 Código principal
├── 📁 config/
│   └── 📄 config.toml     👈 Ajusta aquí
├── 📁 agente/
│   ├── 📁 memoria/        👈 Se llena con uso
│   ├── 📁 skills/         👈 Pre-configurado
│   └── 📁 conocimiento/   👈 Ejemplos incluidos
├── 📄 install.sh          👈 Instalación automática
└── 📄 run.sh              👈 Ejecutar agente
```

## Comandos útiles Termux

```bash
# Ver modelos Ollama
ollama list

# Ver espacio disponible
df -h

# Ver procesos
ps aux | grep ollama

# Matar proceso
killall ollama

# Ver logs
tail -f agente/logs/conversaciones.md

# Crear alias
echo "alias agente='bash ~/agente-ollama/run.sh'" >> ~/.bashrc
```

## Próximos pasos

1. ✅ **Compilar y ejecutar** en Termux
2. 📝 **Crear skills** personalizados
3. 📚 **Agregar conocimiento** en Markdown
4. 🔍 **Usar RAG** para recuperación contextual
5. 💾 **Exportar backups** regularmente

## Video rápido (si necesitas ver)

1. Instala Termux de F-Droid
2. `bash install.sh`
3. `ollama serve`
4. `bash run.sh`
5. ¡Listo!

---

**¿Preguntas?** Lee `README.md` o `TERMUX_GUIDE.md`

**¿Problemas?** Revisa la sección Troubleshooting de `TERMUX_GUIDE.md`

**¿Listo?** ¡Ejecuta `bash install.sh` ahora! 🚀
