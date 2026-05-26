#!/usr/bin/env python3
"""Validate the protocol-763 current evidence bundle mirrors matrix seams/hashes."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"

BLAKE3_RE = re.compile(r"`([0-9a-f]{64})`")


def table_rows(text: str) -> list[tuple[str, str]]:
    rows: list[tuple[str, str]] = []
    for line in text.splitlines():
        if not line.startswith("| ") or line.startswith("| ---"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not cells or cells[0] == "Seam":
            continue
        hashes = BLAKE3_RE.findall(line)
        if hashes:
            rows.append((cells[0], hashes[-1]))
    return rows


def main() -> int:
    matrix_text = MATRIX.read_text()
    bundle_text = BUNDLE.read_text()
    matrix_rows = table_rows(matrix_text)
    bundle_rows = table_rows(bundle_text)
    missing: list[str] = []

    if len(matrix_rows) != 11:
        missing.append(f"expected 11 matrix evidence rows, found {len(matrix_rows)}")
    if len(bundle_rows) != len(matrix_rows):
        missing.append(f"bundle row count {len(bundle_rows)} does not match matrix {len(matrix_rows)}")

    bundle_by_seam = dict(bundle_rows)
    for seam, digest in matrix_rows:
        if seam not in bundle_by_seam:
            missing.append(f"bundle missing seam: {seam}")
        elif bundle_by_seam[seam] != digest:
            missing.append(f"bundle hash mismatch for {seam}: {bundle_by_seam[seam]} != {digest}")

    for required in [
        "python3 tools/check_acceptance_matrix.py",
        "python3 tools/check_current_evidence_bundle.py",
        "nix run .#cairn -- validate --root .",
        "full Minecraft compatibility",
        "armor/enchantment semantics",
        "projectile semantics",
    ]:
        if required not in bundle_text:
            missing.append(f"bundle missing required text: {required}")

    if missing:
        for item in missing:
            print(item, file=sys.stderr)
        return 1

    print(f"current evidence bundle ok: {len(bundle_rows)} seams")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
