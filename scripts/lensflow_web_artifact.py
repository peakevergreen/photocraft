"""Validate a browser-only CI build; LensFlow owns final hosting/license packaging."""

import gzip
import hashlib
import json
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
if not target.is_absolute():
    target = ROOT / target
source = target / "wasm32-unknown-unknown" / "wasm-release" / "photocraft-web.wasm"
content = source.read_bytes()
if content[:8] != b"\x00asm\x01\x00\x00\x00":
    raise SystemExit("Invalid WebAssembly artifact")
compressed = gzip.compress(content, compresslevel=9, mtime=0)
output = target / "lensflow-web-artifact"
output.mkdir(parents=True, exist_ok=True)
parts = []
for offset in range(0, len(compressed), 20 * 1024**2):
    name = f"editor-{offset // (20 * 1024**2)}.part"
    (output / name).write_bytes(compressed[offset : offset + 20 * 1024**2])
    parts.append(name)
report = {
    "editor": "photocraft",
    "wasm_sha256": hashlib.sha256(content).hexdigest(),
    "wasm_bytes": len(content),
    "compressed_bytes": len(compressed),
    "parts": parts,
    "deployment_ready": False,
    "scope": "CI wasm only; final LensFlow host requires wasm-bindgen glue and complete license packaging.",
}
(output / "build.json").write_text(json.dumps(report, indent=2) + "\n")
summary = f"photocraft web: {len(content):,} wasm bytes; {len(compressed):,} compressed bytes; {len(parts)} download parts."
print(summary)
if os.environ.get("GITHUB_STEP_SUMMARY"):
    with Path(os.environ["GITHUB_STEP_SUMMARY"]).open("a") as stream:
        stream.write(summary + "\n")
