// desc: Analiza un directorio y resume su estructura, tamaño y tipo de proyecto
// uso: /script analizar [ruta]  —  si no se da ruta, analiza el directorio actual

interface FileInfo {
  path: string;
  size: number;
  ext: string;
}

async function walkDir(dir: string): Promise<FileInfo[]> {
  const files: FileInfo[] = [];
  try {
    for await (const entry of Deno.readDir(dir)) {
      const fullPath = `${dir}/${entry.name}`;
      if (entry.isDirectory && !entry.name.startsWith(".") && !entry.name.startsWith("target")) {
        files.push(...await walkDir(fullPath));
      } else if (entry.isFile) {
        const stat = await Deno.stat(fullPath);
        const ext = entry.name.includes(".") ? entry.name.split(".").pop()! : "sin-ext";
        files.push({ path: fullPath, size: stat.size, ext });
      }
    }
  } catch {
    // skip permission errors
  }
  return files;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function detectProjectType(files: FileInfo[]): string {
  const exts = new Set(files.map((f) => f.ext));
  if (exts.has("rs") && exts.has("toml")) return "🦀 Rust";
  if (exts.has("ts") || exts.has("tsx")) {
    if (exts.has("json")) return "📦 TypeScript/Node";
    return "📘 TypeScript";
  }
  if (exts.has("js") || exts.has("jsx") || exts.has("mjs")) return "🟨 JavaScript/Node";
  if (exts.has("py")) return "🐍 Python";
  if (exts.has("go") || exts.has("mod")) return "🔷 Go";
  if (exts.has("c") || exts.has("h") || exts.has("cpp")) return "⚙️ C/C++";
  if (exts.has("sh") || exts.has("bash")) return "🐚 Shell Script";
  if (exts.has("md")) return "📝 Documentación";
  return "📁 Desconocido / Mixto";
}

async function main() {
  // Leer input: puede venir de stdin o de argumento CLI
  const args = Deno.args;
  let targetDir = ".";

  const stdinText = await Deno.readTextFile("/dev/stdin").catch(() => "");
  const input = stdinText.trim() || args[0] || "";

  if (input) {
    targetDir = input;
  }

  // Verificar que el directorio existe
  try {
    const stat = await Deno.stat(targetDir);
    if (!stat.isDirectory) {
      console.log(`❌ '${targetDir}' no es un directorio`);
      Deno.exit(1);
    }
  } catch {
    console.log(`❌ Directorio '${targetDir}' no encontrado`);
    Deno.exit(1);
  }

  console.log(`📊 Analizando: ${targetDir}\n`);

  const files = await walkDir(targetDir);
  if (files.length === 0) {
    console.log("No se encontraron archivos.");
    return;
  }

  const totalSize = files.reduce((sum, f) => sum + f.size, 0);
  const extCount: Record<string, number> = {};
  for (const f of files) {
    extCount[f.ext] = (extCount[f.ext] || 0) + 1;
  }

  const projectType = detectProjectType(files);

  console.log(`Tipo de proyecto: ${projectType}`);
  console.log(`Archivos totales: ${files.length}`);
  console.log(`Tamaño total:     ${formatBytes(totalSize)}`);
  console.log(`\nExtensiones:`);
  const sortedExts = Object.entries(extCount).sort((a, b) => b[1] - a[1]);
  for (const [ext, count] of sortedExts.slice(0, 10)) {
    const pct = ((count / files.length) * 100).toFixed(0);
    console.log(`  .${ext}: ${count} archivos (${pct}%)`);
  }
}

await main();
