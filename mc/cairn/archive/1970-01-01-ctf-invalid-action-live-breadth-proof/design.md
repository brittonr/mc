# Design: CTF invalid-action live breadth proof

## Scope

The live proof is limited to `opponent-base-return-drop-without-carrier`. It must run only against owned-local Valence CTF fixtures and must not change the semantics of existing invalid pickup/ownership or invalid own-base return/drop rows.

## Functional core

The invalid-action breadth checker remains a pure validator over in-memory row evidence. Extend it with a live row contract that requires:

- row id `opponent-base-return-drop-without-carrier`
- actor identity and team
- target flag identity and base context
- attempted invalid action identity
- client attempt event or typed event
- server rejection event
- unchanged flag ownership/state
- unchanged score state
- forbidden mutation/score/capture absence
- owned-local authorization and explicit non-claims
- tracked receipt/log/evidence artifact references and BLAKE3 manifests

Positive fixtures include a complete live row evidence record. Negative fixtures omit the server rejection, mutate score state, change actor/team correlation, or drop non-claims.

## Imperative shell

The runner shell owns process execution, fixture startup, receipt paths, typed-event sidecar writing, and artifact copying into `docs/evidence/`. It calls the pure checker after artifacts exist and leaves any broader matrix promotion to explicit docs updates.

## Validation strategy

- Record baseline invalid-action breadth checker and CTF dry-run status.
- Add the live scenario and server/client instrumentation needed for the bounded row.
- Run positive and negative checker fixtures.
- Produce owned-local live receipt, typed-event log, run log, and BLAKE3 manifest under `docs/evidence/`.
- Update matrix/current bundle to cite only the new bounded live row and preserve broad non-claims.
- Run evidence manifest checks, matrix/current-bundle checks, Cairn gates, task-evidence validation, and Cairn validation.

## Non-claims

The row remains one bounded live containment proof. It does not claim all invalid actions, all flag permutations, full CTF correctness, adversarial security, public-server safety, production readiness, broad Minecraft compatibility, or vanilla/reference parity.
