# Design: Promote sign editor open/update evidence

## Context

Sign block-entity persistence covers server-provided sign NBT observation. It does not cover the user-facing sign editor or the serverbound sign update packet. This change should create a separate row so reviewers can distinguish sign payload observation from sign editing.

## Decisions

### 1. Use one fixed sign and one fixed payload

**Choice:** Target one configured sign position and one four-line payload such as `MC|Compat|Edit|OK`.

**Rationale:** A fixed payload makes client/server correlation deterministic and keeps all text variants and UI behavior out of scope.

### 2. Promote two adjacent packet rows only together

**Choice:** Promote `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket` only when one live receipt records both open and accepted update correlation.

**Rationale:** Opening the editor without a server-accepted update is too weak, and an update without editor-open observation could overclaim UI coverage.

### 3. Keep checker logic pure

**Choice:** Implement the row checker as an in-memory comparator over normalized fields.

**Rationale:** The core decision should be testable with positive and negative fixtures without starting Minecraft processes.

## Risks / Trade-offs

- Stevenarella may not support interactive sign UI; if so, add a bounded protocol-level update probe and keep UI semantics as a non-claim.
- Valence may require explicit fixture hooks to open the editor or accept updates deterministically.
- Existing sign persistence evidence must not be reused to claim editing behavior.
