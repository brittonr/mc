#!/usr/bin/env python3
"""Validate protocol-763 survival chest persistence contract and paired evidence."""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
CONTRACT_DOC = ROOT / "docs" / "evidence" / "protocol-763-survival-chest-persistence-contract-2026-05-28.md"
EXPECTED_PROTOCOL = 763
EXPECTED_SCENARIO = "survival-chest-persistence"
REFERENCE_BACKEND = "paper"
VALENCE_BACKEND = "valence"
EXPECTED_CHEST_POSITION = "8,64,0"
EXPECTED_CHEST_SLOT = "0"
EXPECTED_STORED_ITEM = "Dirt"
EXPECTED_STORED_COUNT = "1"
EXPECTED_RECONNECT_SESSION = "1"
FIRST_CHEST_WINDOW = "1"
REOPENED_CHEST_WINDOW = "2"
MISMATCHED_CHEST_SLOT = "1"
MISMATCHED_STORED_ITEM = "Stone"
MISMATCHED_STORED_COUNT = "2"
PRESENT = "present"
ABSENT = "absent"
NO_VALUE = "<missing>"
ANSI_RE = re.compile(r"\x1b\[[0-9;]*m")

CLIENT_MILESTONES = [
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_chest_open_seen",
    "survival_chest_store_sent",
    "survival_chest_close_sent",
    "survival_chest_reconnect_sent",
    "survival_chest_reopen_seen",
    "survival_chest_persisted_seen",
]
SERVER_MILESTONES = [
    "server_survival_chest_open",
    "server_survival_chest_store",
    "server_survival_chest_close",
    "server_survival_chest_reopen",
    "server_survival_chest_persisted",
]
RECEIPT_METRICS = [
    "scenario.name",
    "server.protocol",
    "server.backend",
    "client.username",
    "client.missing_milestones.empty",
    "client.forbidden_matches.empty",
    "server.missing_milestones.empty",
    "server.forbidden_matches.empty",
]
CLIENT_LOG_METRICS = [
    "client.chest.open.window",
    "client.chest.open.position",
    "client.chest.store.window",
    "client.chest.store.slot",
    "client.chest.store.item",
    "client.chest.store.count",
    "client.chest.close.window",
    "client.chest.reconnect.session",
    "client.chest.reopen.window",
    "client.chest.reopen.position",
    "client.chest.persisted.window",
    "client.chest.persisted.slot",
    "client.chest.persisted.item",
    "client.chest.persisted.count",
]
SERVER_LOG_METRICS = [
    "server.chest.open.position",
    "server.chest.open.window",
    "server.chest.store.window",
    "server.chest.store.slot",
    "server.chest.store.item",
    "server.chest.store.count",
    "server.chest.close.window",
    "server.chest.reopen.position",
    "server.chest.reopen.window",
    "server.chest.persisted.slot",
    "server.chest.persisted.item",
    "server.chest.persisted.count",
]
REQUIRED_METRICS = [
    *RECEIPT_METRICS,
    *(f"client.milestone.{milestone}" for milestone in CLIENT_MILESTONES),
    *(f"server.milestone.{milestone}" for milestone in SERVER_MILESTONES),
    *CLIENT_LOG_METRICS,
    *SERVER_LOG_METRICS,
]
COMPARISON_METRICS = [
    "scenario.name",
    "server.protocol",
    "client.chest.open.position",
    "client.chest.store.slot",
    "client.chest.store.item",
    "client.chest.store.count",
    "client.chest.reconnect.session",
    "client.chest.reopen.position",
    "client.chest.persisted.slot",
    "client.chest.persisted.item",
    "client.chest.persisted.count",
    "server.chest.open.position",
    "server.chest.store.slot",
    "server.chest.store.item",
    "server.chest.store.count",
    "server.chest.reopen.position",
    "server.chest.persisted.slot",
    "server.chest.persisted.item",
    "server.chest.persisted.count",
]
CONTRACT_TOKENS = [
    EXPECTED_SCENARIO,
    EXPECTED_CHEST_POSITION,
    EXPECTED_CHEST_SLOT,
    EXPECTED_STORED_ITEM,
    EXPECTED_STORED_COUNT,
    "one chest block",
    "one item stack",
    "one chest slot",
    "close, disconnect/reconnect once, reopen",
    "full survival compatibility",
    "all-container behavior",
    "server restart persistence",
    "world persistence",
    "broader vanilla parity",
    "missing_reference",
    "missing_metric",
    "mismatched_metric:*.slot",
    "mismatched_metric:*.item",
    "mismatched_metric:*.count",
    "wrong_backend",
    *REQUIRED_METRICS,
]
PAIR_ARG_NAMES = [
    "reference_receipt",
    "reference_client_log",
    "reference_server_log",
    "valence_receipt",
    "valence_client_log",
    "valence_server_log",
]


