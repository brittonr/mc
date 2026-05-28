#!/usr/bin/env python3
"""Validate broad protocol-763 coverage ledger and non-overclaiming gate."""
from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
DOC = ROOT / "docs" / "evidence" / "protocol-763-broad-coverage-ledger-2026-05-27.md"

BLAKE3_HEX_LENGTH = 64
BLAKE3_RE = re.compile(rf"`([0-9a-f]{{{BLAKE3_HEX_LENGTH}}})`")
FULL_PROTOCOL_NON_CLAIM = "full protocol-763 compatibility remains a non-claim"
FULL_MINECRAFT_NON_CLAIM = "full Minecraft compatibility remains a non-claim"
FALLBACK_ALIAS_REJECTION = "fallback_alias_rejected"
MALFORMED_SHAPE_REJECTION = "malformed_shape_rejected"

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

COVERED_SURFACES = [
    "status_login_play_join",
    "ctf_score_path",
    "inventory_drop_pickup_click_container_block_place",
    "combat_damage_knockback_armor_projectile",
    "reconnect_flag_state",
]
UNCOVERED_SURFACES = [
    "all_packets_all_states",
    "all_entity_metadata_variants",
    "all_inventory_transactions",
    "all_equipment_permutations",
    "all_biomes_chunks_commands_recipes_advancements",
    "all_vanilla_combat_parity",
]


@dataclass(frozen=True)
class MappingFixture:
    packet_family: str
    mapping_reviewed: bool
    parser_shape_reviewed: bool
    fallback_alias_used: bool
    malformed_shape_accepted: bool
    live_receipt: bool


@dataclass(frozen=True)
class MappingDecision:
    promoted: bool
    diagnostics: tuple[str, ...]


def table_rows(text: str) -> dict[str, str]:
    rows: dict[str, str] = {}
    for line in text.splitlines():
        if not line.startswith("| ") or line.startswith("| ---"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not cells or cells[0] == "Seam":
            continue
        hashes = BLAKE3_RE.findall(line)
        if hashes:
            rows[cells[0]] = hashes[-1]
    return rows


def evaluate_mapping_fixture(fixture: MappingFixture) -> MappingDecision:
    diagnostics: list[str] = []
    if not fixture.mapping_reviewed:
        diagnostics.append("mapping_not_reviewed")
    if not fixture.parser_shape_reviewed:
        diagnostics.append("parser_shape_not_reviewed")
    if fixture.fallback_alias_used:
        diagnostics.append(FALLBACK_ALIAS_REJECTION)
    if fixture.malformed_shape_accepted:
        diagnostics.append(MALFORMED_SHAPE_REJECTION)
    if not fixture.live_receipt:
        diagnostics.append("missing_live_receipt")
    return MappingDecision(promoted=not diagnostics, diagnostics=tuple(diagnostics))


def validate_coverage(matrix_text: str, bundle_text: str, doc_text: str) -> list[str]:
    issues: list[str] = []
    matrix_rows = table_rows(matrix_text)
    bundle_rows = table_rows(bundle_text)
    for seam in REQUIRED_SEAMS:
        if seam not in matrix_rows:
            issues.append(f"matrix missing seam: {seam}")
            continue
        if seam not in bundle_rows:
            issues.append(f"bundle missing seam: {seam}")
            continue
        if matrix_rows[seam] != bundle_rows[seam]:
            issues.append(f"bundle hash mismatch for {seam}: {bundle_rows[seam]} != {matrix_rows[seam]}")
        if seam not in doc_text or matrix_rows[seam] not in doc_text:
            issues.append(f"coverage doc missing seam/hash: {seam}")
    for token in [
        FULL_PROTOCOL_NON_CLAIM,
        FULL_MINECRAFT_NON_CLAIM,
        FALLBACK_ALIAS_REJECTION,
        MALFORMED_SHAPE_REJECTION,
        "owner: agent",
        "next_action:",
        *COVERED_SURFACES,
        *UNCOVERED_SURFACES,
    ]:
        if token not in doc_text:
            issues.append(f"coverage doc missing token: {token}")
    for overclaim in [
        "Broad protocol coverage | Covered",
        "Full Minecraft compatibility | Covered",
        "full protocol-763 compatibility is covered",
    ]:
        if overclaim in matrix_text or overclaim in bundle_text or overclaim in doc_text:
            issues.append(f"forbidden broad overclaim token present: {overclaim}")
    if "Broad protocol coverage | Non-claim" not in matrix_text:
        issues.append("acceptance matrix no longer blocks broad protocol coverage")
    return issues


def assert_self_tests() -> None:
    ok = evaluate_mapping_fixture(
        MappingFixture(
            packet_family="status_login_play_join",
            mapping_reviewed=True,
            parser_shape_reviewed=True,
            fallback_alias_used=False,
            malformed_shape_accepted=False,
            live_receipt=True,
        )
    )
    assert ok.promoted, ok

    fallback = evaluate_mapping_fixture(
        MappingFixture(
            packet_family="entity_metadata",
            mapping_reviewed=True,
            parser_shape_reviewed=True,
            fallback_alias_used=True,
            malformed_shape_accepted=False,
            live_receipt=True,
        )
    )
    assert not fallback.promoted and FALLBACK_ALIAS_REJECTION in fallback.diagnostics, fallback

    malformed = evaluate_mapping_fixture(
        MappingFixture(
            packet_family="chunk_data",
            mapping_reviewed=True,
            parser_shape_reviewed=True,
            fallback_alias_used=False,
            malformed_shape_accepted=True,
            live_receipt=True,
        )
    )
    assert not malformed.promoted and MALFORMED_SHAPE_REJECTION in malformed.diagnostics, malformed

    missing_live = evaluate_mapping_fixture(
        MappingFixture(
            packet_family="commands",
            mapping_reviewed=True,
            parser_shape_reviewed=True,
            fallback_alias_used=False,
            malformed_shape_accepted=False,
            live_receipt=False,
        )
    )
    assert not missing_live.promoted and "missing_live_receipt" in missing_live.diagnostics, missing_live

    digest = "0" * BLAKE3_HEX_LENGTH
    seam_rows = "\n".join(f"| {seam} | `nix run .#x` | `docs/evidence/x.json` | `docs/evidence/x.md` | `{digest}` | parent `abc1234` | bounded | no broad |" for seam in REQUIRED_SEAMS)
    matrix = "| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n" + seam_rows + "\n| Broad protocol coverage | Non-claim | gap | next |\n"
    bundle = "\n".join(f"| {seam} | `nix run .#x` | `{digest}` |" for seam in REQUIRED_SEAMS)
    doc = "\n".join([FULL_PROTOCOL_NON_CLAIM, FULL_MINECRAFT_NON_CLAIM, FALLBACK_ALIAS_REJECTION, MALFORMED_SHAPE_REJECTION, "owner: agent", "next_action:", *COVERED_SURFACES, *UNCOVERED_SURFACES, *[f"{seam} {digest}" for seam in REQUIRED_SEAMS]])
    issues = validate_coverage(matrix, bundle, doc)
    assert not issues, issues

    overclaim = validate_coverage(matrix.replace("Broad protocol coverage | Non-claim", "Broad protocol coverage | Covered"), bundle, doc)
    assert any("overclaim" in issue or "blocks broad" in issue for issue in overclaim), overclaim


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run coverage positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("protocol coverage ledger self-test ok")
        return 0
    assert_self_tests()
    issues = validate_coverage(MATRIX.read_text(), BUNDLE.read_text(), DOC.read_text())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"protocol coverage ledger ok: {len(REQUIRED_SEAMS)} bounded seams, broad claims blocked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
