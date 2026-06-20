# Protocol-763 current evidence bundle

## Scope

Current-head index for the maintained Stevenarella ⇄ Valence CTF protocol-763 compatibility evidence set. This bundle summarizes the acceptance matrix and gives operators one stable checklist for what is covered and what remains a non-claim.

- Matrix: `docs/evidence/protocol-763-acceptance-matrix.md`
- Matrix checker: `tools/check_acceptance_matrix.rs`
- Bundle checker: `tools/check_current_evidence_bundle.rs`
- Evidence manifest checker: `tools/check_evidence_manifests.rs`
- Python checker migration checkpoint: `docs/evidence/protocol-763-python-gate-rust-migration-oracle-2026-05-31.md`
- Latest parent checkout before this bundle refresh: `5d4973d add Paper survival reference fixture`
- Child commits used for the current-head representative refresh: Valence `e5d18ad`, Stevenarella `616ee72`
- Child commits used for the RED/BLUE scoring soak live refresh: Valence `f57a325`, Stevenarella `1ab97d2`; machine-recorded in the copied receipts.
- Child commits used for the survival reference parity refresh: Valence `7d13a24`, Stevenarella `d758630`; machine-recorded in the paired receipts.
- Child commits used for the survival furnace persistence row: Valence `61a1f77`, Stevenarella `d9caec5`; machine-recorded in `docs/evidence/survival-furnace-persistence-valence-2026-06-01.receipt.json` and `docs/evidence/survival-furnace-persistence-paper-2026-06-01.receipt.json`.
- Child commits used for the survival hunger/food row: Valence `573b8f0`, Stevenarella `e996816`; machine-recorded in `docs/evidence/survival-hunger-food-valence-2026-06-02.receipt.json` and `docs/evidence/survival-hunger-food-paper-2026-06-02.receipt.json`.
- Child commits used for the survival mob-drop row: Valence `eba9c8a`, Stevenarella `7122e47`; machine-recorded in `docs/evidence/survival-mob-drop-valence-2026-06-02.receipt.json` and `docs/evidence/survival-mob-drop-paper-2026-06-02.receipt.json`.
- Child commits used for the survival redstone-toggle row: Valence `469558f`, Stevenarella `357021d`; machine-recorded in `docs/evidence/survival-redstone-toggle-valence-2026-06-02.receipt.json` and `docs/evidence/survival-redstone-toggle-paper-2026-06-02.receipt.json`.
- Child commits used for the survival world-persistence row: Valence `6117703`, Stevenarella `0a745b0`; machine-recorded in `docs/evidence/survival-world-persistence-valence-2026-06-02.receipt.json` and `docs/evidence/survival-world-persistence-paper-2026-06-02.receipt.json`.
- Child commits used for the survival crash-recovery row: Valence `ed1ed033`, Stevenarella `75151ca`; machine-recorded in `docs/evidence/survival-crash-recovery-valence-2026-06-04.receipt.json`, `docs/evidence/survival-crash-recovery-paper-2026-06-04.receipt.json`, and `docs/evidence/survival-crash-recovery-revision-oracle-2026-06-04.md`.
- Child commits used for the survival sign block-entity and block-entity sign packet-family rows: Valence `f54e6d0`, Stevenarella `79d766c`; machine-recorded in `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json`, `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json`, and `docs/evidence/block-entity-sign-packet-family-2026-06-06.receipt.json`.
- Child commits used for the MCP-controlled observability row: Valence `3359f85`, Stevenarella `4d1b155`; machine-recorded in `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.json`.
- Child commits used for the bounded Paper-reference combat parity row: Valence `be02847e`, Stevenarella `75151ca`; machine-recorded in `docs/evidence/vanilla-combat-reference-paired-valence-receipt-2026-06-02.json` and `docs/evidence/vanilla-combat-reference-paired-paper-receipt-2026-06-02.json`.
- Child commits used for the bounded Paper-reference armor combat parity row: Valence `f41e8649`, Stevenarella `75151ca`; machine-recorded in `docs/evidence/vanilla-combat-armor-reference-paired-valence-receipt-2026-06-03.json` and `docs/evidence/vanilla-combat-armor-reference-paired-paper-receipt-2026-06-03.json`.
- Child commits used for the inventory stack split/merge row: Valence `a8ff748`, Stevenarella `722d950`; machine-recorded in `docs/evidence/inventory-stack-split-merge-2026-06-05.json`.
- Child commits used for the scoreboard/team packet-family row: Valence `f40d6d6`, Stevenarella `d9caec5`; machine-recorded in `docs/evidence/scoreboard-team-packet-family-2026-06-06.receipt.json`.
- The targeted packet promotion rows added on 2026-06-06 are deterministic fixture rows, not live child-checkout claims; their source-closure evidence is machine-recorded in `docs/evidence/*-2026-06-06.kv`, `docs/evidence/*-2026-06-06.receipt.json`, and `docs/evidence/targeted-packet-promotions-2026-06-06.run.log`.
- The CTF invalid-action breadth fixture row added on 2026-06-19 is deterministic fixture evidence, not live child-checkout gameplay evidence; its source-closure evidence is machine-recorded in `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.md`, `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.receipt.json`, and `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.run.log`.
- The maintained dry-run aggregate includes deterministic receipt-shape coverage for historical rows such as `flag-score-repeat`, `survival-chest-persistence`, and the survival reference rows that already have live/reference evidence below. These dry-run receipts only prove harness wiring and schema shape; they do not replace the promoted live/paired-reference evidence or broaden parity claims.
- Broad survival wording is guarded by `tools/check_survival_aggregate_claim_boundary.rs` and focused check `mc-compat-survival-aggregate-claim-boundary`; the gate keeps aggregate survival claim blocked while active breadth prerequisites lack paired Paper/reference and Valence evidence. Row-scoped reference parity remains bounded survival row evidence only, and full_survival_compatibility remains a non-claim.
- The scenario manifest now distinguishes `typed-event-ready` rows (`smoke` and `inventory-interaction`) from waiver-backed `substring-fallback` rows. Typed-event-ready status is an observability/pass-fail wiring claim only; remaining fallback rows share manifest-level owner/reason/non-claim/next-action metadata until typed-event fixtures cover their required client, server, and forbidden surfaces.
- Runner receipt-schema tests now parse evidence-critical JSON fields structurally for non-claims, child revision status, typed-event artifacts, MCP/frame evidence, backend identity, duplicate/wrong-typed fields, and broad overclaim guards. This hardens receipt validation only; it does not add scenario coverage or broaden compatibility claims.

## Evidence rows

