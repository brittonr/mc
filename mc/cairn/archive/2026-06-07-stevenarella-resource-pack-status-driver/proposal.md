## Why

The resource-pack status row is blocked because Stevenarella does not have a maintained owned-local driver that receives a resource-pack offer and emits a configured status response through the protocol path. Implementing that driver would unlock real local evidence while avoiding external downloads or asset-trust claims.

## What Changes

- Add a bounded Stevenarella resource-pack status driver or control-plane action for one owned-local offer/status exchange.
- Ensure the driver can decline or otherwise report the configured status without fetching external assets.
- Record protocol/client/server evidence suitable for the targeted-packet live KV schema.
- Keep asset download/application, trust/security validation, all statuses, public-server safety, production readiness, and full protocol coverage as non-claims.

## Impact

- **Files**: `stevenarella/**` when implementation begins, `tools/mc-compat-runner/src/**`, `tools/check_targeted_packet_promotions.rs`, `docs/evidence/**`, and targeted packet docs if evidence later promotes the row.
- **Testing**: Stevenarella focused tests via the mc devshell, runner dry-run/unit checks, positive and negative driver validation, no-external-fetch checks, targeted-packet live-evidence checker, Cairn gates and validation.
