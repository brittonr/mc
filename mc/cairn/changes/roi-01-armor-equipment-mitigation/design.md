# Design: Armor/equipment mitigation rail

## Approach

Prove the next bounded combat semantic frontier after melee damage and knockback: armor/equipment mitigation in local Valence CTF without broad combat claims.

Use the existing maintained compatibility pattern:

1. Extend `tools/mc-compat-runner` with a narrowly named scenario or receipt field when runtime evidence is required.
2. Add only the Valence/Stevenarella instrumentation needed for deterministic milestones.
3. Wire a Nix app/check for dry-run shape validation when a new rail is introduced.
4. Record tracked Markdown evidence under `docs/evidence/` with receipt path, BLAKE3, commits, covered claims, and non-claims.
5. Update the acceptance matrix/current evidence bundle only if the slice creates a new maintained seam.

## Verification

- Runner `cargo fmt --check && cargo test` when runner code changes.
- Relevant Valence/Stevenarella focused checks when child instrumentation changes.
- New or updated Nix dry-run check for scenario shape/receipt contract.
- Live app run and BLAKE3 for runtime evidence slices.
- `python3 tools/check_acceptance_matrix.py` and/or `python3 tools/check_current_evidence_bundle.py` when evidence indexes change.
- `nix run .#cairn -- validate --root .` before archive/commit.
- `git diff --check` before commit.

## Risks

- Avoid strengthening existing receipts silently; new semantics should be visible as new scenario names or explicit receipt fields.
- Avoid false positives from unrelated packets/logs by requiring scenario-specific active probe guards and server/client correlation.
- Keep non-claims explicit when evidence proves only a bounded local rail.
