#!/usr/bin/env python3
"""Validate bounded protocol-763 inventory semantics matrix evidence."""
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
DOC = ROOT / "docs" / "evidence" / "protocol-763-inventory-semantics-matrix-2026-05-27.md"

PROTOCOL_763 = 763
EXPECTED_SCENARIO = "inventory-interaction"
REQUIRED_NON_CLAIMS = [
    "full_ctf_correctness",
    "broad_minecraft_compatibility",
    "unbounded_soak",
    "production_load",
]
FORBIDDEN_INVENTORY_ACCEPTANCE = [
    "invalid_inventory_acceptance",
    "stale_state_accepted",
    "invalid_slot_accepted",
    "malformed_click_accepted",
    "state_corruption",
]
BASE_CLIENT = ["protocol_detected", "join_game", "render_tick", "team_red"]
BASE_SERVER = ["server_username_seen", "server_inventory_hotbar_select"]


@dataclass(frozen=True)
class InventoryRow:
    seam: str
    receipt_path: str
    digest: str
    window_kind: str
    slot_class: str
    click_mode: str
    carried_stack: str
    state_id: str
    expected_outcome: str
    client_milestones: list[str]
    server_milestones: list[str]


@dataclass
class InventoryEvidence:
    receipts: dict[str, dict[str, Any]]
    matrix_text: str
    bundle_text: str
    doc_text: str


ROWS = [
    InventoryRow(
        seam="Inventory/drop",
        receipt_path="docs/evidence/protocol-763-inventory-drop.matrix.receipt.json",
        digest="4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46",
        window_kind="player_inventory",
        slot_class="hotbar_weapon_slot_36",
        click_mode="drop_item_action",
        carried_stack="none",
        state_id="fresh_observed",
        expected_outcome="server_inventory_drop",
        client_milestones=BASE_CLIENT + ["inventory_slot_update", "inventory_sword_slot", "inventory_wool_slot", "inventory_drop_sent"],
        server_milestones=BASE_SERVER + ["server_inventory_drop"],
    ),
    InventoryRow(
        seam="Pickup semantics",
        receipt_path="docs/evidence/protocol-763-pickup.matrix.receipt.json",
        digest="bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8",
        window_kind="player_inventory",
        slot_class="hotbar_weapon_slot_36",
        click_mode="pickup_entity_collect",
        carried_stack="none",
        state_id="server_authoritative",
        expected_outcome="server_inventory_pickup",
        client_milestones=BASE_CLIENT + ["inventory_slot_update", "inventory_sword_slot", "inventory_wool_slot", "inventory_drop_sent", "inventory_pickup_seen"],
        server_milestones=BASE_SERVER + ["server_inventory_drop", "server_inventory_pickup"],
    ),
    InventoryRow(
        seam="Player-inventory click/container click",
        receipt_path="docs/evidence/protocol-763-click.matrix.receipt.json",
        digest="c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d",
        window_kind="player_inventory",
        slot_class="hotbar_wool_slot_37",
        click_mode="left_click",
        carried_stack="red_wool_63",
        state_id="fresh_observed",
        expected_outcome="server_inventory_click",
        client_milestones=BASE_CLIENT + ["inventory_slot_update", "inventory_sword_slot", "inventory_wool_slot", "inventory_drop_sent", "inventory_pickup_seen", "inventory_click_sent"],
        server_milestones=BASE_SERVER + ["server_inventory_drop", "server_inventory_pickup", "server_inventory_click"],
    ),
    InventoryRow(
        seam="Open-container semantics",
        receipt_path="docs/evidence/protocol-763-open-container.matrix.receipt.json",
        digest="b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3",
        window_kind="generic_3x3_container",
        slot_class="container_slot_0",
        click_mode="left_click",
        carried_stack="red_wool_63_to_empty",
        state_id="fresh_observed",
        expected_outcome="server_inventory_container_click",
        client_milestones=BASE_CLIENT + ["inventory_slot_update", "inventory_sword_slot", "inventory_wool_slot", "inventory_drop_sent", "inventory_pickup_seen", "inventory_click_sent", "inventory_open_container_seen", "inventory_container_click_sent"],
        server_milestones=BASE_SERVER + ["server_inventory_drop", "server_inventory_pickup", "server_inventory_click", "server_inventory_open_container", "server_inventory_container_click"],
    ),
    InventoryRow(
        seam="Block placement / use-item-on-block",
        receipt_path="docs/evidence/protocol-763-block-place.matrix.receipt.json",
        digest="9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122",
        window_kind="player_inventory",
        slot_class="hotbar_wool_slot_37",
        click_mode="use_item_on_block",
        carried_stack="red_wool_63",
        state_id="fresh_observed",
        expected_outcome="server_block_place",
        client_milestones=BASE_CLIENT + ["inventory_slot_update", "inventory_sword_slot", "inventory_wool_slot", "inventory_drop_sent", "inventory_block_place_sent"],
        server_milestones=BASE_SERVER + ["server_inventory_drop", "server_block_place"],
    ),
]


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


