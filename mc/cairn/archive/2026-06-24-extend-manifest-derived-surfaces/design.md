# Design: Extend manifest-derived harness surfaces

## Context

The repository already has a scenario manifest and generated runner tables. The remaining repeated surfaces are mostly wrappers and documentation/index structures. Generation should expand carefully so human evidence interpretation remains hand-authored.

## Decisions

### 1. Start with a surface inventory

**Choice:** Record every scenario-derived surface, its owner, and whether it is generated, human-authored, or intentionally duplicated.

**Rationale:** A bounded inventory prevents accidental generation of prose or evidence interpretation.

### 2. Generate metadata, not arbitrary Nix prose

**Choice:** Use manifest-derived metadata to render stable wrapper/check data or small marked blocks. Keep complex Nix structure readable and imported from normal Nix modules.

**Rationale:** Generation should remove repetition without making the flake opaque.

### 3. Keep generated output checked in

**Choice:** Generation happens during dev/check flows and writes deterministic checked-in artifacts. Runtime code and flake evaluation consume static files.

**Rationale:** This preserves reviewability and avoids runtime Nickel dependencies.

### 4. Fail closed on unsafe output

**Choice:** New generator paths must reject duplicate output names, path escapes, unsupported migration states, unknown generated fields, and stale checked-in outputs.

**Rationale:** Generated code must not silently broaden claims or overwrite human-authored files.

## Risks / Trade-offs

- Generated wrapper blocks can hide important app behavior; mitigate with small generated data and human-owned orchestration modules.
- README churn can become noisy; mitigate with stable ordering from the manifest and bounded markers.
- The manifest can become too broad; mitigate by keeping only scenario-derived facts in the manifest.
