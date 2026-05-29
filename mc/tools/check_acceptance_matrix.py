#!/usr/bin/env python3
"""Validate the protocol-763 compatibility acceptance matrix has required fields."""
from __future__ import annotations

import argparse
import json
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
    "Survival break/place/pickup",
    "Survival chest persistence",
]

REQUIRED_GAPS = [
    "Residual combat breadth",
    "Broad protocol coverage",
    "Production load / multiplayer scale",
    "Full CTF correctness",
    "Full survival compatibility / vanilla parity",
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
    "projectile damage attribution",
    "projectile travel/collision simulation",
]

REVIEWABLE_RECEIPT_SEAMS = frozenset(
    {
        "Armor equipment mitigation",
        "Equipment update observation",
        "Projectile use/loadout rail",
        "Projectile damage attribution",
        "Survival break/place/pickup",
        "Survival chest persistence",
    }
)

EVIDENCE_TABLE_HEADER = (
    "| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | "
    "Landed commits | Scoped claim | Explicit non-claims |"
)
EVIDENCE_ROW_CELLS = 8
BLAKE3_HEX_LENGTH = 64
JSON_SUFFIX = ".json"
MARKDOWN_SUFFIX = ".md"
DOCS_EVIDENCE_PREFIX = "docs/evidence/"
TARGET_PREFIX = "target/"
MANIFEST_GLOB = "*.b3"
MANIFEST_SEPARATOR = "  "
HISTORICAL_TARGET_RECEIPT_SEAMS = frozenset({"RED/BLUE scoring soak"})
HISTORICAL_TARGET_ORACLES = {
    "RED/BLUE scoring soak": "docs/evidence/protocol-763-red-blue-soak-historical-oracle-2026-05-27.md",
}
ORACLE_REQUIRED_HEADINGS = ["## Question", "## Inspected evidence", "## Decision", "## Owner", "## Next action"]
BLAKE3_RE = re.compile(rf"`[0-9a-f]{{{BLAKE3_HEX_LENGTH}}}`")
FORBIDDEN_COMMIT_MARKERS = ("current ", " diff", "untracked", "dirty")
CHILD_REVISION_COMMIT_MARKERS = ("Valence `", "Stevenarella `")
CHILD_REVISION_ORACLE_RE = re.compile(r"`(docs/evidence/[^`]*oracle[^`]*\.md)`")
LEGACY_CHILD_REVISION_DOCS = frozenset(
    {
        "docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.md",
        "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.md",
    }
)


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


def strip_code_span(text: str) -> str:
    return text.strip().strip("`")


def manifest_entries(root: Path) -> list[tuple[str, str]]:
    evidence_dir = root / DOCS_EVIDENCE_PREFIX
    entries: list[tuple[str, str]] = []
    for manifest in evidence_dir.glob(MANIFEST_GLOB):
        for line in manifest.read_text().splitlines():
            if MANIFEST_SEPARATOR not in line:
                continue
            manifest_digest, manifest_path = line.split(MANIFEST_SEPARATOR, maxsplit=1)
            entries.append((manifest_digest, manifest_path.strip()))
    return entries


def manifest_has_digest(root: Path, relative_path: str, digest: str) -> bool:
    return any(
        manifest_digest == digest and manifest_path == relative_path
        for manifest_digest, manifest_path in manifest_entries(root)
    )


def manifest_has_path(root: Path, relative_path: str) -> bool:
    return any(manifest_path == relative_path for _, manifest_path in manifest_entries(root))


def validate_historical_oracle(row: EvidenceRow, digest: str, root: Path = ROOT) -> list[str]:
    oracle_path = HISTORICAL_TARGET_ORACLES.get(row.seam)
    if oracle_path is None:
        return [f"historical target row lacks oracle mapping: {row.seam}"]
    oracle = root / oracle_path
    if not oracle.is_file():
        return [f"historical target oracle missing: {row.seam}: {oracle_path}"]
    text = oracle.read_text()
    missing = [
        f"historical target oracle missing required heading {heading}: {row.seam}: {oracle_path}"
        for heading in ORACLE_REQUIRED_HEADINGS
        if heading not in text
    ]
    if digest not in text:
        missing.append(f"historical target oracle does not cite row BLAKE3: {row.seam}: {oracle_path}")
    if not manifest_has_path(root, oracle_path):
        missing.append(f"historical target oracle lacks BLAKE3 manifest entry: {row.seam}: {oracle_path}")
    return missing


