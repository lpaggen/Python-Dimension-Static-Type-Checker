import { loadPyodide } from "https://cdn.jsdelivr.net/pyodide/v0.28.3/full/pyodide.mjs";
import initChecker, { analyze_ir } from "./pkg/pdc_rust_check.js?v=unknown-diagnostics-2";

const assetVersion = "unknown-diagnostics-2";

const status = document.querySelector("#status");
const button = document.querySelector("#analyze");
const source = document.querySelector("#source");
const output = document.querySelector("#diagnostics");

// Concrete constraints are solved inside the checker. Symbolic queries will
// use the Z3/WASM bridge installed here in the next integration step.
globalThis.pdcSolveSmt2 = () => "unknown";

let buildIr;

async function initialize() {
  await initChecker(
    new URL(`./pkg/pdc_rust_check_bg.wasm?v=${assetVersion}`, import.meta.url),
  );

  const pyodide = await loadPyodide();
  await pyodide.loadPackage("micropip");
  await pyodide.runPythonAsync(`
import micropip
await micropip.install("protobuf==7.35.1")
  `);

  const archive = await fetch(`./frontend.zip?v=${assetVersion}`).then((response) => {
    if (!response.ok) throw new Error(`frontend download failed: ${response.status}`);
    return response.arrayBuffer();
  });
  pyodide.unpackArchive(archive, "zip");
  buildIr = pyodide.pyimport("browser_api").build_ir;

  status.textContent = "Ready. Analysis runs entirely in this browser.";
  button.disabled = false;
}

function renderDiagnostics(diagnostics) {
  output.replaceChildren();

  if (diagnostics.length === 0) {
    output.textContent = "No diagnostics.";
    return;
  }

  for (const diagnostic of diagnostics) {
    const item = document.createElement("div");
    item.className = "diagnostic";
    const span = diagnostic.span;
    const location = span
      ? `${span.file}:${span.lineno}:${span.col_offset + 1}`
      : "unknown location";
    item.textContent = `${location}: ${diagnostic.message}`;
    output.append(item);
  }
}

button.addEventListener("click", () => {
  button.disabled = true;
  status.textContent = "Analyzing…";

  try {
    const pythonBytes = buildIr(source.value, "playground.py");
    const bytes = pythonBytes.toJs ? pythonBytes.toJs() : pythonBytes;
    const result = JSON.parse(analyze_ir(bytes));
    pythonBytes.destroy?.();

    if (result.error) throw new Error(result.error);
    renderDiagnostics(result);
    status.textContent = "Analysis complete.";
  } catch (error) {
    output.textContent = String(error);
    status.textContent = "Analysis failed.";
  } finally {
    button.disabled = false;
  }
});

initialize().catch((error) => {
  status.textContent = `Could not load checker: ${error}`;
  console.error(error);
});
