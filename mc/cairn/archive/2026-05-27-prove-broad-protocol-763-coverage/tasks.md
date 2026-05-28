# Tasks

- [x] [serial] Build the protocol coverage ledger. r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger]
- [x] [serial] Add mapping/parser promotion-gate fixtures. r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures]
- [x] [serial] Add live scenario-family evidence gates. r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates]
- [x] [serial] Keep broad compatibility claims blocked until every required row is evidenced. r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate]

## Progress

- Protocol coverage ledger is documented in `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-27.md` and backed by `docs/evidence/protocol-763-packet-inventory-2026-05-27.tsv`, a 175-row Valence packet inventory joined to Stevenarella mapping status and scenario evidence.
- Mapping/parser promotion-gate fixtures in `tools/check_protocol_coverage_ledger.py --self-test` reject fallback aliases, malformed shape acceptance, missing packet rows, fallback promotion, and missing live receipts.
- Live scenario-family gate validates the 15 bounded acceptance-matrix seams and current-bundle hash agreement.
- `tools/check_protocol_coverage_ledger.py` keeps full protocol-763 compatibility and full Minecraft compatibility blocked until every required row has mapping/parser fixtures, live receipts, manifests, and index entries.
