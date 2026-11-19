import json
import os
import re
import sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()
out = Path(sys.argv[2])

extensions = {
    ".go": "go",
    ".rs": "rust",
    ".c": "c",
    ".h": "c",
}

skip_dirs = {".git", "vendor", "target", "node_modules", "build", "dist"}
license_words = ("copyright", "license", "spdx", "all rights reserved")

def description_for(text, ext):
    # Try the first comment that doesn't look like a license header.
    comments = re.findall(r"/\*.*?\*/|(?m)^\s*//.*$", text, re.S)
    for comment in comments:
        cleaned = re.sub(r"^\s*/\*|\*/\s*$", "", comment)
        cleaned = re.sub(r"(?m)^\s*//+\s?", "", cleaned)
        cleaned = re.sub(r"(?m)^\s*\*\s?", "", cleaned)
        cleaned = " ".join(cleaned.split()).strip()
        if cleaned and not any(word in cleaned.lower() for word in license_words):
            return cleaned[:240]

    # Fall back to the first recognizable declaration.
    patterns = [
        r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+\w+",
        r"(?m)^\s*(?:pub\s+)?(?:struct|enum|trait|type|const|static)\s+\w+",
        r"(?m)^\s*(?:func|type|var|const)\s+\w+",
        r"(?m)^\s*[\w*]+\s+\w+\s*$[^;]*$\s*\{?",
    ]
    for pattern in patterns:
        match = re.search(pattern, text)
        if match:
            return f"Declaration: {match.group(0).strip()}"[:240]

    return "No description found"

entries = []
for current, dirs, files in os.walk(root):
    dirs[:] = sorted(d for d in dirs if d not in skip_dirs)

    for filename in sorted(files):
        path = Path(current) / filename
        language = extensions.get(path.suffix.lower())
        if not language:
            continue

        try:
            text = path.read_text(encoding="utf-8", errors="replace")[:32768]
        except OSError:
            continue

        entries.append({
            "path": str(path.relative_to(root)),
            "language": language,
            "description": description_for(text, path.suffix.lower()),
        })

out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(entries, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"Indexed {len(entries)} files into {out}")

