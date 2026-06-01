# Proposal: Full CTF correctness aggregate gate

## Why

Full CTF correctness remains a non-claim. An aggregate gate should block broad CTF claims until all rule-family rows have live evidence and negative coverage.

## What Changes

- Add `full-ctf-correctness-gate` as a row-scoped Cairn for an aggregate checker over CTF rule ledger rows requiring every configured rule family to be covered before full CTF correctness can be claimed.
- Define normalized metrics: rule family, status, receipt path, run log path, BLAKE3 manifest, forbidden-transition checks, negative fixture coverage, and current-bundle label.
- Require evidence standard: CTF ledger aggregate checker with negative fixtures for missing rule families and premature full-CTF claims.
- Reject bad evidence and overclaims: missing rule family, missing receipt, missing forbidden scan, missing negative fixture, stale non-claim text, or full CTF overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: full CTF correctness until all rule rows pass, production gameplay readiness, public-server safety, and broad Minecraft compatibility.
