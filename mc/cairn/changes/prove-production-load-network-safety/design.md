# Design: Production load and network safety proof

## Safety strategy

All load/network tests must be limited to owned local or explicitly authorized targets. Receipts must record target ownership, authorization source, client count, duration, reconnect count, latency/jitter/loss settings, and kill limits. Public or unbounded targets fail before launch.

## Envelope strategy

Define bounded envelopes instead of a single production claim: small multiplayer load, extended soak, reconnect churn, WAN-like latency/jitter, packet loss, and combined perturbation. Each envelope has a maximum parameter set and a pass/fail oracle.

## Telemetry strategy

Receipts should record server health, client outcomes, missing milestones, forbidden errors, resource usage summaries, and BLAKE3 hashes for logs. Promotion requires both dry-run fixture checks and live evidence.

## Risks

- Load tests can cause collateral damage. Default to dry-run and owned local environments.
- Unbounded wording invites overclaim. Every promoted row must state exact bounds and keep production/public safety broader than those bounds as a non-claim.
