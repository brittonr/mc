#!/usr/bin/env python3
"""Validate protocol-763 production/load/network safety evidence."""
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
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-production-network-safety-matrix-2026-05-28.md"
ORACLE = ROOT / "docs" / "evidence" / "protocol-763-production-network-adversarial-oracle-2026-05-28.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"
ACCEPTANCE_MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
ACTIVE_TASKS = ROOT / "cairn" / "changes" / "prove-production-network-safety" / "tasks.md"
ARCHIVED_TASKS = ROOT / "cairn" / "archive" / "2026-05-28-prove-production-network-safety" / "tasks.md"
LEGACY_TASKS = ROOT / "cairn" / "archive" / "2026-05-27-prove-production-load-network-safety" / "tasks.md"

PROTOCOL_763 = 763
MAX_LOCAL_CLIENTS = 2
MAX_DURATION_SECS = 600
MAX_RECONNECT_SESSIONS = 2
EXPECTED_RECEIPT_DIGEST = "62aba060f0bc082d08487c5adf83bfd417742d3711fe4295066e44e7668a25b2"
EXPECTED_LOG_DIGEST = "8087221d20405d63e5cd81ffc1afbcdfd8b118b157dbe38e5e1752384e97bce7"
MATRIX_PATH_TEXT = "docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md"
ORACLE_PATH_TEXT = "docs/evidence/protocol-763-production-network-adversarial-oracle-2026-05-28.md"
OWNED_LOCAL_CLAIM = "owned-local load safety"
PUBLIC_CLAIM = "public-server safety"
WAN_CLAIM = "WAN tolerance"
ADVERSARIAL_CLAIM = "adversarial-network safety"
COVERED_OWNED_LOCAL = "covered_owned_local_bounded"
NON_CLAIM_FAIL_CLOSED = "non_claim_fail_closed"
NON_CLAIM_ORACLE_REQUIRED = "non_claim_oracle_required"
TABLE_SEPARATOR_PREFIX = "| ---"
REQUIRED_FALSE_CLAIMS = [
    "claims_public_server_safety",
    "claims_production_readiness",
    "claims_unbounded_soak",
    "claims_unbounded_reconnect",
    "claims_wan_safety",
    "claims_adversarial_network_safety",
]
MATRIX_COLUMNS = [
    "Claim",
    "Status",
    "Target ownership",
    "Authorization",
    "Bounds",
    "Telemetry",
    "Evidence",
    "Explicit non-claims",
    "Next action",
]
ORACLE_REQUIRED_HEADINGS = ["## Question", "## Inspected evidence", "## Decision", "## Owner", "## Next action"]


@dataclass(frozen=True)
class SafetyMatrixRow:
    claim: str
    status: str
    target_ownership: str
    authorization: str
    bounds: str
    telemetry: str
    evidence: str
    non_claims: str
    next_action: str


@dataclass
class LoadSafetyEvidence:
    receipt: dict[str, Any]
    doc_text: str
    matrix_text: str
    oracle_text: str
    bundle_text: str
    acceptance_text: str
    tasks_text: str


@dataclass(frozen=True)
class WanToleranceRequest:
    requested: bool
    perturbation_tool_available: bool
    delay_ms: str
    jitter_ms: str
    loss_percent: str
    timeout_secs: int


@dataclass(frozen=True)
class WanToleranceReceipt:
    requested: bool
    status: str
    fail_closed: bool
    missing_fields: tuple[str, ...]
    promotion_ready: bool
    claims_wan_safety: bool


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


def require_int_between(issues: list[str], label: str, actual: Any, lower: int, upper: int) -> None:
    if not isinstance(actual, int):
        issues.append(f"{label} expected int, found {actual!r}")
        return
    if actual < lower:
        issues.append(f"{label} below minimum {lower}: {actual}")
    if actual > upper:
        issues.append(f"{label}_exceed_max: {actual} > {upper}")


def require_tokens(issues: list[str], label: str, text: str, tokens: list[str]) -> None:
    for token in tokens:
        if token not in text:
            issues.append(f"{label} missing token: {token}")


def parse_markdown_table(text: str, expected_columns: list[str]) -> tuple[list[list[str]], list[str]]:
    issues: list[str] = []
    rows: list[list[str]] = []
    in_table = False
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped.startswith("|"):
            if in_table:
                break
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells == expected_columns:
            in_table = True
            continue
        if not in_table:
            continue
        if stripped.startswith(TABLE_SEPARATOR_PREFIX):
            continue
        if len(cells) != len(expected_columns):
            issues.append(f"matrix row has {len(cells)} cells, expected {len(expected_columns)}: {stripped}")
            continue
        rows.append(cells)
    if not in_table:
        issues.append("matrix table header missing")
    return rows, issues


