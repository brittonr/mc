# Design: Decouple mc-compat runner shell

## Context

The architecture docs already define `compat/runner/src/main.rs` as the imperative shell. The code has partially extracted scenario catalog, scenario core, runtime config, layout, and receipt validation, but the remaining `main.rs` still mixes pure decisions with side effects. The goal is a structural split, not behavior expansion.

## Decisions

### 1. Split by side-effect boundary

Move deterministic functions first: planning, milestone evaluation, typed-event graph evaluation, receipt structs/rendering, and failure-bundle validation. Keep command execution, Docker, sockets, filesystem reads/writes, clocks, stdout/stderr, and process termination in shell modules.

### 2. Preserve compatibility surfaces

Treat CLI syntax, scenario aliases, generated manifests, dry-run receipts, live receipt schemas, log milestone text, and non-claim fields as compatibility surfaces. Any semantic change requires a separate Cairn.

### 3. Introduce explicit shell interfaces

Backend and client shells should accept validated plan/config structs and return typed results. They should not reach back into unrelated global helper functions or parse raw CLI/environment values.

### 4. Migrate in reviewable slices

Extract one responsibility cluster per implementation commit or task. Each move should include parity tests and at least one negative test that proves the moved boundary fails closed.

## Risks / Trade-offs

- Mechanical moves can hide behavior drift; parity tests must compare old and new outputs where practical.
- Over-splitting can create noisy public APIs; keep modules crate-private unless a reusable contract is proven.
- Backend shell extraction still uses side effects; test pure plan construction separately from command execution.
