# Tasks

- [ ] [serial] Select the bounded packet rows for live parity, record the live signal expected for each row, and preserve non-claim scope for unselected rows. r[mc_compatibility.targeted_packet_live_parity.selection]
- [ ] [depends:selection] Run baseline fixture/evidence checks for the selected rows before changing live probes. r[mc_compatibility.targeted_packet_live_parity.baseline]
- [ ] [depends:baseline] Add or extend runner scenarios/probe hooks that exercise the selected packet behavior through live backend/client paths. r[mc_compatibility.targeted_packet_live_parity.probes]
- [ ] [depends:probes] Record live receipts/logs with packet row identifiers, scenario names, backend/client revisions, and explicit non-claims. r[mc_compatibility.targeted_packet_live_parity.receipts]
- [ ] [depends:receipts] Promote only the evidenced packet rows in the matrix/bundle and keep non-exercised rows fixture-bounded. r[mc_compatibility.targeted_packet_live_parity.matrix]
- [ ] [depends:matrix] Add positive and negative targeted-packet checker tests for live evidence presence, packet-row matching, stale receipts, and overclaim rejection. r[mc_compatibility.targeted_packet_live_parity.tests]
- [ ] [depends:tests] Run runner checks, targeted packet checks, evidence-manifest/task-evidence checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.targeted_packet_live_parity.validation]
