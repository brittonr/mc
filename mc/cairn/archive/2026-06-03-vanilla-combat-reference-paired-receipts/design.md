# Design: Paired vanilla combat reference receipts

## Goal

Promote one narrow combat reference-parity row only when the repo contains paired Paper-reference and Valence evidence for the same deterministic melee interaction. The design keeps comparison logic pure and keeps runner, filesystem, and receipt writes in a thin shell.

## Evidence contract

The row is `vanilla-combat-reference-parity`. The configured interaction is a two-client melee hit in an owned-local fixture. The record must name:

- `attacker_identity`
- `victim_identity`
- `weapon`
- `armor_state`
- `pre_health`
- `post_health`
- `damage_delta`
- `knockback_metric`
- `tolerance_bounds`
- `reference_oracle`
- `reference_version`
- `valence_revision`
- `client_revision`

A Paper-reference oracle is acceptable only when the receipt and docs label it as `paper-1.20.1-reference-harness` and keep exact Mojang vanilla parity as a non-claim. If a direct Mojang/vanilla oracle is later substituted, the checker should accept it through a named allow-list rather than by free-form text.

## Functional core

Add a Rust core for the parity checker:

- `CombatParityRecord`: normalized in-memory metric record for one backend.
- `CombatParityPair`: reference record plus Valence record.
- `compare_combat_parity(pair, contract) -> CombatParityDecision`: pure deterministic comparator.
- `CombatParityDecision`: pass/fail plus stable diagnostics.

The pure comparator rejects missing reference evidence, missing Valence evidence, wrong reference version, missing metrics, missing tolerance bounds, out-of-tolerance damage, out-of-tolerance knockback, stale or dirty required revisions, and backend/row mismatches. Positive fixtures cover exact damage equality and within-tolerance knockback. Negative fixtures cover missing reference, Valence-only evidence, wrong reference version, missing tolerance, out-of-tolerance damage, out-of-tolerance knockback, stale revision, and mismatched weapon/armor state.

## Imperative shell

The runner and checker shell own I/O:

- runner starts the selected backend, configures the fixture, runs the Stevenarella clients, captures logs, and writes receipts;
- Paper fixture logs the same normalized metric vocabulary as Valence;
- Valence fixture logs normalized metric vocabulary without altering existing CTF combat scenarios;
- checker CLI reads receipt or `.kv` evidence, calls the pure comparator, prints stable diagnostics, and exits non-zero on any mismatch;
- evidence promotion copies only explicit receipt/log paths under `docs/evidence/` and writes BLAKE3 manifests.

## Promotion rule

Matrix and bundle docs may mark only `vanilla-combat-reference-parity` as covered, and only for the configured Paper-reference interaction, after the Rust checker passes against paired reviewable evidence. Exact Mojang vanilla parity remains false unless the reference oracle is changed to a direct Mojang/vanilla oracle and that evidence is added under the same gate.

## Validation

Closeout must record checker self-tests, paired comparator output, evidence manifest check, task-evidence gate, Cairn proposal/design/tasks gates, and Cairn validation under `docs/evidence/`. Any archive must preserve the non-claim wording for all adjacent combat and production claims.
