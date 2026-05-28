#!/usr/bin/env python3
"""Validate bounded protocol-763 death/respawn lifecycle evidence."""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = ROOT / "docs" / "evidence" / "protocol-763-flag-carrier-death-return.matrix.receipt.json"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
EVIDENCE_DOC = ROOT / "docs" / "evidence" / "protocol-763-death-respawn-lifecycle-2026-05-27.md"

PROTOCOL_763 = 763
EXPECTED_SCENARIO = "flag-carrier-death-return"
EXPECTED_DIGEST = "d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4"
EXPECTED_RECEIPT_PATH = "docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json"
EXPECTED_DOC_PATH = "docs/evidence/protocol-763-death-respawn-lifecycle-2026-05-27.md"

CLIENT_TRANSITIONS = [
    "protocol_detected",
    "join_game",
    "render_tick",
    "team_blue",
    "flag_pickup",
    "remote_player_spawn",
    "combat_attack_sent",
    "combat_death_observed",
    "respawn_request_sent",
    "respawn_health_restored",
]
SERVER_TRANSITIONS = [
    "server_client_a_seen",
    "server_client_b_seen",
    "server_flag_pickup",
    "server_flag_carrier_death",
    "server_flag_return",
]
FORBIDDEN_MILESTONES = [
    "flag_capture",
    "score_red_1",
    "score_red_2",
    "score_blue_1",
    "unexpected_flag_capture",
    "unexpected_red_score",
    "unexpected_blue_score",
]
REQUIRED_NON_CLAIMS = [
    "full_ctf_correctness",
    "broad_minecraft_compatibility",
    "unbounded_soak",
    "production_load",
]
REQUIRED_DOC_TEXT = [
    "alive",
    "lethal_damage_observed",
    "dead",
    "respawn_requested",
    "respawned",
    "post_respawn_playable",
    "duplicate death",
    "missing respawn",
    "out-of-order",
    "full death/respawn lifecycle remains a non-claim",
]


@dataclass
class LifecycleEvidence:
    receipt: dict[str, Any]
    matrix_text: str
    bundle_text: str
    doc_text: str


def as_object(value: Any, field: str, issues: list[str]) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    issues.append(f"{field} is not an object")
    return {}


def as_list(value: Any, field: str, issues: list[str]) -> list[Any]:
    if isinstance(value, list):
        return value
    issues.append(f"{field} is not a list")
    return []


def strings(value: Any, field: str, issues: list[str]) -> list[str]:
    raw = as_list(value, field, issues)
    result: list[str] = []
    for item in raw:
        if not isinstance(item, str):
            issues.append(f"{field} contains non-string item: {item!r}")
            continue
        result.append(item)
    return result


def require_equal(issues: list[str], label: str, actual: Any, expected: Any) -> None:
    if actual != expected:
        issues.append(f"{label} expected {expected!r}, found {actual!r}")


def require_true(issues: list[str], label: str, value: Any) -> None:
    if value is not True:
        issues.append(f"{label} expected true, found {value!r}")


def missing_items(actual: list[str], expected: list[str]) -> list[str]:
    actual_set = set(actual)
    return [item for item in expected if item not in actual_set]


def duplicate_items(actual: list[str]) -> list[str]:
    seen: set[str] = set()
    duplicates: list[str] = []
    for item in actual:
        if item in seen and item not in duplicates:
            duplicates.append(item)
        seen.add(item)
    return duplicates


def order_violations(actual: list[str], expected_order: list[str]) -> list[str]:
    positions: dict[str, int] = {}
    for index, item in enumerate(actual):
        positions.setdefault(item, index)
    violations: list[str] = []
    previous = -1
    for item in expected_order:
        if item not in positions:
            continue
        position = positions[item]
        if position < previous:
            violations.append(item)
        previous = position
    return violations


def validate_transition_list(
    actual: list[str], expected: list[str], forbidden: list[str], label: str
) -> list[str]:
    issues: list[str] = []
    missing = missing_items(actual, expected)
    if missing:
        issues.append(f"{label} missing lifecycle milestones: {missing}")
    duplicates = duplicate_items(actual)
    if duplicates:
        issues.append(f"{label} duplicate lifecycle milestones: {duplicates}")
    out_of_order = order_violations(actual, expected)
    if out_of_order:
        issues.append(f"{label} out-of-order lifecycle milestones: {out_of_order}")
    forbidden_present = [item for item in forbidden if item in actual]
    if forbidden_present:
        issues.append(f"{label} forbidden lifecycle milestones present: {forbidden_present}")
    return issues


