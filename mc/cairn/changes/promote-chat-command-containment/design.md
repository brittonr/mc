# Design: Promote chat and command execution containment evidence

## Context

Chat and command packets are important but easy to overclaim. This row should be containment-focused: a harmless owned-local payload reaches or is rejected by the local fixture exactly as expected.

## Decisions

### 1. Use an owned-local harmless payload

**Choice:** Configure a local-only message or command that cannot mutate external state or leak secrets.

**Rationale:** Public-server and security breadth require separate authorization and threat-model evidence.

### 2. Promote packet receipt, not command semantics breadth

**Choice:** Validate server receipt/correlation and expected bounded output/rejection only.

**Rationale:** Command execution, permissions, signing, and moderation are broad systems.

### 3. Require redaction and safety fields

**Choice:** Evidence must record owned-local scope, payload redaction policy, and no-public-server non-claims.

**Rationale:** Chat/command rows can accidentally imply safety beyond the fixture.

## Risks / Trade-offs

- Protocol versions may have signed chat fields that Stevenarella does not fully model.
- A command path can accidentally mutate fixture state; use a harmless probe.
- Security semantics must remain explicit non-claims.
