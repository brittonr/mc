#!/usr/bin/env python3
"""Validate tracked evidence BLAKE3 manifests and stale receipt markers."""
from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "docs" / "evidence"
BLAKE3_HEX_LENGTH = 64
MANIFEST_GLOB = "*.b3"
RECEIPT_GLOB = "*.receipt.json"
MANIFEST_SEPARATOR = "  "
STALE_RECEIPT_MARKERS = {
    "equipment_packet_observed": "use entity_equipment_update",
}
BLAKE3_RE = re.compile(rf"^[0-9a-f]{{{BLAKE3_HEX_LENGTH}}}$")


@dataclass(frozen=True)
class ManifestEntry:
    manifest: Path
    digest: str
    relative_path: Path


@dataclass(frozen=True)
class CheckResult:
    manifests: int
    entries: int
    receipts_scanned: int


def repo_relative(path: Path, root: Path = ROOT) -> Path:
    return path.resolve().relative_to(root.resolve())


def parse_manifest(manifest: Path, text: str) -> tuple[list[ManifestEntry], list[str]]:
    entries: list[ManifestEntry] = []
    errors: list[str] = []
    for line_number, raw_line in enumerate(text.splitlines(), start=1):
        line = raw_line.strip()
        if not line:
            continue
        if MANIFEST_SEPARATOR not in line:
            errors.append(f"{manifest}:{line_number}: expected '<b3>  <path>'")
            continue
        digest, path_text = line.split(MANIFEST_SEPARATOR, maxsplit=1)
        if not BLAKE3_RE.fullmatch(digest):
            errors.append(f"{manifest}:{line_number}: invalid BLAKE3 digest {digest!r}")
            continue
        relative_path = Path(path_text.strip())
        if relative_path.is_absolute() or ".." in relative_path.parts:
            errors.append(f"{manifest}:{line_number}: path must be repo-relative: {path_text!r}")
            continue
        entries.append(ManifestEntry(manifest=manifest, digest=digest, relative_path=relative_path))
    if not entries and not errors:
        errors.append(f"{manifest}: manifest has no entries")
    return entries, errors


def discover_manifests(evidence_dir: Path = EVIDENCE_DIR) -> list[Path]:
    return sorted(evidence_dir.glob(MANIFEST_GLOB))


def discover_receipts(evidence_dir: Path = EVIDENCE_DIR) -> list[Path]:
    return sorted(evidence_dir.glob(RECEIPT_GLOB))


def validate_manifest_entries(entries: Iterable[ManifestEntry], root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    for entry in entries:
        target = root / entry.relative_path
        if not target.exists():
            errors.append(f"{entry.manifest}: referenced file missing: {entry.relative_path}")
        elif not target.is_file():
            errors.append(f"{entry.manifest}: referenced path is not a file: {entry.relative_path}")
    return errors


def run_b3sum_check(manifests: Iterable[Path], root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    if shutil.which("b3sum") is None:
        return ["b3sum not found on PATH; run under nix develop or Nix check"]
    for manifest in manifests:
        completed = subprocess.run(
            ["b3sum", "--check", str(repo_relative(manifest, root))],
            cwd=root,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )
        if completed.returncode != 0:
            output = completed.stdout.strip() or "<no output>"
            errors.append(f"{manifest}: b3sum --check failed:\n{output}")
    return errors


def referenced_paths(entries: Iterable[ManifestEntry], root: Path = ROOT) -> set[Path]:
    return {(root / entry.relative_path).resolve() for entry in entries}


def receipt_paths(
    entries: Iterable[ManifestEntry],
    root: Path = ROOT,
    evidence_dir: Path = EVIDENCE_DIR,
) -> list[Path]:
    paths = referenced_paths(entries, root)
    paths.update(path.resolve() for path in discover_receipts(evidence_dir))
    return sorted(path for path in paths if path.name.endswith(".receipt.json"))


def stale_marker_errors(receipts: Iterable[Path]) -> list[str]:
    errors: list[str] = []
    for receipt in receipts:
        if not receipt.exists():
            continue
        text = receipt.read_text()
        for marker, replacement in sorted(STALE_RECEIPT_MARKERS.items()):
            if marker in text:
                errors.append(f"{receipt}: stale marker {marker!r}; {replacement}")
    return errors


def check_evidence(root: Path = ROOT, evidence_dir: Path = EVIDENCE_DIR) -> tuple[CheckResult, list[str]]:
    manifests = discover_manifests(evidence_dir)
    all_entries: list[ManifestEntry] = []
    errors: list[str] = []
    for manifest in manifests:
        entries, manifest_errors = parse_manifest(manifest, manifest.read_text())
        all_entries.extend(entries)
        errors.extend(manifest_errors)
    errors.extend(validate_manifest_entries(all_entries, root))
    errors.extend(run_b3sum_check(manifests, root))
    receipts = receipt_paths(all_entries, root, evidence_dir)
    errors.extend(stale_marker_errors(receipts))
    result = CheckResult(
        manifests=len(manifests),
        entries=len(all_entries),
        receipts_scanned=len(receipts),
    )
    return result, errors


def assert_self_tests() -> None:
    with tempfile.TemporaryDirectory(prefix="evidence-manifest-self-test-") as tmp:
        root = Path(tmp)
        evidence = root / "docs" / "evidence"
        evidence.mkdir(parents=True)
        receipt = evidence / "good.receipt.json"
        receipt.write_text('{"required_milestones":["entity_equipment_update"]}\n')
        manifest = evidence / "good.b3"
        digest = subprocess.check_output(["b3sum", str(receipt)], text=True).split()[0]
        manifest.write_text(f"{digest}{MANIFEST_SEPARATOR}docs/evidence/good.receipt.json\n")

        result, errors = check_evidence(root, evidence)
        assert not errors, errors
        assert result.manifests == 1, result
        assert result.entries == 1, result
        assert result.receipts_scanned == 1, result

        stale = evidence / "stale.receipt.json"
        stale.write_text('{"missing_milestones":["equipment_packet_observed"]}\n')
        _, errors = check_evidence(root, evidence)
        assert any("equipment_packet_observed" in error for error in errors), errors
        stale.unlink()

        run_log = evidence / "run.log"
        run_log.write_text("ok\n")
        run_log_digest = subprocess.check_output(["b3sum", str(run_log)], text=True).split()[0]
        log_manifest = evidence / "run-log.b3"
        log_manifest.write_text(f"{run_log_digest}{MANIFEST_SEPARATOR}docs/evidence/run.log\n")
        result, errors = check_evidence(root, evidence)
        assert not errors, errors
        assert result.entries == 2, result

        run_log.unlink()
        _, errors = check_evidence(root, evidence)
        assert any("referenced file missing" in error and "run.log" in error for error in errors), errors
        log_manifest.unlink()

        missing_manifest = evidence / "missing.b3"
        missing_manifest.write_text(
            f"{'0' * BLAKE3_HEX_LENGTH}{MANIFEST_SEPARATOR}docs/evidence/missing.receipt.json\n"
        )
        _, errors = check_evidence(root, evidence)
        assert any("referenced file missing" in error for error in errors), errors

        receipt.write_text('{"required_milestones":["entity_equipment_update"],"changed":true}\n')
        _, errors = check_evidence(root, evidence)
        assert any("b3sum --check failed" in error for error in errors), errors


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("evidence manifest self-test ok")
        return 0
    result, errors = check_evidence()
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print(
        "evidence manifests ok: "
        f"{result.manifests} manifests, {result.entries} entries, "
        f"{result.receipts_scanned} receipts scanned"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
