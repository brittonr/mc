# mc-compat row fixture rail — 2026-06-01

This checkpoint adds a deterministic fixture rail to `tools/check_mc_compat_row_contracts.rs`:

- `--emit-fixtures <dir>` writes one normalized `key=value` fixture per supported row.
- `--check-fixtures <dir>` validates those fixtures with the same positive, negative, evidence-standard, and non-claim rules as the row contract checker.

The fixture rail covers row contract shape only. It does not claim live runner support, Paper/Valence parity, parser semantic coverage, matrix promotion, full CTF correctness, full protocol-763 compatibility, production readiness, full survival compatibility, or broad vanilla/Minecraft compatibility.

Survival row `runner/client rail` tasks remain open because those rows require scenario-specific client/server implementation beyond this generic fixture rail.