def validate_row(row: InventoryRow, receipt: dict[str, Any]) -> list[str]:
    issues: list[str] = []
    require_equal(issues, f"{row.seam}.status", receipt.get("status"), "pass")
    require_equal(issues, f"{row.seam}.mode", receipt.get("mode"), "run")
    require_equal(issues, f"{row.seam}.dry_run", receipt.get("dry_run"), False)

    scenario = as_object(receipt.get("scenario"), f"{row.seam}.scenario", issues)
    require_equal(issues, f"{row.seam}.scenario.name", scenario.get("name"), EXPECTED_SCENARIO)
    require_true(issues, f"{row.seam}.scenario.passed", scenario.get("passed"))
    observed = string_list(scenario.get("observed_milestones"), f"{row.seam}.scenario.observed_milestones", issues)
    missing = string_list(scenario.get("missing_milestones"), f"{row.seam}.scenario.missing_milestones", issues)
    forbidden_matches = string_list(scenario.get("forbidden_matches"), f"{row.seam}.scenario.forbidden_matches", issues)
    if missing:
        issues.append(f"{row.seam} has missing client milestones: {missing}")
    if forbidden_matches:
        issues.append(f"{row.seam} has forbidden matches: {forbidden_matches}")
    missing_client = missing_items(observed, row.client_milestones)
    if missing_client:
        issues.append(f"{row.seam} missing client milestones: {missing_client}")
    repeated_client = duplicates(observed)
    if repeated_client:
        issues.append(f"{row.seam} duplicate client milestones: {repeated_client}")
    invalid_acceptance = [item for item in FORBIDDEN_INVENTORY_ACCEPTANCE if item in observed or item in forbidden_matches]
    if invalid_acceptance:
        issues.append(f"{row.seam} invalid inventory acceptance observed: {invalid_acceptance}")

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
        issues.append(f"{row.seam} missing server milestones: {missing_server}")

    gameplay = as_object(receipt.get("gameplay_oracles"), f"{row.seam}.gameplay_oracles", issues)
    non_claims = string_list(gameplay.get("non_claims"), f"{row.seam}.gameplay_oracles.non_claims", issues)
    for item in missing_items(non_claims, REQUIRED_NON_CLAIMS):
        issues.append(f"{row.seam} missing non-claim: {item}")
    return issues


def validate_inventory_matrix(evidence: InventoryEvidence) -> list[str]:
    issues: list[str] = []
    for row in ROWS:
        receipt = evidence.receipts.get(row.receipt_path)
        if receipt is None:
            issues.append(f"missing receipt: {row.receipt_path}")
            continue
        issues.extend(validate_row(row, receipt))
        for token in [row.seam, row.receipt_path, row.digest]:
            if token not in evidence.matrix_text:
                issues.append(f"matrix missing token for {row.seam}: {token}")
        for token in [row.seam, row.digest]:
            if token not in evidence.bundle_text:
                issues.append(f"bundle missing token for {row.seam}: {token}")
        for token in [row.seam, row.window_kind, row.slot_class, row.click_mode, row.carried_stack, row.state_id, row.expected_outcome, row.digest]:
            if token not in evidence.doc_text:
                issues.append(f"inventory doc missing token for {row.seam}: {token}")
    for token in [
        "full inventory semantics remains a non-claim",
        "stale_state_accepted",
        "invalid_slot_accepted",
        "malformed_click_accepted",
        "state_corruption",
    ]:
        if token not in evidence.doc_text:
            issues.append(f"inventory doc missing required token: {token}")
    return issues


