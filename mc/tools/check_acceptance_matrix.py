#!/usr/bin/env python3
"""Validate the protocol-763 compatibility acceptance matrix has required fields."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"

REQUIRED_SEAMS = [
    "RED/BLUE scoring soak",
    "Inventory/drop",
    "Block placement / use-item-on-block",
    "Pickup semantics",
    "Player-inventory click/container click",
    "Open-container semantics",
    "Two-client combat/damage",
    "Flag-carrier death/return",
    "Reconnect flag-state",
    "Latency/jitter tolerance",
    "Combat knockback",
    "Armor equipment mitigation",
    "Equipment update observation",
    "Projectile use/loadout rail",
]

REQUIRED_GAPS = [
    "Residual combat breadth",
    "Broad protocol coverage",
    "Production load / multiplayer scale",
    "Full CTF correctness",
]

REQUIRED_TEXT = [
    "Explicit non-claims",
    "Scoped claim",
    "BLAKE3",
    "Maintained command",
    "Receipt",
    "roi-04-latency-jitter-tolerance",
    "roi-05-projectile-armor-knockback-combat",
    "roi-07-post-drain-evidence-index",
    "projectile travel/collision/damage attribution",
]

BLAKE3_RE = re.compile(r"`[0-9a-f]{64}`")


def main() -> int:
    text = MATRIX.read_text()
    missing: list[str] = []

    for seam in REQUIRED_SEAMS:
        if seam not in text:
            missing.append(f"missing seam row: {seam}")
    for gap in REQUIRED_GAPS:
        if gap not in text:
            missing.append(f"missing gap row: {gap}")
    for required in REQUIRED_TEXT:
        if required not in text:
            missing.append(f"missing required field/text: {required}")

    hashes = BLAKE3_RE.findall(text)
    if len(hashes) < len(REQUIRED_SEAMS):
        missing.append(
            f"expected at least {len(REQUIRED_SEAMS)} BLAKE3 hashes, found {len(hashes)}"
        )

    for line in text.splitlines():
        if not line.startswith("| ") or line.startswith("| ---"):
            continue
        if not BLAKE3_RE.search(line):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) < 8:
            missing.append(f"evidence row has too few cells: {line}")
            continue
        seam, command, receipt, doc, blake3, commits, claim, non_claims = cells[:8]
        if not command.startswith("`nix run"):
            missing.append(f"row lacks maintained nix command: {seam}")
        receipt_path = receipt.strip("`")
        if not receipt_path.endswith(".json"):
            missing.append(f"row lacks JSON receipt path: {seam}")
        if not (receipt_path.startswith("target/") or receipt_path.startswith("docs/evidence/")):
            missing.append(f"row receipt path must be target/ or docs/evidence/: {seam}")
        if seam in {
            "Armor equipment mitigation",
            "Equipment update observation",
            "Projectile use/loadout rail",
        } and not receipt_path.startswith("docs/evidence/"):
            missing.append(f"ROI 01-03 row lacks reviewable docs/evidence receipt: {seam}")
        if not doc.startswith("`docs/evidence/"):
            missing.append(f"row lacks evidence doc path: {seam}")
        if not BLAKE3_RE.fullmatch(blake3):
            missing.append(f"row lacks single BLAKE3 hash: {seam}")
        if "parent `" not in commits:
            missing.append(f"row lacks parent commit: {seam}")
        if not claim or claim == "-":
            missing.append(f"row lacks scoped claim: {seam}")
        if "No " not in non_claims and "no " not in non_claims:
            missing.append(f"row lacks explicit non-claim: {seam}")

    if missing:
        for item in missing:
            print(item, file=sys.stderr)
        return 1

    print(f"acceptance matrix ok: {len(REQUIRED_SEAMS)} seams, {len(hashes)} hashes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
