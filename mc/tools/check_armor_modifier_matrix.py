#!/usr/bin/env python3
"""Validate bounded protocol-763 armor/enchantment/status modifier matrix evidence."""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MATRIX_RECEIPT = ROOT / "docs" / "evidence" / "protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.receipt.json"
LIVE_RECEIPT = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.receipt.json"
LIVE_LOG = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.run.log"
VALENCE_LOG = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.valence.log"
CLIENT_A_LOG = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbota.log"
CLIENT_B_LOG = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbotb.log"
LIVE_MANIFEST = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-live-2026-05-27.b3"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
DOC = ROOT / "docs" / "evidence" / "protocol-763-armor-modifier-matrix-2026-05-27.md"

PROTOCOL_763 = 763
EXPECTED_SCENARIO = "armor-equipment-mitigation"
EXPECTED_SEAM = "Armor equipment mitigation"
MATRIX_DIGEST = "176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765"
MATRIX_RECEIPT_PATH = "docs/evidence/protocol-763-roi-01-armor-equipment-mitigation-2026-05-27.receipt.json"
LIVE_RECEIPT_PATH = "docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.receipt.json"
LIVE_LOG_PATH = "docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.run.log"
VALENCE_LOG_PATH = "docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.valence.log"
CLIENT_A_LOG_PATH = "docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbota.log"
CLIENT_B_LOG_PATH = "docs/evidence/protocol-763-armor-modifier-matrix-live-2026-05-27.client-compatbotb.log"
EXPECTED_CHEST_ITEM = "DiamondChestplate"
EXPECTED_ATTACKER = "compatbota"
EXPECTED_VICTIM = "compatbotb"
REQUIRED_CLIENT_MILESTONES = [
    "multi_client_count",
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_red",
    "team_blue",
    "remote_player_spawn",
    "armor_inventory_slot",
    "combat_attack_sent",
    "combat_health_update",
]
REQUIRED_SERVER_MILESTONES = [
    "server_client_a_seen",
    "server_client_b_seen",
    "server_equipment_state",
    "server_combat_damage",
    "server_armor_mitigation",
]
REQUIRED_NON_CLAIMS = [
    "full_ctf_correctness",
    "broad_minecraft_compatibility",
    "unbounded_soak",
    "production_load",
]
FORBIDDEN_MODIFIER_MISMATCHES = [
    "wrong_loadout_accepted",
    "missing_modifier_attribution_accepted",
    "mismatched_health_delta_accepted",
    "stale_equipment_state_accepted",
    "vanilla_parity_claim_without_oracle",
]
ARMOR_STATE_RE = re.compile(r"armor_equipment_state username=compatbotb slot=chest\s+item=DiamondChestplate")
MITIGATION_RE = re.compile(
    r"combat_armor_mitigation attacker=compatbota victim=compatbotb\s+"
    r"base_damage=(?P<base>[0-9.]+) mitigation=(?P<mitigation>[0-9.]+) final_damage=(?P<final>[0-9.]+)\s+"
    r"chest_item=DiamondChestplate victim_health_before=(?P<before>[0-9.]+) victim_health_after=(?P<after>-?[0-9.]+)"
)


@dataclass
class ArmorEvidence:
    matrix_receipt: dict[str, Any]
    live_receipt: dict[str, Any]
    live_log_text: str
    valence_log_text: str
    client_a_log_text: str
    client_b_log_text: str
    live_manifest_text: str
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


