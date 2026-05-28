# Design: Armor, enchantment, and status modifier proof

## Matrix strategy

Represent mitigation breadth as a matrix of loadout, armor material, slot coverage, enchantment representative, status-effect representative, attack type, and expected damage delta. Start with representative rows and leave the rest explicit.

## Oracle strategy

Valence server evidence must record loadout, modifiers, raw damage, mitigated damage, and health delta. Stevenarella evidence must record the victim health update for the same bounded event. Negative fixtures must prove that wrong loadout or missing modifier evidence cannot satisfy a row.

## Evidence strategy

Every promoted row needs dry-run fixtures, runner tests, live receipt/logs, BLAKE3 sidecars, and matrix/bundle updates. If vanilla parity is mentioned, the row must name the reference oracle and tolerance; otherwise it claims only Valence/Stevenarella bounded behavior.

## Risks

- Modifier combinations can explode combinatorially. Use representative coverage with explicit non-claims for untested combinations.
- Exact vanilla parity requires a separate reference oracle and should not be implied by Valence-only evidence.
