#!/usr/bin/env python3
"""Validate vanilla combat parity gate and fixtures."""
from __future__ import annotations

import argparse
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs" / "evidence" / "protocol-763-vanilla-combat-parity-2026-05-27.md"
MATRIX = ROOT / "docs" / "evidence" / "protocol-763-acceptance-matrix.md"
BUNDLE = ROOT / "docs" / "evidence" / "protocol-763-current-evidence-bundle.md"

REFERENCE_ORACLE_NAME = "paper-1.20.1-reference-harness"
REFERENCE_VERSION = "minecraft-1.20.1-protocol-763"
PARITY_NON_CLAIM = "exact vanilla combat parity remains a non-claim"
VALENCE_ONLY_REJECTION = "rejects Valence-only evidence"
REQUIRED_METRICS = [
    "damage_delta_half_hearts",
    "knockback_velocity_vector",
    "armor_mitigation_delta",
    "projectile_damage_delta",
]
REQUIRED_NEGATIVE_FIXTURES = [
    "missing_reference",
    "wrong_reference_version",
    "out_of_tolerance",
    "valence_only_evidence",
]
TOLERANCE_EPSILON = 0.001


@dataclass(frozen=True)
class ParityCase:
    metric: str
    reference_value: float | None
    valence_value: float | None
    tolerance: float
    reference_version: str | None
    has_reference_receipt: bool
    has_valence_receipt: bool


@dataclass(frozen=True)
class ParityDecision:
    passed: bool
    diagnostics: tuple[str, ...]


def evaluate_parity(case: ParityCase, expected_version: str = REFERENCE_VERSION) -> ParityDecision:
    diagnostics: list[str] = []
    if case.metric not in REQUIRED_METRICS:
        diagnostics.append(f"unknown_metric:{case.metric}")
    if not case.has_reference_receipt:
        diagnostics.append("missing_reference")
    if not case.has_valence_receipt:
        diagnostics.append("missing_valence")
    if case.reference_version != expected_version:
        diagnostics.append("wrong_reference_version")
    if case.reference_value is None:
        diagnostics.append("missing_reference_value")
    if case.valence_value is None:
        diagnostics.append("missing_valence_value")
    if case.reference_value is not None and case.valence_value is not None:
        delta = abs(case.reference_value - case.valence_value)
        if delta - case.tolerance > TOLERANCE_EPSILON:
            diagnostics.append("out_of_tolerance")
    if not case.has_reference_receipt and case.has_valence_receipt:
        diagnostics.append("valence_only_evidence")
    return ParityDecision(passed=not diagnostics, diagnostics=tuple(diagnostics))


def validate_doc(doc_text: str, matrix_text: str, bundle_text: str) -> list[str]:
    issues: list[str] = []
    for token in [
        REFERENCE_ORACLE_NAME,
        REFERENCE_VERSION,
        PARITY_NON_CLAIM,
        VALENCE_ONLY_REJECTION,
        "decision_owner: agent",
        "reference_receipt: none",
        "valence_reference_pair: none",
        *REQUIRED_METRICS,
        *REQUIRED_NEGATIVE_FIXTURES,
    ]:
        if token not in doc_text:
            issues.append(f"parity doc missing token: {token}")
    forbidden_claims = [
        "Exact vanilla combat parity |",
        "vanilla parity covered",
        "exact vanilla parity covered",
    ]
    for token in forbidden_claims:
        if token in matrix_text or token in bundle_text:
            issues.append(f"forbidden parity promotion token present: {token}")
    return issues


def assert_self_tests() -> None:
    positive = evaluate_parity(
        ParityCase(
            metric="damage_delta_half_hearts",
            reference_value=2.0,
            valence_value=2.0,
            tolerance=0.0,
            reference_version=REFERENCE_VERSION,
            has_reference_receipt=True,
            has_valence_receipt=True,
        )
    )
    assert positive.passed, positive

    within_tolerance = evaluate_parity(
        ParityCase(
            metric="armor_mitigation_delta",
            reference_value=2.0,
            valence_value=2.0005,
            tolerance=0.001,
            reference_version=REFERENCE_VERSION,
            has_reference_receipt=True,
            has_valence_receipt=True,
        )
    )
    assert within_tolerance.passed, within_tolerance

    missing_reference = evaluate_parity(
        ParityCase(
            metric="damage_delta_half_hearts",
            reference_value=None,
            valence_value=2.0,
            tolerance=0.0,
            reference_version=REFERENCE_VERSION,
            has_reference_receipt=False,
            has_valence_receipt=True,
        )
    )
    assert not missing_reference.passed and "missing_reference" in missing_reference.diagnostics, missing_reference
    assert "valence_only_evidence" in missing_reference.diagnostics, missing_reference

    wrong_version = evaluate_parity(
        ParityCase(
            metric="damage_delta_half_hearts",
            reference_value=2.0,
            valence_value=2.0,
            tolerance=0.0,
            reference_version="minecraft-1.18.2-protocol-758",
            has_reference_receipt=True,
            has_valence_receipt=True,
        )
    )
    assert not wrong_version.passed and "wrong_reference_version" in wrong_version.diagnostics, wrong_version

    out_of_tolerance = evaluate_parity(
        ParityCase(
            metric="knockback_velocity_vector",
            reference_value=1.0,
            valence_value=1.25,
            tolerance=0.01,
            reference_version=REFERENCE_VERSION,
            has_reference_receipt=True,
            has_valence_receipt=True,
        )
    )
    assert not out_of_tolerance.passed and "out_of_tolerance" in out_of_tolerance.diagnostics, out_of_tolerance

    valence_only = evaluate_parity(
        ParityCase(
            metric="projectile_damage_delta",
            reference_value=None,
            valence_value=3.0,
            tolerance=0.0,
            reference_version=REFERENCE_VERSION,
            has_reference_receipt=False,
            has_valence_receipt=True,
        )
    )
    assert not valence_only.passed and "valence_only_evidence" in valence_only.diagnostics, valence_only


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="run positive and negative parity fixtures")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.self_test:
        assert_self_tests()
        print("vanilla combat parity self-test ok")
        return 0
    assert_self_tests()
    issues = validate_doc(DOC.read_text(), MATRIX.read_text(), BUNDLE.read_text())
    if issues:
        for issue in issues:
            print(issue, file=sys.stderr)
        return 1
    print("vanilla combat parity ok: no paired parity rows promoted")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
