## Context

Live packet promotion should not depend on each rail inventing its own required keys. The existing live-evidence checker mode validates a minimal set of fields, but future rails need a row-extensible contract that stays pure and deterministic.

## Goals / Non-Goals

Goals:
- Define a shared KV schema for targeted packet live promotion evidence.
- Keep schema validation pure over in-memory key/value records.
- Preserve row-specific extensions for packet families without weakening common requirements.

Non-goals:
- Promoting any packet row by itself.
- Replacing receipt schemas outside targeted packet promotion evidence.

## Design

1. Define required common keys: `row.id`, `live.promotion.status`, `live.evidence.mode`, `live.packet.row`, `live.scenario`, `live.backend`, `live.receipt`, `live.receipt.blake3`, `live.receipt.digest_status`, revision fields when available, and explicit non-claims.
2. Define row-extension hooks for per-row metrics such as slot/item/count, sign position/payload, local resource-pack offer/status, or server correlation.
3. Implement pure validation helpers that return diagnostics without reading files or running commands.
4. Keep the CLI shell responsible only for file reads, argument parsing, and exit-code/reporting.
5. Add self-tests covering valid common evidence, valid row-specific extensions, and negative malformed/missing/stale/overclaim cases.

## Risks

- Overly strict required keys may block useful fixture/blocker evidence. The schema should distinguish live promotion evidence from blocker documentation.
- Revision metadata availability differs across backend/client paths; required fields should fail closed for promotion while allowing explicit `not_available` only when the row contract permits it.

## Validation

Run targeted packet checker self-tests, targeted packet promotion flake check, evidence-manifest/task-evidence checks, Cairn gates/sync/archive, and post-archive validation.
