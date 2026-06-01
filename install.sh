#!/bin/bash
# Script de instalación rápida para Termux

set -e

echo "🚀 Instalador del Agente Ollama para Termux"
echo "============================================"

# Verificar si estamos en Termux
if [ ! -d "$PREFIX" ]; then
    echo "❌ Este script está diseñado para Termux"
    exit 1
fi

echo "📦 Instalando dependencias..."
pkg update -y
pkg install -y rust clang llvm openssl-dev

echo "🔨 Compilando proyecto..."
cargo build --release

echo "📁 Configurando directorios..."
mkdir -p agente/{memoria,skills,conocimiento/{proyectos,personas,ideas,investigaciones},logs,config,backups}

echo "📋 Copiando configuración..."
cp config/config.toml config/config.toml.bak 2>/dev/null || true

echo ""
echo "✅ Instalación completada!"
echo ""
echo "Próximos pasos:"
echo "1. Instalar Ollama: pkg install ollama"
echo "2. Iniciar Ollama: ollama serve"
echo "3. En otra terminal descargar modelo: ollama pull qwen2.5:1.5b"
echo "4. Ejecutar agente: ./target/release/agente"
