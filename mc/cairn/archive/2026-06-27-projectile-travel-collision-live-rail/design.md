# Design: projectile travel/collision live rail

## Scope

This change adds one bounded live projectile travel/collision row. It builds on existing projectile use/loadout and damage-attribution rails, but it does not convert those rows into full projectile physics or vanilla parity evidence.

## Matrix row

The selected row should name:

- weapon representative;
- projectile representative;
- attacker identity;
- target identity or collision target;
- expected server projectile lifecycle events;
- expected client-visible projectile observations;
- ordered sequence requirements;
- tolerance or exact-match rules for the selected metrics;
- explicit non-claims.

The row must remain small enough to be reviewed from one receipt pair or one live receipt bundle.

## Functional core

Add a pure comparator over normalized projectile records:

- server spawn or launch event;
- client spawn visibility or first observation;
- ordered travel observation;
- collision or hit result;
- attacker/target/projectile correlation;
- weapon representative;
- non-claim labels.

The comparator returns stable diagnostics and rejects missing travel observations, missing collision result, wrong target, wrong weapon, unordered observations, duplicate ambiguous projectile identities, and overbroad vanilla-parity claims.

## Imperative shell

Runner/client/server shells emit typed projectile metrics and write receipts/logs. The shell may orchestrate the live owned-local clients and server, but checker logic stays pure over normalized records. Evidence promotion copies receipt, client log, server log, comparator output, and BLAKE3 manifests under `docs/evidence/`.

## Validation strategy

- Positive fixture: complete ordered projectile travel/collision row passes.
- Negative fixture: missing travel observation fails.
- Negative fixture: missing collision or hit result fails.
- Negative fixture: wrong target or wrong weapon fails.
- Negative fixture: unordered projectile observations fail.
- Negative fixture: evidence claims full vanilla projectile physics fails.
- Live closeout records maintained scenario run, comparator output, evidence manifests, task-evidence validation, Cairn gates, and Cairn validation.

## Non-claims

The row proves only the configured owned-local projectile representative and selected travel/collision outcome. It does not claim exact vanilla projectile physics, all projectile weapons, all collision surfaces, all enchantments/status effects, full combat correctness, public-server safety, production readiness, or broad Minecraft compatibility.
