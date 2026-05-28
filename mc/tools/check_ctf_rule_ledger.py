#!/usr/bin/env python3
"""Validate bounded protocol-763 CTF rule ledger evidence."""
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
DOC = ROOT / "docs" / "evidence" / "protocol-763-ctf-rule-ledger-2026-05-27.md"
RED_BLUE_ORACLE = ROOT / "docs" / "evidence" / "protocol-763-red-blue-soak-historical-oracle-2026-05-27.md"

PROTOCOL_763 = 763
FULL_CTF_NON_CLAIM = "full CTF correctness remains a non-claim"
REQUIRED_NON_CLAIMS = ["full_ctf_correctness", "broad_minecraft_compatibility", "unbounded_soak", "production_load"]
FORBIDDEN_RULE_BREAKS = [
    "unexpected_flag_capture",
    "unexpected_flag_capture_milestone",
    "unexpected_red_score",
    "unexpected_blue_score",
    "invalid_pickup_accepted",
    "invalid_return_accepted",
]


@dataclass(frozen=True)
class RuleRow:
    rule: str
    seam: str
    receipt_path: str | None
    digest: str
    scenario: str | None
    client_milestones: list[str]
    server_milestones: list[str]
    forbidden_absent: list[str]
    historical_oracle: bool


ROWS = [
    RuleRow(
        rule="score_capture_red_blue_bounded",
        seam="RED/BLUE scoring soak",
        receipt_path=None,
        digest="b7c861f27ef7ceaf94705a74a5459d3f9df625dada4b14d8715ba8e9c5d921de",
        scenario=None,
        client_milestones=["flag_pickup", "flag_capture", "score_red_1", "score_blue_1"],
        server_milestones=["server_score_path"],
        forbidden_absent=[],
        historical_oracle=True,
    ),
    RuleRow(
        rule="flag_carrier_death_returns_flag_without_score",
        seam="Flag-carrier death/return",
        receipt_path="docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json",
        digest="d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4",
        scenario="flag-carrier-death-return",
        client_milestones=["flag_pickup", "combat_attack_sent", "combat_death_observed", "respawn_request_sent", "respawn_health_restored"],
        server_milestones=["server_flag_pickup", "server_flag_carrier_death", "server_flag_return"],
        forbidden_absent=["unexpected_flag_capture", "unexpected_red_score", "unexpected_blue_score"],
        historical_oracle=False,
    ),
    RuleRow(
        rule="disconnect_returns_flag_and_reconnect_state_coherent",
        seam="Reconnect flag-state",
        receipt_path="docs/evidence/protocol-763-reconnect-flag-state.matrix.receipt.json",
        digest="4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3",
        scenario="reconnect-flag-state",
        client_milestones=["flag_pickup", "reconnect_session"],
        server_milestones=["server_flag_pickup", "server_flag_disconnect_return", "server_reconnect_state_coherent"],
        forbidden_absent=["unexpected_flag_capture", "unexpected_red_score", "unexpected_blue_score"],
        historical_oracle=False,
    ),
]


@dataclass
class CtfEvidence:
    receipts: dict[str, dict[str, Any]]
    matrix_text: str
    bundle_text: str
    doc_text: str
    oracle_text: str


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