def validate_receipt(receipt: dict[str, Any], label: str) -> list[str]:
    issues: list[str] = []
    require_equal(issues, f"{label}.status", receipt.get("status"), "pass")
    require_equal(issues, f"{label}.mode", receipt.get("mode"), "run")
    require_equal(issues, f"{label}.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), f"{label}.scenario", issues)
    require_equal(issues, f"{label}.scenario.name", scenario.get("name"), EXPECTED_SCENARIO)
    require_true(issues, f"{label}.scenario.passed", scenario.get("passed"))
    observed = string_list(scenario.get("observed_milestones"), f"{label}.scenario.observed_milestones", issues)
    missing = string_list(scenario.get("missing_milestones"), f"{label}.scenario.missing_milestones", issues)
    forbidden = string_list(scenario.get("forbidden_matches"), f"{label}.scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"{label} has missing client milestones: {missing}")
    if forbidden:
        issues.append(f"{label} has forbidden matches: {forbidden}")
    missing_client = missing_items(observed, REQUIRED_CLIENT_MILESTONES)
    if missing_client:
        issues.append(f"{label} missing client modifier milestones: {missing_client}")
    mismatches = [item for item in FORBIDDEN_MODIFIER_MISMATCHES if item in observed or item in forbidden]
    if mismatches:
        issues.append(f"{label} forbidden modifier mismatch accepted: {mismatches}")

    server = as_object(receipt.get("server"), f"{label}.server", issues)
    require_equal(issues, f"{label}.server.protocol", server.get("protocol"), PROTOCOL_763)
    require_true(issues, f"{label}.server.passed", server.get("passed"))
    server_observed = string_list(server.get("observed_milestones"), f"{label}.server.observed_milestones", issues)
    server_missing = string_list(server.get("missing_milestones"), f"{label}.server.missing_milestones", issues)
    server_forbidden = string_list(server.get("forbidden_matches"), f"{label}.server.forbidden_matches", issues)
    if server_missing:
        issues.append(f"{label} has missing server milestones: {server_missing}")
    if server_forbidden:
        issues.append(f"{label} has server forbidden matches: {server_forbidden}")
    missing_server = missing_items(server_observed, REQUIRED_SERVER_MILESTONES)
    if missing_server:
        issues.append(f"{label} missing server modifier milestones: {missing_server}")

    gameplay = as_object(receipt.get("gameplay_oracles"), f"{label}.gameplay_oracles", issues)
    non_claims = string_list(gameplay.get("non_claims"), f"{label}.gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"{label} missing non-claim: {item}")
    return issues


def validate_log(text: str) -> list[str]:
    issues: list[str] = []
    if ARMOR_STATE_RE.search(text) is None:
        issues.append("run log missing DiamondChestplate armor equipment state")
    matches = list(MITIGATION_RE.finditer(text))
    if not matches:
        issues.append("run log missing combat armor mitigation calculation")
        return issues
    for match in matches:
        base = float(match.group("base"))
        mitigation = float(match.group("mitigation"))
        final = float(match.group("final"))
        before = float(match.group("before"))
        after = float(match.group("after"))
        if mitigation <= 0.0:
            issues.append(f"mitigation must be positive, found {mitigation}")
        if final >= base:
            issues.append(f"final damage must be less than base damage, found base={base} final={final}")
        if abs((before - final) - after) > 0.001:
            issues.append(f"health delta mismatch: before={before} final={final} after={after}")
    for marker in FORBIDDEN_MODIFIER_MISMATCHES:
        if marker in text:
            issues.append(f"run log contains forbidden mismatch marker: {marker}")
    return issues


def manifest_has_path(manifest_text: str, path: str) -> bool:
    return any(line.strip().endswith(f"  {path}") for line in manifest_text.splitlines())


def validate_armor(evidence: ArmorEvidence) -> list[str]:
    issues: list[str] = []
    issues.extend(validate_receipt(evidence.matrix_receipt, "matrix_receipt"))
    issues.extend(validate_receipt(evidence.live_receipt, "live_receipt"))
    issues.extend(validate_log(evidence.valence_log_text))
    live_scenario = as_object(evidence.live_receipt.get("scenario"), "live_receipt.scenario", issues)
    live_observed = string_list(live_scenario.get("observed_milestones"), "live_receipt.scenario.observed_milestones", issues)
    if "combat_health_update" not in live_observed:
        issues.append("live receipt missing combat health update observation")
    if "update_health health=18.0" not in evidence.client_b_log_text:
        issues.append("victim client log missing mitigated health observation")
    for path in [LIVE_RECEIPT_PATH, LIVE_LOG_PATH, VALENCE_LOG_PATH, CLIENT_A_LOG_PATH, CLIENT_B_LOG_PATH]:
        if not manifest_has_path(evidence.live_manifest_text, path):
            issues.append(f"live manifest missing path: {path}")
    for token in [EXPECTED_SEAM, MATRIX_RECEIPT_PATH, MATRIX_DIGEST]:
        if token not in evidence.matrix_text:
            issues.append(f"acceptance matrix missing token: {token}")
    for token in [EXPECTED_SEAM, MATRIX_DIGEST]:
        if token not in evidence.bundle_text:
            issues.append(f"current bundle missing token: {token}")
    for token in [
        EXPECTED_SEAM,
        "DiamondChestplate",
        "armor_loadout_chest_only",
        "enchantment_none",
        "status_effect_none",
        "all armor loadouts, enchantments, and status-effect modifiers remain a non-claim",
        *FORBIDDEN_MODIFIER_MISMATCHES,
        MATRIX_DIGEST,
        LIVE_RECEIPT_PATH,
        LIVE_LOG_PATH,
        VALENCE_LOG_PATH,
        CLIENT_A_LOG_PATH,
        CLIENT_B_LOG_PATH,
    ]:
        if token not in evidence.doc_text:
            issues.append(f"modifier doc missing token: {token}")
    return issues


def load_repo_evidence() -> ArmorEvidence:
    return ArmorEvidence(
        matrix_receipt=json.loads(MATRIX_RECEIPT.read_text()),
        live_receipt=json.loads(LIVE_RECEIPT.read_text()),
        live_log_text=LIVE_LOG.read_text(),
        valence_log_text=VALENCE_LOG.read_text(),
        client_a_log_text=CLIENT_A_LOG.read_text(),
        client_b_log_text=CLIENT_B_LOG.read_text(),
        live_manifest_text=LIVE_MANIFEST.read_text(),
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=DOC.read_text(),
    )


def valid_receipt() -> dict[str, Any]:
    return {
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


def valid_fixture() -> ArmorEvidence:
    log = (
        "MC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate\n"
        "MC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb "
        "base_damage=4.0 mitigation=1.0 final_damage=3.0 chest_item=DiamondChestplate "
        "victim_health_before=20.0 victim_health_after=17.0\n"
    )
    manifest = f"0  {LIVE_RECEIPT_PATH}\n1  {LIVE_LOG_PATH}\n2  {VALENCE_LOG_PATH}\n3  {CLIENT_A_LOG_PATH}\n4  {CLIENT_B_LOG_PATH}\n"
    matrix = f"{EXPECTED_SEAM} {MATRIX_RECEIPT_PATH} {MATRIX_DIGEST}"
    bundle = f"{EXPECTED_SEAM} {MATRIX_DIGEST}"
    doc = " ".join([
        EXPECTED_SEAM,
        "DiamondChestplate",
        "armor_loadout_chest_only",
        "enchantment_none",
        "status_effect_none",
        "all armor loadouts, enchantments, and status-effect modifiers remain a non-claim",
        *FORBIDDEN_MODIFIER_MISMATCHES,
        MATRIX_DIGEST,
        LIVE_RECEIPT_PATH,
        LIVE_LOG_PATH,
        VALENCE_LOG_PATH,
        CLIENT_A_LOG_PATH,
        CLIENT_B_LOG_PATH,
    ])
    return ArmorEvidence(
        matrix_receipt=valid_receipt(),
        live_receipt=valid_receipt(),
        live_log_text="runner log",
        valence_log_text=log,
        client_a_log_text="attacker log",
        client_b_log_text="update_health health=18.0",
        live_manifest_text=manifest,
        matrix_text=matrix,
        bundle_text=bundle,
        doc_text=doc,
    )


def assert_self_tests() -> None:
    issues = validate_armor(valid_fixture())
    assert not issues, issues

    missing_client = valid_fixture()
    missing_client.live_receipt["scenario"]["observed_milestones"].remove("armor_inventory_slot")
    issues = validate_armor(missing_client)
    assert any("missing client modifier milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.live_receipt["server"]["protocol"] = 758
    issues = validate_armor(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    wrong_loadout = valid_fixture()
    wrong_loadout.valence_log_text = wrong_loadout.valence_log_text.replace("DiamondChestplate", "LeatherChestplate")
    issues = validate_armor(wrong_loadout)
    assert any("armor equipment state" in issue for issue in issues), issues

    missing_mitigation = valid_fixture()
    missing_mitigation.valence_log_text = missing_mitigation.valence_log_text.replace("combat_armor_mitigation", "combat_damage")
    issues = validate_armor(missing_mitigation)
    assert any("missing combat armor mitigation" in issue for issue in issues), issues

    health_mismatch = valid_fixture()
    health_mismatch.valence_log_text = health_mismatch.valence_log_text.replace("victim_health_after=17.0", "victim_health_after=19.0")
    issues = validate_armor(health_mismatch)
    assert any("health delta mismatch" in issue for issue in issues), issues

    parity_claim = valid_fixture()
    parity_claim.live_receipt["scenario"]["observed_milestones"].append("vanilla_parity_claim_without_oracle")
    issues = validate_armor(parity_claim)
    assert any("forbidden modifier mismatch" in issue for issue in issues), issues

    missing_manifest = valid_fixture()
    missing_manifest.live_manifest_text = ""
    issues = validate_armor(missing_manifest)
    assert any("live manifest missing path" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("armor modifier matrix self-test ok")
        return 0
    issues = validate_armor(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("armor modifier matrix ok: 1 bounded protocol-763 row")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
