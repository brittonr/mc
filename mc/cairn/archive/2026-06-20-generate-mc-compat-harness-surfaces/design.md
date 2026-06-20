# Design: Generate mc-compat harness surfaces from the scenario manifest

## Context

The repository already typechecks the Nickel scenario manifest and validates drift against checked-in Rust, flake, README, and evidence-bundle surfaces. Generation should keep the runner runtime Rust-only while making those derived surfaces reproducible.

## Decisions

### 1. Generate only bounded machine-owned blocks

**Choice:** Generate Rust tables and clearly delimited wrapper/index blocks. Keep narrative README prose and evidence interpretation human-authored.

**Rationale:** Machine output should be deterministic and reviewable without flattening important human context.

### 2. Keep runtime free of Nickel evaluation

**Choice:** Generation happens in dev/Nix checks. The runner continues consuming checked-in Rust artifacts at runtime.

**Rationale:** This preserves the existing no-runtime-Nickel contract and avoids adding startup dependencies.

### 3. Use a pure generator core

**Choice:** Parse and validate manifest data into an intermediate model, then render outputs from that model without filesystem access. The CLI shell reads/writes paths and reports diffs.

**Rationale:** The core can be tested with positive and negative manifest fixtures.

### 4. Fail on stale generated output

**Choice:** Add a Nix check that regenerates artifacts into a temporary directory and compares them with checked-in outputs.

**Rationale:** Reviewers should not need to know which files are generated to catch stale surfaces.

## Risks / Trade-offs

- Generating flake snippets can obscure Nix intent; mitigate with small generated blocks and stable comments.
- Generated README command blocks can churn; mitigate by sorting from manifest order and keeping prose outside markers.
- Generator bugs can stamp out bad data; mitigate with existing scenario manifest validation and independent runner parity tests.