@dataclass(frozen=True)
class EvidenceInput:
    receipt: dict[str, Any]
    client_log: str
    server_log: str


@dataclass(frozen=True)
class NormalizedEvidence:
    backend: str
    values: dict[str, str]
    diagnostics: tuple[str, ...]


@dataclass(frozen=True)
class ComparisonResult:
    passed: bool
    diagnostics: tuple[str, ...]


def strip_ansi(text: str) -> str:
    return ANSI_RE.sub("", text)


def dig(value: Any, path: tuple[str, ...]) -> Any:
    current = value
    for key in path:
        if not isinstance(current, dict) or key not in current:
            return None
        current = current[key]
    return current


def status_for_presence(items: Any, expected: str) -> str:
    if isinstance(items, list) and expected in items:
        return PRESENT
    return ABSENT


def empty_status(items: Any) -> str:
    if isinstance(items, list) and not items:
        return PRESENT
    return ABSENT


def parse_key_values(segment: str) -> dict[str, str]:
    fields: dict[str, str] = {}
    for token in segment.split():
        if "=" not in token:
            continue
        key, value = token.split("=", maxsplit=1)
        fields[key] = value.rstrip(",")
    return fields


def find_fields(log_text: str, marker: str, required: tuple[tuple[str, str], ...] = ()) -> dict[str, str] | None:
    for line in strip_ansi(log_text).splitlines():
        if marker not in line:
            continue
        fields = parse_key_values(line.split(marker, maxsplit=1)[1])
        if all(fields.get(key) == value for key, value in required):
            return fields
    return None


def put_metric(values: dict[str, str], diagnostics: list[str], metric: str, fields: dict[str, str] | None, key: str) -> None:
    value = None if fields is None else fields.get(key)
    if value is None:
        diagnostics.append(f"missing_metric:{metric}")
        values[metric] = NO_VALUE
        return
    values[metric] = value


def add_receipt_metrics(values: dict[str, str], receipt: dict[str, Any]) -> None:
    scenario = dig(receipt, ("scenario",))
    server = dig(receipt, ("server",))
    client = dig(receipt, ("client",))
    if not isinstance(scenario, dict):
        scenario = {}
    if not isinstance(server, dict):
        server = {}
    if not isinstance(client, dict):
        client = {}

    values["scenario.name"] = str(scenario.get("name", NO_VALUE))
    values["server.protocol"] = str(server.get("protocol", NO_VALUE))
    values["server.backend"] = str(server.get("backend", NO_VALUE))
    values["client.username"] = str(client.get("username", NO_VALUE))
    values["client.missing_milestones.empty"] = empty_status(scenario.get("missing_milestones"))
    values["client.forbidden_matches.empty"] = empty_status(scenario.get("forbidden_matches"))
    values["server.missing_milestones.empty"] = empty_status(server.get("missing_milestones"))
    values["server.forbidden_matches.empty"] = empty_status(server.get("forbidden_matches"))

    observed_client = scenario.get("observed_milestones")
    observed_server = server.get("observed_milestones")
    for milestone in CLIENT_MILESTONES:
        values[f"client.milestone.{milestone}"] = status_for_presence(observed_client, milestone)
    for milestone in SERVER_MILESTONES:
        values[f"server.milestone.{milestone}"] = status_for_presence(observed_server, milestone)