def validate_receipt(row: RuleRow, receipt: dict[str, Any]) -> list[str]:
    issues: list[str] = []
    require_equal(issues, f"{row.rule}.status", receipt.get("status"), "pass")
    require_equal(issues, f"{row.rule}.mode", receipt.get("mode"), "run")
    require_equal(issues, f"{row.rule}.dry_run", receipt.get("dry_run"), False)
    scenario = as_object(receipt.get("scenario"), f"{row.rule}.scenario", issues)
    require_equal(issues, f"{row.rule}.scenario.name", scenario.get("name"), row.scenario)
    require_true(issues, f"{row.rule}.scenario.passed", scenario.get("passed"))
    observed = string_list(scenario.get("observed_milestones"), f"{row.rule}.scenario.observed_milestones", issues)
    missing = string_list(scenario.get("missing_milestones"), f"{row.rule}.scenario.missing_milestones", issues)
    forbidden = string_list(scenario.get("forbidden_matches"), f"{row.rule}.scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"{row.rule} has missing client milestones: {missing}")
    if forbidden:
        issues.append(f"{row.rule} has forbidden matches: {forbidden}")
    missing_client = missing_items(observed, row.client_milestones)
    if missing_client:
        issues.append(f"{row.rule} missing client rule milestones: {missing_client}")
    accepted_breaks = [item for item in FORBIDDEN_RULE_BREAKS if item in observed or item in forbidden]
    if accepted_breaks:
        issues.append(f"{row.rule} forbidden rule break accepted: {accepted_breaks}")

    server = as_object(receipt.get("server"), f"{row.rule}.server", issues)
    require_equal(issues, f"{row.rule}.server.protocol", server.get("protocol"), PROTOCOL_763)
    require_true(issues, f"{row.rule}.server.passed", server.get("passed"))
    server_observed = string_list(server.get("observed_milestones"), f"{row.rule}.server.observed_milestones", issues)
    server_missing = string_list(server.get("missing_milestones"), f"{row.rule}.server.missing_milestones", issues)
    server_forbidden = string_list(server.get("forbidden_matches"), f"{row.rule}.server.forbidden_matches", issues)
    if server_missing:
        issues.append(f"{row.rule} has missing server milestones: {server_missing}")
    if server_forbidden:
        issues.append(f"{row.rule} has server forbidden matches: {server_forbidden}")
    missing_server = missing_items(server_observed, row.server_milestones)
    if missing_server:
        issues.append(f"{row.rule} missing server rule milestones: {missing_server}")

    gameplay = as_object(receipt.get("gameplay_oracles"), f"{row.rule}.gameplay_oracles", issues)
    non_claims = string_list(gameplay.get("non_claims"), f"{row.rule}.gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"{row.rule} missing non-claim: {item}")
    return issues


def validate_ctf(evidence: CtfEvidence) -> list[str]:
    issues: list[str] = []
    for row in ROWS:
        for token in [row.seam, row.digest]:
            if token not in evidence.matrix_text:
                issues.append(f"acceptance matrix missing token for {row.rule}: {token}")
            if token not in evidence.bundle_text:
                issues.append(f"current bundle missing token for {row.rule}: {token}")
        for token in [row.rule, row.seam, row.digest, *row.client_milestones, *row.server_milestones, *row.forbidden_absent]:
            if token not in evidence.doc_text:
                issues.append(f"ctf rule doc missing token for {row.rule}: {token}")
        if row.historical_oracle:
            for token in ["## Question", "## Inspected evidence", "## Decision", row.digest]:
                if token not in evidence.oracle_text:
                    issues.append(f"historical scoring oracle missing token: {token}")
            continue
        if row.receipt_path is None:
            issues.append(f"non-historical row lacks receipt path: {row.rule}")
            continue
        receipt = evidence.receipts.get(row.receipt_path)
        if receipt is None:
            issues.append(f"missing CTF receipt: {row.receipt_path}")
            continue
        issues.extend(validate_receipt(row, receipt))
    for token in [FULL_CTF_NON_CLAIM, "invalid_pickup_accepted", "invalid_return_accepted"]:
        if token not in evidence.doc_text:
            issues.append(f"ctf rule doc missing required token: {token}")
    if "Full CTF correctness | Non-claim" not in evidence.matrix_text:
        issues.append("acceptance matrix no longer records full CTF correctness as non-claim")
    return issues


def load_repo_evidence() -> CtfEvidence:
    receipts: dict[str, dict[str, Any]] = {}
    for row in ROWS:
        if row.receipt_path is not None:
            receipts[row.receipt_path] = json.loads((ROOT / row.receipt_path).read_text())
    return CtfEvidence(
        receipts=receipts,
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=DOC.read_text(),
        oracle_text=RED_BLUE_ORACLE.read_text(),
    )


def valid_receipt(row: RuleRow) -> dict[str, Any]:
    return {
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


def valid_fixture() -> CtfEvidence:
    receipts = {row.receipt_path: valid_receipt(row) for row in ROWS if row.receipt_path is not None}
    matrix = "Full CTF correctness | Non-claim\n" + "\n".join(f"{row.seam} {row.digest}" for row in ROWS)
    bundle = "\n".join(f"{row.seam} {row.digest}" for row in ROWS)
    doc = "\n".join(
        [FULL_CTF_NON_CLAIM, "invalid_pickup_accepted", "invalid_return_accepted"]
        + [f"{row.rule} {row.seam} {row.digest} {' '.join(row.client_milestones)} {' '.join(row.server_milestones)} {' '.join(row.forbidden_absent)}" for row in ROWS]
    )
    oracle = f"## Question\n## Inspected evidence\n## Decision\n{ROWS[0].digest}"
    return CtfEvidence(receipts=receipts, matrix_text=matrix, bundle_text=bundle, doc_text=doc, oracle_text=oracle)


def assert_self_tests() -> None:
    issues = validate_ctf(valid_fixture())
    assert not issues, issues

    death_row = ROWS[1]
    missing_server = valid_fixture()
    missing_server.receipts[death_row.receipt_path]["server"]["observed_milestones"].remove("server_flag_return")
    issues = validate_ctf(missing_server)
    assert any("missing server rule milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipts[death_row.receipt_path]["server"]["protocol"] = 758
    issues = validate_ctf(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    forbidden_score = valid_fixture()
    forbidden_score.receipts[death_row.receipt_path]["scenario"]["observed_milestones"].append("unexpected_blue_score")
    issues = validate_ctf(forbidden_score)
    assert any("forbidden rule break" in issue for issue in issues), issues

    missing_oracle = valid_fixture()
    missing_oracle.oracle_text = ""
    issues = validate_ctf(missing_oracle)
    assert any("historical scoring oracle" in issue for issue in issues), issues

    overclaim = valid_fixture()
    overclaim.matrix_text = overclaim.matrix_text.replace("Full CTF correctness | Non-claim", "Full CTF correctness | Covered")
    issues = validate_ctf(overclaim)
    assert any("full CTF correctness" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative CTF rule fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("ctf rule ledger self-test ok")
        return 0
    issues = validate_ctf(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"ctf rule ledger ok: {len(ROWS)} bounded protocol-763 clusters")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