| Seam | Maintained command | BLAKE3 |
| --- | --- | --- |
| RED/BLUE scoring soak | `nix run .#mc-compat-valence-ctf-600s-soak`; `nix run .#mc-compat-valence-ctf-blue-600s-soak` | `349b1b7dc84877dd56dce3344611d04ce86a74413738ebc3fdd2a2f720504bed` |
| Inventory/drop | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `4aeb08172b35edd03d57169c63a4942ca149c783fbc51539702922ac246a0e46` |
| Block placement / use-item-on-block | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `9feec3b967b3fd5cb011139eda524c32c73123323823b3ebef7bd93062e1c122` |
| Pickup semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `bcac4aab63857cf0d3b6dd148455324e7f0368dd3e57cfd26841ae7fc1b5ffe8` |
| Player-inventory click/container click | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `c75381feed1d98cd33d584ab9b8efdfe849d85eb3d1bb6cc23a23578cc8d7f7d` |
| Open-container semantics | `nix run .#mc-compat-valence-ctf-inventory-interaction` | `b7913ddd1f000981f411f7f14331b67820761c1d317c528fbf8a5070c139d3f3` |
| Inventory transaction packet family | scenario-bounded inventory click receipts plus negative containment rails | `df02262c86a9b1966e613a8485464ea234aa31c06acd06230f3de3123e349d61` |
| Inventory stack split/merge | `nix run .#mc-compat-valence-inventory-stack-split-merge` | `aed5b9f833e877525cb6444cd51ced99b72c0d06c96f70108ca3e19260252dc7` |
| Inventory drag transactions | `nix run .#mc-compat-valence-inventory-drag-transactions` | `717b2f7f3c95da14af10564c66bc5a962bb03415e169ceb953cc71d60c856339` |
| Two-client combat/damage | `nix run .#mc-compat-valence-ctf-combat-damage` | `b67962dd5d4fe7242b69fd7c879390e80e13528475d55d7feb5305289f762ac8` |
| Entity metadata packet family | bounded Valence `ctf` metadata probe | `7e7fc51be2b85c1ceb37890494cf1dda76b28e91220993bc18d47fd2043fdebc` |
| Flag-carrier death/return | `nix run .#mc-compat-valence-ctf-flag-carrier-death-return` | `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4` |
| Reconnect flag-state | `nix run .#mc-compat-valence-ctf-reconnect-flag-state` | `4d848af56b25ad4b3c466863bac5b2052adbbc1c59e2b2164bfb2a696c225cb3` |
| Invalid flag pickup/ownership | `nix run .#mc-compat-valence-ctf-invalid-pickup-ownership` | `64c353dc5f256526d4ecfb4078516e85491b42fc9da10adf8e91a7c2c166b8ac` |
| Invalid flag return/drop | `nix run .#mc-compat-valence-ctf-invalid-return-drop` | `f0465c4ad154c051ee21bbe96bac939dad875ac3bdaaa785051cdb58636ba2ba` |
| CTF invalid-action breadth fixture | `tools/check_ctf_invalid_action_breadth.rs` deterministic fixture-only invalid-action breadth row | `b14786d1408fc3136284cfd83f68e8b8648d52576641784ebf003ba479a9f797` |
| Score limit / win condition | `nix run .#mc-compat-valence-ctf-score-limit-win-condition` | `7c0d7805e54d95f2768f0164f1b4e62f59f57d524f3a61c3205eb0d611e89e02` |
| Simultaneous pickup/capture race | `nix run .#mc-compat-valence-ctf-simultaneous-pickup-capture-race` | `cc0b21579b8c5d99aa0d2bab04cc1ec3a34ecbdfceee2edc1ba0e497c11f977f` |
| Spawn/team balance/resource reset | `nix run .#mc-compat-valence-ctf-spawn-team-balance-reset` | `ce4ec8f61c956d5083d6701915a44b9e31c8e0adbfd018b25878774e516f2e6f` |
| Latency/jitter tolerance | `nix run .#mc-compat-valence-ctf-latency-jitter-inventory` | `a4a407fb1ac3aceae06faeacb794891ff8411c8ac86470c651c89b37b6c7f33d` |
| Combat knockback | `nix run .#mc-compat-valence-ctf-combat-knockback` | `a5d0ba5ea6155a99b58f245a03195da05b4925d7bd151b5b3f67503ae7a4cf09` |
| Bounded Paper-reference combat parity | paired Paper/Valence `vanilla-combat-reference-parity` receipts plus Rust comparator | `842d5e2d226c547dccaed6f1b3ca50db92d6621d1012b37b593e92c8db4255b7` |
| Bounded Paper-reference armor combat parity | paired Paper/Valence `vanilla-combat-armor-reference-parity` receipts plus Rust comparator | `997e3545cd260f86ea2a5dd7eb231360f77d560539500eb4bd44caf80bc0ea9f` |
| Armor equipment mitigation | `nix run .#mc-compat-valence-ctf-armor-equipment-mitigation` | `3152241bbbca379405a3806987f0b4dc8e4706b291cecebc1f509d0f96914f07` |
| Equipment update observation | `nix run .#mc-compat-valence-ctf-equipment-update-observation` | `8100dde3ebb3476984235009e277d7e973037b7873b2fdb30c413093e1498d3d` |
| Equipment permutation packet family | scenario-bounded equipment observer receipt plus row contract fixture | `bb6bf5c18357e9ebb26ff43d72a094923b0c7dbe7726e52f65186fadd36def02` |
| Projectile use/loadout rail | `nix run .#mc-compat-valence-ctf-projectile-hit` | `22310a0373f86bbff5e6bc116934d092b89f775cf5d539b08d04ff5564ad855b` |
| Projectile damage attribution | `nix run .#mc-compat-valence-ctf-projectile-damage-attribution` | `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529` |
| Command/recipe/advancement packet family | paired `survival-crafting-table` receipts plus parser fixture ledger | `1d5086ce48a86e00e2d96b221e8e32bc843a475c47a8324242d182f05994b670` |
| Chunk/biome packet family | paired `survival-biome-dimension-state` receipts plus parser fixture ledger | `d0eafb817292c04fbab78b019c74568b4cd4bcd6eb32e4f718b8f5d3909faaf1` |
| Scoreboard/team packet family | bounded `ctf-spawn-team-balance-reset` receipt plus packet-family checker | `f5ebee6f2816d00861cb6626545a2ff54992e1979a709c24bcda14863bcd4b79` |
| Movement packet family | `mc-compat-valence-movement-packet-family` bounded movement evidence plus packet-family checker | `54316de8e1eb74af49d010174571ad435eb6e3d809be976ab3cd55d463c0a786` |
| Block-entity sign packet family | paired `survival-block-entity-persistence-parity` receipts plus packet-family checker | `819e7e97b0af8dad3b60e14ad7e5f370250d39d5b101a1b6cd126f970106c0ad` |
| Block-entity update breadth | deterministic non-sign block-entity update fixture plus targeted packet promotion checker | `338c3ee9254588c662be2c399129defbfc27ca1bf47d853613168e4a8378e054` |
| Chat/command containment | deterministic owned-local chat/command containment fixture plus targeted packet promotion checker | `89c19cd42e5cb1a788656bdb18b43f49a7bc568da17ae219c38913d8da4511e0` |
| Chunk biome data packet | deterministic `ChunkBiomeDataS2CPacket` fixture plus targeted packet promotion checker | `f47be42e982e2702edc585712bb4ad9ff235e2cd25fd5fd8c3232664504d185b` |
| Creative inventory action | deterministic `CreativeInventoryActionC2SPacket` fixture plus targeted packet promotion checker | `ee1a04e9b6a48c20963441149f4097ae14c6b18efe1a157958d423412bc318d9` |
| Entity status-effect packets | deterministic status-effect apply/remove fixture plus targeted packet promotion checker | `1c4f1d08a7277993fd9bcd5ffe7bd9f726fae8620630c86aef8f06f7a4c011e7` |
| Recipe-book client settings | deterministic `RecipeBookDataC2SPacket` fixture plus targeted packet promotion checker | `cf0c6876a78cfd42499e819f7c795aff3b1a9fa217c114f6b2e65b00e0753828` |
| Resource-pack status | deterministic local resource-pack offer/status fixture plus targeted packet promotion checker | `f914f5da8fcd35d98b27a135360f281babec0f16097b410f53c246b0f5b29148` |
| Sign editor open/update | deterministic sign editor open/update fixture plus targeted packet promotion checker | `183e79a0a404c717f4ed5853f5df4c136a6afb1ac0d0cd4371799c211f08d675` |
| Survival break/place/pickup | Paper+Valence paired `survival-break-place-pickup` receipts | `a88fe547bfe2dd43fff3ac5bd967f0ebf5a3c539403211dd029865293130090b` |
| Survival chest persistence | Paper+Valence paired `survival-chest-persistence` receipts | `3dd16d3d15f47793505e97a088408d039c6cd45a73f288c7301c5e4f3f4851cf` |
| Survival crafting table | Paper+Valence paired `survival-crafting-table` receipts | `710f64a04451a62604d17a78cc84f3e2db84ec3d7034b7feaa149b1e8af57a15` |
| Survival furnace persistence | Paper+Valence paired `survival-furnace-persistence` receipts | `f68cdd3bfb71fe81752328f2ff98e10e8df13a08e1e29c01fe49924948b3373c` |
| Survival hunger/food | Paper+Valence paired `survival-hunger-food` receipts | `fcc6665435fbbe9a52af2923f295c219df2a9c2cb55a726845d0e0970cf7e579` |
| Survival mob drops | Paper+Valence paired `survival-mob-drop` receipts | `09e75b4824c65326809233b60b0ee3e639f638865d2860c6e8f8346553643701` |
| Survival redstone toggle | Paper+Valence paired `survival-redstone-toggle` receipts | `7a4378075f6392f5893e0797975df01a6dd42ce3ffd797d76057052a7b345eb1` |
| Survival biome/dimension join state | Paper+Valence paired `survival-biome-dimension-state` receipts | `4c8f59af896af4c3c6f0733a5069350441fc95f02fb8282eaac4097a906c7207` |
| Survival world persistence restart | Paper+Valence paired `survival-world-persistence-restart` receipts | `6eb03282f468a6231b367a6aa6bbd3c2d8ff984ece2438641e75e123e934692f` |
| Survival crash recovery | Paper+Valence paired `survival-crash-recovery-parity` receipts | `12be43fa05be32a1f803097c95704ba237778a8b8ab96ba5bc157accffc213fe` |
| Survival sign block-entity persistence | Paper+Valence paired `survival-block-entity-persistence-parity` receipts | `03339dbd540b31b74b9825907a300a89289d86f299d3c77c2666f2528391583e` |
| MCP-controlled observability | `nix run .#mc-compat-mcp-controlled-smoke -- --run` | `5eaa78082bfca069219fed40939e7003c9fce4e5f1d527a68ecdbbae9d610acf` |

