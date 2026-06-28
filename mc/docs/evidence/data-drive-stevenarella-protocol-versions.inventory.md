# data-drive-stevenarella-protocol-versions inventory

## Question
What protocol-name and translation-dispatch behavior existed before introducing typed protocol-version metadata?

## Inspected evidence
- `clients/stevenarella/AGENTS.md` requires focused Cargo tests through the mc devshell and bounded compatibility dry-runs.
- `clients/stevenarella/protocol/src/protocol/versions.rs` before this change owned both public dispatch functions with hand-written match arms.
- `clients/stevenarella/protocol/src/protocol/mod.rs` before this change exposed `SUPPORTED_PROTOCOLS` in descending supported-version order.

## Current protocol-name table

| Input name | Protocol id | Notes |
| --- | ---: | --- |
| empty string | 763 | Uses first `SUPPORTED_PROTOCOLS` entry. |
| `1.20.1` | 763 | Current Valence/Paper compatibility target. |
| `1.18.2` | 758 |  |
| `1.18.1` | 757 |  |
| `1.17.1` | 756 |  |
| `1.16.5` | 754 | Same protocol id as `1.16.4`. |
| `1.16.4` | 754 | Alias-equivalent protocol id. |
| `1.16.3` | 753 | Reuses `v1_16_4` translation module. |
| `1.16.2` | 751 | Reuses `v1_16_4` translation module. |
| `1.16.1` | 736 |  |
| `1.16` | 735 | Reuses `v1_16_1` translation module. |
| `1.15.2` | 578 |  |
| `1.15.1` | 575 | Reuses `v1_15` translation module. |
| `1.14.4` | 498 |  |
| `1.14.3` | 490 |  |
| `1.14.2` | 485 |  |
| `1.14.1` | 480 |  |
| `1.14` | 477 |  |
| `19w02a` | 452 |  |
| `18w50a` | 451 |  |
| `1.13.2` | 404 |  |
| `1.12.2` | 340 |  |
| `1.11.2` | 316 |  |
| `1.11` | 315 | Reuses `v1_11_2` translation module. |
| `1.10.2` | 210 |  |
| `1.9.2` | 109 |  |
| `1.9` | 107 |  |
| `15w39c` | 74 |  |
| `1.8.9` | 47 |  |
| `1.7.10` | 5 |  |
| decimal string | parsed id | Numeric input is accepted even if unsupported by translation dispatch. |
| other string | panic | Message prefix: `Unrecognized protocol name:`. |

## Current translation dispatch table

| Protocol id | Translation module | Reuse/fallback relationship |
| ---: | --- | --- |
| 763 | `v1_20_1` | Owns 1.20.1 packet-boundary overrides. |
| 758 | `v1_18_2` | Own module. |
| 757 | `v1_18_1` | Own module. |
| 756 | `v1_17_1` | Own module. |
| 754 | `v1_16_4` | Canonical 1.16.5/1.16.4 protocol id. |
| 753 | `v1_16_4` | Reuses 754 module. |
| 751 | `v1_16_4` | Reuses 754 module. |
| 736 | `v1_16_1` | Own module. |
| 735 | `v1_16_1` | Reuses 736 module. |
| 578 | `v1_15` | Own module. |
| 575 | `v1_15` | Reuses 578 module. |
| 498 | `v1_14_4` | Own module. |
| 490 | `v1_14_3` | Own module. |
| 485 | `v1_14_2` | Own module. |
| 480 | `v1_14_1` | Own module. |
| 477 | `v1_14` | Own module. |
| 452 | `v19w02a` | Own module. |
| 451 | `v18w50a` | Own module. |
| 404 | `v1_13_2` | Own module. |
| 340 | `v1_12_2` | Own module. |
| 316 | `v1_11_2` | Own module. |
| 315 | `v1_11_2` | Reuses 316 module. |
| 210 | `v1_10_2` | Own module. |
| 109 | `v1_9_2` | Own module. |
| 107 | `v1_9` | Own module. |
| 74 | `v15w39c` | Own module. |
| 47 | `v1_8_9` | Own module. |
| 5 | `v1_7_10` | Own module. |
| other id | panic | Message prefix: `unsupported protocol version:`. |

## Decision
Use this inventory as the baseline parity target. The change remains a protocol-table maintainability change only and does not promote new packet support, broad compatibility, semantic equivalence, production readiness, or public-server safety claims.

## Owner
`clients/stevenarella/protocol/src/protocol/versions.rs`

## Next action
Run focused protocol tests before editing, then replace the hand-written protocol table with typed metadata and validators while preserving the public API behavior above.
