// desc: Pregunta a la IA local (Ollama) directamente desde TypeScript
// uso:  /script preguntar <pregunta>
// ej:   /script preguntar explica qué es Rust en 3 líneas

interface OllamaResponse {
  response: string;
  done: boolean;
}

async function queryOllama(
  prompt: string,
  model = "qwen2.5:1.5b",
  url = "http://localhost:11434",
): Promise<string> {
  const body = JSON.stringify({
    model,
    prompt,
    temperature: 0.7,
    top_k: 40,
    top_p: 0.9,
    stream: false,
  });

  const res = await fetch(`${url}/api/generate`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body,
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Ollama error HTTP ${res.status}: ${text}`);
  }

  const data: OllamaResponse = await res.json();
  return data.response;
}

async function main() {
  const stdinText = await Deno.readTextFile("/dev/stdin").catch(() => "");
  let prompt = stdinText.trim();

  if (!prompt && Deno.args.length > 0) {
    prompt = Deno.args.join(" ");
  }

  if (!prompt) {
    console.log("❌ Uso: pasa una pregunta como argumento o por stdin");
    console.log("   /script preguntar 'qué es Rust?'");
    Deno.exit(1);
  }

  const start = performance.now();

  try {
    const response = await queryOllama(prompt);
    const elapsed = ((performance.now() - start) / 1000).toFixed(1);

    console.log(`🤖 Ollama (${elapsed}s):\n`);
    console.log(response);
  } catch (err) {
    console.error(`❌ Error: ${err.message}`);
    console.log("\n💡 Asegúrate de que Ollama esté corriendo: ollama serve");
    Deno.exit(1);
  }
}

await main();
