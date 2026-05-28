# Tasks

- [x] [serial] Build the CTF rule and invariant ledger. r[mc_compatibility.prove_ctf_rule_correctness.rule_ledger]
- [x] [serial] Add correlated positive rule scenarios. r[mc_compatibility.prove_ctf_rule_correctness.positive_rule_scenarios]
- [x] [serial] Add negative invalid-action scenarios. r[mc_compatibility.prove_ctf_rule_correctness.negative_rule_scenarios]
- [x] [serial] Promote only receipt-backed rule clusters. r[mc_compatibility.prove_ctf_rule_correctness.rule_promotion_gate]

## Progress

- CTF rule ledger is documented in `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md`.
- Positive validation promotes three bounded rule clusters: RED/BLUE scoring soak, flag-carrier death/return, and reconnect flag-state.
- Negative fixtures in `tools/check_ctf_rule_ledger.py --self-test` reject missing server flag return, protocol mismatch, unexpected score/capture evidence, missing historical scoring oracle, and full CTF correctness overclaim.
- `tools/check_ctf_rule_ledger.py` promotes only receipt-backed or explicitly-oracled rule clusters and keeps full CTF correctness as a non-claim.
