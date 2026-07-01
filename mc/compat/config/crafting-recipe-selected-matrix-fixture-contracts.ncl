# Typed contract for the selected-matrix crafting recipe fixture.
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

let NonNegativeInteger = std.contract.from_predicate (fun value =>
  std.is_number value && std.number.is_integer value && value >= 0
) in

let MinecraftItemId = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^minecraft:[a-z0-9_/]+$" value
) in

let PatternSymbol = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value == 1
) in

let PatternRow = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value == 3
) in

let ShapedKind = std.contract.from_predicate (fun value =>
  std.is_string value && value == "shaped"
) in

let ShapelessKind = std.contract.from_predicate (fun value =>
  std.is_string value && value == "shapeless"
) in

let RejectedNoResultKind = std.contract.from_predicate (fun value =>
  std.is_string value && value == "rejected_no_result"
) in

let PrimaryClickMode = std.contract.from_predicate (fun value =>
  std.is_string value && value == "primary_click"
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
  grid_width | PositiveInteger,
  grid_height | PositiveInteger,
  max_stack_size | PositiveInteger,
  result_slot_stack_limit | PositiveInteger,
  selected_chest_output_count | PositiveInteger,
  selected_oak_planks_output_count | PositiveInteger,
  selected_chest_target_slot | NonNegativeInteger,
  selected_oak_planks_target_slot | NonNegativeInteger,
} in

let KeyIngredient = {
  symbol | PatternSymbol,
  item | MinecraftItemId,
  count | PositiveInteger,
} in

let ItemIngredient = {
  item | MinecraftItemId,
  count | PositiveInteger,
} in

let ShapedRecipeRow = {
  fixture_role | NonEmptyString,
  recipe_id | NonEmptyString,
  recipe_kind | ShapedKind,
  pattern | NonEmptyArray | Array PatternRow,
  key | NonEmptyArray | Array KeyIngredient,
  output_item | MinecraftItemId,
  output_count | PositiveInteger,
  collection_mode | PrimaryClickMode,
  target_slot | NonNegativeInteger,
} in

let ShapelessRecipeRow = {
  fixture_role | NonEmptyString,
  recipe_id | NonEmptyString,
  recipe_kind | ShapelessKind,
  ingredients | NonEmptyArray | Array ItemIngredient,
  output_item | MinecraftItemId,
  output_count | PositiveInteger,
  collection_mode | PrimaryClickMode,
  target_slot | NonNegativeInteger,
} in

let InvalidProbe = {
  fixture_role | NonEmptyString,
  probe_id | NonEmptyString,
  probe_kind | RejectedNoResultKind,
  input_item | MinecraftItemId,
  input_count | PositiveInteger,
  expected_diagnostic | NonEmptyString,
} in

let ClaimBoundaries = {
  claims_paper_vanilla_parity | FalseClaim,
  claims_all_recipe_breadth | FalseClaim,
  claims_arbitrary_collection_modes | FalseClaim,
  claims_shift_click_drag_split | FalseClaim,
  claims_data_pack_loading | FalseClaim,
  claims_recipe_book_ui | FalseClaim,
  claims_automated_crafter | FalseClaim,
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
    shaped_recipes | NonEmptyArray | Array ShapedRecipeRow,
    shapeless_recipes | NonEmptyArray | Array ShapelessRecipeRow,
    invalid_probes | NonEmptyArray | Array InvalidProbe,
    non_claims | NonEmptyArray | Array NonEmptyString,
    claim_boundaries | ClaimBoundaries,
  },
}