def has_child_revision_commit(row: EvidenceRow) -> bool:
    return any(marker in row.commits for marker in CHILD_REVISION_COMMIT_MARKERS)


def receipt_has_machine_child_revisions(root: Path, receipt_path: str) -> bool:
    try:
        receipt = json.loads((root / receipt_path).read_text())
    except (OSError, json.JSONDecodeError):
        return False
    if not isinstance(receipt, dict):
        return False
    client = receipt.get("client")
    valence = receipt.get("valence")
    if not isinstance(client, dict) or not isinstance(valence, dict):
        return False
    required_client_fields = ["git_rev", "git_status", "git_dirty", "git_diagnostics"]
    required_valence_fields = [
        "git_rev_requested",
        "git_rev_resolved",
        "git_status",
        "git_dirty",
        "git_diagnostics",
    ]
    return all(field in client for field in required_client_fields) and all(
        field in valence for field in required_valence_fields
    )


def oracle_checkpoint_is_reviewable(root: Path, oracle_path: str) -> bool:
    oracle = root / oracle_path
    if not oracle.is_file():
        return False
    text = oracle.read_text()
    return all(heading in text for heading in ORACLE_REQUIRED_HEADINGS) and manifest_has_path(
        root, oracle_path
    )


def validate_child_revision_evidence(
    row: EvidenceRow,
    receipt_path: str,
    doc_path: str,
    doc_text: str,
    root: Path,
) -> list[str]:
    if not has_child_revision_commit(row):
        return []
    if doc_path in LEGACY_CHILD_REVISION_DOCS:
        return []
    if receipt_has_machine_child_revisions(root, receipt_path):
        return []
    oracle_paths = CHILD_REVISION_ORACLE_RE.findall(doc_text)
    if any(oracle_checkpoint_is_reviewable(root, oracle_path) for oracle_path in oracle_paths):
        return []
    return [
        f"row cites child revisions without machine receipt fields or reviewable oracle checkpoint: {row.seam}"
    ]


def validate_evidence_row(row: EvidenceRow, root: Path = ROOT) -> list[str]:
    missing: list[str] = []
    digest = strip_code_span(row.blake3)
    if not row.command.startswith("`nix run"):
        missing.append(f"row lacks maintained nix command: {row.seam}")
    receipt_path = strip_code_span(row.receipt)
    if not receipt_path.endswith(JSON_SUFFIX):
        missing.append(f"row lacks JSON receipt path: {row.seam}")
    if not (receipt_path.startswith(TARGET_PREFIX) or receipt_path.startswith(DOCS_EVIDENCE_PREFIX)):
        missing.append(f"row receipt path must be target/ or docs/evidence/: {row.seam}")
    if row.seam in REVIEWABLE_RECEIPT_SEAMS and not receipt_path.startswith(DOCS_EVIDENCE_PREFIX):
        missing.append(f"ROI 01-03 row lacks reviewable docs/evidence receipt: {row.seam}")
    if not BLAKE3_RE.fullmatch(row.blake3):
        missing.append(f"row lacks single BLAKE3 hash: {row.seam}")
    elif receipt_path.startswith(DOCS_EVIDENCE_PREFIX):
        if not (root / receipt_path).is_file():
            missing.append(f"row docs/evidence receipt is missing: {row.seam}: {receipt_path}")
        elif not manifest_has_digest(root, receipt_path, digest):
            missing.append(f"row docs/evidence receipt lacks matching BLAKE3 manifest entry: {row.seam}: {receipt_path}")
    elif receipt_path.startswith(TARGET_PREFIX) and row.seam not in HISTORICAL_TARGET_RECEIPT_SEAMS:
        missing.append(f"row uses target-only receipt without historical exception: {row.seam}: {receipt_path}")
    elif receipt_path.startswith(TARGET_PREFIX):
        missing.extend(validate_historical_oracle(row, digest, root))

    doc_path = strip_code_span(row.doc)
    if not doc_path.startswith(DOCS_EVIDENCE_PREFIX):
        missing.append(f"row lacks evidence doc path: {row.seam}")
    elif not doc_path.endswith(MARKDOWN_SUFFIX):
        missing.append(f"row evidence doc is not markdown: {row.seam}: {doc_path}")
    elif not (root / doc_path).is_file():
        missing.append(f"row evidence doc is missing: {row.seam}: {doc_path}")
    else:
        doc_text = (root / doc_path).read_text()
        if BLAKE3_RE.fullmatch(row.blake3) and digest not in doc_text:
            missing.append(f"row evidence doc does not cite BLAKE3 hash: {row.seam}: {doc_path}")
        missing.extend(validate_child_revision_evidence(row, receipt_path, doc_path, doc_text, root))

    if "parent `" not in row.commits:
        missing.append(f"row lacks parent commit: {row.seam}")
    lowered_commits = row.commits.lower()
    for marker in FORBIDDEN_COMMIT_MARKERS:
        if marker in lowered_commits:
            missing.append(f"row landed commits cite uncommitted state marker {marker!r}: {row.seam}")
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