def validate_lifecycle(evidence: LifecycleEvidence) -> list[str]:
    issues: list[str] = []
    receipt = evidence.receipt
    require_equal(issues, "receipt.status", receipt.get("status"), "pass")
    require_equal(issues, "receipt.mode", receipt.get("mode"), "run")
    require_equal(issues, "receipt.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), "scenario", issues)
    require_equal(issues, "scenario.name", scenario.get("name"), EXPECTED_SCENARIO)
    require_true(issues, "scenario.passed", scenario.get("passed"))
    observed = strings(scenario.get("observed_milestones"), "scenario.observed_milestones", issues)
    missing = strings(scenario.get("missing_milestones"), "scenario.missing_milestones", issues)
    forbidden_matches = strings(scenario.get("forbidden_matches"), "scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"scenario has missing milestones: {missing}")
    if forbidden_matches:
        issues.append(f"scenario has forbidden matches: {forbidden_matches}")
    issues.extend(validate_transition_list(observed, CLIENT_TRANSITIONS, FORBIDDEN_MILESTONES, "client"))

    server = as_object(receipt.get("server"), "server", issues)
    require_equal(issues, "server.protocol", server.get("protocol"), PROTOCOL_763)
    require_true(issues, "server.passed", server.get("passed"))
    server_observed = strings(server.get("observed_milestones"), "server.observed_milestones", issues)
    server_missing = strings(server.get("missing_milestones"), "server.missing_milestones", issues)
    server_forbidden = strings(server.get("forbidden_matches"), "server.forbidden_matches", issues)
    if server_missing:
        issues.append(f"server has missing milestones: {server_missing}")
    if server_forbidden:
        issues.append(f"server has forbidden matches: {server_forbidden}")
    issues.extend(validate_transition_list(server_observed, SERVER_TRANSITIONS, FORBIDDEN_MILESTONES, "server"))

    gameplay = as_object(receipt.get("gameplay_oracles"), "gameplay_oracles", issues)
    non_claims = strings(gameplay.get("non_claims"), "gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"receipt missing non-claim: {item}")

    matrix_required = [EXPECTED_RECEIPT_PATH, EXPECTED_DIGEST, "Flag-carrier death/return"]
    for token in matrix_required:
        if token not in evidence.matrix_text:
            issues.append(f"acceptance matrix missing lifecycle token: {token}")

    bundle_required = ["Flag-carrier death/return", EXPECTED_DIGEST, "No all death/drop/recovery permutations"]
    for token in bundle_required:
        if token not in evidence.bundle_text:
            issues.append(f"current bundle missing lifecycle token: {token}")

    for token in REQUIRED_DOC_TEXT:
        if token not in evidence.doc_text:
            issues.append(f"lifecycle doc missing required text: {token}")
    if EXPECTED_DIGEST not in evidence.doc_text:
        issues.append("lifecycle doc missing live receipt digest")

    return issues


def load_repo_evidence() -> LifecycleEvidence:
    return LifecycleEvidence(
        receipt=json.loads(RECEIPT.read_text()),
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=EVIDENCE_DOC.read_text(),
    )


def valid_fixture() -> LifecycleEvidence:
    receipt = {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {
            "name": EXPECTED_SCENARIO,
            "observed_milestones": CLIENT_TRANSITIONS.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "server": {
            "protocol": PROTOCOL_763,
            "observed_milestones": SERVER_TRANSITIONS.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "gameplay_oracles": {"non_claims": REQUIRED_NON_CLAIMS.copy()},
    }
    matrix = f"Flag-carrier death/return {EXPECTED_RECEIPT_PATH} {EXPECTED_DIGEST}"
    bundle = f"Flag-carrier death/return {EXPECTED_DIGEST} No all death/drop/recovery permutations"
    doc = "\n".join(REQUIRED_DOC_TEXT + [EXPECTED_DIGEST, EXPECTED_DOC_PATH])
    return LifecycleEvidence(receipt=receipt, matrix_text=matrix, bundle_text=bundle, doc_text=doc)


def assert_self_tests() -> None:
    issues = validate_lifecycle(valid_fixture())
    assert not issues, issues

    missing_respawn = valid_fixture()
    missing_respawn.receipt["scenario"]["observed_milestones"].remove("respawn_health_restored")
    issues = validate_lifecycle(missing_respawn)
    assert any("missing lifecycle milestones" in issue for issue in issues), issues

    duplicate_death = valid_fixture()
    duplicate_death.receipt["scenario"]["observed_milestones"].append("combat_death_observed")
    issues = validate_lifecycle(duplicate_death)
    assert any("duplicate lifecycle milestones" in issue for issue in issues), issues

    out_of_order = valid_fixture()
    milestones = out_of_order.receipt["scenario"]["observed_milestones"]
    death_index = milestones.index("combat_death_observed")
    respawn_index = milestones.index("respawn_request_sent")
    milestones[death_index], milestones[respawn_index] = milestones[respawn_index], milestones[death_index]
    issues = validate_lifecycle(out_of_order)
    assert any("out-of-order lifecycle milestones" in issue for issue in issues), issues

    forbidden_score = valid_fixture()
    forbidden_score.receipt["scenario"]["observed_milestones"].append("score_blue_1")
    issues = validate_lifecycle(forbidden_score)
    assert any("forbidden lifecycle milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipt["server"]["protocol"] = 758
    issues = validate_lifecycle(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    missing_doc = valid_fixture()
    missing_doc.doc_text = re.sub(r"dead", "", missing_doc.doc_text, count=1)
    issues = validate_lifecycle(missing_doc)
    assert any("lifecycle doc missing required text" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("death/respawn lifecycle self-test ok")
        return 0

    issues = validate_lifecycle(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("death/respawn lifecycle ok: 1 bounded protocol-763 row")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
