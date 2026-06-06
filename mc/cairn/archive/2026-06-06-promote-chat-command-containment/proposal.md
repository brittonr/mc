# Proposal: Promote chat and command execution containment evidence

## Why

Chat and command execution packets are high-risk because they border user input, server actions, and security expectations. They remain unpromoted in the packet inventory. A bounded owned-local containment row can prove one safe local command/chat path without claiming public-server safety or command semantics breadth.

## What Changes

- Add one bounded owned-local chat or command execution containment rail with a harmless configured payload.
- Require client action, Valence server receipt/correlation, output or rejection metric, redaction policy, and explicit no-public-server/no-security-breadth non-claims.
- Promote only the configured packet row or rows, keeping all chat signing/security, all commands, permissions, moderation, public-server safety, and production readiness as non-claims.

## Impact

- **Files**: Valence fixture instrumentation, Stevenarella action probe, runner metadata, packet inventory/current bundle docs, checker, evidence artifacts, and Cairn specs/tasks.
- **Testing**: positive/negative checker fixtures, focused scenario tests, packet inventory/current-bundle checks, evidence manifests, task-evidence gate, and Cairn validation.
