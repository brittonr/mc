## Context

The targeted packet registry already names `chat-command-containment` as blocked because no maintained owned-local chat or command rail exists. This change narrows the work to one harmless payload and one server containment metric.

## Goals / Non-Goals

Goals:
- Define a deterministic chat/command live contract for one owned-local harmless payload.
- Drive the payload through the existing client/protocol path or record a fail-closed blocker if the driver is missing.
- Promote only the `chat-command-containment` targeted packet row when live evidence passes.

Non-goals:
- Proving chat signing, command permissions, moderation, all commands, malicious-client resilience, public-server safety, production readiness, or full protocol 763 compatibility.
- Exercising any public or third-party server.

## Design

1. Define pure contract data for actor, payload identity, packet row, owned-local target scope, redaction policy, expected server receipt or rejection metric, and non-claims.
2. Add an isolated scenario or deterministic fixture path that emits one client chat/command action and one server containment observation.
3. Normalize evidence to targeted-packet live KV with row-specific chat fields.
4. Extend or reuse the targeted-packet checker so positive chat evidence passes and missing scope, wrong payload, missing server correlation, stale digest, and public-server/security overclaims fail closed.
5. Update matrix/current-bundle/packet-inventory docs only if checker-backed live evidence passes.

## Risks

- The existing Stevenarella MCP chat path may need a small driver fix before reliable live evidence is possible.
- Server logs must avoid leaking hostnames, addresses, or secrets; evidence must use the redaction policy.

## Validation

- Run baseline targeted packet, matrix, and current-bundle checks before runner edits.
- Run focused runner tests or dry-runs for the chat/command scenario.
- Run positive and negative targeted-packet live-evidence checker tests.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
