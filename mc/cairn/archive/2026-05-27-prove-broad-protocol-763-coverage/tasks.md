# Tasks

- [x] [serial] Build the protocol coverage ledger. r[mc_compatibility.prove_broad_protocol_763_coverage.coverage_ledger]
- [x] [serial] Add mapping and parser verification fixtures. r[mc_compatibility.prove_broad_protocol_763_coverage.mapping_parser_fixtures]
- [x] [serial] Add live scenario-family evidence gates. r[mc_compatibility.prove_broad_protocol_763_coverage.live_scenario_gates]
- [x] [serial] Keep broad compatibility claims blocked until every required row is evidenced. r[mc_compatibility.prove_broad_protocol_763_coverage.non_overclaiming_gate]

## Progress

- Protocol coverage ledger is documented in `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-27.md`.
- Mapping/parser fixtures in `tools/check_protocol_coverage_ledger.py --self-test` reject fallback aliases, malformed shape acceptance, and missing live receipts.
- Live scenario-family gate validates the 15 bounded acceptance-matrix seams and current-bundle hash agreement.
- `tools/check_protocol_coverage_ledger.py` keeps full protocol-763 compatibility and full Minecraft compatibility blocked until every required row has mapping/parser fixtures, live receipts, manifests, and index entries.
