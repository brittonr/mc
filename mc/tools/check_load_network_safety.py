#!/usr/bin/env python3
"""Validate protocol-763 load/network safety live receipt evidence."""
from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
RECEIPT = ROOT / "docs" / "evidence" / "protocol-763-load-network-safety-live-2026-05-27.receipt.json"
DOC = ROOT / "docs" / "evidence" / "protocol-763-load-network-safety-2026-05-27.md"
TASKS = ROOT / "cairn" / "archive" / "2026-05-27-prove-production-load-network-safety" / "tasks.md"

PROTOCOL_763 = 763
MAX_LOCAL_CLIENTS = 2
MAX_DURATION_SECS = 600
EXPECTED_RECEIPT_DIGEST = "62aba060f0bc082d08487c5adf83bfd417742d3711fe4295066e44e7668a25b2"
EXPECTED_LOG_DIGEST = "8087221d20405d63e5cd81ffc1afbcdfd8b118b157dbe38e5e1752384e97bce7"
REQUIRED_FALSE_CLAIMS = [
    "claims_public_server_safety",
    "claims_production_readiness",
    "claims_unbounded_soak",
    "claims_unbounded_reconnect",
    "claims_wan_safety",
    "claims_adversarial_network_safety",
]


@dataclass
class LoadSafetyEvidence:
    receipt: dict[str, Any]
    doc_text: str
    tasks_text: str


def as_object(value: Any, field: str, issues: list[str]) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    issues.append(f"{field} is not an object")
    return {}


def require_equal(issues: list[str], label: str, actual: Any, expected: Any) -> None:
    if actual != expected:
        issues.append(f"{label} expected {expected!r}, found {actual!r}")


def require_true(issues: list[str], label: str, actual: Any) -> None:
    if actual is not True:
        issues.append(f"{label} expected true, found {actual!r}")


def require_false(issues: list[str], label: str, actual: Any) -> None:
    if actual is not False:
        issues.append(f"{label} expected false, found {actual!r}")


def validate_load_safety(evidence: LoadSafetyEvidence) -> list[str]:
    issues: list[str] = []
    receipt = evidence.receipt
    require_equal(issues, "receipt.status", receipt.get("status"), "pass")
    require_equal(issues, "receipt.mode", receipt.get("mode"), "run")
    require_equal(issues, "receipt.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), "scenario", issues)
    require_equal(issues, "scenario.name", scenario.get("name"), "valence-compat-bot-probe")
    require_true(issues, "scenario.passed", scenario.get("passed"))

    server = as_object(receipt.get("server"), "server", issues)
    require_equal(issues, "server.protocol", server.get("protocol"), PROTOCOL_763)
    require_equal(issues, "server.version", server.get("version"), "1.20.1")
    require_true(issues, "server.passed", server.get("passed"))

    safety = as_object(receipt.get("load_network_safety"), "load_network_safety", issues)
    require_equal(issues, "load_network_safety.target_scope", safety.get("target_scope"), "owned-local-loopback")
    require_true(issues, "load_network_safety.owned_local_target", safety.get("owned_local_target"))
    require_true(issues, "load_network_safety.authorized", safety.get("authorized"))
    require_equal(issues, "load_network_safety.max_clients", safety.get("max_clients"), MAX_LOCAL_CLIENTS)
    require_equal(issues, "load_network_safety.max_duration_secs", safety.get("max_duration_secs"), MAX_DURATION_SECS)
    require_equal(issues, "load_network_safety.bound_violations", safety.get("bound_violations"), [])
    require_equal(issues, "load_network_safety.missing_fields", safety.get("missing_fields"), [])
    require_true(issues, "load_network_safety.telemetry_present", safety.get("telemetry_present"))
    require_true(issues, "load_network_safety.live_receipt", safety.get("live_receipt"))
    require_true(issues, "load_network_safety.preflight_passed", safety.get("preflight_passed"))
    require_true(issues, "load_network_safety.promotion_ready", safety.get("promotion_ready"))
    for claim in REQUIRED_FALSE_CLAIMS:
        require_false(issues, f"load_network_safety.{claim}", safety.get(claim))

    for token in [
        EXPECTED_RECEIPT_DIGEST,
        EXPECTED_LOG_DIGEST,
        "server.protocol=763",
        "promotion_ready=true",
    ]:
        if token not in evidence.doc_text:
            issues.append(f"load/network doc missing token: {token}")
    if "server.protocol=763" not in evidence.tasks_text:
        issues.append("archived task note does not cite protocol 763 live receipt")
    return issues


def load_repo_evidence() -> LoadSafetyEvidence:
    return LoadSafetyEvidence(
        receipt=json.loads(RECEIPT.read_text()),
        doc_text=DOC.read_text(),
        tasks_text=TASKS.read_text(),
    )


def valid_fixture() -> LoadSafetyEvidence:
    receipt = {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {"name": "valence-compat-bot-probe", "passed": True},
        "server": {"protocol": PROTOCOL_763, "version": "1.20.1", "passed": True},
        "load_network_safety": {
            "target_scope": "owned-local-loopback",
            "owned_local_target": True,
            "authorized": True,
            "max_clients": MAX_LOCAL_CLIENTS,
            "max_duration_secs": MAX_DURATION_SECS,
            "bound_violations": [],
            "missing_fields": [],
            "telemetry_present": True,
            "live_receipt": True,
            "preflight_passed": True,
            "promotion_ready": True,
            "claims_public_server_safety": False,
            "claims_production_readiness": False,
            "claims_unbounded_soak": False,
            "claims_unbounded_reconnect": False,
            "claims_wan_safety": False,
            "claims_adversarial_network_safety": False,
        },
    }
    doc = f"{EXPECTED_RECEIPT_DIGEST} {EXPECTED_LOG_DIGEST} server.protocol=763 promotion_ready=true"
    tasks = "server.protocol=763"
    return LoadSafetyEvidence(receipt=receipt, doc_text=doc, tasks_text=tasks)


def assert_self_tests() -> None:
    issues = validate_load_safety(valid_fixture())
    assert not issues, issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipt["server"]["protocol"] = 758
    issues = validate_load_safety(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    dry_run = valid_fixture()
    dry_run.receipt["dry_run"] = True
    issues = validate_load_safety(dry_run)
    assert any("receipt.dry_run" in issue for issue in issues), issues

    no_telemetry = valid_fixture()
    no_telemetry.receipt["load_network_safety"]["telemetry_present"] = False
    issues = validate_load_safety(no_telemetry)
    assert any("telemetry_present" in issue for issue in issues), issues

    public_claim = valid_fixture()
    public_claim.receipt["load_network_safety"]["claims_public_server_safety"] = True
    issues = validate_load_safety(public_claim)
    assert any("claims_public_server_safety" in issue for issue in issues), issues

    missing_doc_digest = valid_fixture()
    missing_doc_digest.doc_text = "server.protocol=763 promotion_ready=true"
    issues = validate_load_safety(missing_doc_digest)
    assert any("doc missing token" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("load/network safety self-test ok")
        return 0
    issues = validate_load_safety(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("load/network safety ok: protocol-763 live receipt")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