def parse_safety_matrix(text: str) -> tuple[dict[str, SafetyMatrixRow], list[str]]:
    raw_rows, issues = parse_markdown_table(text, MATRIX_COLUMNS)
    rows: dict[str, SafetyMatrixRow] = {}
    for cells in raw_rows:
        row = SafetyMatrixRow(*cells)
        if row.claim in rows:
            issues.append(f"duplicate safety matrix row: {row.claim}")
        rows[row.claim] = row
    return rows, issues


def validate_oracle(oracle_text: str) -> list[str]:
    issues: list[str] = []
    require_tokens(issues, "adversarial oracle", oracle_text, ORACLE_REQUIRED_HEADINGS)
    require_tokens(
        issues,
        "adversarial oracle",
        oracle_text,
        [
            "Decision: no adversarial-network safety claim",
            "No adversarial live receipt",
            "explicit authorization",
            "Owner: agent",
        ],
    )
    return issues


def evaluate_wan_tolerance_request(request: WanToleranceRequest) -> WanToleranceReceipt:
    missing_fields: list[str] = []
    if request.requested and not request.perturbation_tool_available:
        missing_fields.append("perturbation_tool")
    if request.requested and request.delay_ms == "":
        missing_fields.append("delay_ms")
    if request.requested and request.jitter_ms == "":
        missing_fields.append("jitter_ms")
    if request.requested and request.loss_percent == "":
        missing_fields.append("loss_percent")
    if request.requested and request.timeout_secs <= 0:
        missing_fields.append("timeout_secs")
    fail_closed = request.requested and bool(missing_fields)
    status = "failed_closed" if fail_closed else "not_selected" if not request.requested else "ready_for_authorized_fixture"
    return WanToleranceReceipt(
        requested=request.requested,
        status=status,
        fail_closed=fail_closed,
        missing_fields=tuple(missing_fields),
        promotion_ready=False,
        claims_wan_safety=False,
    )


def validate_wan_tolerance_fail_closed_receipt(receipt: WanToleranceReceipt) -> list[str]:
    issues: list[str] = []
    require_true(issues, "wan.requested", receipt.requested)
    require_equal(issues, "wan.status", receipt.status, "failed_closed")
    require_true(issues, "wan.fail_closed", receipt.fail_closed)
    if "perturbation_tool" not in receipt.missing_fields:
        issues.append("wan.missing_fields missing perturbation_tool")
    require_false(issues, "wan.promotion_ready", receipt.promotion_ready)
    require_false(issues, "wan.claims_wan_safety", receipt.claims_wan_safety)
    return issues


def validate_receipt(evidence: LoadSafetyEvidence) -> list[str]:
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
    require_int_between(issues, "load_network_safety.planned_clients", safety.get("planned_clients"), 1, MAX_LOCAL_CLIENTS)
    require_int_between(issues, "load_network_safety.duration_secs", safety.get("duration_secs"), 1, MAX_DURATION_SECS)
    require_int_between(issues, "load_network_safety.reconnect_sessions", safety.get("reconnect_sessions"), 0, MAX_RECONNECT_SESSIONS)
    require_equal(issues, "load_network_safety.bound_violations", safety.get("bound_violations"), [])
    require_equal(issues, "load_network_safety.missing_fields", safety.get("missing_fields"), [])
    require_true(issues, "load_network_safety.telemetry_present", safety.get("telemetry_present"))
    require_true(issues, "load_network_safety.live_receipt", safety.get("live_receipt"))
    require_true(issues, "load_network_safety.preflight_passed", safety.get("preflight_passed"))
    require_true(issues, "load_network_safety.promotion_ready", safety.get("promotion_ready"))
    for claim in REQUIRED_FALSE_CLAIMS:
        require_false(issues, f"load_network_safety.{claim}", safety.get(claim))
    return issues


