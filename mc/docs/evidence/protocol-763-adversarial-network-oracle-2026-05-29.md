# Protocol-763 adversarial-network oracle fixture evidence — 2026-05-29

## Scope

This evidence promotes one bounded `adversarial-network-oracle` fixture row. It proves the fixture-only parser/checker rail for threat model `protocol763-custom-payload-truncated-varint-v1`, not live adversarial-network safety.

## Evidence artifacts

| Artifact | Path |
| --- | --- |
| Contract | `docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.md` |
| Contract validation log | `docs/evidence/protocol-763-adversarial-network-oracle-contract-2026-05-29.run.log` |
| Checker validation log | `docs/evidence/protocol-763-adversarial-network-oracle-checker-2026-05-29.run.log` |
| Fixture record | `docs/evidence/protocol-763-adversarial-network-oracle-fixture-2026-05-29.record` |
| Fixture checkpoint | `docs/evidence/protocol-763-adversarial-network-oracle-checkpoint-2026-05-29.md` |
| Fixture validation log | `docs/evidence/protocol-763-adversarial-network-oracle-fixture-2026-05-29.run.log` |
| Fixture receipt | `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.receipt.json` |
| Row validation log | `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.run.log` |
| Row BLAKE3 manifest | `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.b3` |

## Normalized row

| Metric | Value |
| --- | --- |
| `rail.name` | `adversarial-network-oracle` |
| `threat_model.id` | `protocol763-custom-payload-truncated-varint-v1` |
| `threat_model.approved` | `true` |
| `threat_model.oracle_decision` | `approved_for_deterministic_fixture_only` |
| `mutation.types` | `custom_payload_truncated_varint` |
| `mutation.max_packets` | `1` |
| `mutation.max_payload_bytes` | `64` |
| `mutation.live_network_enabled` | `false` |
| `target.ownership` | `owned-local-fixture` |
| `target.authorization` | `fixture-only-approved` |
| `observed.containment` | `failed_closed_before_live_traffic` |
| `claims.adversarial_network_oracle_fixture` | `true` |
| `claims.adversarial_network_safety` | `false` |
| `claims.malicious_client_resilience` | `false` |
| `claims.hostile_internet_safety` | `false` |
| `claims.production_readiness` | `false` |
| `claims.public_server_safety` | `false` |
| `claims.unbounded_adversarial_robustness` | `false` |
| `claims.full_protocol_security` | `false` |

## Validation

- `tools/check_adversarial_network_oracle.rs --self-test` passed positive and fail-closed negative fixtures.
- `tools/check_adversarial_network_oracle.rs --record docs/evidence/protocol-763-adversarial-network-oracle-fixture-2026-05-29.record` passed.
- `nix build --no-update-lock-file .#checks.x86_64-linux.mc-compat-adversarial-network-oracle --no-link -L` passed.
- The receipt records `mode=fixture`, `dry_run=true`, `live_network_enabled=false`, and `observed_containment=failed_closed_before_live_traffic`.

## Decision

Decision: promote only the bounded fixture oracle row. This is deterministic contract/checker evidence for one packet-mutation model and does not run or authorize live adversarial traffic.

## Non-claims

No live adversarial-network safety, malicious-client resilience, hostile-network resilience, hostile internet safety, public-server safety, production readiness, WAN safety, packet-loss tolerance, unbounded adversarial robustness, exploit resistance, denial-of-service resistance, data-confidentiality protection, authentication correctness, or full protocol security claim is made.
