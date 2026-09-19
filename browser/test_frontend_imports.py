import sys
import tempfile
from pathlib import Path
from zipfile import ZipFile


ROOT = Path(__file__).resolve().parent.parent

with tempfile.TemporaryDirectory() as directory:
    with ZipFile(ROOT / "browser" / "frontend.zip") as archive:
        archive.extractall(directory)

    sys.path.insert(0, directory)
    from browser_api import build_ir

    protobuf = build_ir(
        """\
def example():
    try:
        return ...
    except Exception:
        return None
"""
    )
    assert protobuf

print("Imported packaged frontend and serialized forward-reference constructs")