## MCP-controlled observability checkpoint

The MCP-controlled observability row is validated by `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.md`, `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.json`, the MCP transcript `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt.mcp-transcript.log`, the captured frame `docs/evidence/mcp-controlled-compat-rail-live-2026-05-31.receipt-frames/mcp-controlled-smoke/latest-frame.png`, and `tools/check_mcp_controlled_compat_rail.rs`. Covered row is one owned-local Valence `survival_compat` run driven through Stevenarella MCP stdio with initialize, tools/list, status, look, key, chat, and `capture_latest_frame`; the receipt records `stdout_clean=true`, `revision_status=clean`, child revisions, `capture_latest_frame.captured`, and one BLAKE3-addressed PNG frame artifact under `docs/evidence/`. Visual regression approval, semantic gameplay equivalence, full Minecraft compatibility, production readiness, public-server safety, load testing, broad MCP API coverage, and screenshot-only correctness remain non-claims.

## Public server authorized safety checkpoint

The public-server authorization fixture row is validated by `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.md`, `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.receipt.json`, `docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md`, and `tools/check_public_server_authorized_safety.rs`. The production/network matrix marks only `covered_authorized_fixture_only` for the `public-server safety` row, with `target_owner=review-fixture-owner`, `target_scope=authorized-non-loopback-fixture`, `client_count=1`, `duration_secs=30`, `traffic_limits=client_count<=1,duration_secs<=30,status_probe_only,live_traffic_enabled=false`, and `redaction_policy=no_secrets_no_raw_public_address`. live public-server safety remains a non-claim; third-party target safety without authorization, production readiness, adversarial safety, WAN tolerance, load safety beyond configured bounds, and unbounded public testing remain non-claims.

## WAN tolerance bounded telemetry checkpoint

The bounded owned-local WAN telemetry row is validated by `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.md`, `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.receipt.json`, and `tools/check_wan_tolerance_bounded_telemetry.rs`. The production/network matrix marks only `covered_owned_local_bounded_telemetry` for the `WAN tolerance` row, with `target_ownership=owned-local-loopback`, `authorization=owned-local-fixture-approved`, `delay_ms=80`, `jitter_ms=30`, `loss_percent=0`, `client_count=1`, and `reconnect_count=0`. Public/internet WAN safety remains a non-claim. Packet-loss tolerance beyond the recorded zero-loss envelope, adversarial-network safety, production readiness, unbounded soak/reconnect safety, and third-party target safety remain non-claims.

## Adversarial-network fixture oracle checkpoint

The fixture-only adversarial-network oracle row is validated by `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.md`, `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.receipt.json`, `docs/evidence/protocol-763-adversarial-network-oracle-2026-05-29.b3`, and `tools/check_adversarial_network_oracle.rs`. The production/network matrix marks only `covered_fixture_oracle_only` for this deterministic owned-local fixture row, with threat model `protocol763-custom-payload-truncated-varint-v1`, mutation type `custom_payload_truncated_varint`, `max_mutated_packets=1`, `max_payload_bytes=64`, `live_network_enabled=false`, and `fixture-only-approved` authorization. Live adversarial-network safety remains a non-claim; malicious-client resilience, hostile-internet safety, public-server safety, production readiness, unbounded adversarial robustness, and full protocol security remain non-claims.

## Inventory semantics matrix checkpoint

The maintained inventory rows include the five-row bounded matrix in `docs/evidence/protocol-763-inventory-semantics-matrix-2026-05-27.md`, the dedicated stack split/merge row in `docs/evidence/inventory-stack-split-merge-2026-06-05.md`, the dedicated drag transaction row in `docs/evidence/inventory-drag-transactions-2026-06-06.md`, and the deterministic creative action fixture in `docs/evidence/creative-inventory-action-2026-06-06.md`. Covered rows are drop, pickup, player-inventory click, open-container click, block placement/use-item-on-block, one `RedWool x64` slot `37` to slot `38` split/merge-back fixture, one `RedWool x64` slot `37` drag across slots `38` and `39` ending `0/32/32`, and one creative-mode hotbar mutation fixture. `docs/evidence/protocol-763-negative-live-rails-2026-05-29.md` adds bounded negative containment receipts for stale state-id and invalid slot/window probes, but these do not promote full inventory semantics, malformed-click breadth, drag modes or distributions beyond the configured row, broad creative inventory semantics, or all-window coverage.

## Inventory transaction packet-family checkpoint

