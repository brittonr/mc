## Why

`chat-command-containment` remains fixture-bounded even though an owned-local chat/command path is a high-signal, low-risk live candidate. A narrow rail can prove one harmless payload is contained by the local fixture without claiming command/security breadth.

## What Changes

- Add an isolated owned-local chat/command live rail for one harmless payload.
- Record client action, server receipt or rejection, packet row identity, redaction policy, child revisions when available, and explicit non-claims.
- Validate normalized KV evidence with positive and negative targeted-packet checker coverage before any matrix or packet-inventory promotion.
- Keep public-server safety, moderation, chat signing/security, arbitrary commands, and production readiness as non-claims.

## Impact

- **Files**: `tools/mc-compat-runner/src/**`, `tools/check_targeted_packet_promotions.rs`, `docs/evidence/**`, acceptance matrix/current bundle/packet inventory if live evidence passes.
- **Testing**: Baseline targeted packet checks, runner unit/dry-run checks, live-evidence checker positive and negative fixtures, evidence-manifest/task-evidence checks, Cairn gates and validation.
