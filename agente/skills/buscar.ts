// desc: Busca archivos por patrón de nombre y contenido, con filtros
// uso:  /script buscar <patrón> [directorio]  — ej: /script buscar "*.rs" ./src

interface SearchResult {
  file: string;
  lines: { num: number; content: string }[];
}

function globToRegex(pattern: string): RegExp {
  const escaped = pattern.replace(/[.+^${}()|[\]\\]/g, "\\$&");
  const regexStr = escaped.replace(/\*/g, ".*").replace(/\?/g, ".");
  return new RegExp(`^${regexStr}$`);
}

async function searchFiles(
  dir: string,
  namePattern: string,
  contentPattern?: string,
): Promise<SearchResult[]> {
  const results: SearchResult[] = [];
  const nameRegex = globToRegex(namePattern);

  async function walk(currentDir: string) {
    try {
      for await (const entry of Deno.readDir(currentDir)) {
        const fullPath = `${currentDir}/${entry.name}`;
        if (entry.isDirectory && !entry.name.startsWith(".")) {
          if (entry.name !== "target" && entry.name !== "node_modules") {
            await walk(fullPath);
          }
        } else if (entry.isFile) {
          if (nameRegex.test(entry.name)) {
            if (contentPattern) {
              try {
                const content = await Deno.readTextFile(fullPath);
                const lines = content.split("\n");
                const matchingLines = lines
                  .map((line, i) => ({ num: i + 1, content: line.trim() }))
                  .filter((l) => l.content.toLowerCase().includes(contentPattern.toLowerCase()));
                if (matchingLines.length > 0) {
                  results.push({ file: fullPath, lines: matchingLines.slice(0, 10) });
                }
              } catch {
                // skip binary or unreadable files
              }
            } else {
              results.push({ file: fullPath, lines: [] });
            }
          }
        }
      }
    } catch {
      // skip permission denied
    }
  }

  await walk(dir);
  return results;
}

async function main() {
  const stdinText = await Deno.readTextFile("/dev/stdin").catch(() => "");
  const args = Deno.args;

  // Parse input: puede venir de stdin o argumentos
  let input = stdinText.trim();
  if (!input && args.length > 0) {
    input = args.join(" ");
  }

  const parts = input.split(/\s+/).filter((s) => s);
  if (parts.length === 0) {
    console.log("🔍 Buscador de archivos");
    console.log("");
    console.log("Uso: pasa el patrón de búsqueda como argumento o por stdin");
    console.log("  /script buscar *.rs              — busca archivos .rs");
    console.log("  /script buscar *.rs ./src        — en directorio específico");
    console.log("  /script buscar *.ts 'function'   — busca archivos .ts que contengan 'function'");
    Deno.exit(0);
  }

  const namePattern = parts[0];
  const secondArg = parts[1];
  let searchDir = ".";
  let contentPattern: string | undefined;

  if (parts.length >= 2) {
    // Determinar si el segundo argumento es directorio o contenido
    if (secondArg.startsWith(".") || secondArg.startsWith("/") || secondArg.startsWith("~")) {
      searchDir = secondArg;
      contentPattern = parts.slice(2).join(" ") || undefined;
    } else {
      contentPattern = parts.slice(1).join(" ");
    }
  }

  console.log(`🔍 Buscando: "${namePattern}"`);
  if (contentPattern) console.log(`   Contenido: "${contentPattern}"`);
  console.log(`   Directorio: ${searchDir}\n`);

  const results = await searchFiles(searchDir, namePattern, contentPattern);

  if (results.length === 0) {
    console.log("😴 No se encontraron archivos.");
    Deno.exit(0);
  }

  console.log(`📁 ${results.length} archivo(s) encontrado(s):\n`);
  for (const r of results.slice(0, 30)) {
    console.log(`  📄 ${r.file}`);
    if (r.lines.length > 0) {
      for (const line of r.lines.slice(0, 3)) {
        console.log(`     :${line.num}  ${line.content.slice(0, 80)}`);
      }
      if (r.lines.length > 3) console.log(`     ... y ${r.lines.length - 3} línea(s) más`);
      console.log();
    }
  }
  if (results.length > 30) {
    console.log(`... y ${results.length - 30} archivo(s) más`);
  }
}

await main();
