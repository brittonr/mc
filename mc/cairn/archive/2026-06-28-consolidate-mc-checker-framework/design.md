# Design: mc checker framework consolidation

## Context

This workspace uses many standalone checkers as review gates. The checkers are valuable but duplicate common code. Consolidation should improve consistency without turning the root into a shared Cargo workspace or changing checker invocation surfaces.

## Decisions

### 1. Extend the existing checker framework

**Choice:** Build on `tools/checker_framework.rs` for shared diagnostics, path handling, fixture execution, and receipt/evidence helpers.

**Rationale:** A local framework already exists and can become the common owner without adding a broad workspace dependency.

### 2. Keep checkers as standalone tools

**Choice:** Individual checkers keep their existing command-line entrypoints and flake check wiring while importing or copying only approved framework components.

**Rationale:** Review gates remain stable and targeted.

### 3. Migrate touched Python gates

**Choice:** When a Python checker is extended for this consolidation, migrate it to Rust or Steel rather than growing the Python surface.

**Rationale:** This matches workspace guidance while avoiding churn for untouched historical gates.

### 4. Add duplication guards

**Choice:** Add checks or tests that reject unsafe path traversal and ad hoc receipt parsing in new/updated checkers where the framework should be used.

**Rationale:** Consolidation needs guardrails to prevent regressions.

## Risks / Trade-offs

- Some checkers may intentionally parse specialized receipts; allow narrow local logic when documented and tested.
- Standalone Rust scripts cannot share code as easily as crates; keep framework APIs small and copy-friendly until a proper tool crate is justified.
- Diagnostic wording may be consumed by evidence logs; preserve or explicitly refresh affected logs when diagnostics change.
