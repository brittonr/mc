#!/usr/bin/env python3
"""Validate protocol-763 coverage inventory and non-overclaiming gate."""
from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
DOC = ROOT / "docs" / "evidence" / "protocol-763-broad-coverage-ledger-2026-05-28.md"
INVENTORY = ROOT / "docs" / "evidence" / "protocol-763-packet-inventory-2026-05-28.tsv"
VALENCE_PACKETS = ROOT / "valence" / "crates" / "valence_generated" / "extracted" / "packets.json"
STEVENARELLA_763 = ROOT / "stevenarella" / "protocol" / "src" / "protocol" / "versions" / "v1_20_1.rs"

BLAKE3_HEX_LENGTH = 64
BLAKE3_RE = re.compile(rf"`([0-9a-f]{{{BLAKE3_HEX_LENGTH}}})`")
OVERRIDE_CONST_RE = re.compile(r"const\s+([A-Z_]+)_OVERRIDES\s*:[^=]+\s*=\s*&\s*\[(.*?)\];", re.DOTALL)
OVERRIDE_ROW_RE = re.compile(r"\(\s*(0x[0-9a-fA-F]+|\d+)\s*,\s*packet::.*?::internal_ids::([A-Za-z0-9_]+)\s*,?\s*\)", re.DOTALL)
WIRE_ID_WIDTH = 2
PROTOCOL_PACKET_COUNT = 175
FULL_PROTOCOL_NON_CLAIM = "full protocol-763 compatibility remains a non-claim"
FULL_MINECRAFT_NON_CLAIM = "full Minecraft compatibility remains a non-claim"
FALLBACK_ALIAS_REJECTION = "fallback_alias_rejected"
MALFORMED_SHAPE_REJECTION = "malformed_shape_rejected"
FALLBACK_ALIAS_NON_CLAIM = "fallback_alias_non_claim"
REVIEWED_OVERRIDE_NO_SHAPE_CLAIM = "reviewed_override_no_shape_claim"
SHAPE_REVIEW_MISSING = "shape_review_missing"
SCENARIO_BOUNDED = "scenario_bounded"
BROAD_COVERED = "broad_covered"
NON_CLAIM = "non_claim"
NONE = "none"
PROMOTED_COVERAGE_STATUSES = frozenset({BROAD_COVERED})
ALLOWED_MAPPING_STATUSES = frozenset({FALLBACK_ALIAS_NON_CLAIM, REVIEWED_OVERRIDE_NO_SHAPE_CLAIM})
ALLOWED_PARSER_SHAPE_STATUSES = frozenset({SHAPE_REVIEW_MISSING, "parser_shape_reviewed"})
ALLOWED_COVERAGE_STATUSES = frozenset({NON_CLAIM, SCENARIO_BOUNDED, BROAD_COVERED})

INVENTORY_COLUMNS = [
    "state",
    "side",
    "wire_id",
    "valence_packet",
    "stevenarella_mapping_status",
    "stevenarella_internal_id",
    "parser_shape_status",
    "scenario_evidence",
    "coverage_status",
    "owner",
    "next_action",
]

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
]

COVERED_SURFACES = [
    "status_login_play_join",
    "ctf_score_path",
    "inventory_drop_pickup_click_container_block_place",
    "combat_damage_knockback_armor_projectile",
    "survival_break_place_pickup",
    "survival_reference_packet_acceptance",
    "reconnect_flag_state",
]
UNCOVERED_SURFACES = [
    "all_packets_all_states",
    "all_entity_metadata_variants",
    "all_inventory_transactions",
    "all_equipment_permutations",
    "all_biomes_chunks_commands_recipes_advancements",
    "full_survival_compatibility",
    "all_vanilla_combat_parity",
]

