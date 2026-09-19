from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile


ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / "browser" / "frontend.zip"
SOURCE_DIRS = ("common", "frontend", "generated", "ir")
FUTURE_ANNOTATIONS = "from __future__ import annotations\n"


def add_python_source(archive: ZipFile, path: Path, archive_name: str | Path) -> None:
    source = path.read_text(encoding="utf-8")

    # Pyodide currently runs Python 3.13. Postpone evaluation everywhere in
    # the packaged frontend so forward references and circular IR type hints
    # are never resolved while modules are being imported.
    if path.name != "_pb2.py" and FUTURE_ANNOTATIONS not in source:
        source = FUTURE_ANNOTATIONS + source

    archive.writestr(str(archive_name), source)


with ZipFile(OUTPUT, "w", ZIP_DEFLATED) as archive:
    add_python_source(archive, ROOT / "main.py", "main.py")
    add_python_source(archive, ROOT / "browser" / "browser_api.py", "browser_api.py")

    for directory in SOURCE_DIRS:
        for path in (ROOT / directory).rglob("*.py"):
            add_python_source(archive, path, path.relative_to(ROOT))

print(f"Built {OUTPUT.relative_to(ROOT)} ({OUTPUT.stat().st_size} bytes)")
