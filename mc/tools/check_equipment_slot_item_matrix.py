#!/usr/bin/env python3
"""Validate bounded protocol-763 equipment slot/item matrix evidence."""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = ROOT / "docs" / "evidence" / "protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json"
RUN_LOG = ROOT / "docs" / "evidence" / "protocol-763-roi-02-equipment-update-observation-2026-05-27.run.log"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
DOC = ROOT / "docs" / "evidence" / "protocol-763-equipment-slot-item-matrix-2026-05-27.md"

PROTOCOL_763 = 763
EXPECTED_SCENARIO = "equipment-update-observation"
EXPECTED_SEAM = "Equipment update observation"
EXPECTED_RECEIPT_PATH = "docs/evidence/protocol-763-roi-02-equipment-update-observation-2026-05-27.receipt.json"
EXPECTED_DIGEST = "8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d"
EXPECTED_ENTITY_ID = "4"
EXPECTED_SLOT = "slot4"
EXPECTED_ITEM_ID = "829"
EXPECTED_COUNT = "1"
EXPECTED_LOG_PATTERN = re.compile(
    rf"equipment_probe_entity_equipment entity_id={EXPECTED_ENTITY_ID} entries=1 slots={EXPECTED_SLOT}:id={EXPECTED_ITEM_ID}:count={EXPECTED_COUNT}"
)
EXPECTED_SPAWN_PATTERN = re.compile(rf"remote_player_spawn entity_id={EXPECTED_ENTITY_ID}\b")
REQUIRED_CLIENT_MILESTONES = [
    "multi_client_count",
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "team_blue",
    "remote_player_spawn",
    "entity_equipment_update",
]
REQUIRED_SERVER_MILESTONES = [
    "server_client_a_seen",
    "server_client_b_seen",
    "server_equipment_update_state",
]
REQUIRED_NON_CLAIMS = [
    "full_ctf_correctness",
    "broad_minecraft_compatibility",
    "unbounded_soak",
    "production_load",
]
FORBIDDEN_EQUIPMENT_MISMATCHES = [
    "wrong_entity_equipment_accepted",
    "wrong_slot_equipment_accepted",
    "wrong_item_equipment_accepted",
    "duplicate_equipment_update_accepted",
    "stale_equipment_update_accepted",
]


@dataclass
class EquipmentEvidence:
    receipt: dict[str, Any]
    log_text: str
    matrix_text: str
    bundle_text: str
    doc_text: str


def as_object(value: Any, field: str, issues: list[str]) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    issues.append(f"{field} is not an object")
    return {}


def string_list(value: Any, field: str, issues: list[str]) -> list[str]:
    if not isinstance(value, list):
        issues.append(f"{field} is not a list")
        return []
    result: list[str] = []
    for item in value:
        if not isinstance(item, str):
            issues.append(f"{field} contains non-string item: {item!r}")
            continue
        result.append(item)
    return result


def require_equal(issues: list[str], label: str, actual: Any, expected: Any) -> None:
    if actual != expected:
        issues.append(f"{label} expected {expected!r}, found {actual!r}")


def require_true(issues: list[str], label: str, actual: Any) -> None:
    if actual is not True:
        issues.append(f"{label} expected true, found {actual!r}")


def missing_items(actual: list[str], expected: list[str]) -> list[str]:
    actual_set = set(actual)
    return [item for item in expected if item not in actual_set]


def duplicates(actual: list[str]) -> list[str]:
    seen: set[str] = set()
    repeated: list[str] = []
    for item in actual:
        if item in seen and item not in repeated:
            repeated.append(item)
        seen.add(item)
    return repeated