The bounded inventory transaction row is validated by `docs/evidence/protocol-inventory-transaction-family-2026-06-01.md`, receipt `docs/evidence/protocol-inventory-transaction-family-2026-06-01.receipt.json`, row contract evidence `docs/evidence/protocol-inventory-transaction-family-2026-06-01.kv`, the inventory semantics matrix, and negative live inventory containment rails. Covered packet row is `play/serverbound/0x0b ClickSlotC2SPacket -> ClickWindow_StateBeforeSlot` for scenario-bounded player-inventory/open-container clicks plus one stale-state and one invalid-click containment row. The dedicated stack split/merge row is separately validated by `docs/evidence/inventory-stack-split-merge-2026-06-05.md`, `docs/evidence/inventory-stack-split-merge-2026-06-05.kv`, and `tools/check_inventory_stack_split_merge_evidence.rs`; the dedicated drag row is separately validated by `docs/evidence/inventory-drag-transactions-2026-06-06.md`, `docs/evidence/inventory-drag-transactions-2026-06-06.kv`, and `tools/check_inventory_drag_transactions_evidence.rs`; the creative action fixture is separately validated by `docs/evidence/creative-inventory-action-2026-06-06.kv` and `tools/check_targeted_packet_promotions.rs`. The packet inventory still marks inventory transactions as scenario-bounded with parser shape missing, so broad parser-shape coverage, all inventory semantics, all windows, all click modes, drag modes or distributions beyond the configured row, broad creative inventory semantics, full protocol-763 compatibility, full Minecraft compatibility, and production readiness remain non-claims.

## Entity metadata packet-family checkpoint

The bounded entity metadata row is validated by `docs/evidence/protocol-entity-metadata-family-2026-06-01.md`, receipt `docs/evidence/protocol-entity-metadata-family-2026-06-01.receipt.json`, row contract evidence `docs/evidence/protocol-entity-metadata-family-2026-06-01.kv`, and historical Stevenarella/Valence metadata-boundary evidence. Covered packet row is `play/clientbound/0x52 EntityTrackerUpdateS2CPacket -> EntityMetadata` with bounded probe evidence showing protocol `763` reached join/chunk/render milestones without recurrence of the prior `EntityMetadata` `FromUtf8Error`, `UnexpectedEof`, panic, or failed packet parse. The packet inventory still marks this row as scenario-bounded with parser shape missing, so broad parser-shape coverage, all entity metadata, all metadata value types, mob AI semantics, malformed metadata breadth, full protocol-763 compatibility, full Minecraft compatibility, and production readiness remain non-claims.

## Equipment slot/item matrix checkpoint

The maintained equipment update row is validated as one bounded slot/item matrix row in `docs/evidence/protocol-763-equipment-slot-item-matrix-2026-05-27.md` and bound to the normalized `equipment-slot-item-matrix-expansion` contract in `docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.md`. Covered row is `remote_main_hand_slot4_item829_count1_non_empty` / `main_hand_remote_entity / slot4 / item id 829 / count 1 / non_empty_update / after_remote_spawn` with one remote-spawn-correlated equipment update. All equipment slots/items/permutations beyond this row remain a non-claim.

## Equipment permutation packet-family checkpoint

The bounded equipment packet-family row is validated by `docs/evidence/protocol-equipment-permutation-family-2026-06-01.md`, receipt `docs/evidence/protocol-equipment-permutation-family-2026-06-01.receipt.json`, row contract evidence `docs/evidence/protocol-equipment-permutation-family-2026-06-01.kv`, the equipment slot/item matrix docs, and the live equipment observer receipt. Covered packet row is `play/clientbound/0x55 EntityEquipmentUpdateS2CPacket -> EntityEquipment_Array` for one scenario-bounded remote-player update: entity `4`, `slot4/main_hand_remote_entity`, item id `829`, count `1`. The packet inventory still marks this row as scenario-bounded with parser shape missing, so broad parser-shape coverage, all equipment permutations, armor mitigation, combat balancing, full protocol-763 compatibility, full Minecraft compatibility, and production readiness remain non-claims.

## Armor/enchantment/status modifier checkpoint

The maintained armor mitigation row is validated as one bounded modifier row in `docs/evidence/protocol-763-armor-modifier-matrix-2026-05-27.md` and bound to the normalized `armor-loadout-enchantment-status-matrix` contract in `docs/evidence/protocol-763-armor-loadout-enchantment-status-row-2026-05-29.md`. Covered row is `chest_diamond_none_none_melee` / `armor_loadout_chest_only / DiamondChestplate / enchantment_none / status_effect_none / melee`; the historical ROI receipt digest `176fdf33d2b8b9047471f577a98f9093904a44ab8da2785baeb80acfc8d97765` remains review history. All armor loadouts beyond this row, enchantments beyond `enchantment_none`, status effects beyond `status_effect_none`, modifier stacking, and exact vanilla parity remain non-claims.

## Projectile travel/collision checkpoint

The maintained projectile rows are validated as two bounded projectile state rows in `docs/evidence/protocol-763-projectile-travel-collision-2026-05-27.md`: projectile use/loadout and pinned projectile damage attribution. The damage row covers bounded server projectile hit/damage attribution; continuous projectile travel/collision simulation, all projectile weapons, and full projectile physics remain non-claims.

## Command/recipe/advancement packet-family checkpoint

The bounded command/recipe family row is validated by `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.md`, receipt `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.receipt.json`, row contract evidence `docs/evidence/protocol-command-recipe-advancement-family-2026-06-01.kv`, parser ledger `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md`, and the paired survival crafting receipts. Covered packet rows are `play/clientbound/0x10 CommandTreeS2CPacket -> DeclareCommandsRaw` and `play/clientbound/0x6d SynchronizeRecipesS2CPacket -> SynchronizeRecipesRaw`, with raw byte-preservation parser fixtures and one live crafting feature path. No advancement packet row, all-command semantics, all-recipe semantics, recipe-book behavior, command execution semantics, malformed semantic rejection for byte-opaque raw consumers, full protocol-763 compatibility, full Minecraft compatibility, or production readiness is claimed.

## Chunk/biome packet-family checkpoint

The bounded chunk/biome family row is validated by `docs/evidence/protocol-chunk-biome-family-2026-06-01.md`, receipt `docs/evidence/protocol-chunk-biome-family-2026-06-01.receipt.json`, row contract evidence `docs/evidence/protocol-chunk-biome-family-2026-06-01.kv`, parser ledger `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md`, and the paired survival biome/dimension receipts. Covered packet row is `play/clientbound/0x43 ChunkDeltaUpdateS2CPacket -> ChunkDeltaUpdateRaw`, with raw byte-preservation parser fixtures and one live overworld environment path. `docs/evidence/chunk-biome-data-packet-2026-06-06.md` separately covers one deterministic `ChunkBiomeDataS2CPacket` fixture. No live chunk-biome parity, all-worldgen-packet, all-chunk, all-biome, biome lookup, chunk-delta semantic, dimension travel, Nether/End, world persistence, full protocol-763 compatibility, full Minecraft compatibility, or production readiness claim is made.

## Scoreboard/team packet-family checkpoint

The bounded scoreboard/team packet row is validated by `docs/evidence/scoreboard-team-packet-family-2026-06-06.md`, normalized receipt `docs/evidence/scoreboard-team-packet-family-2026-06-06.receipt.json`, normalized KV input `docs/evidence/scoreboard-team-packet-family-2026-06-06.kv`, source live receipt `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json`, and `tools/check_scoreboard_team_packet_family.rs`. Covered packet row is `play/clientbound/0x5a TeamS2CPacket -> Teams_VarInt` for one bounded two-client CTF spawn/team context: `compatbota` RED, `compatbotb` BLUE, team counts `red=1,blue=1`, client `team_red`/`team_blue` observations, and Valence server red/blue assignment plus team-balance correlation. The packet inventory still marks this row as scenario-bounded with parser shape missing, so scoreboard UI parity, all scoreboards, all team rules, objective/display/score variants, full CTF correctness, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain non-claims.

