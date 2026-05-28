#!/usr/bin/env python3
"""Validate survival break/place/pickup parity metrics and paired-receipt fixtures."""
from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs" / "evidence" / "protocol-763-survival-reference-parity-2026-05-28.md"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"

EXPECTED_PROTOCOL = 763
EXPECTED_SCENARIO = "survival-break-place-pickup"
REFERENCE_BACKEND = "paper"
VALENCE_BACKEND = "valence"
PRESENT = "present"
ABSENT = "absent"
NO_VALUE = "<missing>"
REFERENCE_VERSION = "minecraft-1.20.1-protocol-763"
PARITY_NON_CLAIM = "exact survival break/place/pickup parity remains a non-claim"
VALENCE_ONLY_REJECTION = "rejects Valence-only survival evidence"
DECISION_OWNER_TOKEN = "decision_owner: agent"
REFERENCE_RECEIPT_TOKEN = "reference_receipt: none"
PAIR_RECEIPT_TOKEN = "valence_reference_pair: none"
ANSI_RE = re.compile(r"\x1b\[[0-9;]*m")

CLIENT_MILESTONES = [
    "protocol_detected",
    "join_game",
    "render_tick",
    "survival_break_sent",
    "survival_break_update",
    "survival_pickup_seen",
    "survival_place_sent",
    "survival_place_update",
]
SERVER_MILESTONES = [
    "server_username_seen",
    "server_survival_join",
    "server_survival_break",
    "server_survival_pickup",
    "server_survival_place",
]
LOG_METRICS = [
    "client.break.sent.location",
    "client.break.sent.status",
    "client.break.update.location",
    "client.break.update.raw_id",
    "client.pickup.count",
    "client.inventory.slot",
    "client.inventory.item_id",
    "client.inventory.count",
    "client.place.sent.location",
    "client.place.sent.face",
    "client.place.sent.hand",
    "client.place.update.location",
    "client.place.update.raw_id",
    "server.join.gamemode",
    "server.join.target",
    "server.break.item",
    "server.break.at",
    "server.pickup.slot",
    "server.pickup.item",
    "server.pickup.count",
    "server.place.item",
    "server.place.from_slot",
    "server.place.at",
]
RECEIPT_METRICS = [
    "scenario.name",
    "server.protocol",
    "client.username",
    "client.missing_milestones.empty",
    "client.forbidden_matches.empty",
    "server.missing_milestones.empty",
    "server.forbidden_matches.empty",
]
REQUIRED_METRICS = [
    *RECEIPT_METRICS,
    *(f"client.milestone.{name}" for name in CLIENT_MILESTONES),
    *(f"server.milestone.{name}" for name in SERVER_MILESTONES),
    *LOG_METRICS,
]
REQUIRED_DOC_TOKENS = [
    REFERENCE_VERSION,
    PARITY_NON_CLAIM,
    VALENCE_ONLY_REJECTION,
    DECISION_OWNER_TOKEN,
    REFERENCE_RECEIPT_TOKEN,
    PAIR_RECEIPT_TOKEN,
    "missing_reference",
    "missing_metric",
    "mismatched_metric",
    "wrong_backend",
    *REQUIRED_METRICS,
]
FORBIDDEN_PROMOTION_TOKENS = [
    "survival break/place/pickup parity covered",
    "exact survival break/place/pickup parity covered",
    "vanilla survival parity covered",
    "full survival compatibility covered",
]
PAIR_ARG_FIELDS = (
    "reference_receipt",
    "reference_client_log",
    "reference_server_log",
    "valence_receipt",
    "valence_client_log",
    "valence_server_log",
)


@dataclass(frozen=True)
class NormalizedMetrics:
    backend: str
    values: dict[str, str]
    diagnostics: tuple[str, ...]


@dataclass(frozen=True)
class Comparison:
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
        key, value = token.split("=", 1)
        fields[key] = value.rstrip(",")
    return fields


def find_fields(log_text: str, marker: str, required: tuple[str, ...] = ()) -> dict[str, str] | None:
    clean = strip_ansi(log_text)
    for line in clean.splitlines():
        if marker not in line:
            continue
        fields = parse_key_values(line.split(marker, 1)[1])
        if all(fields.get(key) == value for key, value in required):
            return fields
    return None


def find_inventory_slot_item(log_text: str, slot: str) -> dict[str, str] | None:
    clean = strip_ansi(log_text)
    fallback = None
    for line in clean.splitlines():
        if "inventory_probe_set_slot" not in line:
            continue
        fields = parse_key_values(line.split("inventory_probe_set_slot", 1)[1])
        if fields.get("slot") != slot:
            continue
        if fallback is None:
            fallback = fields
        if fields.get("item", "").startswith("id="):
            return fields
    return fallback


