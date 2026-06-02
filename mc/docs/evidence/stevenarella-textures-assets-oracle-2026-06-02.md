# Stevenarella textures/assets oracle — 2026-06-02

## Question
How should Stevenarella load textures/assets for protocol 763 / Minecraft 1.20.1 without regressing legacy internal callers?

## Inspected evidence
- `stevenarella/src/resources.rs` previously pinned `RESOURCES_VERSION = "1.12.2"`, a 1.12 asset index, and `http://resources.download.minecraft.net/{hash_path}` object downloads while loading from relative `./resources-*`, `./index`, and `./objects` paths.
- `stevenarella/src/render/mod.rs` previously downloaded skins from `http://textures.minecraft.net/texture/{hash}` and sliced skin hashes using the literal HTTP prefix.
- `stevenarella/src/model/mod.rs` parsed blockstate model refs as `models/block/{model}.json`, which does not resolve modern namespaced refs like `minecraft:block/oak_planks` from 1.20 blockstate JSON.
- `stevenarella/blocks/src/lib.rs` mapped the solid grass block to model name `grass`; in 1.20.1 assets, `grass.json` is the plant/cross model while the solid block is `grass_block.json`.

## Decision
Use the 1.20.1 client jar and asset index as the native asset source, require HTTPS/status-checked downloads, accept both HTTP and HTTPS skin URLs for incoming server metadata, resolve namespaced model parents/blockstate refs, and add compatibility aliases for legacy `textures/blocks/*`, `textures/items/*`, and `textures/entity/steve.png` callers.

## Owner
Agent / Stevenarella compatibility work.

## Verification
- Baseline before asset/model changes: `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test --lib resource_reference -- --nocapture` from `stevenarella/` passed with 0 matching tests and compiled the library.
- Post-change: `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test --lib model_references -- --nocapture` passed 4 focused model-reference tests.
- Post-change: `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test --lib -- --nocapture` passed 125 library tests.
- Post-change: `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo test grass_block_model -- --nocapture` from `stevenarella/blocks/` passed the grass model regression.
- Post-change: `nix develop --no-update-lock-file /home/brittonr/git/mc -c cargo fmt --all -- --check` from `stevenarella/` passed.

## Next action
If live screenshots still show missing textures, capture the missing resource names from `TextureManager::load_texture`/model parse logs and add narrow aliases or block model renames backed by the same oracle pattern.