def add_client_log_metrics(values: dict[str, str], diagnostics: list[str], client_log: str) -> None:
    open_fields = find_fields(client_log, "survival_chest_open_seen")
    put_metric(values, diagnostics, "client.chest.open.window", open_fields, "window")
    put_metric(values, diagnostics, "client.chest.open.position", open_fields, "position")

    store_fields = find_fields(client_log, "survival_chest_store_sent")
    put_metric(values, diagnostics, "client.chest.store.window", store_fields, "window")
    put_metric(values, diagnostics, "client.chest.store.slot", store_fields, "slot")
    put_metric(values, diagnostics, "client.chest.store.item", store_fields, "item")
    put_metric(values, diagnostics, "client.chest.store.count", store_fields, "count")

    close_fields = find_fields(client_log, "survival_chest_close_sent")
    put_metric(values, diagnostics, "client.chest.close.window", close_fields, "window")

    reconnect_fields = find_fields(client_log, "survival_chest_reconnect_sent")
    put_metric(values, diagnostics, "client.chest.reconnect.session", reconnect_fields, "session")

    reopen_fields = find_fields(client_log, "survival_chest_reopen_seen")
    put_metric(values, diagnostics, "client.chest.reopen.window", reopen_fields, "window")
    put_metric(values, diagnostics, "client.chest.reopen.position", reopen_fields, "position")

    persisted_fields = find_fields(client_log, "survival_chest_persisted_seen")
    put_metric(values, diagnostics, "client.chest.persisted.window", persisted_fields, "window")
    put_metric(values, diagnostics, "client.chest.persisted.slot", persisted_fields, "slot")
    put_metric(values, diagnostics, "client.chest.persisted.item", persisted_fields, "item")
    put_metric(values, diagnostics, "client.chest.persisted.count", persisted_fields, "count")


def add_server_log_metrics(values: dict[str, str], diagnostics: list[str], server_log: str) -> None:
    open_fields = find_fields(server_log, "survival_chest_open")
    put_metric(values, diagnostics, "server.chest.open.position", open_fields, "position")
    put_metric(values, diagnostics, "server.chest.open.window", open_fields, "window")

    store_fields = find_fields(server_log, "survival_chest_store")
    put_metric(values, diagnostics, "server.chest.store.window", store_fields, "window")
    put_metric(values, diagnostics, "server.chest.store.slot", store_fields, "slot")
    put_metric(values, diagnostics, "server.chest.store.item", store_fields, "item")
    put_metric(values, diagnostics, "server.chest.store.count", store_fields, "count")

    close_fields = find_fields(server_log, "survival_chest_close")
    put_metric(values, diagnostics, "server.chest.close.window", close_fields, "window")

    reopen_fields = find_fields(server_log, "survival_chest_reopen")
    put_metric(values, diagnostics, "server.chest.reopen.position", reopen_fields, "position")
    put_metric(values, diagnostics, "server.chest.reopen.window", reopen_fields, "window")

    persisted_fields = find_fields(server_log, "survival_chest_persisted")
    put_metric(values, diagnostics, "server.chest.persisted.slot", persisted_fields, "slot")
    put_metric(values, diagnostics, "server.chest.persisted.item", persisted_fields, "item")
    put_metric(values, diagnostics, "server.chest.persisted.count", persisted_fields, "count")


def normalize_evidence(evidence: EvidenceInput, expected_backend: str) -> NormalizedEvidence:
    values: dict[str, str] = {}
    diagnostics: list[str] = []
    add_receipt_metrics(values, evidence.receipt)
    add_client_log_metrics(values, diagnostics, evidence.client_log)
    add_server_log_metrics(values, diagnostics, evidence.server_log)

    if values["server.backend"] != expected_backend:
        diagnostics.append(f"wrong_backend:{values['server.backend']} expected {expected_backend}")
    if values["scenario.name"] != EXPECTED_SCENARIO:
        diagnostics.append(f"wrong_scenario:{values['scenario.name']}")
    if values["server.protocol"] != str(EXPECTED_PROTOCOL):
        diagnostics.append(f"wrong_protocol:{values['server.protocol']}")
    for metric in REQUIRED_METRICS:
        if values.get(metric) in (None, NO_VALUE, ABSENT):
            diagnostics.append(f"missing_metric:{metric}")

    return NormalizedEvidence(
        backend=values.get("server.backend", NO_VALUE),
        values=values,
        diagnostics=tuple(sorted(set(diagnostics))),
    )