def validate_safety_matrix(matrix_text: str, oracle_text: str) -> list[str]:
    rows, issues = parse_safety_matrix(matrix_text)
    for claim in [OWNED_LOCAL_CLAIM, PUBLIC_CLAIM, WAN_CLAIM, ADVERSARIAL_CLAIM]:
        if claim not in rows:
            issues.append(f"safety matrix missing claim row: {claim}")
    if issues:
        return issues

    owned = rows[OWNED_LOCAL_CLAIM]
    require_equal(issues, "owned-local status", owned.status, COVERED_OWNED_LOCAL)
    require_tokens(issues, "owned-local target", owned.target_ownership, ["owned-local-loopback", "owned_local_target=true"])
    require_tokens(issues, "owned-local authorization", owned.authorization, ["authorized=true", "owned local loopback"])
    require_tokens(
        issues,
        "owned-local bounds",
        owned.bounds,
        [f"max_clients={MAX_LOCAL_CLIENTS}", f"max_duration_secs={MAX_DURATION_SECS}", f"reconnect_sessions<={MAX_RECONNECT_SESSIONS}"],
    )
    require_tokens(issues, "owned-local telemetry", owned.telemetry, ["telemetry_present=true", "live_receipt=true", "promotion_ready=true"])
    require_tokens(issues, "owned-local evidence", owned.evidence, [EXPECTED_RECEIPT_DIGEST, EXPECTED_LOG_DIGEST, "protocol-763-load-network-safety-live-2026-05-27.receipt.json"])
    require_tokens(issues, "owned-local non-claims", owned.non_claims, ["No public-server safety", "no production readiness", "no WAN safety", "no adversarial-network safety"])

    public = rows[PUBLIC_CLAIM]
    require_equal(issues, "public status", public.status, NON_CLAIM_FAIL_CLOSED)
    require_tokens(issues, "public ownership", public.target_ownership, ["target ownership not established"])
    require_tokens(issues, "public authorization", public.authorization, ["explicit public authorization missing", "MC_COMPAT_PUBLIC_TARGET=1", "MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED=1"])
    require_tokens(issues, "public evidence", public.evidence, ["preflight", "no live public receipt"])
    require_tokens(issues, "public non-claims", public.non_claims, ["No public-server safety", "no production readiness"])

    wan = rows[WAN_CLAIM]
    require_equal(issues, "WAN status", wan.status, NON_CLAIM_FAIL_CLOSED)
    require_tokens(issues, "WAN bounds", wan.bounds, ["delay", "jitter", "loss", "timeout"])
    require_tokens(issues, "WAN evidence", wan.evidence, ["tooling unavailable fails closed", "no WAN claim"])
    require_tokens(issues, "WAN non-claims", wan.non_claims, ["No WAN safety", "no packet-loss tolerance"])

    adversarial = rows[ADVERSARIAL_CLAIM]
    require_equal(issues, "adversarial status", adversarial.status, NON_CLAIM_ORACLE_REQUIRED)
    require_tokens(issues, "adversarial authorization", adversarial.authorization, ["adversarial authorization absent"])
    require_tokens(issues, "adversarial evidence", adversarial.evidence, [ORACLE_PATH_TEXT, "no adversarial live receipt"])
    require_tokens(issues, "adversarial non-claims", adversarial.non_claims, ["No adversarial-network safety", "no malicious-client resilience"])
    require_tokens(issues, "adversarial next action", adversarial.next_action, ["human/oracle approval"])

    issues.extend(validate_oracle(oracle_text))
    return issues


def validate_docs(evidence: LoadSafetyEvidence) -> list[str]:
    issues: list[str] = []
    for token in [
        EXPECTED_RECEIPT_DIGEST,
        EXPECTED_LOG_DIGEST,
        "server.protocol=763",
        "promotion_ready=true",
        MATRIX_PATH_TEXT,
        ORACLE_PATH_TEXT,
        "public-server safety",
        "WAN safety",
        "adversarial-network safety",
    ]:
        if token not in evidence.doc_text and token not in evidence.matrix_text:
            issues.append(f"load/network docs missing token: {token}")
    for label, text in [("current bundle", evidence.bundle_text), ("acceptance matrix", evidence.acceptance_text)]:
        require_tokens(issues, label, text, [MATRIX_PATH_TEXT, "production readiness", "public-server"])
    if "server.protocol=763" not in evidence.tasks_text:
        issues.append("task note does not cite protocol 763 live receipt")
    require_tokens(issues, "task note", evidence.tasks_text, [MATRIX_PATH_TEXT])
    return issues