## Movement packet-family checkpoint

The bounded movement packet row is validated by `docs/evidence/movement-packet-family-2026-06-06.md`, normalized receipt `docs/evidence/movement-packet-family-2026-06-06.receipt.json`, normalized KV input `docs/evidence/movement-packet-family-2026-06-06.kv`, source live receipt `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json`, source snapshot `docs/evidence/movement-packet-family-stevenarella-source-2026-06-06.rs`, and `tools/check_movement_packet_family.rs`. Covered packet row is `play/serverbound/0x15 Full -> PlayerPositionLook` for one bounded CTF team-selection movement: `compatbota` sends the RED portal target `-4.0,84.0,4.0` with `yaw=0.0`, `pitch=0.0`, and `on_ground=true`, and Valence records the corresponding RED team assignment. The packet inventory still marks this row as scenario-bounded with parser shape missing, so movement physics, collision, anti-cheat, latency tolerance, malicious-client resilience, all movement variants, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain non-claims.

## Block-entity sign packet-family checkpoint

The bounded block-entity sign packet row is validated by `docs/evidence/block-entity-sign-packet-family-2026-06-06.md`, normalized receipt `docs/evidence/block-entity-sign-packet-family-2026-06-06.receipt.json`, normalized KV input `docs/evidence/block-entity-sign-packet-family-2026-06-06.kv`, paired source receipts `docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json` and `docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json`, source row parity log `docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log`, and `tools/check_block_entity_sign_packet_family.rs`. Covered packet row is `play/clientbound/0x08 BlockEntityUpdateS2CPacket` for one configured sign payload: actor `compatbot`, kind `Sign`, position `28,64,0`, text `MC|Compat|Sign|Persist`, Paper/reference and Valence client observation, and paired row parity. `docs/evidence/block-entity-update-breadth-2026-06-06.md` separately covers one non-sign block-entity fixture, and `docs/evidence/sign-editor-open-update-2026-06-06.md` separately covers one deterministic sign editor open/update fixture. Sign editing UI parity, all block entities, arbitrary NBT, all sign text variants, all sign sides, all block-entity packet shapes, broad parser-shape coverage, full protocol-763 compatibility, broad Minecraft compatibility, public-server safety, and production readiness remain non-claims.

## Targeted packet promotions checkpoint

The targeted fixture rows are validated by `tools/check_targeted_packet_promotions.rs`, normalized KV inputs under `docs/evidence/*-2026-06-06.kv`, JSON receipts under `docs/evidence/*-2026-06-06.receipt.json`, and `docs/evidence/targeted-packet-promotions-2026-06-06.run.log` with `exit_status=0`. Covered rows are one non-sign `BlockEntityUpdateS2CPacket`, one owned-local chat/command containment fixture, one `ChunkBiomeDataS2CPacket` byte-preservation fixture, one `CreativeInventoryActionC2SPacket` fixture, one status-effect apply/remove fixture, one `RecipeBookDataC2SPacket` settings transition fixture, one `ResourcePackSendS2CPacket`/`ResourcePackStatusC2SPacket` offer/status fixture, and one `SignEditorOpenS2CPacket`/`UpdateSignC2SPacket` sign editor open/update fixture. These are deterministic fixture promotions and do not claim live Paper/Valence parity or child-checkout behavior.

## Survival break/place/pickup checkpoint

The maintained survival parity row is validated by `docs/evidence/protocol-763-survival-reference-parity-2026-05-28.md` with paired Paper and Valence receipt/log bundles from committed child revisions. The paired receipts record `client.git_rev`, `client.git_status`, `valence.git_rev_requested`, and `valence.git_rev_resolved`; `docs/evidence/protocol-763-survival-child-revision-oracle-2026-05-28.md` remains as review history. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture with exact join/render, fixed-coordinate block break, pickup/inventory observation, and block placement metrics. Full survival compatibility and vanilla parity remain non-claims.

## Survival chest persistence checkpoint

The maintained chest persistence row is validated by `docs/evidence/protocol-763-survival-chest-persistence-2026-05-29.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture opening chest `8,64,0`, storing one `Dirt` item in slot `0`, closing, reconnecting once, reopening, and observing the same slot/item/count. `tools/check_survival_chest_persistence.rs` rejects Valence-only evidence and mismatched slot/item/count/position/session metrics, and passes the paired Paper/Valence bundle. Full survival compatibility, all-container behavior, restart/world persistence, and broad vanilla parity remain non-claims.

## Survival crafting table checkpoint

The maintained crafting row is validated by `docs/evidence/protocol-763-survival-crafting-table-2026-05-31.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture opening crafting table `4,64,0`, placing `OakPlanks` in input slots `1` and `4`, observing `Stick x4` result slot `0` with recipe `minecraft:stick`, collecting the result, and observing inventory slot `36` on both backends. `tools/check_survival_crafting_table.rs` rejects Valence-only evidence and mismatched slot/item/count/recipe/position metrics, and passes the paired Paper/Valence bundle. Full survival compatibility, full crafting coverage, recipe-book behavior, shift-click/drag/split semantics, all recipe variants, all-container behavior, and broad vanilla parity remain non-claims.

## Survival furnace persistence checkpoint

The maintained furnace persistence row is validated by `docs/evidence/survival-furnace-persistence-receipts-2026-06-01.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture opening furnace `12,64,0`, inserting `RawIron` and `Coal`, observing burn progress and `IronIngot` output, collecting the output into inventory slot `36`, reconnecting once, reopening the same furnace, and observing `collected=true;session_persistent=true` on both backends. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence furnace metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, restart/world persistence, and broad vanilla parity remain non-claims.

## Survival hunger/food checkpoint

The maintained hunger/food row is validated by `docs/evidence/survival-hunger-food-receipts-2026-06-02.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture starting at health `20.0`, food `15`, saturation `0.0`, consuming one Bread item from inventory slot `36`, observing food `20`, saturation `6.0`, and observing inventory decrement from Bread x1 to empty on both backends. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence hunger/food metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, all foods, exhaustion, regeneration/starvation, potion effects, offhand consumption, broad hunger mechanics, and broad vanilla parity remain non-claims.

## Survival mob-drop checkpoint

The maintained mob-drop row is validated by `docs/evidence/survival-mob-drop-receipts-2026-06-02.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture observing one `IronGolem` at `16.5,65.0,2.5`, sending one attack, observing server death, one `IronIngot` drop, one pickup by `compatbot`, and an inventory increment to slot `36` on both backends. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence mob-drop metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, broad mob AI, loot-table distribution, all mob classes, pickup races, and broad vanilla parity remain non-claims.

## Survival redstone-toggle checkpoint

The maintained redstone-toggle row is validated by `docs/evidence/survival-redstone-toggle-receipts-2026-06-02.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture toggling Lever `20,64,0`, observing Redstone Lamp `21,64,0` powered on, sending the return input, and observing the same lamp powered off on both backends. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence redstone metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, general redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, and broad vanilla parity remain non-claims.

