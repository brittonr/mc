# Design: Promote resource-pack status evidence

## Context

Resource-pack packets are security-sensitive. The row should record one local offer/status exchange while explicitly avoiding claims about downloading, applying, trusting, or validating resource packs.

## Decisions

### 1. Use a local non-download fixture offer

**Choice:** Configure a deterministic local fixture offer with redacted or inert URL/hash fields where possible.

**Rationale:** The evidence should avoid network fetches and public-server safety implications.

### 2. Promote status response, not asset behavior

**Choice:** Validate the client status packet and server correlation only.

**Rationale:** Asset loading and trust decisions require separate safety evidence.

### 3. Require safety and redaction metadata

**Choice:** Evidence must name local scope, no external fetch, redaction policy, and non-claims.

**Rationale:** Resource-pack flows can otherwise imply unsafe external behavior.

## Risks / Trade-offs

- Stevenarella may auto-reject or ignore resource-pack offers; the row should accept one configured status response.
- Valence fixture may need a minimal offer implementation.
- Do not include live public URLs or secrets in evidence.
