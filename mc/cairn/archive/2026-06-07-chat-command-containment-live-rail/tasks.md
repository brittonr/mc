# Tasks

- [x] [serial] Record the chat/command live contract: actor, harmless payload, packet row, owned-local scope, server containment metric, redaction policy, and non-claims. r[mc_compatibility.chat_command_live_rail.contract]
  - Evidence: `docs/evidence/chat-command-containment-live-contract-2026-06-07.md`, `docs/evidence/chat-command-containment-live-contract-2026-06-07.run.log`, and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:contract] Run baseline targeted packet, matrix, current-bundle, and packet-inventory checks before live-rail changes. r[mc_compatibility.chat_command_live_rail.baseline]
  - Evidence: `docs/evidence/chat-command-containment-live-baseline-2026-06-07.run.log` and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:baseline] Add the isolated owned-local chat/command rail or record a deterministic missing-driver blocker without changing existing scenario semantics. r[mc_compatibility.chat_command_live_rail.rail]
  - Evidence: `docs/evidence/chat-command-containment-live-rail-blocker-2026-06-07.run.log`, `docs/evidence/chat-command-containment-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:rail] Emit reviewable chat/command KV, receipt, and log evidence with row id, payload, server correlation, redaction policy, and explicit non-claims. r[mc_compatibility.chat_command_live_rail.evidence]
  - Evidence: `docs/evidence/chat-command-containment-live-evidence-2026-06-07.run.log`, `docs/evidence/chat-command-containment-live-blocker-2026-06-07.kv`, `docs/evidence/chat-command-containment-live-blocker-2026-06-07.receipt.json`, and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:evidence] Validate chat/command evidence with positive checker coverage and negative missing-scope, wrong-payload, stale-digest, missing-correlation, and overclaim fixtures. r[mc_compatibility.chat_command_live_rail.checker]
  - Evidence: `docs/evidence/chat-command-containment-live-checker-2026-06-07.run.log` and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:checker] Promote only `chat-command-containment` in matrix/current-bundle/packet-inventory docs when live evidence passes; otherwise leave it fixture-bounded with a blocker. r[mc_compatibility.chat_command_live_rail.promotion]
  - Evidence: `docs/evidence/chat-command-containment-live-promotion-2026-06-07.run.log` and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
- [x] [depends:promotion] Run evidence-manifest/task-evidence checks, Cairn gates, sync/archive checks, and post-archive validation. r[mc_compatibility.chat_command_live_rail.validation]
  - Evidence: `docs/evidence/chat-command-containment-live-precloseout-validation-2026-06-07.run.log` and `docs/evidence/chat-command-containment-live-2026-06-07.b3`.
