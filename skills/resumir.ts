// desc: Resumir texto extenso extrayendo líneas clave y estadísticas
// uso:  /script resumir  (escribe o pega el texto, Ctrl+D para terminar)

function extractKeywords(text: string, topN = 10): string[] {
  const stopWords = new Set([
    "de", "la", "que", "el", "en", "y", "a", "los", "del", "se", "las",
    "por", "un", "para", "con", "no", "una", "su", "al", "lo", "como",
    "más", "pero", "sus", "le", "ya", "o", "este", "entre", "todo",
    "esta", "sin", "ello", "cada", "otro", "ese", "esa", "eso", "era",
    "the", "and", "for", "are", "but", "not", "you", "all", "any",
    "can", "had", "her", "was", "one", "our", "out", "has", "have",
  ]);

  const words = text.toLowerCase().split(/\W+/).filter((w) => w.length > 3 && !stopWords.has(w));
  const freq: Record<string, number> = {};
  for (const w of words) freq[w] = (freq[w] || 0) + 1;

  return Object.entries(freq)
    .sort((a, b) => b[1] - a[1])
    .slice(0, topN)
    .map(([w]) => w);
}

async function main() {
  const stdinText = await Deno.readTextFile("/dev/stdin").catch(() => "");
  let text = stdinText.trim();

  // Si pasaron texto como argumento, usarlo
  if (!text && Deno.args.length > 0) {
    text = Deno.args.join(" ");
  }

  if (!text) {
    console.log("📝 Modo resumen interactivo");
    console.log("Pega tu texto y presiona Ctrl+D cuando termines:\n");
    const reader = Deno.stdin.readable.getReader();
    const decoder = new TextDecoder();
    let input = "";
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      input += decoder.decode(value, { stream: true });
    }
    text = input.trim();
    reader.releaseLock();
  }

  if (!text) {
    console.log("❌ No hay texto para resumir.");
    Deno.exit(1);
  }

  const lines = text.split("\n").filter((l) => l.trim());
  const words = text.split(/\s+/).filter((w) => w.length > 0);
  const chars = text.length;

  // Estimación de tiempo de lectura (250 ppm)
  const readTimeMin = Math.ceil(words.length / 250);

  console.log(`📊 Estadísticas del texto:`);
  console.log(`  Líneas:   ${lines.length}`);
  console.log(`  Palabras: ${words.length}`);
  console.log(`  Caracteres: ${chars}`);
  console.log(`  Lectura estimada: ~${readTimeMin} min\n`);

  // Keywords relevantes
  const keywords = extractKeywords(text);
  if (keywords.length > 0) {
    console.log(`🔑 Palabras clave:`);
    console.log(`  ${keywords.join(", ")}\n`);
  }

  // Extraer oraciones relevantes (las que contienen palabras clave)
  const sentences = text.split(/[.!?]+/).filter((s) => s.trim().length > 20);
  const topKW = keywords.slice(0, 5);
  const scored = sentences.map((s) => ({
    sentence: s.trim(),
    score: topKW.filter((kw) => s.toLowerCase().includes(kw)).length,
  }));
  scored.sort((a, b) => b.score - a.score);

  const topSentences = scored.filter((s) => s.score > 0).slice(0, 5);
  if (topSentences.length > 0) {
    console.log(`📌 Oraciones más representativas:`);
    for (const s of topSentences) {
      console.log(`  • ${s.sentence}.`);
    }
  }

  // Progresión de temas (primeras palabras de cada párrafo)
  const paragraphs = text.split(/\n\s*\n/).filter((p) => p.trim().length > 50);
  if (paragraphs.length > 1) {
    console.log(`\n📑 Estructura: ${paragraphs.length} párrafos detectados`);
    const firstLines = paragraphs.map((p) => p.split("\n")[0].trim().slice(0, 60));
    console.log(`  Temas: ${firstLines.join(" → ")}`);
  }
}

await main();