def compare_evidence(reference: EvidenceInput | None, valence: EvidenceInput | None) -> ComparisonResult:
    diagnostics: list[str] = []
    if reference is None:
        diagnostics.append("missing_reference")
    if valence is None:
        diagnostics.append("missing_valence")
    if reference is None and valence is not None:
        diagnostics.append("valence_only")
    if reference is None or valence is None:
        return ComparisonResult(passed=False, diagnostics=tuple(diagnostics))

    reference_metrics = normalize_evidence(reference, REFERENCE_BACKEND)
    valence_metrics = normalize_evidence(valence, VALENCE_BACKEND)
    diagnostics.extend(f"reference:{item}" for item in reference_metrics.diagnostics)
    diagnostics.extend(f"valence:{item}" for item in valence_metrics.diagnostics)

    for metric in COMPARISON_METRICS:
        reference_value = reference_metrics.values.get(metric, NO_VALUE)
        valence_value = valence_metrics.values.get(metric, NO_VALUE)
        if reference_value != valence_value:
            diagnostics.append(f"mismatched_metric:{metric}: reference={reference_value} valence={valence_value}")

    return ComparisonResult(passed=not diagnostics, diagnostics=tuple(diagnostics))


def validate_contract_doc(text: str) -> list[str]:
    return [f"contract missing token: {token}" for token in CONTRACT_TOKENS if token not in text]


def good_receipt(backend: str) -> dict[str, Any]:
    return {
        "scenario": {
            "name": EXPECTED_SCENARIO,
            "observed_milestones": CLIENT_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": [],
        },
        "client": {"username": "compatbot"},
        "server": {
            "backend": backend,
            "protocol": EXPECTED_PROTOCOL,
            "observed_milestones": SERVER_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": [],
        },
    }


def good_client_log() -> str:
    return "\n".join(
        [
            f"MC-COMPAT-MILESTONE survival_chest_open_seen window={FIRST_CHEST_WINDOW} position={EXPECTED_CHEST_POSITION}",
            f"MC-COMPAT-MILESTONE survival_chest_store_sent window={FIRST_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}",
            f"MC-COMPAT-MILESTONE survival_chest_close_sent window={FIRST_CHEST_WINDOW}",
            f"MC-COMPAT-MILESTONE survival_chest_reconnect_sent session={EXPECTED_RECONNECT_SESSION}",
            f"MC-COMPAT-MILESTONE survival_chest_reopen_seen window={REOPENED_CHEST_WINDOW} position={EXPECTED_CHEST_POSITION}",
            f"MC-COMPAT-MILESTONE survival_chest_persisted_seen window={REOPENED_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}",
        ]
    )


def good_server_log() -> str:
    return "\n".join(
        [
            f"MC-COMPAT-MILESTONE survival_chest_open username=compatbot position={EXPECTED_CHEST_POSITION} window={FIRST_CHEST_WINDOW}",
            f"MC-COMPAT-MILESTONE survival_chest_store username=compatbot window={FIRST_CHEST_WINDOW} slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}",
            f"MC-COMPAT-MILESTONE survival_chest_close username=compatbot window={FIRST_CHEST_WINDOW}",
            f"MC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position={EXPECTED_CHEST_POSITION} window={REOPENED_CHEST_WINDOW}",
            f"MC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot={EXPECTED_CHEST_SLOT} item={EXPECTED_STORED_ITEM} count={EXPECTED_STORED_COUNT}",
        ]
    )


def good_evidence(backend: str) -> EvidenceInput:
    return EvidenceInput(receipt=good_receipt(backend), client_log=good_client_log(), server_log=good_server_log())


