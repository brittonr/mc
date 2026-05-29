# Protocol-763 adversarial-network oracle contract — 2026-05-29

## Scope

This contract defines one bounded `adversarial-network-oracle` row before any runner, fixture, parser, or promotion work starts.

Covered claim, once deterministic evidence exists: the harness has one explicitly approved adversarial-network oracle model for a fixture-only packet-mutation rail. The model is `protocol763-custom-payload-truncated-varint-v1`; it mutates exactly one owned-local fixture packet shape, records the configured bounds and telemetry, records an oracle decision, and fails closed before any live network traffic.

This is not a live hostile-network test. It is a deterministic fixture rail for review of the promotion contract and checker behavior. Live adversarial-network safety, malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded adversarial robustness, and full protocol security remain non-claims.

## Fixed oracle envelope

| Field | Value |
| --- | --- |
| Rail | `adversarial-network-oracle` |
| Threat model id | `protocol763-custom-payload-truncated-varint-v1` |
| Mutation types | `custom_payload_truncated_varint` |
| Target ownership | `owned-local-fixture` |
| Authorization | `fixture-only-approved` |
| Live network traffic | `disabled` |
| Packet bound | `max_mutated_packets=1` |
| Payload bound | `max_payload_bytes=64` |
| Telemetry | `threat_model_id`, `mutation_types`, `packet_bounds`, `target_ownership`, `authorization`, `abort_criteria`, `observed_containment`, `oracle_decision` |
| Abort criteria | fail before live traffic if approval, ownership, bounds, telemetry, or non-claim fields are missing |
| Oracle decision | `approved_for_deterministic_fixture_only` |

## Required normalized metrics

| Metric | Meaning |
| --- | --- |
| `rail.name` | Evidence rail is `adversarial-network-oracle`. |
| `threat_model.id` | Threat model is `protocol763-custom-payload-truncated-varint-v1`. |
| `threat_model.approved` | Human/oracle approval is present for deterministic fixture use. |
| `threat_model.oracle_decision` | Oracle decision is `approved_for_deterministic_fixture_only`. |
| `mutation.types` | Mutation set is exactly `custom_payload_truncated_varint`. |
| `mutation.max_packets` | Packet mutation bound is `1`. |
| `mutation.max_payload_bytes` | Payload mutation bound is `64`. |
| `mutation.live_network_enabled` | Live network mutation is `false`. |
| `target.ownership` | Target is `owned-local-fixture`. |
| `target.authorization` | Authorization is `fixture-only-approved`. |
| `telemetry.threat_model_id` | Telemetry records threat model id. |
| `telemetry.mutation_types` | Telemetry records mutation types. |
| `telemetry.packet_bounds` | Telemetry records packet and payload bounds. |
| `telemetry.abort_criteria` | Telemetry records fail-closed abort criteria. |
| `observed.containment` | Fixture observation is `failed_closed_before_live_traffic`. |
| `claims.adversarial_network_oracle_fixture` | Bounded deterministic fixture claim is `true` only after checker evidence passes. |
| `claims.adversarial_network_safety` | Broad adversarial-network safety claim remains `false`. |
| `claims.malicious_client_resilience` | Malicious-client resilience claim remains `false`. |
| `claims.hostile_internet_safety` | Hostile internet safety claim remains `false`. |
| `claims.production_readiness` | Production readiness claim remains `false`. |
| `claims.public_server_safety` | Public-server safety claim remains `false`. |
| `claims.full_protocol_security` | Full protocol security claim remains `false`. |

## Checker contract

`tools/check_adversarial_network_oracle.rs` is the Rust promotion gate for this row. It must pass positive fixtures and reject:

- `missing_oracle_approval` when fixture approval or oracle decision is absent;
- `missing_threat_model_id` when the model id is absent or mismatched;
- `missing_target_ownership` when the target is not owned-local fixture scope;
- `missing_authorization` when authorization is absent or not fixture-only;
- `unbounded_mutation` when packet or payload bounds are missing or too large;
- `live_network_enabled` when fixture evidence would mutate live traffic;
- `missing_telemetry` when any normalized telemetry field above is absent;
- `missing_abort_criteria` when fail-closed criteria are absent;
- `security_overclaim` when evidence claims adversarial-network safety, malicious-client resilience, hostile internet safety, production readiness, public-server safety, unbounded robustness, or full protocol security.

## Oracle checkpoint template

Future evidence for this row must include an oracle checkpoint with these sections before promotion:

- `## Question`
- `## Inspected evidence`
- `## Decision`
- `## Owner`
- `## Next action`

The decision must say whether the exact threat model id is approved for fixture-only evidence, whether live traffic remains disabled, and which broader claims remain false.

## Non-claims

This contract does not claim live adversarial-network safety, malicious-client resilience, hostile-network resilience, hostile internet safety, public-server safety, production readiness, WAN safety, packet-loss tolerance, unbounded adversarial robustness, exploit resistance, denial-of-service resistance, data-confidentiality protection, authentication correctness, or full protocol security.
