# Tasks

- [ ] [serial] Run baseline CTF invalid-action, CTF rule ledger, current-bundle, manifest, and maintained dry-run gates. r[mc_compatibility.ctf_invalid_action_breadth.validation]
- [ ] [depends:validation] Define the invalid-action breadth matrix with the next bounded pickup/return permutations and explicit non-claims. r[mc_compatibility.ctf_invalid_action_breadth.matrix]
- [ ] [depends:matrix] Choose one additional bounded invalid-action row for first promotion and lock its expected pre-state, rejection, postcondition, milestones, and forbidden transitions. r[mc_compatibility.ctf_invalid_action_breadth.additional_row]
- [ ] [depends:additional_row] Add or extend runner/fixture support to emit normalized evidence for the selected permutation without changing existing CTF rows. r[mc_compatibility.ctf_invalid_action_breadth.rail]
- [ ] [depends:rail] Implement a parameterized checker with positive and negative fixtures for valid, missing, mismatched, and overclaiming evidence. r[mc_compatibility.ctf_invalid_action_breadth.checker]
- [ ] [depends:checker] Record reviewable receipts/logs/manifests and update the CTF rule ledger, acceptance matrix, and current bundle for only the selected bounded row. r[mc_compatibility.ctf_invalid_action_breadth.additional_row] r[mc_compatibility.ctf_invalid_action_breadth.validation]
- [ ] [depends:validation] Run focused row gates, maintained dry-runs, evidence-manifest checks, task-evidence checks, Cairn gates, sync/archive checks, and final validation. r[mc_compatibility.ctf_invalid_action_breadth.validation]
