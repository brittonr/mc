#!/usr/bin/env python3
"""Validate the protocol-763 current evidence bundle mirrors matrix seams/hashes."""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"

BLAKE3_RE = re.compile(r"`([0-9a-f]{64})`")

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
    "Projectile damage attribution",
]


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


def validate_bundle_text(
    matrix_text: str,
    bundle_text: str,
    required_seams: list[str] = REQUIRED_SEAMS,
) -> tuple[int, list[str]]:
    matrix_rows = table_rows(matrix_text)
    bundle_rows = table_rows(bundle_text)
    missing: list[str] = []

    expected_rows = len(required_seams)
    if len(matrix_rows) != expected_rows:
        missing.append(f"expected {expected_rows} matrix evidence rows, found {len(matrix_rows)}")
    if len(bundle_rows) != len(matrix_rows):
        missing.append(f"bundle row count {len(bundle_rows)} does not match matrix {len(matrix_rows)}")

    matrix_by_seam = dict(matrix_rows)
    bundle_by_seam = dict(bundle_rows)
    for seam in required_seams:
        if seam not in matrix_by_seam:
            missing.append(f"matrix missing seam: {seam}")
        if seam not in bundle_by_seam:
            missing.append(f"bundle missing seam: {seam}")
    for seam, digest in matrix_rows:
        if seam not in bundle_by_seam:
            missing.append(f"bundle missing seam: {seam}")
        elif bundle_by_seam[seam] != digest:
            missing.append(f"bundle hash mismatch for {seam}: {bundle_by_seam[seam]} != {digest}")

    for required in [
        "python3 tools/check_acceptance_matrix.py",
        "python3 tools/check_current_evidence_bundle.py",
        "nix run --no-update-lock-file .#cairn -- validate --root .",
        "full Minecraft compatibility",
        "armor loadouts",
        "projectile damage attribution",
        "projectile travel/collision simulation",
    ]:
        if required not in bundle_text:
            missing.append(f"bundle missing required text: {required}")

    return len(bundle_rows), missing


def fixture_text(seam: str, digest: str) -> tuple[str, str]:
    matrix = f"""# Matrix

| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| {seam} | `nix run .#x` | `docs/evidence/x.receipt.json` | `docs/evidence/x.md` | `{digest}` | parent `abc1234` | Bounded claim. | No broad claim. |
"""
    bundle = f"""# Bundle

| Seam | Maintained command | BLAKE3 |
| --- | --- | --- |
| {seam} | `nix run .#x` | `{digest}` |

python3 tools/check_acceptance_matrix.py
python3 tools/check_current_evidence_bundle.py
nix run --no-update-lock-file .#cairn -- validate --root .
full Minecraft compatibility
armor loadouts
projectile damage attribution
projectile travel/collision simulation
"""
    return matrix, bundle


def assert_self_tests() -> None:
    seam = "Fixture seam"
    digest = "0" * 64
    matrix, bundle = fixture_text(seam, digest)
    rows, missing = validate_bundle_text(matrix, bundle, required_seams=[seam])
    assert rows == 1, rows
    assert not missing, missing

    _, missing = validate_bundle_text(matrix.replace(seam, "Other seam", 1), bundle, required_seams=[seam])
    assert any("matrix missing seam" in item for item in missing), missing

    _, missing = validate_bundle_text(matrix, bundle.replace(f"| {seam} |", "| Missing seam |"), required_seams=[seam])
    assert any("bundle missing seam" in item for item in missing), missing

    _, missing = validate_bundle_text(matrix, bundle.replace(digest, "1" * 64, 1), required_seams=[seam])
    assert any("bundle hash mismatch" in item for item in missing), missing

    _, missing = validate_bundle_text(matrix, bundle.replace("python3 tools/check_acceptance_matrix.py", ""), required_seams=[seam])
    assert any("bundle missing required text" in item for item in missing), missing


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("current evidence bundle self-test ok")
        return 0

    rows, missing = validate_bundle_text(MATRIX.read_text(), BUNDLE.read_text())
    if missing:
        for item in missing:
            print(item, file=sys.stderr)
        return 1

    print(f"current evidence bundle ok: {rows} seams")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
