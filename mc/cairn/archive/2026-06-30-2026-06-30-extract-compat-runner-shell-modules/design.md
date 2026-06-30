# Design: mc-compat runner shell modules

## Context

The runner already has pure modules for scenario data, runtime config, evidence parsing, receipts, planning, and layout, but `lib.rs` remains a large shell. Modularization should keep `main.rs` and `run_main()` stable while making side-effect boundaries visible.

## Decisions

### 1. Keep `lib.rs` as a façade

**Choice:** Leave `run_main()` and public crate entrypoints in `lib.rs`, then delegate to focused modules.

**Rationale:** Flake apps and tests keep their entrypoint while implementation modules can be reviewed independently.

### 2. Separate CLI parsing from execution

**Choice:** Move legacy flags, scenario router parsing, config patch discovery, and usage rendering into a CLI/config module that returns data structures.

**Rationale:** Parser behavior can be tested with string vectors without launching servers or clients.

### 3. Separate environment patch planning from command mutation

**Choice:** Keep pure env patch construction separate from `Command` mutation and Paper Docker argument application.

**Rationale:** Environment decisions are high-risk compatibility surfaces and should be deterministic and fixture-testable.

### 4. Separate artifact rendering from writes

**Choice:** Keep receipt/failure-bundle model construction and path selection pure, with tiny shells for filesystem writes and BLAKE3 file hashing.

**Rationale:** Reviewers can validate receipt and failure-bundle shape without relying on transient target output.

## Risks / Trade-offs

- Some modules currently import helper constants from `lib.rs`; extraction should move constants with their owning domain instead of widening root imports.
- The runner has many compatibility shims; keep aliases intact unless a separate Cairn retires them.
- Failure-bundle and receipt file writes are review-sensitive; retain existing parent-dir and path-safety diagnostics.
