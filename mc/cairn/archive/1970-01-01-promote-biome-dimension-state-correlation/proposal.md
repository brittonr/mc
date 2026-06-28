# Proposal: Promote biome/dimension state correlation

## Why

`survival-biome-dimension-state` remains a substring-fallback scenario and is currently bounded as a client environment observation. That keeps it useful as a smoke signal, but weak as reviewable evidence: it does not fail closed when the client observation and server-configured join state diverge.

A focused Cairn should add typed client/server correlation for the join-state row without claiming dimension travel, portal behavior, all biome semantics, or full survival compatibility.

## What Changes

- Inventory current biome/dimension state evidence, fixture configuration, and non-claims.
- Define a bounded typed receipt contract for the join-state observation: scenario identity, client-observed dimension/biome data, server-configured state, protocol version, and non-claim fields.
- Add pure validation that rejects client-only observations, server/client mismatches, missing protocol context, and overbroad dimension-travel claims.
- Update the scenario manifest from substring fallback to typed-event-ready only after validation exists.
- Refresh generated scenario surfaces, evidence docs, fallback budget, and BLAKE3 manifests.

## Impact

- **Files**: `compat/config/scenario-manifest.ncl`, runner receipt validation, Valence survival fixture/client probe as needed, generated surfaces, fallback-budget baseline, evidence docs/manifests, Cairn specs/tasks.
- **Testing**: positive typed join-state fixture; negative fixtures for missing server state, mismatched dimension, missing protocol context, and overbroad travel/parity claims; scenario manifest/generated/evidence checks; Cairn gates and validation.
- **Non-claims**: no all-biome semantics, dimension travel, portal behavior, respawn rules, full survival compatibility, broad vanilla parity, public-server safety, or production readiness claim.
