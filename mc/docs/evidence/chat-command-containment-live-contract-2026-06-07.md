# Chat/command containment live rail contract — 2026-06-07

## Contract

- Targeted row: `chat-command-containment`.
- Actor: `compatbot`.
- Harmless payload identity: `harmless-chat-command-containment`.
- Packet rows: `play/serverbound/0x05 ChatMessageC2SPacket` and `play/serverbound/0x04 CommandExecutionC2SPacket`.
- Owned-local target scope: `owned-local-fixture` only; no public or third-party server target is authorized by this change.
- Expected containment metric: one server receipt/rejection/correlation proving the configured payload stayed inside the owned-local fixture.
- Redaction policy: `no-secrets-no-public-addresses`.
- Backend path: `owned-local-chat-or-command-rail-missing` until a maintained live rail exists.
- Client path: `stevenarella-mcp-chat-control-candidate` until it emits a maintained targeted containment receipt.

## Current decision

The existing deterministic fixture remains valid for the targeted packet row, but the scenario capability registry records `chat-command-containment` as `targeted-packet-live-blocker` with evidence mode `fixture-bounded-blocker`. The blocker reason is: MCP control exists, but no targeted chat/command containment receipt is maintained.

## Non-claims

This change does not claim chat signing/security, command permissions, moderation, all commands, malicious-client resilience, public-server safety, production readiness, broad Minecraft compatibility, or full protocol 763 compatibility.

## Owner / next action

Owner: local Cairn drain agent. Next action: leave matrix/current-bundle/packet-inventory status fixture-bounded, record blocker evidence, and require a future maintained owned-local chat/command receipt before live promotion.