def load_repo_evidence() -> InventoryEvidence:
    return InventoryEvidence(
        receipts={row.receipt_path: json.loads((ROOT / row.receipt_path).read_text()) for row in ROWS},
        matrix_text=MATRIX.read_text(),
        bundle_text=BUNDLE.read_text(),
        doc_text=DOC.read_text(),
    )


def valid_receipt(row: InventoryRow) -> dict[str, Any]:
    return {
        "status": "pass",
        "mode": "run",
        "dry_run": False,
        "scenario": {
            "name": EXPECTED_SCENARIO,
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


def valid_fixture() -> InventoryEvidence:
    receipts = {row.receipt_path: valid_receipt(row) for row in ROWS}
    matrix = "\n".join(f"{row.seam} {row.receipt_path} {row.digest}" for row in ROWS)
    bundle = "\n".join(f"{row.seam} {row.digest}" for row in ROWS)
    doc = "\n".join(
        ["full inventory semantics remains a non-claim", *FORBIDDEN_INVENTORY_ACCEPTANCE]
        + [
            f"{row.seam} {row.window_kind} {row.slot_class} {row.click_mode} {row.carried_stack} {row.state_id} {row.expected_outcome} {row.digest}"
            for row in ROWS
        ]
    )
    return InventoryEvidence(receipts=receipts, matrix_text=matrix, bundle_text=bundle, doc_text=doc)


def assert_self_tests() -> None:
    issues = validate_inventory_matrix(valid_fixture())
    assert not issues, issues

    missing_client = valid_fixture()
    first_row = ROWS[0]
    missing_client.receipts[first_row.receipt_path]["scenario"]["observed_milestones"].remove(first_row.client_milestones[-1])
    issues = validate_inventory_matrix(missing_client)
    assert any("missing client milestones" in issue for issue in issues), issues

    missing_server = valid_fixture()
    missing_server.receipts[first_row.receipt_path]["server"]["observed_milestones"].remove(first_row.server_milestones[-1])
    issues = validate_inventory_matrix(missing_server)
    assert any("missing server milestones" in issue for issue in issues), issues

    wrong_protocol = valid_fixture()
    wrong_protocol.receipts[first_row.receipt_path]["server"]["protocol"] = 758
    issues = validate_inventory_matrix(wrong_protocol)
    assert any("server.protocol" in issue for issue in issues), issues

    stale_state = valid_fixture()
    stale_state.receipts[first_row.receipt_path]["scenario"]["observed_milestones"].append("stale_state_accepted")
    issues = validate_inventory_matrix(stale_state)
    assert any("invalid inventory acceptance" in issue for issue in issues), issues

    corrupted_state = valid_fixture()
    corrupted_state.receipts[first_row.receipt_path]["server"]["forbidden_matches"].append("state_corruption")
    issues = validate_inventory_matrix(corrupted_state)
    assert any("server forbidden matches" in issue for issue in issues), issues

    missing_doc = valid_fixture()
    missing_doc.doc_text = missing_doc.doc_text.replace("full inventory semantics remains a non-claim", "")
    issues = validate_inventory_matrix(missing_doc)
    assert any("inventory doc missing required token" in issue for issue in issues), issues


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run checker positive and negative fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("inventory semantics matrix self-test ok")
        return 0
    issues = validate_inventory_matrix(load_repo_evidence())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print(f"inventory semantics matrix ok: {len(ROWS)} bounded protocol-763 rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
