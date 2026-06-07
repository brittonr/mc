## Context

The targeted-packet live-parity drain selected `resource-pack-status` but found no deterministic owned-local resource-pack offer hook or asset-serving fixture. This change defines that rail while keeping resource-pack safety and asset application out of scope.

## Goals / Non-Goals

Goals:
- Exercise one local resource-pack offer/status packet exchange.
- Record no-external-fetch and redaction metrics.
- Validate live promotion through fail-closed checker rules.

Non-goals:
- Proving pack download/application, trust decisions, all status variants, public-server safety, production readiness, or full protocol 763 compatibility.

## Design

1. Define a local offer contract: scenario id, actor, packet rows, local fixture identity/hash, expected client status, backend correlation, redaction policy, and non-claims.
2. Add a rail that injects or serves the local fixture through owned-local infrastructure only.
3. Record explicit `no_external_fetch=true` or equivalent normalized evidence, plus packet row and status response metrics.
4. Validate evidence with `tools/check_targeted_packet_promotions.rs --live-evidence <kv>` and resource-pack-specific negative cases.
5. Promote docs only after the checker passes.

## Risks

- Stevenarella may require pack download plumbing that is not deterministic in headless mode.
- If a local asset server is added, the rail must prove binding/target ownership and avoid public-server claims.

## Validation

Run baseline targeted-packet/doc checks, the resource-pack rail or deterministic fixture check, live-evidence checker positives/negatives, acceptance/current-bundle/packet-inventory checks, evidence-manifest/task-evidence checks, Cairn gates/sync/archive, and post-archive validation.