## Survival biome/dimension join-state checkpoint

The maintained biome/dimension row is validated by `docs/evidence/survival-biome-dimension-receipts-2026-06-01.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture observing protocol-763 join-game environment identifiers and matching server fixture state: `spawn_environment=minecraft:overworld`, `environment_identifier=minecraft:overworld`, `client_environment_update=minecraft:overworld`, `server_environment_state=minecraft:overworld`, and `normalized_identifier=minecraft:overworld`. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence biome/dimension metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, biome lookup semantics, dimension travel, Nether/End behavior, long-term world persistence durability, and broad vanilla parity remain non-claims.

## Survival world-persistence restart checkpoint

The maintained world-persistence row is validated by `docs/evidence/survival-world-persistence-receipts-2026-06-02.md` with paired Paper and Valence receipt/log bundles. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture mutating `Dirt` at `24,64,0`, recording runner-orchestrated graceful shutdown/restart evidence, reconnecting, and observing the same persisted block state on both backends. The Paper observation is loaded from the mounted Paper world directory after restart rather than recreated from an auxiliary marker file. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence world-persistence metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, long-term durability, arbitrary crash consistency, multi-chunk persistence, all containers, all block entities, concurrent saves, backups, broad vanilla parity, and production readiness remain non-claims.

## Survival crash-recovery checkpoint

The maintained crash-recovery row is validated by `docs/evidence/survival-crash-recovery-receipts-2026-06-04.md` with paired Paper and Valence receipt/log bundles, normalized row evidence, revision oracle, and `docs/evidence/survival-crash-recovery-row-parity-2026-06-04.run.log` with `exit_status=0`. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture mutating `Dirt` at `24,64,0`, recording runner-forced stop with isolated storage, restarting the backend, reconnecting, and observing the same post-crash block state on both backends. The Paper fixture flushes the configured mutation before the forced stop to make the bounded precondition reviewable; this is not arbitrary crash-consistency proof. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, and mismatched Paper/Valence crash-recovery metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, long-term durability, arbitrary crash consistency, multi-chunk persistence, all containers, all block entities, concurrent saves, backups, broad vanilla parity, public-server safety, and production readiness remain non-claims.

## Survival sign block-entity persistence checkpoint

The maintained sign block-entity row is validated by `docs/evidence/survival-block-entity-persistence-receipts-2026-06-04.md` with paired Paper and Valence receipt/log bundles, normalized row evidence, and `docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log` with `exit_status=0`. Covered row is one deterministic client in the Paper fixture and Valence `survival_compat` fixture observing one configured `Sign` at `28,64,0` with text payload `MC|Compat|Sign|Persist`, recording runner-orchestrated graceful shutdown/restart evidence, reconnecting, and observing the same sign text after restart on both backends. The Paper observation is loaded from the mounted Paper world directory; the Valence fixture uses isolated marker storage for this bounded payload. `tools/check_survival_row_parity.rs` rejects Valence-only evidence, missing metrics, stale revisions, unknown child revisions, wrong kind/position/text, and mismatched Paper/Valence sign metrics, and passes the paired Paper/Valence evidence. Full survival compatibility, all block-entity parity, arbitrary NBT parity, sign editing UI parity, multi-chunk persistence, broad vanilla parity, public-server safety, and production readiness remain non-claims.

## CTF invalid-action breadth checkpoint

The deterministic fixture-only `opponent-base-return-drop-without-carrier` row is validated by `docs/evidence/protocol-763-ctf-invalid-action-breadth-matrix-2026-06-19.md`, `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.md`, `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.receipt.json`, copied client/server fixture logs, `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.b3`, and `tools/check_ctf_invalid_action_breadth.rs`. Covered row is one fixture contract where RED `compatbot` attempts a BLUE opponent-base return/drop without carrier ownership, records `ctf_invalid_opponent_base_return_drop_contained` and `server_invalid_opponent_base_return_drop_rejected`, keeps flag state `at_base`, keeps scores `0/0`, and records no forbidden state mutation/score/capture transition. The row is not live gameplay evidence and does not change the existing live invalid pickup or invalid return/drop rows. Full CTF correctness, all invalid actions, all flag permutations, live CTF semantics breadth, adversarial security, public-server safety, production readiness, broad Minecraft compatibility, and vanilla/reference parity remain non-claims.

## Vanilla combat parity checkpoint

The bounded `vanilla-combat-reference-parity` row is validated by `docs/evidence/vanilla-combat-reference-paired-receipts-2026-06-02.md`, paired Paper/Valence receipts, normalized KV inputs, and `tools/check_vanilla_combat_reference_parity.rs`; the comparator log is `docs/evidence/vanilla-combat-reference-paired-compare-2026-06-02.run.log` with `exit_status=0`. This promotes only the named Paper-reference interaction (`compatbota` hits `compatbotb` with iron sword/no armor, first hit `20.0 -> 14.0`, `damage_delta=6.0`, `knockback_metric=0.00`).

The bounded `vanilla-combat-armor-reference-parity` row is validated by `docs/evidence/vanilla-combat-armor-reference-paired-receipts-2026-06-03.md`, paired Paper/Valence receipts, normalized KV inputs, and the same Rust comparator; the comparator log is `docs/evidence/vanilla-combat-armor-reference-paired-compare-2026-06-03.run.log` with `exit_status=0`. This promotes only the named Paper-reference armor interaction (`compatbota` hits `compatbotb` with iron sword/diamond chestplate, no enchantments or status effects, first hit `20.0 -> 15.3`, `damage_delta=4.7`, `knockback_metric=0.00`). Exact Mojang vanilla combat parity, broad armor/loadout coverage, enchantment/status-effect behavior, modifier stacking, and broad combat balancing remain non-claims, and Valence-only evidence is still rejected for parity promotion.

## CTF rule ledger checkpoint

CTF rule scope is guarded by `docs/evidence/protocol-763-ctf-rule-ledger-2026-05-27.md`, `tools/check_red_blue_scoring_soak_live_refresh.rs`, `tools/check_ctf_invalid_pickup_ownership.rs`, `tools/check_ctf_invalid_return_drop.rs`, `tools/check_ctf_invalid_action_breadth.rs`, `tools/check_ctf_score_limit_win_condition.rs`, and `tools/check_mc_compat_row_contracts.rs`. Promoted clusters are bounded RED/BLUE scoring soak, flag-carrier death/return, reconnect flag-state, Invalid flag pickup/ownership, Invalid flag return/drop, CTF invalid-action breadth fixture, Score limit / win condition, Simultaneous pickup/capture race, and Spawn/team balance/resource reset. The RED/BLUE scoring soak checkpoint is validated by `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.md`, `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json`, `docs/evidence/protocol-763-red-blue-scoring-soak-red-2026-05-30.receipt.json`, `docs/evidence/protocol-763-red-blue-scoring-soak-blue-2026-05-30.receipt.json`, and `tools/check_red_blue_scoring_soak_live_refresh.rs`; it records `multi-client-load-score`, `blue-flag-score`, `score_red_1`, `score_blue_1`, `server_flag_or_score`, fresh live RED/BLUE scoring soak refresh, and historical exception removed while full CTF correctness remains a non-claim. The invalid flag pickup/ownership checkpoint is validated by `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.receipt.json`, and `tools/check_ctf_invalid_pickup_ownership.rs`; it records `invalid_action=own_flag_pickup_without_ownership_transfer`, `ctf_invalid_pickup_contained`, and `server_invalid_pickup_rejected`. The invalid flag return/drop checkpoint is validated by `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.md`, `docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.receipt.json`, and `tools/check_ctf_invalid_return_drop.rs`; it records `invalid_action=own_base_return_without_carrier`, `ctf_invalid_return_drop_contained`, and `server_invalid_return_drop_rejected`. The invalid-action breadth fixture checkpoint is validated by `docs/evidence/protocol-763-ctf-invalid-action-breadth-2026-06-19.md`, `docs/evidence/protocol-763-ctf-invalid-opponent-base-return-drop-2026-06-19.receipt.json`, and `tools/check_ctf_invalid_action_breadth.rs`; it records row `opponent-base-return-drop-without-carrier`, `ctf_invalid_opponent_base_return_drop_contained`, and `server_invalid_opponent_base_return_drop_rejected` as deterministic fixture-only evidence. The score-limit checkpoint is validated by `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.md`, `docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.receipt.json`, and `tools/check_ctf_score_limit_win_condition.rs`; it records `ctf-score-limit-win-condition`, `ctf_score_limit_win_seen`, `server_score_limit_pre_state`, `server_score_limit_final_capture`, and `server_score_limit_win_condition`; it is only a near-limit capture checkpoint. The simultaneous pickup/capture race checkpoint is validated by `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.md`, `docs/evidence/protocol-763-ctf-simultaneous-pickup-capture-race-2026-06-01.receipt.json`, and `tools/check_mc_compat_row_contracts.rs`; it records `ctf_race_client_count`, `server_ctf_race_accepted_transition`, `server_ctf_race_rejected_transition`, `server_ctf_race_final_state`, accepted client `compatbotb`, rejected duplicate client `compatbota`, final score RED `1` / BLUE `0`, and no double-accept or duplicate score mutation. The spawn/team balance/resource reset checkpoint is validated by `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.md`, `docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json`, and `tools/check_mc_compat_row_contracts.rs`; it records `ctf_spawn_team_reset_client_count`, `server_ctf_spawn_red_assignment`, `server_ctf_spawn_blue_assignment`, `server_ctf_spawn_team_balance`, `server_ctf_spawn_resource_reset`, RED `1` / BLUE `1` team counts, and no imbalance or stale-resource mutation. Full CTF correctness, all CTF concurrency, all race conditions, latency tolerance, adversarial network safety, live invalid-action breadth, all team balancing algorithms, all maps/loadouts/reset triggers, and unpromoted invalid-action breadth remain non-claims.

## Broad protocol coverage checkpoint

Broad coverage is guarded by `docs/evidence/protocol-763-broad-coverage-ledger-2026-05-28.md`, `docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv`, `docs/evidence/protocol-763-broad-parser-fixtures-stevenarella-2026-05-28.md`, `docs/evidence/protocol-763-broad-parser-fixture-oracle-2026-05-28.md`, and `tools/check_protocol_coverage_ledger.py`. The ledger indexes bounded seams plus 175 Valence protocol-763 packet rows, promotes only four high-risk parser-fixture-backed packet rows, splits the command/recipe, chunk/biome, scoreboard/team, movement, and block-entity sign family rows out as bounded matrix seams, and blocks full protocol-763 compatibility/full Minecraft compatibility until packet-family mapping/parser fixtures and live receipts exist for every claimed row.

## Death/respawn lifecycle checkpoint

The maintained `Flag-carrier death/return` row is also validated as a bounded lifecycle row in `docs/evidence/protocol-763-death-respawn-lifecycle-2026-05-27.md`. The row covers one flag-carrier death, respawn request, restored health, server flag return/reset, and no unexpected score/capture. No all death/drop/recovery permutations are claimed; full death/respawn lifecycle remains a non-claim.

## Representative current-head live refresh

Most maintained matrix rows above stay historical so their BLAKE3-backed receipt hashes do not move silently. The RED/BLUE scoring soak row now uses fresh copied live receipts instead of the historical target-only exception. A fresh representative current-head live run was also added for projectile row freshness:

- Seam: projectile hit rail.
- Maintained command: `VALENCE_REV=HEAD VALENCE_WORKTREE=/tmp/valence-compat-current-head-projectile-refresh VALENCE_TARGET_DIR=/tmp/valence-compat-current-head-projectile-refresh-target CLIENT_TIMEOUT=300 MC_COMPAT_PROJECTILE_HIT_RECEIPT=target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json nix run --no-update-lock-file .#mc-compat-valence-ctf-projectile-hit -- --run`.
- Source receipt: `target/mc-compat-current-head-live-refresh/projectile-hit-current-head.json`.
- Reviewable receipt copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.receipt.json`.
- Reviewable run log copy: `docs/evidence/protocol-763-current-head-projectile-hit-2026-05-27.run.log`.
- BLAKE3: `756b6f732e71ae370808b2a653d1310baa88875f2c3345a1c87444fcffb51c6c`.
- Run log BLAKE3: `05429930472e764a6a2b140ce9c0a7652552659210b4bb1407d93d0d2cd7fada`.
- Payload commits at run time: parent `a2dddea`, Valence `e5d18ad`, Stevenarella `616ee72`.
- Receipt outcome: `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, no missing client/server milestones, `triage.suggested_boundary=none`.
- Scoped non-claims remain: no full projectile physics, projectile travel/collision simulation, all-weapons, enchantment/status-effect, production-load, broad protocol, or full CTF/combat correctness claim.

