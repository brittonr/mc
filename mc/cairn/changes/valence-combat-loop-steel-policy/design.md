# Design: Valence combat-loop Steel arrow policy

## Boundary and ownership

The Valence combat loop must not evaluate arbitrary Steel during entity/event processing. This change uses a restricted Steel-compatible literal module normalizer in the runtime shell when a startup load or explicit reload request is received. The shell produces a typed `ArrowDamagePolicySnapshot`; active combat reads only the latest atomically published Rust snapshot. Full Steel evaluator execution is not claimed in this slice.

Rust remains the authority for:

- accepted Steel-compatible exports and policy hook names;
- context and decision schemas;
- numeric ranges and clamping behavior;
- provenance and redaction metadata;
- reload mutability and publish/rollback rules;
- default fallback behavior.

Steel owns only the editable policy expression and static numeric parameters.

## Functional core

The pure core should expose small deterministic functions:

- normalize a restricted Steel-compatible arrow policy candidate into `ArrowDamagePolicySnapshot` data;
- build an `ArrowDamageContext` from explicit combat inputs;
- evaluate the active policy against context into an `ArrowDamageDecision`;
- validate decision range and policy metadata;
- compute redacted diffs between policy snapshots;
- decide whether a candidate can be published or must be rejected.

The core does not read files, inspect environment variables, mutate Valence state, log, spawn tasks, or touch global policy storage. It accepts plain data and returns typed output or diagnostics.

## Imperative shell

The shell owns side effects:

- locating the Steel-compatible module path from an explicit CLI/env/runtime setting;
- reading the module text;
- rejecting forbidden capability tokens and normalizing only the supported literal export subset;
- invoking the pure core;
- publishing the new policy through a single atomic swap only after validation succeeds;
- keeping the previous snapshot on normalization, validation, capability-token, or apply failure;
- emitting milestone/evidence records with module hash, snapshot generation, policy name, redacted diff, and rejection diagnostics.

The first implementation should use explicit reload requests, not a filesystem watcher. A later change can add watching after this path is proven.

## Combat-loop integration

Two Valence CTF projectile-probe call sites currently consume fixed projectile damage:

- melee/combat event path when `projectile_probe_hit` is true;
- `handle_projectile_events` interaction path that simulates projectile use/hit.

Both call sites should route through one helper that takes an explicit `ArrowDamageContext` and the current published snapshot. The helper returns a typed decision containing at least damage, policy id, clamped flag, and snapshot generation/hash. Milestone lines must include enough data to prove the server used the published policy rather than the old constant.

The old `PROJECTILE_PROBE_DAMAGE` value remains only as the default policy parameter and test oracle for the default snapshot. It must not be a hidden alternate runtime path once a valid snapshot is published.

## Atomicity and rollback

Reload behavior is all-or-nothing:

1. Read candidate Steel-compatible module text.
2. Reject forbidden capability tokens and unsupported policy shape.
3. Normalize supported literal exports to a typed snapshot.
4. Validate context/decision contract by running representative sample contexts.
5. Atomically publish the new snapshot.
6. Emit success evidence.

If any step fails, emit diagnostics and leave the previous snapshot active. Combat reads must never observe partial candidate state.

## Evidence strategy

Evidence must be stronger than token presence. The checker should validate:

- inventory row for `combat.arrow.*` marks Valence combat-loop consumer as migrated;
- Steel-compatible module exports and typed Rust policy fields agree on config path and mutability;
- Valence call-site list names both projectile-probe consumers;
- milestone/evidence receipt shows non-default Steel damage in live Valence projectile combat;
- negative reload evidence shows a malformed or range-invalid policy rejected with the old snapshot still active.

Review-critical receipts and BLAKE3 manifests belong under `docs/evidence/`.

## Risks / trade-offs

- Evaluating policy in the combat loop would be simpler but unsafe. Avoid this by publishing typed snapshots only.
- A global mutable policy handle can hide ordering bugs. Mitigate with a single publish API, generation ids, and tests that assert rollback preserves generation.
- Projectile-probe behavior is bounded compatibility behavior, not vanilla parity. Evidence and spec wording must keep that boundary explicit.
