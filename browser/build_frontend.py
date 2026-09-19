from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile


ROOT = Path(__file__).resolve().parent.parent
OUTPUT = ROOT / "browser" / "frontend.zip"
SOURCE_DIRS = ("common", "frontend", "generated", "ir")


with ZipFile(OUTPUT, "w", ZIP_DEFLATED) as archive:
    archive.write(ROOT / "main.py", "main.py")
    archive.write(ROOT / "browser" / "browser_api.py", "browser_api.py")

    for directory in SOURCE_DIRS:
        for path in (ROOT / directory).rglob("*.py"):
            archive.write(path, path.relative_to(ROOT))

print(f"Built {OUTPUT.relative_to(ROOT)} ({OUTPUT.stat().st_size} bytes)")