def put_if_present(values: dict[str, str], diagnostics: list[str], metric: str, fields: dict[str, str] | None, key: str) -> None:
    value = None if fields is None else fields.get(key)
    if value is None:
        diagnostics.append(f"missing_metric:{metric}")
        values[metric] = NO_VALUE
        return
    values[metric] = value


def add_receipt_metrics(values: dict[str, str], diagnostics: list[str], receipt: dict[str, Any]) -> None:
    scenario = dig(receipt, ("scenario",))
    server = dig(receipt, ("server",))
    client = dig(receipt, ("client",))
    if not isinstance(scenario, dict):
        diagnostics.append("missing_object:scenario")
        scenario = {}
    if not isinstance(server, dict):
        diagnostics.append("missing_object:server")
        server = {}
    if not isinstance(client, dict):
        diagnostics.append("missing_object:client")
        client = {}

    values["scenario.name"] = str(scenario.get("name", NO_VALUE))
    values["server.protocol"] = str(server.get("protocol", NO_VALUE))
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
    break_sent = find_fields(client_log, "survival_probe_break_block_sent")
    put_if_present(values, diagnostics, "client.break.sent.location", break_sent, "location")
    put_if_present(values, diagnostics, "client.break.sent.status", break_sent, "status")

    break_update = find_fields(client_log, "survival_probe_block_update")
    put_if_present(values, diagnostics, "client.break.update.location", break_update, "location")
    put_if_present(values, diagnostics, "client.break.update.raw_id", break_update, "raw_id")

    pickup = find_fields(client_log, "survival_probe_pickup_seen")
    put_if_present(values, diagnostics, "client.pickup.count", pickup, "count")

    inventory = find_inventory_slot_item(client_log, "36")
    put_if_present(values, diagnostics, "client.inventory.slot", inventory, "slot")
    item = None if inventory is None else inventory.get("item")
    if item is None or not item.startswith("id="):
        diagnostics.append("missing_metric:client.inventory.item_id")
        values["client.inventory.item_id"] = NO_VALUE
    else:
        values["client.inventory.item_id"] = item.removeprefix("id=")
    put_if_present(values, diagnostics, "client.inventory.count", inventory, "count")

    place_sent = find_fields(client_log, "survival_probe_place_block_sent")
    put_if_present(values, diagnostics, "client.place.sent.location", place_sent, "location")
    put_if_present(values, diagnostics, "client.place.sent.face", place_sent, "face")
    put_if_present(values, diagnostics, "client.place.sent.hand", place_sent, "hand")

    place_update = find_fields(client_log, "survival_probe_place_update")
    put_if_present(values, diagnostics, "client.place.update.location", place_update, "location")
    put_if_present(values, diagnostics, "client.place.update.raw_id", place_update, "raw_id")


def add_server_log_metrics(values: dict[str, str], diagnostics: list[str], server_log: str) -> None:
    join = find_fields(server_log, "survival_join")
    put_if_present(values, diagnostics, "server.join.gamemode", join, "gamemode")
    put_if_present(values, diagnostics, "server.join.target", join, "target")

    block_break = find_fields(server_log, "survival_block_break")
    put_if_present(values, diagnostics, "server.break.item", block_break, "item")
    put_if_present(values, diagnostics, "server.break.at", block_break, "at")

    pickup = find_fields(server_log, "survival_pickup_item")
    put_if_present(values, diagnostics, "server.pickup.slot", pickup, "slot")
    put_if_present(values, diagnostics, "server.pickup.item", pickup, "item")
    put_if_present(values, diagnostics, "server.pickup.count", pickup, "count")

    place = find_fields(server_log, "survival_block_place")
    put_if_present(values, diagnostics, "server.place.item", place, "item")
    put_if_present(values, diagnostics, "server.place.from_slot", place, "from_slot")
    put_if_present(values, diagnostics, "server.place.at", place, "at")


def normalize_metrics(receipt: dict[str, Any], client_log: str, server_log: str) -> NormalizedMetrics:
    values: dict[str, str] = {}
    diagnostics: list[str] = []
    server = dig(receipt, ("server",))
    backend = server.get("backend", NO_VALUE) if isinstance(server, dict) else NO_VALUE
    add_receipt_metrics(values, diagnostics, receipt)
    add_client_log_metrics(values, diagnostics, client_log)
    add_server_log_metrics(values, diagnostics, server_log)
    for metric in REQUIRED_METRICS:
        if metric not in values:
            diagnostics.append(f"missing_metric:{metric}")
            values[metric] = NO_VALUE
    if values["scenario.name"] != EXPECTED_SCENARIO:
        diagnostics.append("wrong_scenario")
    if values["server.protocol"] != str(EXPECTED_PROTOCOL):
        diagnostics.append("wrong_protocol")
    return NormalizedMetrics(backend=str(backend), values=values, diagnostics=tuple(diagnostics))