SCENARIO_EVIDENCE_BY_PACKET = {
    "GameJoinS2CPacket": "status_login_play_join",
    "KeepAliveS2CPacket": "latency_jitter_tolerance",
    "KeepAliveC2SPacket": "latency_jitter_tolerance",
    "SetPlayerPositionAndRotationS2CPacket": "status_login_play_join",
    "PlayerMoveC2SPacket.PositionAndOnGround": "status_login_play_join",
    "PlayerMoveC2SPacket.Full": "status_login_play_join",
    "PlayerInteractBlockC2SPacket": "inventory_drop_pickup_click_container_block_place",
    "PlayerInteractItemC2SPacket": "inventory_drop_pickup_click_container_block_place",
    "ClickSlotC2SPacket": "inventory_drop_pickup_click_container_block_place",
    "UpdateSelectedSlotC2SPacket": "inventory_drop_pickup_click_container_block_place",
    "ScreenHandlerSlotUpdateS2CPacket": "inventory_drop_pickup_click_container_block_place",
    "InventoryS2CPacket": "inventory_drop_pickup_click_container_block_place",
    "OpenScreenS2CPacket": "inventory_drop_pickup_click_container_block_place",
    "EntityEquipmentUpdateS2CPacket": "combat_damage_knockback_armor_projectile",
    "EntityVelocityUpdateS2CPacket": "combat_damage_knockback_armor_projectile",
    "EntityAttributesS2CPacket": "combat_damage_knockback_armor_projectile",
    "HealthUpdateS2CPacket": "combat_damage_knockback_armor_projectile",
    "PlayerInteractEntityC2SPacket": "combat_damage_knockback_armor_projectile",
    "ClientStatusC2SPacket": "reconnect_flag_state",
    "CombatDeathS2CPacket": "reconnect_flag_state",
    "BundleSplitterPacket": "survival_reference_packet_acceptance",
    "DifficultyS2CPacket": "survival_reference_packet_acceptance",
    "CommandTreeS2CPacket": "survival_reference_packet_acceptance",
    "CustomPayloadS2CPacket": "survival_reference_packet_acceptance",
    "WorldBorderInitializeS2CPacket": "survival_reference_packet_acceptance",
    "WorldEventS2CPacket": "survival_reference_packet_acceptance",
    "LightUpdateS2CPacket": "survival_reference_packet_acceptance",
    "UnlockRecipesS2CPacket": "survival_reference_packet_acceptance",
    "EntitiesDestroyS2CPacket": "survival_reference_packet_acceptance",
    "ChunkDeltaUpdateS2CPacket": "survival_reference_packet_acceptance",
    "ServerMetadataS2CPacket": "survival_reference_packet_acceptance",
    "EntityPassengersSetS2CPacket": "survival_reference_packet_acceptance",
    "TeamS2CPacket": "survival_reference_packet_acceptance",
    "SimulationDistanceS2CPacket": "survival_reference_packet_acceptance",
    "WorldTimeUpdateS2CPacket": "survival_reference_packet_acceptance",
    "PlaySoundS2CPacket": "survival_reference_packet_acceptance",
    "SynchronizeRecipesS2CPacket": "survival_reference_packet_acceptance",
    "CustomPayloadC2SPacket": "survival_reference_packet_acceptance",
}


@dataclass(frozen=True, order=True)
class PacketKey:
    state: str
    side: str
    wire_id: int
    name: str


@dataclass(frozen=True)
class PacketInventoryRow:
    key: PacketKey
    mapping_status: str
    internal_id: str
    parser_shape_status: str
    scenario_evidence: str
    coverage_status: str
    owner: str
    next_action: str


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


HIGH_RISK_PACKET_FAMILY_FIXTURES = [
    MappingFixture(
        packet_family="command_tree_raw",
        mapping_reviewed=True,
        parser_shape_reviewed=True,
        fallback_alias_used=False,
        malformed_shape_accepted=False,
        live_receipt=True,
    ),
    MappingFixture(
        packet_family="chunk_delta_raw",
        mapping_reviewed=True,
        parser_shape_reviewed=True,
        fallback_alias_used=False,
        malformed_shape_accepted=False,
        live_receipt=True,
    ),
    MappingFixture(
        packet_family="recipe_book_raw",
        mapping_reviewed=True,
        parser_shape_reviewed=True,
        fallback_alias_used=False,
        malformed_shape_accepted=False,
        live_receipt=True,
    ),
    MappingFixture(
        packet_family="custom_payload_brand",
        mapping_reviewed=True,
        parser_shape_reviewed=True,
        fallback_alias_used=False,
        malformed_shape_accepted=False,
        live_receipt=True,
    ),
]


def packet_key(packet: dict[str, Any]) -> PacketKey:
    return PacketKey(
        state=str(packet["state"]),
        side=str(packet["side"]),
        wire_id=int(packet["id"]),
        name=str(packet["name"]),
    )


def inventory_identity(key: PacketKey) -> tuple[str, str, str, str]:
    return (key.state, key.side, format_wire_id(key.wire_id), key.name)


def format_wire_id(wire_id: int) -> str:
    return f"0x{wire_id:0{WIRE_ID_WIDTH}x}"


def load_valence_packets() -> list[PacketKey]:
    packets = json.loads(VALENCE_PACKETS.read_text())
    if not isinstance(packets, list):
        raise ValueError("Valence packets JSON is not a list")
    return sorted(packet_key(packet) for packet in packets)


