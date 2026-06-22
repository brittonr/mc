# Steel runtime config contract — 2026-05-27

## Scope

This checkpoint implements the first Steel-first configuration slice for `hotload-runtime-configuration`. It does not claim full runtime migration or live file watching yet. It does define the reviewable inventory, Steel module contract, typed boundary, normalized snapshot shape, and arrow-damage policy contract that later runtime code must consume.

## Source files

- Inventory: `docs/evidence/runtime-config-inventory-2026-05-27.tsv`
- Steel module: `compat/config/steel/default.scm`
- Normalized snapshot: `docs/evidence/steel-runtime-config-default.snapshot.json`
- Checker: `tools/check_runtime_steel_config.rs`
- Pure runtime core/restricted evaluator: `tools/mc-compat-runner/src/runtime_config.rs`
- Startup shell: `tools/mc-compat-runner/src/main.rs` (`--steel-config` / `MC_COMPAT_STEEL_CONFIG`)

## Steel module contract

The editable config source is a Steel module using sandbox profile `mc-compat/pure-v1`.

Required value exports:

- `config-version`
- `sandbox-profile`
- `server-backend`
- `server-version`
- `server-protocol`
- `server-port`
- `valence-rev`
- `valence-example`
- `valence-worktree`
- `valence-target-dir`
- `valence-log`
- `valence-pid-file`
- `client-username`
- `client-timeout-secs`
- `client-success-patterns`
- `receipt-dir`
- `scenario`
- `arrow-base-damage`
- `arrow-velocity-multiplier`
- `arrow-max-damage`

Required policy export:

- `arrow-damage`

The `arrow-damage` policy must call the host-provided pure helper `damage-linear` with explicit `ctx`, `arrow-base-damage`, `arrow-velocity-multiplier`, and `arrow-max-damage` arguments. This gives Steel the editable policy expression while Rust owns validation and final apply.

## Rust-owned typed boundary

The Rust boundary must validate Steel exports before use. `mc-compat-runner` now applies Steel exports at startup through `--steel-config` / `MC_COMPAT_STEEL_CONFIG` for server backend/version/protocol/port, Valence rail fields, client username/timeout/success patterns, receipt dir, scenario, and arrow-damage policy fields. Later environment variables and CLI flags still have precedence.


| Path | Type | Contract | Mutability |
| --- | --- | --- | --- |
| `runtime.config_version` | `u32` | supported schema version | restart-only |
| `runtime.steel.sandbox_profile` | `String` | `mc-compat/pure-v1` | restart-only |
| `server.backend` | enum | `valence` or `paper` | next-run |
| `server.version` | `String` | semantic Minecraft version label | next-run |
| `server.protocol` | `u32` | scenario-supported protocol | next-run |
| `server.port` | `u16` | `1..=65535` | restart-only |
| `valence.rev` | `String` | revision or branch label | next-run |
| `valence.example` | `String` | Valence example name | next-run |
| `valence.worktree` | path | checkout path | next-run |
| `valence.target_dir` | path | cargo target dir path | next-run |
| `valence.log` | path | log path | hot |
| `valence.pid_file` | path | pid file path | restart-only |
| `client.username` | `String` | nonempty username | next-run |
| `client.timeout_secs` | `u64` | positive seconds | hot |
| `client.success_patterns` | `Vec<String>` | nonempty strings | hot |
| `receipt.dir` | path | receipt directory path | next-run |
| `scenario.name` | enum | supported scenario name | next-run |
| `combat.arrow.base_damage` | `f64` | `0.0..=100.0` | hot |
| `combat.arrow.velocity_multiplier` | `f64` | `0.0..=100.0` | hot |
| `combat.arrow.max_damage` | `f64` | `0.0..=100.0` | hot |

The checker requires every inventory row to declare one of: `hot`, `next-run`, `restart-only`, or `fixed-protocol-fact`.

## Arrow damage policy contract

Input context, provided by Rust only:

- `projectile_velocity: f64`
- `pull_strength: f64`
- `attacker_team: enum`
- `victim_team: enum`
- `scenario: enum`

Output decision, validated by Rust:

- `damage: f64`, bounded to `0.0..=100.0`
- `policy: String`
- `clamped: bool`

Representative formula:

```scheme
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
```

## Sandbox contract

`mc-compat/pure-v1` allows only:

- literal data exports;
- pure arithmetic inside policy hooks;
- explicit host helpers such as `damage-linear`;
- explicit context reads supplied by Rust.

It forbids:

- filesystem access;
- network access;
- process spawning;
- ambient wall-clock reads;
- randomness;
- mutation of live Rust state;
- unbounded evaluation.

## Normalized snapshot

`docs/evidence/steel-runtime-config-default.snapshot.json` records:

- schema version;
- source module path;
- Steel module BLAKE3;
- sandbox profile;
- evaluated exports;
- policy export names;
- redacted fields;
- mutability summary.

## Positive validation

`tools/check_runtime_steel_config.rs --self-test` accepts:

- required inventory rows;
- valid mutability classes;
- default Steel module exports;
- arrow-damage policy shape;
- normalized snapshot referencing the default module and hash;
- path-specific agreement between inventory mutability, contract rows, and `mutability_summary` buckets.

## Negative validation

`tools/check_runtime_steel_config.rs --self-test` rejects:

- missing inventory rows;
- unknown mutability classes;
- missing required Steel exports;
- forbidden sandbox capability tokens;
- invalid arrow-damage policy shape;
- path moved to the wrong snapshot `mutability_summary` bucket;
- snapshot/module hash mismatch.

## Current non-claims

No filesystem watcher, Valence server-side Steel integration, remote config distribution, or full hot-reload rollout is claimed. The archived slice is startup-time restricted Steel module loading, a runner-side arrow-damage policy path for the projectile dry-run/evidence rail, and an explicit reload-request controller with rollback-safe hot apply handlers. Archive record: `cairn/archive/2026-05-27-hotload-runtime-configuration/tasks.md`.
