#!/usr/bin/env python3
"""Validate the protocol-763 compatibility acceptance matrix has required fields."""
from __future__ import annotations

import argparse
import re
import sys
import tempfile
from dataclasses import dataclass
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
    "Projectile damage attribution",
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
    "projectile travel/collision simulation",
]

REVIEWABLE_RECEIPT_SEAMS = frozenset(
    {
        "Armor equipment mitigation",
        "Equipment update observation",
        "Projectile use/loadout rail",
        "Projectile damage attribution",
    }
)

EVIDENCE_TABLE_HEADER = (
    "| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | "
    "Landed commits | Scoped claim | Explicit non-claims |"
)
EVIDENCE_ROW_CELLS = 8
BLAKE3_HEX_LENGTH = 64
JSON_SUFFIX = ".json"
DOCS_EVIDENCE_PREFIX = "docs/evidence/"
TARGET_PREFIX = "target/"
BLAKE3_RE = re.compile(rf"`[0-9a-f]{{{BLAKE3_HEX_LENGTH}}}`")


@dataclass(frozen=True)
class EvidenceRow:
    seam: str
    command: str
    receipt: str
    doc: str
    blake3: str
    commits: str
    claim: str
    non_claims: str


def evidence_table_lines(text: str) -> list[str]:
    lines: list[str] = []
    in_table = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == EVIDENCE_TABLE_HEADER:
            in_table = True
            continue
        if in_table and stripped.startswith("## "):
            break
        if not in_table:
            continue
        if stripped.startswith("| ---"):
            continue
        if stripped.startswith("| "):
            lines.append(stripped)
    return lines


def parse_evidence_rows(text: str) -> tuple[list[EvidenceRow], list[str]]:
    rows: list[EvidenceRow] = []
    errors: list[str] = []
    for line in evidence_table_lines(text):
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) < EVIDENCE_ROW_CELLS:
            errors.append(f"evidence row has too few cells: {line}")
            continue
        rows.append(EvidenceRow(*cells[:EVIDENCE_ROW_CELLS]))
    return rows, errors


def validate_evidence_row(row: EvidenceRow, root: Path = ROOT) -> list[str]:
    missing: list[str] = []
    if not row.command.startswith("`nix run"):
        missing.append(f"row lacks maintained nix command: {row.seam}")
    receipt_path = row.receipt.strip("`")
    if not receipt_path.endswith(JSON_SUFFIX):
        missing.append(f"row lacks JSON receipt path: {row.seam}")
    if not (receipt_path.startswith(TARGET_PREFIX) or receipt_path.startswith(DOCS_EVIDENCE_PREFIX)):
        missing.append(f"row receipt path must be target/ or docs/evidence/: {row.seam}")
    if row.seam in REVIEWABLE_RECEIPT_SEAMS and not receipt_path.startswith(DOCS_EVIDENCE_PREFIX):
        missing.append(f"ROI 01-03 row lacks reviewable docs/evidence receipt: {row.seam}")
    if receipt_path.startswith(DOCS_EVIDENCE_PREFIX) and not (root / receipt_path).is_file():
        missing.append(f"row docs/evidence receipt is missing: {row.seam}: {receipt_path}")
    if not row.doc.startswith("`docs/evidence/"):
        missing.append(f"row lacks evidence doc path: {row.seam}")
    if not BLAKE3_RE.fullmatch(row.blake3):
        missing.append(f"row lacks single BLAKE3 hash: {row.seam}")
    if "parent `" not in row.commits:
        missing.append(f"row lacks parent commit: {row.seam}")
    if not row.claim or row.claim == "-":
        missing.append(f"row lacks scoped claim: {row.seam}")
    if "No " not in row.non_claims and "no " not in row.non_claims:
        missing.append(f"row lacks explicit non-claim: {row.seam}")
    return missing


def validate_matrix_text(
    text: str,
    root: Path = ROOT,
    required_seams: list[str] = REQUIRED_SEAMS,
    required_gaps: list[str] = REQUIRED_GAPS,
    required_text: list[str] = REQUIRED_TEXT,
) -> tuple[int, int, list[str]]:
    missing: list[str] = []

    for seam in required_seams:
        if seam not in text:
            missing.append(f"missing seam row: {seam}")
    for gap in required_gaps:
        if gap not in text:
            missing.append(f"missing gap row: {gap}")
    for required in required_text:
        if required not in text:
            missing.append(f"missing required field/text: {required}")

    rows, row_errors = parse_evidence_rows(text)
    missing.extend(row_errors)
    if len(rows) != len(required_seams):
        missing.append(f"expected {len(required_seams)} evidence rows, found {len(rows)}")

    row_by_seam = {row.seam: row for row in rows}
    for seam in required_seams:
        if seam not in row_by_seam:
            missing.append(f"missing parsed seam row: {seam}")

    for row in rows:
        missing.extend(validate_evidence_row(row, root))

    hashes = BLAKE3_RE.findall(text)
    if len(hashes) < len(required_seams):
        missing.append(f"expected at least {len(required_seams)} BLAKE3 hashes, found {len(hashes)}")

    return len(rows), len(hashes), missing


def self_test_text(blake3: str, receipt: str = "docs/evidence/test.receipt.json") -> str:
    return f"""# Matrix

## Landed evidence rows

{EVIDENCE_TABLE_HEADER}
| --- | --- | --- | --- | --- | --- | --- | --- |
| Armor equipment mitigation | `nix run .#x` | `{receipt}` | `docs/evidence/test.md` | `{blake3}` | parent `abc1234` | Bounded claim. | No broad claim. |

## Remaining gaps and non-claims
"""


def assert_self_tests() -> None:
    good_hash = "0" * BLAKE3_HEX_LENGTH
    required_seams = ["Armor equipment mitigation"]
    with tempfile.TemporaryDirectory(prefix="acceptance-matrix-self-test-") as tmp:
        root = Path(tmp)
        receipt = root / "docs" / "evidence" / "test.receipt.json"
        receipt.parent.mkdir(parents=True)
        receipt.write_text('{"status":"pass"}\n')

        rows, hashes, missing = validate_matrix_text(
            self_test_text(good_hash),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert rows == 1, rows
        assert hashes == 1, hashes
        assert not missing, missing

        _, _, missing = validate_matrix_text(
            self_test_text("not-a-blake3"),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("row lacks single BLAKE3 hash" in item for item in missing), missing

        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, receipt="docs/evidence/index.md"),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("row lacks JSON receipt path" in item for item in missing), missing

        receipt.unlink()
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("row docs/evidence receipt is missing" in item for item in missing), missing


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("acceptance matrix self-test ok")
        return 0

    text = MATRIX.read_text()
    rows, hashes, missing = validate_matrix_text(text)
    if missing:
        for item in missing:
            print(item, file=sys.stderr)
        return 1

    print(f"acceptance matrix ok: {rows} seams, {hashes} hashes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