def const_to_packet_scope(const_name: str) -> tuple[str, str] | None:
    match const_name:
        case "PLAY_CLIENTBOUND":
            return ("play", "clientbound")
        case "PLAY_SERVERBOUND":
            return ("play", "serverbound")
        case "LOGIN_CLIENTBOUND":
            return ("login", "clientbound")
        case "LOGIN_SERVERBOUND":
            return ("login", "serverbound")
        case _:
            return None


def load_stevenarella_overrides() -> dict[tuple[str, str, int], str]:
    text = STEVENARELLA_763.read_text()
    overrides: dict[tuple[str, str, int], str] = {}
    for const_name, body in OVERRIDE_CONST_RE.findall(text):
        scope = const_to_packet_scope(const_name)
        if scope is None:
            continue
        state, side = scope
        for wire_id_text, internal_id in OVERRIDE_ROW_RE.findall(body):
            wire_id = int(wire_id_text, 0)
            overrides[(state, side, wire_id)] = internal_id
    return overrides


def build_inventory_rows(packets: list[PacketKey], overrides: dict[tuple[str, str, int], str]) -> list[PacketInventoryRow]:
    rows: list[PacketInventoryRow] = []
    for key in packets:
        internal_id = overrides.get((key.state, key.side, key.wire_id), NONE)
        scenario_evidence = SCENARIO_EVIDENCE_BY_PACKET.get(key.name, NONE)
        has_override = internal_id != NONE
        rows.append(
            PacketInventoryRow(
                key=key,
                mapping_status=REVIEWED_OVERRIDE_NO_SHAPE_CLAIM if has_override else FALLBACK_ALIAS_NON_CLAIM,
                internal_id=internal_id,
                parser_shape_status=SHAPE_REVIEW_MISSING,
                scenario_evidence=scenario_evidence,
                coverage_status=SCENARIO_BOUNDED if scenario_evidence != NONE else NON_CLAIM,
                owner="agent",
                next_action="add_mapping_parser_fixture_and_live_receipt" if not has_override else "add_parser_shape_fixture_before_broad_promotion",
            )
        )
    return rows


def write_inventory(rows: list[PacketInventoryRow], path: Path) -> None:
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=INVENTORY_COLUMNS, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow(
                {
                    "state": row.key.state,
                    "side": row.key.side,
                    "wire_id": format_wire_id(row.key.wire_id),
                    "valence_packet": row.key.name,
                    "stevenarella_mapping_status": row.mapping_status,
                    "stevenarella_internal_id": row.internal_id,
                    "parser_shape_status": row.parser_shape_status,
                    "scenario_evidence": row.scenario_evidence,
                    "coverage_status": row.coverage_status,
                    "owner": row.owner,
                    "next_action": row.next_action,
                }
            )