## Pinned projectile damage attribution refresh

ROI 10 re-promotes projectile damage attribution with pinned dependency and causal receipt proof:

- Maintained command: `nix run .#mc-compat-valence-ctf-projectile-damage-attribution`.
- Source receipt: `target/roi-10-live/projectile-damage-pinned-live.receipt.json`.
- Reviewable receipt copy: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json`.
- Reviewable run log copy: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.run.log`.
- Reviewable Valence/client logs: `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.valence.log`, `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbota.log`, `docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbotb.log`.
- BLAKE3: `cf84fcb81ae557ecfbd2ff0b1f8b94af7bf07eaa85c20b1cde442929e3e3e529`.
- Payload commits at run time: parent `4d29f46` plus current runner diff, Valence `e5d18ad`, Stevenarella `616ee72`.
- Receipt outcome: `status=pass`, `mode=run`, `dry_run=false`, `scenario.passed=true`, `server.passed=true`, `projectile_damage_causality.passed=true`, missing steps `[]`, order violations `[]`.
- Scoped non-claims remain: no full projectile physics, projectile travel/collision simulation, all-weapons, exact vanilla damage parity, enchantment/status-effect, production-load, broad protocol, or full CTF/combat correctness claim.

## Current maintained checks

```sh
./target/check-acceptance-matrix
./target/check-current-evidence-bundle
./tools/check_adversarial_network_oracle.rs --self-test
./tools/check_adversarial_network_oracle.rs --record docs/evidence/protocol-763-adversarial-network-oracle-fixture-2026-05-29.record
./tools/check_wan_tolerance_bounded_telemetry.rs --self-test
./tools/check_wan_tolerance_bounded_telemetry.rs --record docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.record
./tools/check_public_server_authorized_safety.rs --self-test
./tools/check_public_server_authorized_safety.rs --record docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.record
./tools/check_ctf_invalid_pickup_ownership.rs --self-test
./tools/check_ctf_invalid_pickup_ownership.rs --record docs/evidence/protocol-763-ctf-invalid-pickup-ownership-2026-05-30.record
./tools/check_ctf_invalid_return_drop.rs --self-test
./tools/check_ctf_invalid_return_drop.rs --record docs/evidence/protocol-763-ctf-invalid-return-drop-2026-05-30.record
./tools/check_ctf_score_limit_win_condition.rs --self-test
./tools/check_ctf_score_limit_win_condition.rs --record docs/evidence/protocol-763-ctf-score-limit-win-condition-2026-05-30.record
./tools/check_red_blue_scoring_soak_live_refresh.rs --self-test
./tools/check_red_blue_scoring_soak_live_refresh.rs --record docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.record
./tools/check_equipment_slot_item_expansion.rs --self-test
./tools/check_equipment_slot_item_expansion.rs --record docs/evidence/protocol-763-equipment-slot-item-expansion-row-2026-05-29.record
./tools/check_armor_loadout_enchantment_status.rs --self-test
./tools/check_armor_loadout_enchantment_status.rs --record docs/evidence/protocol-763-armor-loadout-enchantment-status-row-2026-05-29.record
nix develop --no-update-lock-file -c rustc --edition=2021 tools/check_survival_coverage_matrix.rs -o target/check-survival-coverage-matrix
target/check-survival-coverage-matrix --self-test
target/check-survival-coverage-matrix
./tools/check_survival_chest_persistence.rs --self-test
nix develop --no-update-lock-file -c rustc --edition=2021 tools/check_evidence_manifests.rs -o target/check-evidence-manifests
nix develop --no-update-lock-file -c target/check-evidence-manifests --self-test
nix develop --no-update-lock-file -c target/check-evidence-manifests
nix build --no-update-lock-file .#checks.x86_64-linux.mc-compat-adversarial-network-oracle --no-link -L
nix run --no-update-lock-file .#cairn -- validate --root .
```