def assert_self_tests() -> None:
    good = compare_evidence(good_evidence(REFERENCE_BACKEND), good_evidence(VALENCE_BACKEND))
    assert good.passed, good.diagnostics

    missing_reference = compare_evidence(None, good_evidence(VALENCE_BACKEND))
    assert not missing_reference.passed, missing_reference
    assert "missing_reference" in missing_reference.diagnostics, missing_reference
    assert "valence_only" in missing_reference.diagnostics, missing_reference

    missing_metric = compare_evidence(
        EvidenceInput(good_receipt(REFERENCE_BACKEND), good_client_log().replace("survival_chest_persisted_seen", "survival_chest_missing_seen"), good_server_log()),
        good_evidence(VALENCE_BACKEND),
    )
    assert not missing_metric.passed, missing_metric
    assert any("missing_metric:client.chest.persisted.slot" in item for item in missing_metric.diagnostics), missing_metric

    mismatched_slot = compare_evidence(
        good_evidence(REFERENCE_BACKEND),
        EvidenceInput(
            good_receipt(VALENCE_BACKEND),
            good_client_log().replace(f"slot={EXPECTED_CHEST_SLOT}", f"slot={MISMATCHED_CHEST_SLOT}"),
            good_server_log().replace(f"slot={EXPECTED_CHEST_SLOT}", f"slot={MISMATCHED_CHEST_SLOT}"),
        ),
    )
    assert not mismatched_slot.passed, mismatched_slot
    assert any("mismatched_metric:client.chest.store.slot" in item for item in mismatched_slot.diagnostics), mismatched_slot

    mismatched_item = compare_evidence(
        good_evidence(REFERENCE_BACKEND),
        EvidenceInput(
            good_receipt(VALENCE_BACKEND),
            good_client_log().replace(f"item={EXPECTED_STORED_ITEM}", f"item={MISMATCHED_STORED_ITEM}"),
            good_server_log().replace(f"item={EXPECTED_STORED_ITEM}", f"item={MISMATCHED_STORED_ITEM}"),
        ),
    )
    assert not mismatched_item.passed, mismatched_item
    assert any("mismatched_metric:client.chest.store.item" in item for item in mismatched_item.diagnostics), mismatched_item

    mismatched_count = compare_evidence(
        good_evidence(REFERENCE_BACKEND),
        EvidenceInput(
            good_receipt(VALENCE_BACKEND),
            good_client_log().replace(f"count={EXPECTED_STORED_COUNT}", f"count={MISMATCHED_STORED_COUNT}"),
            good_server_log().replace(f"count={EXPECTED_STORED_COUNT}", f"count={MISMATCHED_STORED_COUNT}"),
        ),
    )
    assert not mismatched_count.passed, mismatched_count
    assert any("mismatched_metric:client.chest.store.count" in item for item in mismatched_count.diagnostics), mismatched_count

    wrong_backend = compare_evidence(good_evidence(VALENCE_BACKEND), good_evidence(VALENCE_BACKEND))
    assert not wrong_backend.passed, wrong_backend
    assert any("wrong_backend" in item for item in wrong_backend.diagnostics), wrong_backend

    contract_issues = validate_contract_doc(CONTRACT_DOC.read_text())
    assert not contract_issues, contract_issues


def load_json(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text())
    if not isinstance(data, dict):
        raise ValueError(f"{path} did not contain a JSON object")
    return data


def maybe_load_pair(args: argparse.Namespace) -> tuple[EvidenceInput | None, EvidenceInput | None, list[str]]:
    provided = [name for name in PAIR_ARG_NAMES if getattr(args, name) is not None]
    if not provided:
        return None, None, []
    missing = [name for name in PAIR_ARG_NAMES if getattr(args, name) is None]
    if missing:
        return None, None, [f"missing pair argument: {name}" for name in missing]
    reference = EvidenceInput(
        receipt=load_json(Path(args.reference_receipt)),
        client_log=Path(args.reference_client_log).read_text(),
        server_log=Path(args.reference_server_log).read_text(),
    )
    valence = EvidenceInput(
        receipt=load_json(Path(args.valence_receipt)),
        client_log=Path(args.valence_client_log).read_text(),
        server_log=Path(args.valence_server_log).read_text(),
    )
    return reference, valence, []


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative fixtures")
    parser.add_argument("--contract-doc", type=Path, default=CONTRACT_DOC, help="contract markdown to validate")
    parser.add_argument("--reference-receipt", type=Path)
    parser.add_argument("--reference-client-log", type=Path)
    parser.add_argument("--reference-server-log", type=Path)
    parser.add_argument("--valence-receipt", type=Path)
    parser.add_argument("--valence-client-log", type=Path)
    parser.add_argument("--valence-server-log", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("survival chest persistence self-test ok")
        return 0

    issues = validate_contract_doc(args.contract_doc.read_text())
    reference, valence, pair_issues = maybe_load_pair(args)
    issues.extend(pair_issues)
    if reference is not None or valence is not None:
        comparison = compare_evidence(reference, valence)
        issues.extend(comparison.diagnostics)
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"survival chest persistence contract ok: {len(REQUIRED_METRICS)} metrics")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
