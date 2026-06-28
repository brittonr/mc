# Design: biome/dimension state correlation

## Scope

This change turns one join-state scenario into typed client/server correlation evidence. It does not cover dimension travel, portal transitions, respawn behavior, all biome lookup semantics, or world persistence.

## Evidence contract

The selected row is `survival-biome-dimension-state`. The typed receipt contract should record:

- scenario identity;
- protocol version;
- client-observed join-state dimension marker;
- client-observed environment or biome marker used by the existing probe;
- server-configured dimension/biome state for the fixture;
- correlation result;
- explicit non-claims.

The exact field names should reuse existing receipt vocabulary where possible so generated surfaces and evidence checkers do not grow a second naming scheme.

## Functional core

Add a pure validator over normalized join-state records. It accepts client observation, server fixture state, protocol context, and non-claim labels. It returns pass/fail diagnostics for missing fields, mismatched dimension, mismatched biome/environment marker, missing protocol context, and overbroad travel/parity claims.

## Imperative shell

Runner/client/server code emits the typed join-state fields and writes receipts. The shell may still preserve raw logs for review, but the row check uses structured receipt fields. Scenario manifest and fallback budget updates happen after the structured validator is in place.

## Validation strategy

- Positive fixture: matching server/client join-state record passes.
- Negative fixture: client-only observation fails.
- Negative fixture: server/client dimension mismatch fails.
- Negative fixture: missing protocol context fails.
- Negative fixture: receipt claims dimension travel or full survival parity fails.
- Closeout records focused scenario checks, generated surfaces, fallback budget checks, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation.

## Non-claims

This change proves only the configured join-state correlation row. It does not claim all biome semantics, all dimensions, portals, dimension travel, respawn behavior, world persistence, full survival compatibility, broad vanilla parity, public-server safety, or production readiness.
