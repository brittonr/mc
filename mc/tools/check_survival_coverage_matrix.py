#!/usr/bin/env python3
"""Validate the protocol-763 survival coverage matrix blocks full-survival overclaims."""
from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs" / "evidence" / "protocol-763-survival-coverage-matrix-2026-05-28.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"

TABLE_HEADER = (
    "| Survival system | Status | Valence evidence | Reference evidence | "
    "Promotion requirement | Explicit non-claim | Next action |"
)
REQUIRED_ROW_COUNT = 9
REQUIRED_SYSTEMS = [
    "break/place/pickup",
    "crafting",
    "chest persistence",
    "furnace persistence",
    "hunger/food",
    "mob drops",
    "redstone",
    "biome/dimension",
    "world persistence",
]
REQUIRED_TEXT = [
    "full_survival_compatibility remains a non-claim",
    "No vanilla parity or full survival compatibility",
    "No crafting coverage",
    "No chest or persistence coverage",
    "No furnace coverage",
    "No hunger or food coverage",
    "No mob AI or mob drop coverage",
    "No redstone coverage",
    "No biome or dimension coverage",
    "No world persistence coverage",
    "paired reference receipt",
    "BLAKE3 manifest entries",
]
FORBIDDEN_CLAIMS = [
    "full_survival_compatibility is covered",
    "full survival compatibility is covered",
    "vanilla parity is covered",
    "full survival compatibility passes",
]
STATUS_MISSING = "missing"
REFERENCE_NONE = "none"
COVERED_ROW = "break/place/pickup"


@dataclass(frozen=True)
class SurvivalRow:
    system: str
    status: str
    valence_evidence: str
    reference_evidence: str
    requirement: str
    non_claim: str
    next_action: str


def coverage_table_lines(text: str) -> list[str]:
    lines: list[str] = []
    in_table = False
    for line in text.splitlines():
        stripped = line.strip()
        if stripped == TABLE_HEADER:
            in_table = True
            continue
        if in_table and stripped.startswith("## "):
            break
        if not in_table or stripped.startswith("| ---"):
            continue
        if stripped.startswith("| "):
            lines.append(stripped)
    return lines


def parse_rows(text: str) -> tuple[list[SurvivalRow], list[str]]:
    rows: list[SurvivalRow] = []
    errors: list[str] = []
    for line in coverage_table_lines(text):
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) != len(SurvivalRow.__dataclass_fields__):
            errors.append(f"survival coverage row has wrong cell count: {line}")
            continue
        rows.append(SurvivalRow(*cells))
    return rows, errors


def validate_text(doc_text: str, bundle_text: str, matrix_text: str) -> tuple[int, list[str]]:
    rows, issues = parse_rows(doc_text)
    rows_by_system = {row.system: row for row in rows}
    if len(rows) != REQUIRED_ROW_COUNT:
        issues.append(f"expected {REQUIRED_ROW_COUNT} survival rows, found {len(rows)}")
    for system in REQUIRED_SYSTEMS:
        if system not in rows_by_system:
            issues.append(f"missing survival row: {system}")
    for required in REQUIRED_TEXT:
        if required not in doc_text:
            issues.append(f"survival matrix missing required text: {required}")
    for forbidden in FORBIDDEN_CLAIMS:
        if forbidden in doc_text or forbidden in bundle_text or forbidden in matrix_text:
            issues.append(f"forbidden full-survival claim present: {forbidden}")

    for row in rows:
        if row.system == COVERED_ROW:
            if row.reference_evidence == REFERENCE_NONE and "parity" not in row.non_claim.lower():
                issues.append("covered break/place/pickup row lacks parity non-claim")
            continue
        if row.status != STATUS_MISSING:
            issues.append(f"unimplemented survival row is not marked missing: {row.system}")
        if row.valence_evidence != REFERENCE_NONE or row.reference_evidence != REFERENCE_NONE:
            issues.append(f"unimplemented survival row unexpectedly cites evidence: {row.system}")
        if "No " not in row.non_claim:
            issues.append(f"unimplemented survival row lacks explicit non-claim: {row.system}")

    if "full survival compatibility" not in bundle_text.lower():
        issues.append("current bundle no longer names full survival compatibility non-claim")
    if "Full survival compatibility / vanilla parity" not in matrix_text:
        issues.append("acceptance matrix no longer names full survival compatibility gap")
    return len(rows), issues


def fixture_doc(rows: str) -> str:
    return f"""# Fixture

## Coverage rows

{TABLE_HEADER}
| --- | --- | --- | --- | --- | --- | --- |
{rows}

## Gate decision

full_survival_compatibility remains a non-claim.

paired reference receipt
BLAKE3 manifest entries
No vanilla parity or full survival compatibility
No crafting coverage
No chest or persistence coverage
No furnace coverage
No hunger or food coverage
No mob AI or mob drop coverage
No redstone coverage
No biome or dimension coverage
No world persistence coverage
"""


def good_rows() -> str:
    return "\n".join(
        [
            "| break/place/pickup | valence_covered_reference_missing | `docs/evidence/x.json` | none | Add paired reference receipt before parity promotion. | No vanilla parity or full survival compatibility. | next |",
            "| crafting | missing | none | none | Add receipts. | No crafting coverage. | next |",
            "| chest persistence | missing | none | none | Add receipts. | No chest or persistence coverage. | next |",
            "| furnace persistence | missing | none | none | Add receipts. | No furnace coverage. | next |",
            "| hunger/food | missing | none | none | Add receipts. | No hunger or food coverage. | next |",
            "| mob drops | missing | none | none | Add receipts. | No mob AI or mob drop coverage. | next |",
            "| redstone | missing | none | none | Add receipts. | No redstone coverage. | next |",
            "| biome/dimension | missing | none | none | Add receipts. | No biome or dimension coverage. | next |",
            "| world persistence | missing | none | none | Add receipts. | No world persistence coverage. | next |",
        ]
    )


def assert_self_tests() -> None:
    bundle = "Full survival compatibility remains a non-claim."
    matrix = "Full survival compatibility / vanilla parity"
    row_count, issues = validate_text(fixture_doc(good_rows()), bundle, matrix)
    assert row_count == REQUIRED_ROW_COUNT, row_count
    assert not issues, issues

    missing_row = good_rows().replace("| crafting |", "| crafting-missing |", 1)
    _, issues = validate_text(fixture_doc(missing_row), bundle, matrix)
    assert any("missing survival row: crafting" in item for item in issues), issues

    promoted_without_evidence = good_rows().replace("| redstone | missing |", "| redstone | covered |", 1)
    _, issues = validate_text(fixture_doc(promoted_without_evidence), bundle, matrix)
    assert any("unimplemented survival row is not marked missing: redstone" in item for item in issues), issues

    _, issues = validate_text(
        fixture_doc(good_rows()) + "\nfull survival compatibility is covered\n",
        bundle,
        matrix,
    )
    assert any("forbidden full-survival claim" in item for item in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative checker fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("survival coverage matrix self-test ok")
        return 0

    row_count, issues = validate_text(DOC.read_text(), BUNDLE.read_text(), MATRIX.read_text())
    if issues:
        for item in issues:
            print(item, file=sys.stderr)
        return 1
    print(f"survival coverage matrix ok: {row_count} rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
