# Typed contract for the selected-row furnace smelting fixture.
# This is local unit fixture metadata only; it is not Paper/vanilla parity evidence.

let TargetEdition = std.contract.from_predicate (fun value =>
  std.is_string value && value == "Java Edition"
) in

let TargetGameVersion = std.contract.from_predicate (fun value =>
  std.is_string value && value == "1.20.1"
) in

let TargetProtocol = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value == 763
) in

let NonEmptyString = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0
) in

let NonEmptyArray = std.contract.from_predicate (fun value =>
  std.is_array value && std.array.length value > 0
) in

let PositiveInteger = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value > 0
) in

let MinecraftItemId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^minecraft:[a-z0-9_/]+$" value
) in

let StandardFurnaceKind = std.contract.from_predicate (fun value =>
  std.is_string value && value == "standard"
) in

let FalseClaim = std.contract.from_predicate (fun value =>
  std.is_bool value && value == false
) in

let TargetScope = {
  edition | TargetEdition,
  game_version | TargetGameVersion,
  protocol | TargetProtocol,
} in

let Source = {
  name | NonEmptyString,
  url | NonEmptyString,
  retrieval_date | NonEmptyString,
  evidence_role | NonEmptyString,
} in

let Provenance = {
  data_origin | NonEmptyString,
  extraction_status | NonEmptyString,
  sources | NonEmptyArray | Array Source,
  predecessor_docs | NonEmptyArray | Array NonEmptyString,
} in

let Constants = {
  standard_furnace_cook_ticks | PositiveInteger,
  coal_burn_ticks | PositiveInteger,
  max_stack_size | PositiveInteger,
  selected_recipe_output_count | PositiveInteger,
} in

let RecipeRow = {
  fixture_role | NonEmptyString,
  recipe_id | NonEmptyString,
  furnace_kind | StandardFurnaceKind,
  input_item | MinecraftItemId,
  output_item | MinecraftItemId,
  output_count | PositiveInteger,
  cook_ticks | PositiveInteger,
} in

let FuelRow = {
  fixture_role | NonEmptyString,
  fuel_item | MinecraftItemId,
  burn_ticks | PositiveInteger,
} in

let ClaimBoundaries = {
  claims_paper_vanilla_parity | FalseClaim,
  claims_all_recipe_breadth | FalseClaim,
  claims_valence_runtime_integration | FalseClaim,
  claims_default_plugin_membership | FalseClaim,
  claims_public_server_safety | FalseClaim,
  claims_production_readiness | FalseClaim,
} in

{
  Fixture = {
    schema | NonEmptyString,
    target | TargetScope,
    provenance | Provenance,
    constants | Constants,
    recipes | NonEmptyArray | Array RecipeRow,
    fuels | NonEmptyArray | Array FuelRow,
    non_claims | NonEmptyArray | Array NonEmptyString,
    claim_boundaries | ClaimBoundaries,
  },
}
