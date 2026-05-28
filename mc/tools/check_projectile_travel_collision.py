#!/usr/bin/env python3
"""Validate bounded protocol-763 projectile travel/collision evidence."""
from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
DOC = ROOT / "docs" / "evidence" / "protocol-763-projectile-travel-collision-2026-05-27.md"
PROTOCOL_763 = 763

COMMON_CLIENT = ["multi_client_count", "protocol_detected", "join_game", "render_tick", "team_red", "remote_player_spawn"]
COMMON_SERVER = ["server_client_a_seen", "server_client_b_seen", "server_projectile_loadout"]
REQUIRED_NON_CLAIMS = [
    "full_ctf_correctness",
    "broad_minecraft_compatibility",
    "unbounded_soak",
    "production_load",
    "full_projectile_physics",
    "all_projectile_weapons",
    "enchantments_or_status_effects",
]
FORBIDDEN_PROJECTILE_MISMATCHES = [
    "missing_projectile_use_accepted",
    "wrong_attacker_projectile_accepted",
    "wrong_target_projectile_accepted",
    "wrong_weapon_projectile_accepted",
    "out_of_order_projectile_accepted",
]


@dataclass(frozen=True)
class ProjectileRow:
    seam: str
    receipt_path: str
    digest: str
    scenario: str
    weapon: str
    target_type: str
    state_sequence: list[str]
    client_milestones: list[str]
    server_milestones: list[str]
    causality_required: bool


ROWS = [
    ProjectileRow(
        seam="Projectile use/loadout rail",
        receipt_path="docs/evidence/protocol-763-roi-03-projectile-hit-2026-05-27.receipt.json",
        digest="22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b",
        scenario="projectile-hit",
        weapon="bow_like_projectile_probe",
        target_type="remote_player_setup_no_damage_claim",
        state_sequence=["client_projectile_use", "client_projectile_swing", "server_projectile_loadout"],
        client_milestones=COMMON_CLIENT + ["projectile_use_sent", "projectile_swing_sent"],
        server_milestones=COMMON_SERVER,
        causality_required=False,
    ),
    ProjectileRow(
        seam="Projectile damage attribution",
        receipt_path="docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json",
        digest="cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529",
        scenario="projectile-damage-attribution",
        weapon="bow_like_projectile_probe",
        target_type="remote_player_victim",
        state_sequence=[
            "client_projectile_use",
            "client_projectile_swing",
            "server_projectile_use",
            "server_projectile_hit",
            "victim_client_damage_update",
        ],
        client_milestones=COMMON_CLIENT + ["team_blue", "projectile_use_sent", "projectile_swing_sent", "projectile_damage_update"],
        server_milestones=COMMON_SERVER + ["server_projectile_use", "server_projectile_hit"],
        causality_required=True,
    ),
]

EXPECTED_CAUSALITY_STEPS = [
    "attacker_client_projectile_use_sent",
    "attacker_client_projectile_swing_sent",
    "server_projectile_use_attacker_victim",
    "server_projectile_hit_attacker_victim_health_delta",
    "victim_client_damage_update",
]
EXPECTED_ATTACKER = "compatbotb"
EXPECTED_VICTIM = "compatbota"


@dataclass
class ProjectileEvidence:
    receipts: dict[str, dict[str, Any]]
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


def order_violations(actual: list[str], expected: list[str]) -> list[str]:
    positions: dict[str, int] = {}
    for index, item in enumerate(actual):
        positions.setdefault(item, index)
    previous = -1
    violations: list[str] = []
    for item in expected:
        if item not in positions:
            continue
        position = positions[item]
        if position < previous:
            violations.append(item)
        previous = position
    return violations


