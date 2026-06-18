## Context

`resource-pack-status` has a local rail contract, but live evidence remains blocked by the missing Stevenarella status-response driver. The driver must stay native, bounded, and owned-local.

## Goals / Non-Goals

Goals:
- Add a typed Stevenarella path that recognizes a configured owned-local resource-pack offer and emits a configured status response through the protocol path.
- Prove no external resource-pack fetch is required for the bounded evidence row.
- Integrate the driver into the runner or MCP-controlled path only enough to produce reviewable resource-pack status evidence.

Non-goals:
- Downloading or applying resource packs.
- Trust/security validation, hash verification breadth, all status variants, public-server behavior, production readiness, or full protocol 763 compatibility.

## Design

1. Define a pure driver contract for offer id, URL/scope classification, expected status, no-external-fetch guarantee, redaction policy, and protocol status packet output.
2. Implement a thin Stevenarella shell that maps the owned-local offer to the pure decision and writes the protocol response without host OS input synthesis.
3. Add positive tests for the configured local offer/status response and negative tests for external URL scope, malformed offer metadata, unsupported status, missing control state, and overlarge/redacted fields.
4. Expose the driver to the runner as an isolated resource-pack status scenario or capability update.
5. Emit targeted-packet live KV/receipt/log evidence only when the checker can verify no-external-fetch and server correlation.

## Risks

- Stevenarella protocol state may not surface the resource-pack offer cleanly; if not, preserve a blocker and avoid broadening the row.
- The driver must not accidentally fetch or write unbounded assets while trying to answer an offer.

## Validation

- Run focused Stevenarella tests through the mc devshell.
- Run runner dry-runs/unit tests for the resource-pack status path.
- Run targeted-packet live-evidence checker positive and negative fixtures.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
