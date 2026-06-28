# Design: Introduce composable runner environment patches

## Context

The runner applies many environment variables for scenarios, probes, fixtures, backend selection, and session state. Directly mutating `Command` couples pure scenario decisions to process construction.

## Decisions

### 1. Add an `EnvPatch` value type

**Choice:** Represent env changes as data: key/value pairs, optional removals, source labels, and conflict diagnostics.

**Rationale:** Env derivation becomes pure, mergeable, serializable for diagnostics, and directly testable.

### 2. Use pure builders per concern

**Choice:** Provide builders for client probe env, Valence server env, Paper server env, persistence env, combat env, inventory env, survival env, and CTF env fragments.

**Rationale:** Reusable fragments let related scenarios compose behavior instead of copy-pasting `cmd.env` chains.

### 3. Shell applies patches to commands

**Choice:** Only backend/client shell code mutates `Command` by applying already validated patches.

**Rationale:** This preserves functional-core and imperative-shell boundaries.

### 4. Fail closed on conflicts

**Choice:** Patch composition should reject incompatible duplicate keys unless an explicit override policy is present.

**Rationale:** Env conflicts should be caught during planning or command construction, not discovered through live failures.

## Risks / Trade-offs

- Some existing env chains rely on ordering; represent ordering explicitly only when required.
- Patch types can become too generic; keep the first version focused on runner env needs.
- Backend-specific env needs may diverge; keep client, Valence, and Paper patch builders separate.
