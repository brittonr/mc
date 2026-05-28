# Design: Vanilla combat parity proof

## Oracle strategy

Name one reference oracle before any parity proof: vanilla server, Paper configured as vanilla-compatible reference, or checked deterministic fixture derived from authoritative vanilla behavior. The oracle choice, version, configuration, and limitations must be tracked in evidence.

## Comparison strategy

Treat parity as comparison of metrics, not raw log equality. Metrics include damage delta, health after event, velocity vector/magnitude, mitigation amount, and projectile damage where applicable. Each metric needs a tolerance and a reason.

## Verification strategy

Positive fixtures prove equal-within-tolerance rows pass. Negative fixtures prove out-of-tolerance, missing-reference, wrong-version, and Valence-only evidence fails. Live evidence must pair Valence and reference runs for the same scenario row.

## Risks

- Reference behavior may vary by implementation or configuration. Pin versions and config.
- Physics/numeric tolerances can mask real divergences. Keep tolerances named, narrow, and justified.
