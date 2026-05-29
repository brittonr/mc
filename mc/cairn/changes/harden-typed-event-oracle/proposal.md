# Proposal: Harden typed event oracle

## Why

The compatibility runner still treats most scenario evidence as substring matches over client/server logs in `tools/mc-compat-runner/src/main.rs`. That is useful for bootstrapping, but it is fragile: a stale line can satisfy a milestone, ordering is mostly implicit, and client/server correlation is hard to audit after a failure. Projectile damage already needs stronger causality checks; the rest of the harness should move toward the same explicit model.

## What Changes

- Define a versioned JSONL event schema for client and server milestones.
- Emit typed client/server events alongside existing text logs for maintained rails.
- Evaluate scenarios through a pure event-graph oracle that checks session, username, sequence, required events, forbidden events, and ordering where relevant.
- Add event-timeline references and BLAKE3 hashes to receipts without recording raw packet payloads by default.
- Keep the existing substring oracle as a migration fallback until each scenario has typed-event coverage.

## Impact

- **Files**: `tools/mc-compat-runner/src/main.rs`, Stevenarella probe logging, Valence/Paper fixture milestone emission, receipt docs, evidence checkers.
- **Testing**: positive typed-event fixtures, missing-event negative fixtures, forbidden-event negative fixtures, ordering/correlation negative fixtures, dry-run receipt checks, Cairn validation.
- **Non-claims**: typed events improve harness evidence quality; they do not by themselves prove full Minecraft compatibility, vanilla parity, production readiness, or broad CTF correctness.
