# Stevenarella sign-editor driver contract (2026-06-07)

## Bounded open/update contract

Change: `stevenarella-sign-editor-driver`.

Configured sign edit:
- actor: `compatbot`
- scenario: `survival-block-entity-persistence-parity`
- sign position: `28,64,0`
- initial state: `blank`
- submitted payload: `MC|Compat|Sign|Edit`
- line count: exactly `4`
- max line length: `64` Unicode scalar values per line
- required open state: Stevenarella must observe `SignEditorOpen`/`SignEditorOpen_i32` at the same position before protocol output
- expected client open milestone: `sign_editor_open_observed`
- expected client update milestone: `sign_update_sent`
- protocol output: Stevenarella control-plane action to serverbound `SetSign` / row `play/serverbound/0x2e UpdateSignC2SPacket`
- backend path: `deterministic-sign-editor-contract`
- client path: `stevenarella-sign-editor-driver`

## Evidence and validation shape

Driver tests must cover a valid configured open/update and invalid missing-open, wrong-position, malformed-payload, line-count, line-length, disconnected-state, and overclaim fixtures. Runner integration remains isolated to the `sign-editor-open-update` capability registry row and does not promote live evidence without a maintained live backend correlation receipt.

## Non-claims

No sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server behavior, production readiness, full protocol 763 compatibility, or broad Minecraft compatibility is claimed.