def compare_normalized(reference: NormalizedMetrics | None, valence: NormalizedMetrics | None) -> Comparison:
    diagnostics: list[str] = []
    if reference is None:
        diagnostics.append("missing_reference")
    if valence is None:
        diagnostics.append("missing_valence")
    if reference is None or valence is None:
        return Comparison(False, tuple(diagnostics))
    if reference.backend != REFERENCE_BACKEND:
        diagnostics.append(f"wrong_backend:reference:{reference.backend}")
    if valence.backend != VALENCE_BACKEND:
        diagnostics.append(f"wrong_backend:valence:{valence.backend}")
    diagnostics.extend(reference.diagnostics)
    diagnostics.extend(valence.diagnostics)
    for metric in REQUIRED_METRICS:
        left = reference.values.get(metric, NO_VALUE)
        right = valence.values.get(metric, NO_VALUE)
        if left == NO_VALUE or right == NO_VALUE:
            diagnostics.append(f"missing_metric:{metric}")
        elif left != right:
            diagnostics.append(f"mismatched_metric:{metric}:reference={left}:valence={right}")
    return Comparison(not diagnostics, tuple(dict.fromkeys(diagnostics)))


def read_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ValueError(f"receipt is not a JSON object: {path}")
    return value


def compare_paths(args: argparse.Namespace) -> Comparison:
    reference = normalize_metrics(
        read_json(Path(args.reference_receipt)),
        Path(args.reference_client_log).read_text(),
        Path(args.reference_server_log).read_text(),
    )
    valence = normalize_metrics(
        read_json(Path(args.valence_receipt)),
        Path(args.valence_client_log).read_text(),
        Path(args.valence_server_log).read_text(),
    )
    return compare_normalized(reference, valence)


def validate_doc(doc_text: str, matrix_text: str, bundle_text: str) -> list[str]:
    issues: list[str] = []
    for token in REQUIRED_DOC_TOKENS:
        if token not in doc_text:
            issues.append(f"survival parity doc missing token: {token}")
    for token in FORBIDDEN_PROMOTION_TOKENS:
        if token in doc_text.lower() or token in matrix_text.lower() or token in bundle_text.lower():
            issues.append(f"forbidden survival parity promotion token present: {token}")
    if "Full survival compatibility / vanilla parity" not in matrix_text:
        issues.append("acceptance matrix no longer names full survival compatibility gap")
    if "full survival compatibility" not in bundle_text.lower() or "vanilla parity" not in bundle_text.lower():
        issues.append("current bundle no longer names survival parity non-claims")
    return issues


def receipt_fixture(backend: str) -> dict[str, Any]:
    return {
        "schema": "mc.compat.scenario.receipt.v2",
        "status": "pass",
        "scenario": {
            "name": EXPECTED_SCENARIO,
            "observed_milestones": CLIENT_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": [],
        },
        "server": {
            "backend": backend,
            "protocol": EXPECTED_PROTOCOL,
            "observed_milestones": SERVER_MILESTONES,
            "missing_milestones": [],
            "forbidden_matches": [],
        },
        "client": {"username": "compatbot"},
    }


def client_log_fixture(raw_id: str = "10") -> str:
    return "\n".join(
        [
            "MC-COMPAT-MILESTONE survival_probe_break_block_sent status=start_destroy location=0,64,1 sequence=404",
            "MC-COMPAT-MILESTONE survival_probe_pickup_seen collected_entity_id=7630101 collector_entity_id=1 count=1",
            "MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=1 slot=36 item=empty",
            "MC-COMPAT-MILESTONE inventory_probe_set_slot window=0 state_id=2 slot=36 item=id=15 count=1",
            "MC-COMPAT-MILESTONE survival_probe_block_update location=0,64,1 raw_id=0",
            "MC-COMPAT-MILESTONE survival_probe_place_block_sent hand=main location=0,64,1 face=up sequence=405",
            f"MC-COMPAT-MILESTONE survival_probe_place_update location=0,65,1 raw_id={raw_id}",
        ]
    )


