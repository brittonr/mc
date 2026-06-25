# decouple-mc-compat-runner-shell inventory

## Owner subtree

`compat/runner` owns the implementation. Review evidence and Cairn task citations live under `docs/evidence/` in the parent mc workspace.

## Requirement IDs and dependencies

| Requirement | Depends on | Acceptance criteria |
| --- | --- | --- |
| `r[mc_compatibility.runner_shell_decoupling.inventory]` | serial first task | Responsibility clusters, side effects, compatibility surfaces, and owners are listed before extraction. |
| `r[mc_compatibility.runner_shell_decoupling.module_boundaries]` | inventory | Crate-private module boundaries use explicit inputs/outputs and avoid raw CLI/env parsing outside the CLI shell. |
| `r[mc_compatibility.runner_shell_decoupling.pure_cores]` | module boundaries | Planning, scenario/evidence evaluation, typed-event graphing, and receipt shaping run as deterministic in-memory cores. |
| `r[mc_compatibility.runner_shell_decoupling.shell_modules]` | pure cores | Backend and client side effects are isolated in shell modules invoked from validated `Config`/plan data. |
| `r[mc_compatibility.runner_shell_decoupling.compatibility]` | shell modules | CLI behavior, aliases, dry-run receipts, receipt fields, non-claims, and generated surfaces remain stable. |
| `r[mc_compatibility.runner_shell_decoupling.tests]` | compatibility | Existing positive parity and negative fail-closed runner tests cover moved cores and shells. |
| `r[mc_compatibility.runner_shell_decoupling.validation]` | tests | Focused tests, dry-runs, generated/receipt checks, Cairn gates, validation, and task evidence pass before archive. |

## Responsibility clusters

| Cluster | Previous location | Module owner after split | Side effects | Compatibility surfaces |
| --- | --- | --- | --- | --- |
| CLI parsing and env/config loading | `main.rs` | `main.rs` shell plus existing `runtime_config`/`layout` cores | reads env/files for config | flags, env var names, scenario aliases, diagnostics |
| Execution orchestration | `main.rs` | `main.rs` shell | process sequencing, receipt/failure-bundle writes | mode behavior and error composition |
| Planning and cleanup plan evaluation | `main.rs` | `planning.rs` | none; in-memory `Config` to `HarnessPlan` | dry-run plan text, failure-bundle path diagnostics, non-claims |
| Scenario and server evidence evaluation | `main.rs` | `evidence_core.rs` | none; string/log inputs only | milestone names, forbidden markers, pass/fail semantics |
| Typed-event parsing/graphing/oracle | `main.rs` | `evidence_core.rs` | none; receipt evidence to event graph data | typed-event schema, event names, graph diagnostics, pass/fail gating |
| Projectile causality evaluation | `main.rs` | `evidence_core.rs` | none; client/server log slices only | required step names, ordering diagnostics |
| Receipt JSON shaping and triage | `main.rs` | `receipts.rs` | none; returns JSON strings | receipt schemas/fields, non-claim fields, triage fields |
| Valence/Paper backend lifecycle | `main.rs` | `backend_shell.rs` | git, docker, cargo, sockets, pid files, logs | backend names, ports, log labels, dry-run command shape |
| Client driving and MCP control | `main.rs` | `client_driver.rs` | client process, Xvfb, MCP stdio, temp logs, restart orchestration | client classification, log paths, MCP control evidence |
| Wire helpers | `main.rs` | `wire.rs` | in-memory `Read`/`Write` only | Minecraft status packet framing bytes |
| JSON field/string helpers | `main.rs` | `main.rs` shared helpers | none | receipt/schema parsing compatibility |
| Tests | `main.rs` | unchanged test module using crate-private module APIs | temp files/git fixtures in test-only code | positive parity and fail-closed fixtures |

## Boundary notes

The split keeps `Config`, receipt evidence structs, and scenario behavior contracts crate-private in `main.rs` to avoid widening public API. New modules are crate-private and receive explicit `Config`, plan, evidence, or log inputs. Filesystem, process, socket, clock, stdout/stderr, Docker, and Git calls are confined to `main.rs`, `backend_shell.rs`, and `client_driver.rs`; `planning.rs`, `evidence_core.rs`, `receipts.rs`, and `wire.rs` contain deterministic in-memory logic.

## Non-claims preserved

This change is structural only. It does not claim broad Minecraft compatibility, semantic equivalence, production readiness, public-server safety, full CTF correctness, or full survival correctness.
