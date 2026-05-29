# Protocol-763 production/network safety matrix — 2026-05-28

## Scope

This matrix separates owned-local load safety, public-server safety, WAN tolerance, and adversarial-network safety. It promotes the bounded owned-local loopback envelope plus one deterministic fixture-only adversarial-network oracle row. Public-server, production-readiness, WAN, live adversarial-network, and unbounded safety remain non-claims.

owner: agent
next_action: require a new authorized live row, telemetry, run log, and oracle checkpoint before promoting any public, WAN, live adversarial, or production-readiness claim.

## Safety claim matrix

| Claim | Status | Target ownership | Authorization | Bounds | Telemetry | Evidence | Explicit non-claims | Next action |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| owned-local load safety | covered_owned_local_bounded | target_scope=owned-local-loopback; owned_local_target=true | authorized=true by owned local loopback only; no external target authorization | max_clients=2; max_duration_secs=600; reconnect_sessions<=2; packet_loss=0 | telemetry_present=true; live_receipt=true; preflight_passed=true; promotion_ready=true | `docs/evidence/protocol-763-load-network-safety-live-2026-05-27.receipt.json`; receipt BLAKE3 `62aba060f0bc082d08487c5adf83bfd417742d3711fe4295066e44e7668a25b2`; run log BLAKE3 `8087221d20405d63e5cd81ffc1afbcdfd8b118b157dbe38e5e1752384e97bce7` | No public-server safety, no production readiness, no WAN safety, no adversarial-network safety, no unbounded soak/reconnect/latency claim. | Keep current owned-local loopback envelope bounded; rerun checker before promotion. |
| public-server safety | non_claim_fail_closed | target ownership not established for public targets | explicit public authorization missing; `MC_COMPAT_PUBLIC_TARGET=1` without `MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED=1` must fail before traffic | no public load bounds authorized | no public telemetry | Runner preflight is required to reject unowned/public target claims; no live public receipt exists. | No public-server safety, no production readiness, no third-party target safety. | Obtain explicit written authorization, owner, bounds, telemetry plan, and human checkpoint before any public-target run. |
| WAN tolerance | non_claim_fail_closed | owned-local loopback only; no WAN target is promoted | WAN authorization absent | delay/jitter/loss/timeout parameters are required; perturbation tooling unavailable must fail closed | no WAN telemetry | Deterministic WAN receipt request fixture in `tools/check_load_network_safety.py --self-test` fails closed when perturbation tooling unavailable; existing latency/jitter row records bounded metadata without privileged WAN mutation; tooling unavailable fails closed; no WAN claim. | No WAN safety, no packet-loss tolerance, no internet-path tolerance, no production readiness. | Add approved perturbation tooling and telemetry, then produce a bounded WAN receipt. |
| adversarial-network safety | covered_fixture_oracle_only | target_scope=owned-local-fixture; no live target is promoted | authorized=true for `fixture-only-approved`; no public or live adversarial authorization | threat_model_id=`protocol763-custom-payload-truncated-varint-v1`; mutation_types=`custom_payload_truncated_varint`; max_mutated_packets=1; max_payload_bytes=64; live_network_enabled=false; abort_criteria=`fail_closed_before_live_traffic` | telemetry_present=true for threat model, mutation, bounds, target ownership, authorization, abort criteria, containment, and oracle decision | `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.receipt.json`; receipt BLAKE3 `d60a401911bab20890cf8fb7e5dd248ae668c9c9baab01070f38522af246455a`; row manifest `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.b3`; checkpoint `docs/evidence/protocol-763-adversarial-network-oracle-checkpoint-2026-05-29.md` | No live adversarial-network safety, no malicious-client resilience, no hostile-network resilience, no public-server safety, no production readiness, no unbounded adversarial robustness, no full protocol security. | Require explicit human/oracle approval, owned or authorized target, bounds, telemetry, and live run evidence before any live adversarial-network claim. |

## Validation evidence

- Gate log: `docs/evidence/protocol-763-production-network-safety-gate-2026-05-28.run.log`.
- BLAKE3 manifest: `docs/evidence/protocol-763-production-network-safety-gate-2026-05-28.b3`.
- Manifest-check log: `docs/evidence/protocol-763-production-network-safety-manifest-verify-2026-05-28.run.log` (intentionally excluded from its own manifest to avoid a self-referential hash loop).

## Decision

- Question: Can current evidence promote production/public/WAN/adversarial network safety?
- Inspected evidence: runner `load_network_safety` receipt fields, `tools/check_load_network_safety.py`, bounded live receipt, run log digests, current bundle non-claims, and adversarial oracle checkpoint.
- Decision: Bounded owned-local loopback load safety is covered, and the deterministic fixture-only adversarial-network oracle row is covered. Production readiness, public-server safety, WAN safety, live adversarial-network safety, unbounded soak, unbounded reconnect, unbounded adversarial robustness, and packet-loss tolerance remain non-claims.
- Owner: agent.
- Next action: add a new authorized live matrix row with telemetry and a checkpoint before promoting any broader claim.