def server_log_fixture(item: str = "Dirt") -> str:
    return "\n".join(
        [
            "MC-COMPAT-MILESTONE survival_join username=compatbot gamemode=Survival target=0,64,1",
            f"MC-COMPAT-MILESTONE survival_block_break username=compatbot item={item} at=0,64,1",
            f"MC-COMPAT-MILESTONE survival_pickup_item username=compatbot slot=36 item={item} count=1",
            f"MC-COMPAT-MILESTONE survival_block_place username=compatbot item={item} from_slot=36 at=0,65,1",
        ]
    )


def normalized_fixture(backend: str, *, client_raw_id: str = "10", server_item: str = "Dirt") -> NormalizedMetrics:
    return normalize_metrics(receipt_fixture(backend), client_log_fixture(client_raw_id), server_log_fixture(server_item))


def assert_self_tests() -> None:
    positive = compare_normalized(normalized_fixture(REFERENCE_BACKEND), normalized_fixture(VALENCE_BACKEND))
    assert positive.passed, positive

    missing_reference = compare_normalized(None, normalized_fixture(VALENCE_BACKEND))
    assert not missing_reference.passed and "missing_reference" in missing_reference.diagnostics, missing_reference

    mismatched_metric = compare_normalized(
        normalized_fixture(REFERENCE_BACKEND, client_raw_id="10"),
        normalized_fixture(VALENCE_BACKEND, client_raw_id="11"),
    )
    assert not mismatched_metric.passed, mismatched_metric
    assert any("mismatched_metric:client.place.update.raw_id" in item for item in mismatched_metric.diagnostics), mismatched_metric

    wrong_backend = compare_normalized(normalized_fixture(VALENCE_BACKEND), normalized_fixture(VALENCE_BACKEND))
    assert not wrong_backend.passed and "wrong_backend:reference:valence" in wrong_backend.diagnostics, wrong_backend

    missing_metric_log = client_log_fixture().replace(
        "MC-COMPAT-MILESTONE survival_probe_place_update location=0,65,1 raw_id=10",
        "",
    )
    missing_metric = compare_normalized(
        normalize_metrics(receipt_fixture(REFERENCE_BACKEND), missing_metric_log, server_log_fixture()),
        normalized_fixture(VALENCE_BACKEND),
    )
    assert not missing_metric.passed and "missing_metric:client.place.update.raw_id" in missing_metric.diagnostics, missing_metric

    partial_args = argparse.Namespace(
        reference_receipt="paper.json",
        reference_client_log=None,
        reference_server_log=None,
        valence_receipt=None,
        valence_client_log=None,
        valence_server_log=None,
    )
    partial_arg_diagnostics = missing_pair_arg_diagnostics(partial_args)
    assert "missing_reference_client_log_arg" in partial_arg_diagnostics, partial_arg_diagnostics
    assert "missing_valence_receipt_arg" in partial_arg_diagnostics, partial_arg_diagnostics


def pair_arg_values(args: argparse.Namespace) -> tuple[str | None, ...]:
    return tuple(getattr(args, field) for field in PAIR_ARG_FIELDS)


def missing_pair_arg_diagnostics(args: argparse.Namespace) -> tuple[str, ...]:
    values = pair_arg_values(args)
    if not any(value is not None for value in values):
        return ()
    return tuple(
        f"missing_{field}_arg"
        for field, value in zip(PAIR_ARG_FIELDS, values, strict=True)
        if value is None
    )


def all_pair_args_present(args: argparse.Namespace) -> bool:
    return all(value is not None for value in pair_arg_values(args))


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative parity fixtures")
    parser.add_argument("--reference-receipt")
    parser.add_argument("--reference-client-log")
    parser.add_argument("--reference-server-log")
    parser.add_argument("--valence-receipt")
    parser.add_argument("--valence-client-log")
    parser.add_argument("--valence-server-log")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("survival reference parity self-test ok")
        return 0
    assert_self_tests()
    partial_arg_diagnostics = missing_pair_arg_diagnostics(args)
    if partial_arg_diagnostics:
        for diagnostic in partial_arg_diagnostics:
            print(diagnostic, file=sys.stderr)
        return 1
    if all_pair_args_present(args):
        comparison = compare_paths(args)
        if not comparison.passed:
            for diagnostic in comparison.diagnostics:
                print(diagnostic, file=sys.stderr)
            return 1
        print(f"survival reference parity comparison ok: {len(REQUIRED_METRICS)} metrics")
        return 0
    issues = validate_doc(DOC.read_text(), MATRIX.read_text(), BUNDLE.read_text())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("survival reference parity ok: metrics defined; paired reference receipt still missing")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
