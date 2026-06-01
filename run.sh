#!/bin/bash
# Script para ejecutar el agente

set -e

BINARY="./target/release/agente"

# Compilar si no existe
if [ ! -f "$BINARY" ]; then
    echo "Compilando agente..."
    cargo build --release
fi

echo "🤖 Iniciando Agente..."
echo "Asegúrate de que Ollama está corriendo: ollama serve"
echo ""

exec "$BINARY"