## Evidence freshness promotion gate

Before adding or replacing a maintained evidence row, run the freshness gate from `/home/brittonr/git/mc` and copy the output under `docs/evidence/`:

```sh
nix develop --no-update-lock-file -c rustc --edition=2021 tools/check_acceptance_matrix.rs -o target/check-acceptance-matrix
target/check-acceptance-matrix --self-test
target/check-acceptance-matrix
nix develop --no-update-lock-file -c rustc --edition=2021 tools/check_current_evidence_bundle.rs -o target/check-current-evidence-bundle
target/check-current-evidence-bundle --self-test
target/check-current-evidence-bundle
nix develop --no-update-lock-file -c rustc --edition=2021 tools/check_evidence_manifests.rs -o target/check-evidence-manifests
nix develop --no-update-lock-file -c target/check-evidence-manifests --self-test
nix develop --no-update-lock-file -c target/check-evidence-manifests
nix run --no-update-lock-file .#cairn -- validate --root .
```

Rows that cite live receipts should point at tracked `docs/evidence/*.receipt.json` copies and BLAKE3 manifests. Historical `target/` rows require an explicit evidence/oracle note that records the original digest, inspected evidence, decision, owner, and next action. The RED/BLUE scoring soak historical oracle remains review history only; its row now points at `docs/evidence/protocol-763-red-blue-scoring-soak-live-refresh-2026-05-30.receipt.json`.

Reviewable copied receipts for matrix rows are indexed at `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.md` with manifest `docs/evidence/protocol-763-matrix-reviewable-receipts-2026-05-27.b3`.

## Production/load/network safety envelope

The runner receipt surface includes a `load_network_safety` block that records owned-local or explicit authorization, client/duration/reconnect/network bounds, telemetry readiness, and fail-closed diagnostics. Evidence: `docs/evidence/protocol-763-load-network-safety-2026-05-27.md` and `docs/evidence/protocol-763-production-network-safety-matrix-2026-05-28.md`.

The production/network matrix promotes bounded owned-local loopback load safety, one deterministic public-server authorized fixture, one bounded owned-local WAN telemetry row, and one deterministic fixture-only adversarial-network oracle row. The public-server fixture row is backed by `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.md`, receipt `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.receipt.json`, BLAKE3 manifest `docs/evidence/protocol-763-public-server-authorized-safety-2026-05-30.b3`, checkpoint `docs/evidence/protocol-763-public-server-authorized-safety-checkpoint-2026-05-30.md`, and checker `tools/check_public_server_authorized_safety.rs`. Broader production readiness, live public-server safety, third-party target safety without authorization, live adversarial-network safety, packet-loss tolerance, unbounded adversarial robustness, and unbounded safety remain non-claims unless a future authorized live envelope has telemetry, BLAKE3-backed evidence, and an updated matrix/bundle row.

## Reference parity labels

- `reference-parity-covered`: Survival break/place/pickup, chest persistence, crafting table, furnace persistence, hunger/food, mob drops, redstone toggle, biome/dimension join state, bounded world-persistence restart, bounded crash recovery, bounded sign block-entity persistence, bounded `vanilla-combat-reference-parity`, and bounded `vanilla-combat-armor-reference-parity` Paper-reference rows only.
- `valence-only-containment`: CTF scoring, inventory, non-reference combat, projectile, reconnect, latency/jitter, MCP-controlled observability, and load/network rows.
- `fixture-only-oracle`: The adversarial-network oracle row only; no live traffic or broad security claim.
- `explicit-non-claim`: exact Mojang vanilla combat parity, broad survival, full Minecraft/CTF/protocol correctness, and rows not named as reference-parity covered.

Policy/checkpoint: `docs/evidence/protocol-763-reference-parity-expansion-2026-05-29.md`.

## Non-claims

This bundle still does not claim full Minecraft compatibility, full survival compatibility, broad vanilla parity, full CTF correctness, visual regression approval, semantic gameplay equivalence from screenshots, broad MCP API coverage, all commands, all recipes, all advancements, broad recipe-book UI semantics, command execution semantics, semantic raw command/recipe decoding, all worldgen packets, all chunks, all biomes, live `ChunkBiomeDataS2CPacket` parity, chunk-delta semantics, scoreboard UI parity, all scoreboards, all team rules, objective/display/score variants, movement physics, collision, anti-cheat, latency tolerance, malicious-client movement resilience, all movement packet variants, projectile travel/collision simulation, all projectile weapon variants, all equipment slots/items, all equipment transition/order permutations, broad equipment parser-shape coverage, all inventory semantics, all inventory windows, all click modes, stack split/merge beyond the configured row, drag transactions beyond the configured row, broad creative inventory semantics, broad inventory parser-shape coverage, all entity metadata, all metadata value types, malformed metadata breadth, mob AI semantics, broad entity metadata parser-shape coverage, all armor loadouts, enchantment/status-effect gameplay semantics, exact vanilla knockback/damage/mitigation balancing, all-container behavior, long-term world persistence durability, arbitrary crash consistency, multi-chunk persistence, all block entities, arbitrary block-entity NBT, sign editing UI parity, concurrent saves, backups, full crafting coverage, all smelting recipes, long-running furnace timing parity, hopper automation, furnace minecarts, broad redstone circuit parity, tick-order parity, pistons, observers, comparators, clocks, farms, biome lookup semantics, dimension travel, Nether/End behavior, production readiness, public-server load safety, live adversarial-network safety, malicious-client resilience, hostile internet safety, full protocol security, or unbounded soak/reconnect/latency safety.