def validate_row(row: ProjectileRow, receipt: dict[str, Any]) -> list[str]:
    issues: list[str] = []
    require_equal(issues, f"{row.seam}.status", receipt.get("status"), "pass")
    require_equal(issues, f"{row.seam}.mode", receipt.get("mode"), "run")
    require_equal(issues, f"{row.seam}.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), f"{row.seam}.scenario", issues)
    require_equal(issues, f"{row.seam}.scenario.name", scenario.get("name"), row.scenario)
    require_true(issues, f"{row.seam}.scenario.passed", scenario.get("passed"))
    observed = string_list(scenario.get("observed_milestones"), f"{row.seam}.scenario.observed_milestones", issues)
    missing = string_list(scenario.get("missing_milestones"), f"{row.seam}.scenario.missing_milestones", issues)
    forbidden = string_list(scenario.get("forbidden_matches"), f"{row.seam}.scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"{row.seam} has missing client milestones: {missing}")
    if forbidden:
        issues.append(f"{row.seam} has forbidden matches: {forbidden}")
    missing_client = missing_items(observed, row.client_milestones)
    if missing_client:
        issues.append(f"{row.seam} missing client projectile milestones: {missing_client}")
    accepted_mismatches = [item for item in FORBIDDEN_PROJECTILE_MISMATCHES if item in observed or item in forbidden]
    if accepted_mismatches:
        issues.append(f"{row.seam} forbidden projectile mismatch accepted: {accepted_mismatches}")

    server = as_object(receipt.get("server"), f"{row.seam}.server", issues)
    require_equal(issues, f"{row.seam}.server.protocol", server.get("protocol"), PROTOCOL_763)
    require_true(issues, f"{row.seam}.server.passed", server.get("passed"))
    server_observed = string_list(server.get("observed_milestones"), f"{row.seam}.server.observed_milestones", issues)
    server_missing = string_list(server.get("missing_milestones"), f"{row.seam}.server.missing_milestones", issues)
    server_forbidden = string_list(server.get("forbidden_matches"), f"{row.seam}.server.forbidden_matches", issues)
    if server_missing:
        issues.append(f"{row.seam} has missing server milestones: {server_missing}")
    if server_forbidden:
        issues.append(f"{row.seam} has server forbidden matches: {server_forbidden}")
    missing_server = missing_items(server_observed, row.server_milestones)
    if missing_server:
        issues.append(f"{row.seam} missing server projectile milestones: {missing_server}")

    gameplay = as_object(receipt.get("gameplay_oracles"), f"{row.seam}.gameplay_oracles", issues)
    non_claims = string_list(gameplay.get("non_claims"), f"{row.seam}.gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"{row.seam} missing non-claim: {item}")

    if row.causality_required:
        causality = as_object(receipt.get("projectile_damage_causality"), f"{row.seam}.projectile_damage_causality", issues)
        require_true(issues, f"{row.seam}.projectile_damage_causality.passed", causality.get("passed"))
        require_equal(issues, f"{row.seam}.projectile_damage_causality.attacker", causality.get("attacker"), EXPECTED_ATTACKER)
        require_equal(issues, f"{row.seam}.projectile_damage_causality.victim", causality.get("victim"), EXPECTED_VICTIM)
        observed_steps = string_list(causality.get("observed_steps"), f"{row.seam}.projectile_damage_causality.observed_steps", issues)
        missing_steps = string_list(causality.get("missing_steps"), f"{row.seam}.projectile_damage_causality.missing_steps", issues)
        order = string_list(causality.get("order_violations"), f"{row.seam}.projectile_damage_causality.order_violations", issues)
        if missing_steps:
            issues.append(f"{row.seam} has missing causality steps: {missing_steps}")
        if order:
            issues.append(f"{row.seam} has causality order violations: {order}")
        missing_causality = missing_items(observed_steps, EXPECTED_CAUSALITY_STEPS)
        if missing_causality:
            issues.append(f"{row.seam} missing causality steps: {missing_causality}")
        out_of_order = order_violations(observed_steps, EXPECTED_CAUSALITY_STEPS)
        if out_of_order:
            issues.append(f"{row.seam} out-of-order causality steps: {out_of_order}")
    return issues


def validate_projectiles(evidence: ProjectileEvidence) -> list[str]:
    issues: list[str] = []
    for row in ROWS:
        receipt = evidence.receipts.get(row.receipt_path)
        if receipt is None:
            issues.append(f"missing projectile receipt: {row.receipt_path}")
            continue
        issues.extend(validate_row(row, receipt))
        for token in [row.seam, row.receipt_path, row.digest]:
            if token not in evidence.matrix_text:
                issues.append(f"acceptance matrix missing token for {row.seam}: {token}")
        for token in [row.seam, row.digest]:
            if token not in evidence.bundle_text:
                issues.append(f"current bundle missing token for {row.seam}: {token}")
        for token in [row.seam, row.weapon, row.target_type, *row.state_sequence, row.digest]:
            if token not in evidence.doc_text:
                issues.append(f"projectile doc missing token for {row.seam}: {token}")
    for token in [
        "projectile travel/collision simulation remains a non-claim",
        "full projectile physics remains a non-claim",
        *FORBIDDEN_PROJECTILE_MISMATCHES,
    ]:
        if token not in evidence.doc_text:
            issues.append(f"projectile doc missing required token: {token}")
    return issues


def load_repo_evidence() -> ProjectileEvidence:
    return ProjectileEvidence(
        receipts={row.receipt_path: json.loads((ROOT / row.receipt_path).read_text()) for row in ROWS},
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=DOC.read_text(),
    )


def valid_receipt(row: ProjectileRow) -> dict[str, Any]:
    receipt: dict[str, Any] = {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {
            "name": row.scenario,
            "observed_milestones": row.client_milestones.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "server": {
            "protocol": PROTOCOL_763,
            "observed_milestones": row.server_milestones.copy(),
            "missing_milestones": [],
            "forbidden_matches": [],
            "passed": True,
        },
        "gameplay_oracles": {"non_claims": REQUIRED_NON_CLAIMS.copy()},
    }
    if row.causality_required:
        receipt["projectile_damage_causality"] = {
            "attacker": EXPECTED_ATTACKER,
            "victim": EXPECTED_VICTIM,
            "observed_steps": EXPECTED_CAUSALITY_STEPS.copy(),
            "missing_steps": [],
            "order_violations": [],
            "passed": True,
        }
    return receipt


def valid_fixture() -> ProjectileEvidence:
    receipts = {row.receipt_path: valid_receipt(row) for row in ROWS}
    matrix = "\n".join(f"{row.seam} {row.receipt_path} {row.digest}" for row in ROWS)
    bundle = "\n".join(f"{row.seam} {row.digest}" for row in ROWS)
    doc = "\n".join(
        [
            "projectile travel/collision simulation remains a non-claim",
            "full projectile physics remains a non-claim",
            *FORBIDDEN_PROJECTILE_MISMATCHES,
        ]
        + [f"{row.seam} {row.weapon} {row.target_type} {' '.join(row.state_sequence)} {row.digest}" for row in ROWS]
    )
    return ProjectileEvidence(receipts=receipts, matrix_text=matrix, bundle_text=bundle, doc_text=doc)


def assert_self_tests() -> None:
    issues = validate_projectiles(valid_fixture())
    assert not issues, issues

    damage_row = ROWS[1]
    missing_client = valid_fixture()
    missing_client.receipts[damage_row.receipt_path]["scenario"]["observed_milestones"].remove("projectile_damage_update")
    issues = validate_projectiles(missing_client)
    assert any("missing client projectile milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipts[damage_row.receipt_path]["server"]["protocol"] = 758
    issues = validate_projectiles(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    wrong_target = valid_fixture()
    wrong_target.receipts[damage_row.receipt_path]["projectile_damage_causality"]["victim"] = "wrong"
    issues = validate_projectiles(wrong_target)
    assert any("victim" in issue for issue in issues), issues

    missing_hit = valid_fixture()
    missing_hit.receipts[damage_row.receipt_path]["projectile_damage_causality"]["observed_steps"].remove("server_projectile_hit_attacker_victim_health_delta")
    issues = validate_projectiles(missing_hit)
    assert any("missing causality steps" in issue for issue in issues), issues

    out_of_order = valid_fixture()
    steps = out_of_order.receipts[damage_row.receipt_path]["projectile_damage_causality"]["observed_steps"]
    steps[0], steps[-1] = steps[-1], steps[0]
    issues = validate_projectiles(out_of_order)
    assert any("out-of-order causality" in issue for issue in issues), issues

    forbidden = valid_fixture()
    forbidden.receipts[damage_row.receipt_path]["scenario"]["observed_milestones"].append("wrong_weapon_projectile_accepted")
    issues = validate_projectiles(forbidden)
    assert any("forbidden projectile mismatch" in issue for issue in issues), issues

    missing_doc = valid_fixture()
    missing_doc.doc_text = missing_doc.doc_text.replace("full projectile physics remains a non-claim", "")
    issues = validate_projectiles(missing_doc)
    assert any("projectile doc missing required token" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("projectile travel/collision self-test ok")
        return 0
    issues = validate_projectiles(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"projectile travel/collision ok: {len(ROWS)} bounded protocol-763 rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
