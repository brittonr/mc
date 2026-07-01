# Selected-row furnace smelting fixture for local unit validation.
# This file is intentionally narrow: one standard-furnace recipe row and one fuel row.

let Contracts = import "furnace-smelting-selected-row-fixture-contracts.ncl" in
let target_edition = "Java Edition" in
let target_game_version = "1.20.1" in
let target_protocol = 763 in
let standard_furnace_kind = "standard" in
let selected_standard_furnace_cook_ticks = 200 in
let selected_coal_burn_ticks = 1600 in
let selected_max_stack_size = 64 in
let selected_recipe_output_count_value = 1 in
let selected_recipe_input_item = "minecraft:raw_iron" in
let selected_recipe_output_item = "minecraft:iron_ingot" in
let selected_fuel_item = "minecraft:coal" in
{
  schema = "mc.compat.furnace-smelting.selected-row.fixture.v1",
  target = {
    edition = target_edition,
    game_version = target_game_version,
    protocol = target_protocol,
  },
  provenance = {
    data_origin = "selected Java Edition 1.20.1 row fixture for local unit validation",
    extraction_status = "selected-row-contract-fixture; not all-recipe extraction",
    sources = [
      {
        name = "Minecraft Wiki: Smelting",
        url = "https://minecraft.wiki/w/Smelting",
        retrieval_date = "2026-07-01",
        evidence_role = "vocabulary and version-drift guide only",
      },
      {
        name = "Minecraft Wiki: Java Edition 1.20.1",
        url = "https://minecraft.wiki/w/Java_Edition_1.20.1",
        retrieval_date = "2026-07-01",
        evidence_role = "target scope guide only",
      },
    ],
    predecessor_docs = [
      "docs/furnace-smelting-behavior-card.md",
      "docs/furnace-smelting-selected-row-core.md",
    ],
  },
  constants = {
    standard_furnace_cook_ticks = selected_standard_furnace_cook_ticks,
    coal_burn_ticks = selected_coal_burn_ticks,
    max_stack_size = selected_max_stack_size,
    selected_recipe_output_count = selected_recipe_output_count_value,
  },
  recipes = [
    {
      fixture_role = "selected_standard_furnace_recipe",
      recipe_id = "minecraft:iron_ingot_from_raw_iron_smelting",
      furnace_kind = standard_furnace_kind,
      input_item = selected_recipe_input_item,
      output_item = selected_recipe_output_item,
      output_count = selected_recipe_output_count_value,
      cook_ticks = selected_standard_furnace_cook_ticks,
    },
  ],
  fuels = [
    {
      fixture_role = "selected_fuel",
      fuel_item = selected_fuel_item,
      burn_ticks = selected_coal_burn_ticks,
    },
  ],
  non_claims = [
    "no broad Minecraft compatibility",
    "no broad vanilla parity",
    "no Paper/vanilla parity",
    "no all-recipe breadth",
    "no smoker behavior",
    "no blast-furnace behavior",
    "no hopper automation",
    "no XP behavior",
    "no recipe-book synchronization",
    "no chunk-unload semantics",
    "no Valence runtime integration",
    "no DefaultPlugins membership changes",
    "no public-server safety",
    "no production readiness",
  ],
  claim_boundaries = {
    claims_paper_vanilla_parity = false,
    claims_all_recipe_breadth = false,
    claims_valence_runtime_integration = false,
    claims_default_plugin_membership = false,
    claims_public_server_safety = false,
    claims_production_readiness = false,
  },
} | Contracts.Fixture