def load_inventory(path: Path) -> list[PacketInventoryRow]:
    with path.open(newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        if reader.fieldnames != INVENTORY_COLUMNS:
            raise ValueError(f"inventory columns mismatch: {reader.fieldnames}")
        rows: list[PacketInventoryRow] = []
        for row in reader:
            rows.append(
                PacketInventoryRow(
                    key=PacketKey(
                        state=row["state"],
                        side=row["side"],
                        wire_id=int(row["wire_id"], 0),
                        name=row["valence_packet"],
                    ),
                    mapping_status=row["stevenarella_mapping_status"],
                    internal_id=row["stevenarella_internal_id"],
                    parser_shape_status=row["parser_shape_status"],
                    scenario_evidence=row["scenario_evidence"],
                    coverage_status=row["coverage_status"],
                    owner=row["owner"],
                    next_action=row["next_action"],
                )
            )
    return rows


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


def validate_inventory(expected: list[PacketInventoryRow], actual: list[PacketInventoryRow]) -> list[str]:
    issues: list[str] = []
    if len(expected) != PROTOCOL_PACKET_COUNT:
        issues.append(f"expected source packet count changed: {len(expected)} != {PROTOCOL_PACKET_COUNT}")
    expected_by_key = {inventory_identity(row.key): row for row in expected}
    actual_by_key = {inventory_identity(row.key): row for row in actual}
    if len(actual_by_key) != len(actual):
        issues.append("inventory has duplicate packet rows")
    for key, expected_row in expected_by_key.items():
        actual_row = actual_by_key.get(key)
        if actual_row is None:
            issues.append(f"inventory missing packet row: {key}")
            continue
        for field in ["mapping_status", "internal_id", "parser_shape_status", "scenario_evidence", "coverage_status", "owner", "next_action"]:
            if getattr(actual_row, field) != getattr(expected_row, field):
                issues.append(f"inventory {key} field {field} expected {getattr(expected_row, field)!r}, found {getattr(actual_row, field)!r}")
        if actual_row.mapping_status not in ALLOWED_MAPPING_STATUSES:
            issues.append(f"inventory {key} has invalid mapping status: {actual_row.mapping_status}")
        if actual_row.parser_shape_status not in ALLOWED_PARSER_SHAPE_STATUSES:
            issues.append(f"inventory {key} has invalid parser-shape status: {actual_row.parser_shape_status}")
        if actual_row.coverage_status not in ALLOWED_COVERAGE_STATUSES:
            issues.append(f"inventory {key} has invalid coverage status: {actual_row.coverage_status}")
        if not actual_row.owner or actual_row.owner == NONE:
            issues.append(f"inventory {key} missing owner")
        if not actual_row.next_action or actual_row.next_action == NONE:
            issues.append(f"inventory {key} missing next action")
        if actual_row.mapping_status == FALLBACK_ALIAS_NON_CLAIM and actual_row.coverage_status != NON_CLAIM:
            issues.append(f"fallback alias row promoted: {key}")
        if actual_row.coverage_status in PROMOTED_COVERAGE_STATUSES:
            if actual_row.mapping_status == FALLBACK_ALIAS_NON_CLAIM:
                issues.append(f"promoted row uses fallback alias: {key}")
            if actual_row.parser_shape_status == SHAPE_REVIEW_MISSING:
                issues.append(f"promoted row missing parser-shape fixture: {key}")
            if actual_row.scenario_evidence == NONE:
                issues.append(f"promoted row missing receipt evidence: {key}")
    for key in sorted(set(actual_by_key) - set(expected_by_key)):
        issues.append(f"inventory has unknown packet row: {key}")
    return issues


def validate_coverage(matrix_text: str, bundle_text: str, doc_text: str, inventory_rows: list[PacketInventoryRow]) -> list[str]:
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
        FALLBACK_ALIAS_NON_CLAIM,
        REVIEWED_OVERRIDE_NO_SHAPE_CLAIM,
        "protocol-763-packet-inventory-2026-05-28.tsv",
        "175 Valence protocol-763 packet rows",
        "owner: agent",
        "next_action:",
        BROAD_COVERED,
        "command_tree_raw",
        "chunk_delta_raw",
        "recipe_book_raw",
        "custom_payload_brand",
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
    fallback_rows = [row for row in inventory_rows if row.mapping_status == FALLBACK_ALIAS_NON_CLAIM]
    if not fallback_rows:
        issues.append("inventory has no fallback alias non-claim rows")
    promoted_fallbacks = [row.key for row in fallback_rows if row.coverage_status != NON_CLAIM]
    if promoted_fallbacks:
        issues.append(f"fallback aliases promoted: {promoted_fallbacks[:3]}")
    return issues


def fixture_inventory(rows: list[PacketInventoryRow]) -> tuple[str, str, str, list[PacketInventoryRow]]:
    digest = "0" * BLAKE3_HEX_LENGTH
    seam_rows = "\n".join(f"| {seam} | `nix run .#x` | `docs/evidence/x.json` | `docs/evidence/x.md` | `{digest}` | parent `abc1234` | bounded | no broad |" for seam in REQUIRED_SEAMS)
    matrix = "| Seam | Maintained command | Receipt | Evidence doc | BLAKE3 | Landed commits | Scoped claim | Explicit non-claims |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n" + seam_rows + "\n| Broad protocol coverage | Non-claim | gap | next |\n"
    bundle = "\n".join(f"| {seam} | `nix run .#x` | `{digest}` |" for seam in REQUIRED_SEAMS)
    inventory = rows.copy()
    doc = "\n".join(
        [
            FULL_PROTOCOL_NON_CLAIM,
            FULL_MINECRAFT_NON_CLAIM,
            FALLBACK_ALIAS_REJECTION,
            MALFORMED_SHAPE_REJECTION,
            FALLBACK_ALIAS_NON_CLAIM,
            REVIEWED_OVERRIDE_NO_SHAPE_CLAIM,
            "protocol-763-packet-inventory-2026-05-28.tsv",
            "175 Valence protocol-763 packet rows",
            "owner: agent",
            "next_action:",
            BROAD_COVERED,
            "command_tree_raw",
            "chunk_delta_raw",
            "recipe_book_raw",
            "custom_payload_brand",
            *COVERED_SURFACES,
            *UNCOVERED_SURFACES,
            *[f"{seam} {digest}" for seam in REQUIRED_SEAMS],
        ]
    )
    return matrix, bundle, doc, inventory


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

    for fixture in HIGH_RISK_PACKET_FAMILY_FIXTURES:
        decision = evaluate_mapping_fixture(fixture)
        assert decision.promoted, (fixture, decision)

    high_risk_negative = evaluate_mapping_fixture(
        MappingFixture(
            packet_family="command_tree_raw",
            mapping_reviewed=True,
            parser_shape_reviewed=True,
            fallback_alias_used=False,
            malformed_shape_accepted=True,
            live_receipt=True,
        )
    )
    assert not high_risk_negative.promoted and MALFORMED_SHAPE_REJECTION in high_risk_negative.diagnostics, high_risk_negative

    source_rows = build_inventory_rows(load_valence_packets(), load_stevenarella_overrides())
    issues = validate_inventory(source_rows, source_rows)
    assert not issues, issues

    missing_row = source_rows[:-1]
    issues = validate_inventory(source_rows, missing_row)
    assert any("missing packet row" in issue for issue in issues), issues

    bad_fallback = list(source_rows)
    fallback_index = next(index for index, row in enumerate(bad_fallback) if row.mapping_status == FALLBACK_ALIAS_NON_CLAIM)
    fallback_row = bad_fallback[fallback_index]
    bad_fallback[fallback_index] = PacketInventoryRow(
        key=fallback_row.key,
        mapping_status=fallback_row.mapping_status,
        internal_id=fallback_row.internal_id,
        parser_shape_status=fallback_row.parser_shape_status,
        scenario_evidence=fallback_row.scenario_evidence,
        coverage_status=SCENARIO_BOUNDED,
        owner=fallback_row.owner,
        next_action=fallback_row.next_action,
    )
    issues = validate_inventory(source_rows, bad_fallback)
    assert any("field coverage_status" in issue or "fallback alias row promoted" in issue for issue in issues), issues

    bad_promoted = list(source_rows)
    override_index = next(index for index, row in enumerate(bad_promoted) if row.mapping_status == REVIEWED_OVERRIDE_NO_SHAPE_CLAIM)
    override_row = bad_promoted[override_index]
    bad_promoted[override_index] = PacketInventoryRow(
        key=override_row.key,
        mapping_status=override_row.mapping_status,
        internal_id=override_row.internal_id,
        parser_shape_status=override_row.parser_shape_status,
        scenario_evidence=override_row.scenario_evidence,
        coverage_status=BROAD_COVERED,
        owner=override_row.owner,
        next_action=override_row.next_action,
    )
    issues = validate_inventory(source_rows, bad_promoted)
    assert any("promoted row missing parser-shape fixture" in issue or "field coverage_status" in issue for issue in issues), issues

    missing_owner = list(source_rows)
    owner_row = missing_owner[override_index]
    missing_owner[override_index] = PacketInventoryRow(
        key=owner_row.key,
        mapping_status=owner_row.mapping_status,
        internal_id=owner_row.internal_id,
        parser_shape_status=owner_row.parser_shape_status,
        scenario_evidence=owner_row.scenario_evidence,
        coverage_status=owner_row.coverage_status,
        owner=NONE,
        next_action=owner_row.next_action,
    )
    issues = validate_inventory(source_rows, missing_owner)
    assert any("missing owner" in issue or "field owner" in issue for issue in issues), issues

    matrix, bundle, doc, inventory = fixture_inventory(source_rows)
    issues = validate_coverage(matrix, bundle, doc, inventory)
    assert not issues, issues

    overclaim = validate_coverage(matrix.replace("Broad protocol coverage | Non-claim", "Broad protocol coverage | Covered"), bundle, doc, inventory)
    assert any("overclaim" in issue or "blocks broad" in issue for issue in overclaim), overclaim


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run coverage positive and negative fixtures")
    parser.add_argument("--write-inventory", action="store_true", help="regenerate the packet inventory evidence TSV")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    source_rows = build_inventory_rows(load_valence_packets(), load_stevenarella_overrides())
    if args.write_inventory:
        write_inventory(source_rows, INVENTORY)
        print(f"wrote {len(source_rows)} packet rows to {INVENTORY.relative_to(ROOT)}")
        return 0
    if args.self_test:
        assert_self_tests()
        print("protocol coverage ledger self-test ok")
        return 0
    assert_self_tests()
    inventory_rows = load_inventory(INVENTORY)
    issues = validate_inventory(source_rows, inventory_rows)
    issues.extend(validate_coverage(MATRIX.read_text(), BUNDLE.read_text(), DOC.read_text(), inventory_rows))
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"protocol coverage ledger ok: {len(REQUIRED_SEAMS)} bounded seams, {len(inventory_rows)} packet rows, broad claims blocked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
