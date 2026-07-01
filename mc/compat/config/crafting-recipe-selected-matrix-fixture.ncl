# Selected-matrix crafting recipe fixture for local unit validation.
# This file is intentionally narrow: one shaped row, one shapeless row, and one no-result probe.

let Contracts = import "crafting-recipe-selected-matrix-fixture-contracts.ncl" in
let target_edition = "Java Edition" in
let target_game_version = "1.20.1" in
let target_protocol = 763 in
let selected_grid_width = 3 in
let selected_grid_height = 3 in
let selected_max_stack_size = 64 in
let selected_result_slot_stack_limit = 64 in
let selected_chest_output_count_value = 1 in
let selected_oak_planks_output_count_value = 4 in
let selected_chest_target_slot_value = 36 in
let selected_oak_planks_target_slot_value = 37 in
let selected_shaped_recipe_kind = "shaped" in
let selected_shapeless_recipe_kind = "shapeless" in
let selected_rejected_no_result_kind = "rejected_no_result" in
let selected_collection_mode = "primary_click" in
let selected_chest_recipe_id = "minecraft:chest" in
let selected_chest_key_symbol = "P" in
let selected_chest_pattern_top = "PPP" in
let selected_chest_pattern_middle = "P P" in
let selected_chest_pattern_bottom = "PPP" in
let selected_chest_key_item = "minecraft:oak_planks" in
let selected_chest_key_count = 1 in
let selected_chest_output_item = "minecraft:chest" in
let selected_oak_planks_recipe_id = "minecraft:oak_planks" in
let selected_shapeless_input_item = "minecraft:oak_log" in
let selected_shapeless_input_count = 1 in
let selected_shapeless_output_item = "minecraft:oak_planks" in
let selected_invalid_probe_id = "minecraft:stick_insufficient_input_rejection" in
let selected_invalid_probe_input_item = "minecraft:oak_planks" in
let selected_invalid_probe_input_count = 1 in
let selected_invalid_probe_diagnostic = "no_result" in
{
  schema = "mc.compat.crafting-recipe.selected-matrix.fixture.v1",
  target = {
    edition = target_edition,
    game_version = target_game_version,
    protocol = target_protocol,
  },
  provenance = {
    data_origin = "selected Java Edition 1.20.1 crafting recipe row fixture for local unit validation",
    extraction_status = "selected-matrix-contract-fixture; not all-recipe extraction",
    sources = [
      {
        name = "Minecraft Wiki: Crafting",
        url = "https://minecraft.wiki/w/Crafting",
        retrieval_date = "2026-07-01",
        evidence_role = "vocabulary and version-drift guide only",
      },
      {
        name = "Minecraft Wiki: Recipe (Java Edition)",
        url = "https://minecraft.wiki/w/Recipe_(Java_Edition)",
        retrieval_date = "2026-07-01",
        evidence_role = "recipe-data vocabulary guide only",
      },
      {
        name = "Minecraft Wiki: Java Edition 1.20.1",
        url = "https://minecraft.wiki/w/Java_Edition_1.20.1",
        retrieval_date = "2026-07-01",
        evidence_role = "target scope guide only",
      },
    ],
    predecessor_docs = [
      "docs/crafting-recipe-behavior-card.md",
      "docs/crafting-recipe-selected-matrix-core.md",
    ],
  },
  constants = {
    grid_width = selected_grid_width,
    grid_height = selected_grid_height,
    max_stack_size = selected_max_stack_size,
    result_slot_stack_limit = selected_result_slot_stack_limit,
    selected_chest_output_count = selected_chest_output_count_value,
    selected_oak_planks_output_count = selected_oak_planks_output_count_value,
    selected_chest_target_slot = selected_chest_target_slot_value,
    selected_oak_planks_target_slot = selected_oak_planks_target_slot_value,
  },
  shaped_recipes = [
    {
      fixture_role = "selected_shaped_recipe",
      recipe_id = selected_chest_recipe_id,
      recipe_kind = selected_shaped_recipe_kind,
      pattern = [
        selected_chest_pattern_top,
        selected_chest_pattern_middle,
        selected_chest_pattern_bottom,
      ],
      key = [
        {
          symbol = selected_chest_key_symbol,
          item = selected_chest_key_item,
          count = selected_chest_key_count,
        },
      ],
      output_item = selected_chest_output_item,
      output_count = selected_chest_output_count_value,
      collection_mode = selected_collection_mode,
      target_slot = selected_chest_target_slot_value,
    },
  ],
  shapeless_recipes = [
    {
      fixture_role = "selected_shapeless_recipe",
      recipe_id = selected_oak_planks_recipe_id,
      recipe_kind = selected_shapeless_recipe_kind,
      ingredients = [
        {
          item = selected_shapeless_input_item,
          count = selected_shapeless_input_count,
        },
      ],
      output_item = selected_shapeless_output_item,
      output_count = selected_oak_planks_output_count_value,
      collection_mode = selected_collection_mode,
      target_slot = selected_oak_planks_target_slot_value,
    },
  ],
  invalid_probes = [
    {
      fixture_role = "selected_invalid_probe",
      probe_id = selected_invalid_probe_id,
      probe_kind = selected_rejected_no_result_kind,
      input_item = selected_invalid_probe_input_item,
      input_count = selected_invalid_probe_input_count,
      expected_diagnostic = selected_invalid_probe_diagnostic,
    },
  ],
  non_claims = [
    "no broad Minecraft compatibility",
    "no broad vanilla parity",
    "no Paper/vanilla parity",
    "no all-recipe breadth",
    "no arbitrary collection modes",
    "no shift-click/drag/split handling",
    "no data-pack loading",
    "no recipe-book UI behavior",
    "no recipe discovery, advancement, or doLimitedCrafting behavior",
    "no automated crafter behavior",
    "no Valence runtime integration",
    "no DefaultPlugins membership changes",
    "no public-server safety",
    "no production readiness",
  ],
  claim_boundaries = {
    claims_paper_vanilla_parity = false,
    claims_all_recipe_breadth = false,
    claims_arbitrary_collection_modes = false,
    claims_shift_click_drag_split = false,
    claims_data_pack_loading = false,
    claims_recipe_book_ui = false,
    claims_automated_crafter = false,
    claims_valence_runtime_integration = false,
    claims_default_plugin_membership = false,
    claims_public_server_safety = false,
    claims_production_readiness = false,
  },
} | Contracts.Fixture
