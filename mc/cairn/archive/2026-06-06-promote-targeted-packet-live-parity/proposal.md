# Proposal: Promote targeted packet rows to live parity

## Why

The targeted packet promotion batch raised selected protocol 763 rows to deterministic fixture-backed evidence, but the evidence deliberately stayed `scenario_bounded` and did not claim live Paper/Valence parity. The next compatibility step should exercise a small, reviewable subset through live runner probes so packet rows can advance only when real server/client behavior supports the claim.

## What Changes

- Select a bounded subset from the existing targeted packet promotion rows for live parity attempts.
- Add or extend runner scenarios/probes that exercise the selected packet behavior against local Paper and Valence paths where applicable.
- Record receipts/logs that identify packet rows, scenario names, backend revisions, and non-claims.
- Promote matrix rows only when live evidence passes, while preserving fixture-only status for rows not exercised live.

## Impact

- **Files**: `tools/mc-compat-runner`, evidence matrix/bundle docs, targeted packet checker, new evidence receipts/logs, and possibly Valence/Paper fixture setup.
- **Testing**: baseline fixture checks, live probe checks or documented local-run receipts, positive/negative targeted-packet checker tests, evidence-manifest/task-evidence checks, and Cairn gates/validation.