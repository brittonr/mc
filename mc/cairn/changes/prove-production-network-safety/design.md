# Design: Production and network safety proof

## Claim separation

Separate four claims:

- owned-local load safety;
- public-server safety;
- WAN/jitter/loss tolerance;
- adversarial-network safety.

Each claim needs its own scope, authorization, bounds, telemetry, and pass/fail criteria.

## Evidence gates

The gate should reject production/public/WAN/adversarial claims unless evidence records target ownership, explicit authorization, client counts, duration bounds, network perturbation parameters, telemetry, and failure-mode handling.

## Safety

No public target or destructive load step should run without explicit user authorization. Dry-run receipts must remain available for command-shape validation.