def self_test_text(
    blake3: str,
    receipt: str = "docs/evidence/test.receipt.json",
    commits: str = "parent `abc1234`",
) -> str:
    return f"""# Matrix

## Landed evidence rows

{EVIDENCE_TABLE_HEADER}
| --- | --- | --- | --- | --- | --- | --- | --- |
| Armor equipment mitigation | `nix run .#x` | `{receipt}` | `docs/evidence/test.md` | `{blake3}` | {commits} | Bounded claim. | No broad claim. |

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
        doc = root / "docs" / "evidence" / "test.md"
        doc.write_text(f"Receipt BLAKE3: `{good_hash}`\n")
        manifest = root / "docs" / "evidence" / "test.b3"
        manifest.write_text(f"{good_hash}  docs/evidence/test.receipt.json\n")

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

        receipt.write_text('{"status":"pass"}\n')
        manifest.unlink()
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("lacks matching BLAKE3 manifest entry" in item for item in missing), missing

        manifest.write_text(f"{good_hash}  docs/evidence/test.receipt.json\n")
        doc.unlink()
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("row evidence doc is missing" in item for item in missing), missing

        doc.write_text("missing digest\n")
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("does not cite BLAKE3 hash" in item for item in missing), missing

        doc.write_text(f"Receipt BLAKE3: `{good_hash}`\n")
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, receipt="target/live.receipt.json"),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("target-only receipt" in item for item in missing), missing

        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, commits="parent `abc1234`, Valence `def5678` plus current fixture diff"),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("uncommitted state marker" in item for item in missing), missing

        child_commits = "parent `abc1234`, Valence `def5678`, Stevenarella `abc9999`"
        receipt.write_text('{"client": {}, "valence": {}}\n')
        doc.write_text(f"Receipt BLAKE3: `{good_hash}`\n")
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, commits=child_commits),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert any("child revisions" in item for item in missing), missing

        receipt.write_text(
            '{"client": {"git_rev": "abc", "git_status": "clean", "git_dirty": false, "git_diagnostics": []}, '
            '"valence": {"git_rev_requested": "def", "git_rev_resolved": "def", "git_status": "clean", "git_dirty": false, "git_diagnostics": []}}\n'
        )
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, commits=child_commits),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert not missing, missing

        oracle_hash = "1" * BLAKE3_HEX_LENGTH
        oracle_path = root / "docs" / "evidence" / "child-oracle.md"
        oracle_path.write_text(
            "# Oracle\n\n## Question\nq\n\n## Inspected evidence\ne\n\n## Decision\nd\n\n## Owner\no\n\n## Next action\nn\n"
        )
        manifest.write_text(
            f"{good_hash}  docs/evidence/test.receipt.json\n{oracle_hash}  docs/evidence/child-oracle.md\n"
        )
        receipt.write_text('{"client": {}, "valence": {}}\n')
        doc.write_text(
            f"Receipt BLAKE3: `{good_hash}`\nChild-revision oracle checkpoint: `docs/evidence/child-oracle.md`.\n"
        )
        _, _, missing = validate_matrix_text(
            self_test_text(good_hash, commits=child_commits),
            root=root,
            required_seams=required_seams,
            required_gaps=[],
            required_text=[],
        )
        assert not missing, missing


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