def validate_load_safety(evidence: LoadSafetyEvidence) -> list[str]:
    issues = validate_receipt(evidence)
    issues.extend(validate_safety_matrix(evidence.matrix_text, evidence.oracle_text))
    issues.extend(validate_docs(evidence))
    return issues


def load_tasks_text() -> str:
    for path in [ACTIVE_TASKS, ARCHIVED_TASKS, LEGACY_TASKS]:
        if path.is_file():
            return path.read_text()
    return ""


def load_repo_evidence() -> LoadSafetyEvidence:
    return LoadSafetyEvidence(
        receipt=json.loads(RECEIPT.read_text()),
        doc_text=DOC.read_text(),
        matrix_text=MATRIX.read_text(),
        oracle_text=ORACLE.read_text(),
        bundle_text=BUNDLE.read_text(),
        acceptance_text=ACCEPTANCE_MATRIX.read_text(),
        tasks_text=load_tasks_text(),
    )


def valid_receipt() -> dict[str, Any]:
    return {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {"name": "valence-compat-bot-probe", "passed": True},
        "server": {"protocol": PROTOCOL_763, "version": "1.20.1", "passed": True},
        "load_network_safety": {
            "target_scope": "owned-local-loopback",
            "owned_local_target": True,
            "authorized": True,
            "planned_clients": 1,
            "max_clients": MAX_LOCAL_CLIENTS,
            "duration_secs": 30,
            "max_duration_secs": MAX_DURATION_SECS,
            "reconnect_sessions": 1,
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


def matrix_fixture() -> str:
    return f"""
| Claim | Status | Target ownership | Authorization | Bounds | Telemetry | Evidence | Explicit non-claims | Next action |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| {OWNED_LOCAL_CLAIM} | {COVERED_OWNED_LOCAL} | target_scope=owned-local-loopback; owned_local_target=true | authorized=true by owned local loopback only | max_clients={MAX_LOCAL_CLIENTS}; max_duration_secs={MAX_DURATION_SECS}; reconnect_sessions<={MAX_RECONNECT_SESSIONS}; packet_loss=0 | telemetry_present=true; live_receipt=true; preflight_passed=true; promotion_ready=true | `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.receipt.json`; `{EXPECTED_RECEIPT_DIGEST}`; `{EXPECTED_LOG_DIGEST}` | No public-server safety, no production readiness, no WAN safety, no adversarial-network safety. | Keep bounded. |
| {PUBLIC_CLAIM} | {NON_CLAIM_FAIL_CLOSED} | target ownership not established for public targets | explicit public authorization missing; MC_COMPAT_PUBLIC_TARGET=1 without MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED=1 fails | no public bounds | no public telemetry | preflight rejects public target; no live public receipt | No public-server safety, no production readiness. | Add authorization. |
| {WAN_CLAIM} | {NON_CLAIM_FAIL_CLOSED} | owned-local loopback only | WAN authorization absent | delay; jitter; loss; timeout required | no WAN telemetry | deterministic WAN receipt request fixture fails closed when perturbation tooling unavailable; tooling unavailable fails closed; no WAN claim | No WAN safety, no packet-loss tolerance. | Add perturbation tooling. |
| {ADVERSARIAL_CLAIM} | {NON_CLAIM_ORACLE_REQUIRED} | no adversarial target | adversarial authorization absent | adversarial model missing | no adversarial telemetry | Oracle checkpoint: `{ORACLE_PATH_TEXT}`; no adversarial live receipt | No adversarial-network safety, no malicious-client resilience. | Require human/oracle approval. |
"""


def oracle_fixture() -> str:
    return "\n".join(
        [
            "## Question",
            "## Inspected evidence",
            "No adversarial live receipt",
            "## Decision",
            "Decision: no adversarial-network safety claim",
            "explicit authorization",
            "## Owner",
            "Owner: agent",
            "## Next action",
        ]
    )


def valid_fixture() -> LoadSafetyEvidence:
    doc = f"{EXPECTED_RECEIPT_DIGEST} {EXPECTED_LOG_DIGEST} server.protocol=763 promotion_ready=true {MATRIX_PATH_TEXT} {ORACLE_PATH_TEXT} public-server safety WAN safety adversarial-network safety"
    bundle = f"{MATRIX_PATH_TEXT} production readiness public-server"
    acceptance = f"{MATRIX_PATH_TEXT} production readiness public-server"
    tasks = f"server.protocol=763 {MATRIX_PATH_TEXT}"
    return LoadSafetyEvidence(
        receipt=valid_receipt(),
        doc_text=doc,
        matrix_text=matrix_fixture(),
        oracle_text=oracle_fixture(),
        bundle_text=bundle,
        acceptance_text=acceptance,
        tasks_text=tasks,
    )


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

    missing_authorization = valid_fixture()
    missing_authorization.matrix_text = missing_authorization.matrix_text.replace("explicit public authorization missing", "")
    issues = validate_load_safety(missing_authorization)
    assert any("public authorization" in issue for issue in issues), issues

    missing_target_ownership = valid_fixture()
    missing_target_ownership.matrix_text = missing_target_ownership.matrix_text.replace("target ownership not established", "")
    issues = validate_load_safety(missing_target_ownership)
    assert any("public ownership" in issue for issue in issues), issues

    over_limit_clients = valid_fixture()
    over_limit_clients.receipt["load_network_safety"]["planned_clients"] = MAX_LOCAL_CLIENTS + 1
    issues = validate_load_safety(over_limit_clients)
    assert any("planned_clients_exceed_max" in issue for issue in issues), issues

    over_limit_duration = valid_fixture()
    over_limit_duration.receipt["load_network_safety"]["duration_secs"] = MAX_DURATION_SECS + 1
    issues = validate_load_safety(over_limit_duration)
    assert any("duration_secs_exceed_max" in issue for issue in issues), issues

    zero_reconnect_sessions = valid_fixture()
    zero_reconnect_sessions.receipt["load_network_safety"]["reconnect_sessions"] = 0
    issues = validate_load_safety(zero_reconnect_sessions)
    assert not issues, issues

    over_limit_reconnect_sessions = valid_fixture()
    over_limit_reconnect_sessions.receipt["load_network_safety"]["reconnect_sessions"] = MAX_RECONNECT_SESSIONS + 1
    issues = validate_load_safety(over_limit_reconnect_sessions)
    assert any("reconnect_sessions_exceed_max" in issue for issue in issues), issues

    missing_bounds = valid_fixture()
    missing_bounds.matrix_text = missing_bounds.matrix_text.replace(f"max_clients={MAX_LOCAL_CLIENTS}", "")
    issues = validate_load_safety(missing_bounds)
    assert any("owned-local bounds" in issue for issue in issues), issues

    missing_nonclaims = valid_fixture()
    missing_nonclaims.matrix_text = missing_nonclaims.matrix_text.replace("No WAN safety", "WAN safety allowed")
    issues = validate_load_safety(missing_nonclaims)
    assert any("non-claims" in issue for issue in issues), issues

    wan_unavailable = evaluate_wan_tolerance_request(
        WanToleranceRequest(
            requested=True,
            perturbation_tool_available=False,
            delay_ms="100",
            jitter_ms="20",
            loss_percent="1",
            timeout_secs=30,
        )
    )
    issues = validate_wan_tolerance_fail_closed_receipt(wan_unavailable)
    assert not issues, issues

    wan_bad_receipt = WanToleranceReceipt(
        requested=True,
        status="pass",
        fail_closed=False,
        missing_fields=(),
        promotion_ready=True,
        claims_wan_safety=True,
    )
    issues = validate_wan_tolerance_fail_closed_receipt(wan_bad_receipt)
    assert any("wan.status" in issue for issue in issues), issues
    assert any("perturbation_tool" in issue for issue in issues), issues
    assert any("claims_wan_safety" in issue for issue in issues), issues

    missing_wan_fail_closed = valid_fixture()
    missing_wan_fail_closed.matrix_text = missing_wan_fail_closed.matrix_text.replace("tooling unavailable fails closed", "tooling skipped")
    issues = validate_load_safety(missing_wan_fail_closed)
    assert any("WAN evidence" in issue for issue in issues), issues

    missing_oracle = valid_fixture()
    missing_oracle.oracle_text = missing_oracle.oracle_text.replace("## Decision", "")
    issues = validate_load_safety(missing_oracle)
    assert any("adversarial oracle" in issue for issue in issues), issues

    missing_doc_digest = valid_fixture()
    missing_doc_digest.doc_text = "server.protocol=763 promotion_ready=true"
    issues = validate_load_safety(missing_doc_digest)
    assert any("docs missing token" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("load/network safety self-test ok: over-limit clients/duration/reconnect, reconnect-zero, WAN fail-closed")
        return 0
    issues = validate_load_safety(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("load/network safety ok: owned-local covered, public/WAN/adversarial claims blocked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
