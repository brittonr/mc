# Proposal: Refresh RED BLUE scoring soak live evidence

## Why

The RED/BLUE scoring soak row is currently backed by a historical oracle because the original mutable target receipt was overwritten. A fresh live evidence refresh would replace the exception with reviewable copied receipt/log artifacts.

## What Changes

- Add `red-blue-scoring-soak-live-refresh` as a row-scoped Cairn for one fresh live rerun of the maintained RED and BLUE scoring soak rails with copied receipts, run logs, and BLAKE3 manifests.
- Define normalized metrics: scenario status, RED score milestone, BLUE score milestone, server score path milestones, missing milestone lists, forbidden score/capture patterns, child revisions, receipt digests, and run-log digests.
- Require evidence standard: live Valence receipts/logs copied under docs/evidence with BLAKE3 manifests plus acceptance matrix/current bundle hash updates.
- Reject bad evidence and overclaims: historical target-only evidence, missing copied receipt, digest mismatch, missing child revisions, missing server correlation, unexpected score/capture, or broad CTF overclaim.
- Update docs only after validation, preserving explicit non-claims.

## Impact

- **Files**: runner/client probes, fixtures/checkers, evidence docs/manifests, acceptance matrix/current bundle, and Cairn specs/tasks as applicable.
- **Validation**: row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: full CTF correctness, production load, public-server safety, unbounded soak, broad Minecraft compatibility, and unrelated CTF rule rows.