def validate_equipment(evidence: EquipmentEvidence) -> list[str]:
    issues: list[str] = []
    receipt = evidence.receipt
    require_equal(issues, "receipt.status", receipt.get("status"), "pass")
    require_equal(issues, "receipt.mode", receipt.get("mode"), "run")
    require_equal(issues, "receipt.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), "scenario", issues)
    require_equal(issues, "scenario.name", scenario.get("name"), EXPECTED_SCENARIO)
    require_true(issues, "scenario.passed", scenario.get("passed"))
    observed = string_list(scenario.get("observed_milestones"), "scenario.observed_milestones", issues)
    missing = string_list(scenario.get("missing_milestones"), "scenario.missing_milestones", issues)
    forbidden = string_list(scenario.get("forbidden_matches"), "scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"scenario has missing milestones: {missing}")
    if forbidden:
        issues.append(f"scenario has forbidden matches: {forbidden}")
    missing_client = missing_items(observed, REQUIRED_CLIENT_MILESTONES)
    if missing_client:
        issues.append(f"missing client equipment milestones: {missing_client}")
    repeated_client = duplicates(observed)
    if repeated_client:
        issues.append(f"duplicate client equipment milestones: {repeated_client}")
    mismatches = [item for item in FORBIDDEN_EQUIPMENT_MISMATCHES if item in observed or item in forbidden]
    if mismatches:
        issues.append(f"forbidden equipment mismatch accepted: {mismatches}")

    server = as_object(receipt.get("server"), "server", issues)
    require_equal(issues, "server.protocol", server.get("protocol"), PROTOCOL_763)
    require_true(issues, "server.passed", server.get("passed"))
    server_observed = string_list(server.get("observed_milestones"), "server.observed_milestones", issues)
    server_missing = string_list(server.get("missing_milestones"), "server.missing_milestones", issues)
    server_forbidden = string_list(server.get("forbidden_matches"), "server.forbidden_matches", issues)
    if server_missing:
        issues.append(f"server has missing milestones: {server_missing}")
    if server_forbidden:
        issues.append(f"server has forbidden matches: {server_forbidden}")
    missing_server = missing_items(server_observed, REQUIRED_SERVER_MILESTONES)
    if missing_server:
        issues.append(f"missing server equipment milestones: {missing_server}")

    gameplay = as_object(receipt.get("gameplay_oracles"), "gameplay_oracles", issues)
    non_claims = string_list(gameplay.get("non_claims"), "gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"receipt missing non-claim: {item}")

    if EXPECTED_SPAWN_PATTERN.search(evidence.log_text) is None:
        issues.append(f"run log missing remote spawn for entity {EXPECTED_ENTITY_ID}")
    equipment_matches = EXPECTED_LOG_PATTERN.findall(evidence.log_text)
    if len(equipment_matches) != 1:
        issues.append(f"run log expected exactly one matching equipment update, found {len(equipment_matches)}")
    for marker in FORBIDDEN_EQUIPMENT_MISMATCHES:
        if marker in evidence.log_text:
            issues.append(f"run log contains forbidden mismatch marker: {marker}")

    for token in [EXPECTED_SEAM, EXPECTED_RECEIPT_PATH, EXPECTED_DIGEST]:
        if token not in evidence.matrix_text:
            issues.append(f"acceptance matrix missing token: {token}")
    for token in [EXPECTED_SEAM, EXPECTED_DIGEST]:
        if token not in evidence.bundle_text:
            issues.append(f"current bundle missing token: {token}")
    for token in [
        EXPECTED_SEAM,
        "main_hand_remote_entity",
        EXPECTED_SLOT,
        EXPECTED_ITEM_ID,
        "non_empty_update",
        "all equipment slots/items/permutations remain a non-claim",
        *FORBIDDEN_EQUIPMENT_MISMATCHES,
        EXPECTED_DIGEST,
    ]:
        if token not in evidence.doc_text:
            issues.append(f"equipment doc missing token: {token}")
    return issues


def load_repo_evidence() -> EquipmentEvidence:
    return EquipmentEvidence(
        receipt=json.loads(RECEIPT.read_text()),
        log_text=RUN_LOG.read_text(),
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=DOC.read_text(),
    )


def valid_fixture() -> EquipmentEvidence:
    receipt = {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {
            "name": EXPECTED_SCENARIO,
            "observed_milestones": REQUIRED_CLIENT_MILESTONES.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "server": {
            "protocol": PROTOCOL_763,
            "observed_milestones": REQUIRED_SERVER_MILESTONES.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "gameplay_oracles": {"non_claims": REQUIRED_NON_CLAIMS.copy()},
    }
    log_text = (
        f"MC-COMPAT-MILESTONE remote_player_spawn entity_id={EXPECTED_ENTITY_ID}\n"
        f"MC-COMPAT-MILESTONE equipment_probe_entity_equipment entity_id={EXPECTED_ENTITY_ID} entries=1 slots={EXPECTED_SLOT}:id={EXPECTED_ITEM_ID}:count={EXPECTED_COUNT}\n"
    )
    matrix = f"{EXPECTED_SEAM} {EXPECTED_RECEIPT_PATH} {EXPECTED_DIGEST}"
    bundle = f"{EXPECTED_SEAM} {EXPECTED_DIGEST}"
    doc = " ".join([
        EXPECTED_SEAM,
        "main_hand_remote_entity",
        EXPECTED_SLOT,
        EXPECTED_ITEM_ID,
        "non_empty_update",
        "all equipment slots/items/permutations remain a non-claim",
        *FORBIDDEN_EQUIPMENT_MISMATCHES,
        EXPECTED_DIGEST,
    ])
    return EquipmentEvidence(receipt=receipt, log_text=log_text, matrix_text=matrix, bundle_text=bundle, doc_text=doc)


def assert_self_tests() -> None:
    issues = validate_equipment(valid_fixture())
    assert not issues, issues

    missing_client = valid_fixture()
    missing_client.receipt["scenario"]["observed_milestones"].remove("entity_equipment_update")
    issues = validate_equipment(missing_client)
    assert any("missing client equipment milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipt["server"]["protocol"] = 758
    issues = validate_equipment(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    wrong_entity = valid_fixture()
    wrong_entity.log_text = wrong_entity.log_text.replace(f"entity_id={EXPECTED_ENTITY_ID}", "entity_id=999", 1)
    issues = validate_equipment(wrong_entity)
    assert any("missing remote spawn" in issue for issue in issues), issues

    wrong_slot = valid_fixture()
    wrong_slot.log_text = wrong_slot.log_text.replace(EXPECTED_SLOT, "slot5")
    issues = validate_equipment(wrong_slot)
    assert any("matching equipment update" in issue for issue in issues), issues

    duplicate_update = valid_fixture()
    duplicate_update.log_text += duplicate_update.log_text.splitlines()[-1] + "\n"
    issues = validate_equipment(duplicate_update)
    assert any("exactly one matching equipment update" in issue for issue in issues), issues

    mismatch_marker = valid_fixture()
    mismatch_marker.receipt["scenario"]["observed_milestones"].append("wrong_item_equipment_accepted")
    issues = validate_equipment(mismatch_marker)
    assert any("forbidden equipment mismatch" in issue for issue in issues), issues

    missing_doc = valid_fixture()
    missing_doc.doc_text = missing_doc.doc_text.replace("all equipment slots/items/permutations remain a non-claim", "")
    issues = validate_equipment(missing_doc)
    assert any("equipment doc missing token" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("equipment slot/item matrix self-test ok")
        return 0
    issues = validate_equipment(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("equipment slot/item matrix ok: 1 bounded protocol-763 row")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
