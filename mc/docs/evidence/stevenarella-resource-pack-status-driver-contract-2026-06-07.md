# Stevenarella resource-pack status driver contract (2026-06-07)

## Bounded offer/status contract

Change: `stevenarella-resource-pack-status-driver`.

Configured owned-local offer:
- actor: `compatbot`
- scenario: `mcp-controlled-smoke`
- fixture identity: `owned-local-resource-pack-offer-fixture`
- offer id: `mc-compat-local-resource-pack`
- accepted URL scope: loopback or local fixture URLs only (`localhost`, `127.0.0.1`, `[::1]`, or `file://` fixture path)
- expected status response: `declined`
- protocol output: Stevenarella control-plane action to serverbound `ResourcePackStatus` with status code `1`
- no-external-fetch guarantee: pure decision logic rejects external/off-scope URLs before protocol output and the shell only writes the status packet
- redaction policy: `no-secrets-no-public-addresses`
- backend path: `deterministic-resource-pack-offer-contract`
- client path: `stevenarella-resource-pack-status-driver`

## Evidence and validation shape

Driver tests must cover one valid owned-local declined response and negative malformed offer metadata, external/off-scope URL, unsupported status, missing offer state, and overlarge/redaction inputs. Runner integration remains isolated to the `resource-pack-status` capability registry row and does not promote live evidence without a maintained live receipt.

## Non-claims

No asset download/application, trust or security validation, hash verification breadth, all status variants, public-server behavior, production readiness, full protocol 763 compatibility, broad Minecraft compatibility, or WAN behavior is claimed.
