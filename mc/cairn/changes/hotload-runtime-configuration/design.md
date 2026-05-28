# Design: Hotloaded runtime configuration

## Configuration language decision

Nickel is the primary configuration language because it gives typed contracts, defaults, documentation, and merge semantics at the config boundary. Operators edit Nickel overlays. CI and runtime tooling export a normalized machine-readable snapshot for Rust code to load.

Steel Scheme is not the initial config language. It may become a sandboxed policy/plugin layer only after the config schema exists and after a dedicated proof shows deterministic evaluation, bounded capabilities, audit logging, and no hidden state mutation. Steel code must return typed values that pass the same Nickel-derived schema or normalized snapshot contract.

## Functional core

The pure core should own:

- config schema representation;
- normalization from raw evaluated config to typed domain config;
- defaulting and validation;
- redaction rules;
- diff calculation between current and candidate snapshots;
- apply-plan generation with field mutability classes;
- rollback decision logic.

The core takes plain data and returns either typed config, diagnostics, or an apply plan. It does not watch files, read environment variables, open sockets, write logs, or mutate live state.

## Imperative shell

The shell should own:

- loading Nickel/exported snapshots from disk;
- optional Nickel CLI or library invocation;
- filesystem watching or explicit reload command handling;
- publishing reload events;
- applying approved hot fields to live systems;
- scheduling next-run or restart-required changes;
- writing evidence receipts and operator logs.

## Configuration inventory

The first implementation task is an inventory. Each row records name, owner, source file or command, current default, type, contract, mutability, runtime consumer, evidence path, and migration status. Rows must classify values as:

- hot: safe to apply atomically while the process runs;
- next-run: safe only for the next scenario/run;
- restart-only: requires process restart or server/client reinitialization;
- fixed-protocol-fact: documented, not operator-configurable.

## Hot reload behavior

Reload is validate-before-swap:

- read candidate config into an isolated candidate snapshot;
- validate schema version, types, ranges, required fields, unknown fields, and field mutability;
- compute a redacted diff;
- build an apply plan;
- apply hot fields through explicit handlers;
- publish the new snapshot only after every handler succeeds;
- keep the previous snapshot active if validation or apply fails;
- record rollback diagnostics and operator-visible reason.

No partial reload is allowed. If any hot apply step fails, the old snapshot remains authoritative.

## Snapshot and provenance

Every normalized snapshot carries:

- schema version;
- BLAKE3 hash of the evaluated snapshot;
- source file paths or overlay identifiers;
- generation command;
- evaluation timestamp or monotonic generation identifier;
- redacted fields list;
- mutability classification summary.

BLAKE3 is the default hash because this repo uses it for evidence and integrity receipts.

## Testing strategy

Tests must include positive and negative cases.

Positive tests:

- valid default config normalizes;
- overlay merge changes a hot value;
- hot reload applies a safe value atomically;
- next-run and restart-only changes are reported without live mutation;
- redacted fields do not appear in logs.

Negative tests:

- unknown field rejected;
- missing required field rejected;
- wrong type rejected;
- invalid range rejected;
- malformed Nickel/exported snapshot rejected;
- hot apply failure rolls back;
- restart-only field cannot be hot-applied;
- Steel-derived value rejected unless it passes the typed boundary.

## Evidence strategy

Implementation must land with:

- config inventory under `docs/evidence/`;
- Nickel schema and sample overlays;
- checker that validates inventory/schema/snapshot agreement;
- reload test receipts or run logs;
- BLAKE3 sidecars for review-critical evidence;
- updated tasks showing exactly which values are migrated.

## Risks / Trade-offs

- Nickel evaluation at runtime can add tooling dependency and latency. Mitigate by loading normalized snapshots and treating direct Nickel evaluation as an operator/dev convenience until proven safe.
- Hot reload can hide partial state bugs. Mitigate with pure apply-plan tests, explicit mutability classes, and all-or-nothing apply semantics.
- Steel Scheme can become hidden control flow. Mitigate by keeping it out of initial config and requiring a typed boundary plus separate proof before enabling it.
- Configuring protocol facts can create invalid states. Mitigate by classifying protocol facts as documented constants, not hot fields.
