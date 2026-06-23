# Design: Define a typed workspace component registry

## Context

The workspace has several component roles and a few special VCS boundaries. The same facts are repeated manually. A registry should be authoritative without forcing runtime Nickel evaluation.

## Decisions

### 1. Nickel owns the editable registry

**Choice:** Author the registry in Nickel with contracts for component role, path, owner, VCS boundary, commands, gate participation, and evidence policy.

**Rationale:** Typed configuration catches missing fields and invalid enum values before generated docs or checks drift.

### 2. Runtime consumes checked-in static outputs only when needed

**Choice:** Registry-derived Rust or docs artifacts are generated during dev/check flows and checked in. Runtime code does not evaluate Nickel.

**Rationale:** This matches the existing manifest-derived surface model.

### 3. Registry starts descriptive, then becomes prescriptive

**Choice:** First encode current component facts, then use follow-on checks to reject undocumented roots, nested Git directories, and gate participation drift.

**Rationale:** Capturing current truth lowers migration risk.

### 4. Keep evidence policy first-class

**Choice:** Every component row records whether review evidence may cite source paths directly, must copy artifacts to `docs/evidence/`, or is excluded from default evidence gates.

**Rationale:** Layout and evidence boundaries are coupled in this repo.

## Risks / Trade-offs

- A registry can become another stale source; mitigate with generated freshness and layout guards.
- Too much detail can make simple moves expensive; mitigate with minimal required fields and optional notes.
- Moving resolver behavior to generated data can be risky; start with validation before runtime consumption.
