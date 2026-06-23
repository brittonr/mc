# Proposal: Add a profile and skin cache with rate limiting

## Why

Hyperion includes Mojang profile lookup helpers, optional mirror provider support, and local skin storage. Valence could benefit from a reusable profile/skin cache for login flows, player list data, and offline diagnostics, but the integration must avoid hard-coded providers, hard-coded paths, and unbounded external requests.

## What Changes

- Review Hyperion's Mojang client and local skin storage alongside Valence's login/profile flow.
- Define typed configuration for API providers, request budgets, cache storage, TTL policy, offline behavior, and privacy boundaries.
- Implement pure response parsing and cache-decision logic, with HTTP and storage in thin adapters.
- Add positive and negative fixtures for valid profile responses, missing fields, provider errors, rate-limit exhaustion, cache hits, cache misses, corrupted cache entries, and plugin-disabled behavior.
- Document provider configuration, data retention, and evidence/non-claim boundaries.

## Impact

- **Files**: optional Valence profile/cache crate or plugin, network/login adapters, config/docs, tests, and Cairn artifacts.
- **Testing**: parser fixtures, cache decision tests, fake-provider tests, storage corruption tests, login/profile smoke tests, and Cairn gates/validation.
- **Non-claims**: this does not require a specific cache backend, does not make a third-party provider default without policy approval, and does not bypass Mojang/Velocity authentication rules.
