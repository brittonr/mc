# Design: Valence packet capture oracle

## Context

Hyperion `tools/packet-inspector` can proxy and display packets between a client and server. For our fork, the valuable piece is a headless receipt-producing oracle, not necessarily a GUI.

## Decisions

### 1. Prefer headless CLI first

**Choice:** The first Valence fork slice should work in CI/Nix and not depend on a GUI.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 2. Normalize rather than dump raw traffic

**Choice:** Receipts should summarize packet ids/states/decode failures and avoid committing sensitive/raw session data by default.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

### 3. Attach to scenario triage

**Choice:** Capture summaries should explain whether a failure is client-probe, server-correlation, or protocol-runtime.

**Rationale:** This keeps the Valence fork work independently drainable and evidence-backed.

## Risks / Trade-offs

- Full packet payload capture can expose noisy or private data if not bounded/redacted.
- Proxying live traffic may perturb timing; initial gates should use fixtures/dry-run capture summaries.
