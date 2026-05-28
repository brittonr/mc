# Design: Steel-first hotloaded runtime configuration

## Configuration language decision

Steel Scheme is the primary editable runtime configuration and policy language. Operators edit Steel modules that export typed config values and optional policy functions. This supports both static values and hotloaded behavior, such as computing projectile damage from a host-provided context.

Rust remains the trusted boundary. Steel modules do not get direct authority to mutate live state. They return values or decisions into Rust-owned types, and Rust validates contracts, mutability, sandbox permissions, determinism, and apply plans before anything changes.

Nickel is not the initial source language for this change. It can remain a future static import/export option, but the first implementation should avoid maintaining two config languages.

## Steel module shape

A runtime module should have explicit exports, for example:

- `config-version`;
- data exports such as `network`, `scenario`, `combat`, `evidence`, and `clients`;
- policy exports such as `arrow-damage` only when the value is real logic rather than static data;
- metadata exports for docs, provenance, and mutability hints.

Host functions available to Steel must be minimal and pure. Policy hooks receive explicit context records and return explicit decision records. For arrow damage, Steel should receive a `ProjectileDamageContext` and return a `ProjectileDamageDecision`; Rust clamps, validates, logs, and applies the result.

## Functional core

The pure core should own:

- typed config and policy-export contracts;
- normalization from evaluated Steel exports to typed domain config;
- defaulting and validation;
- sandbox permission descriptors;
- redaction rules;
- diff calculation between current and candidate snapshots;
- apply-plan generation with field mutability classes;
- rollback decision logic.

The core takes plain data and returns either typed config, diagnostics, or an apply plan. It does not watch files, read environment variables, open sockets, write logs, evaluate Steel, or mutate live state.

## Imperative shell

The shell should own:

- loading Steel modules from disk;
- compiling/evaluating candidates in an isolated sandbox;
- installing host functions with explicit capability descriptors;
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

## Sandbox and determinism

Steel hot reload is allowed only through a constrained evaluator:

- no ambient filesystem, network, process, wall-clock, or randomness access;
- no mutation of Rust live state from Steel;
- bounded evaluation steps or fuel;
- bounded memory where supported;
- explicit host functions only;
- deterministic inputs and outputs;
- typed return values;
- versioned hook names and context schemas.

Any sandbox or determinism violation fails reload and keeps the previous snapshot active.

## Hot reload behavior

Reload is validate-before-swap:

- read candidate Steel module into an isolated candidate evaluator;
- compile/evaluate with bounded capabilities;
- validate schema version, exports, types, ranges, required fields, unknown exports, sandbox permissions, and field mutability;
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
- BLAKE3 hash of the Steel module and evaluated normalized snapshot;
- source file paths or overlay identifiers;
- generation/evaluation command;
- sandbox profile;
- evaluation timestamp or monotonic generation identifier;
- redacted fields list;
- mutability classification summary.

BLAKE3 is the default hash because this repo uses it for evidence and integrity receipts.

## Testing strategy

Tests must include positive and negative cases.

Positive tests:

- valid default Steel module normalizes;
- module edit changes a hot value;
- hot reload applies a safe value atomically;
- arrow-damage policy receives explicit context and returns a valid decision;
- next-run and restart-only changes are reported without live mutation;
- redacted fields do not appear in logs.

Negative tests:

- unknown export rejected;
- missing required export rejected;
- wrong type rejected;
- invalid range rejected;
- malformed Steel module rejected;
- forbidden host capability rejected;
- nondeterministic policy attempt rejected;
- hot apply failure rolls back;
- restart-only field cannot be hot-applied;
- policy output rejected if it violates typed bounds.

## Evidence strategy

Implementation must land with:

- config inventory under `docs/evidence/`;
- Steel module contract docs and sample modules;
- checker that validates inventory, Steel exports, typed contracts, snapshot, and migrated call sites agree;
- reload test receipts or run logs;
- BLAKE3 sidecars for review-critical evidence;
- updated tasks showing exactly which values are migrated.

## Risks / Trade-offs

- Steel can hide control flow and make config harder to review. Mitigate with explicit exports, small modules, typed contracts, diffs, and evidence receipts.
- Hot reload can hide partial state bugs. Mitigate with pure apply-plan tests, explicit mutability classes, and all-or-nothing apply semantics.
- Sandbox holes can become security bugs. Mitigate with no ambient capabilities and narrow host APIs.
- Configuring protocol facts can create invalid states. Mitigate by classifying protocol facts as documented constants, not hot fields.
