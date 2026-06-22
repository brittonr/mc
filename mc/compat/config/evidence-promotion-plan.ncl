# Typed plan shape for reviewable evidence promotion.
# The Rust promotion tool computes values and writes exact destination artifacts; this Nickel file defines the expected plan contract.

let Artifact = {
  class | String,
  source | String,
  destination | String,
  blake3 | String,
} in
let ChildRevision = {
  field | String,
  value | String,
} in
let MatrixBundlePlan = {
  row | String,
  action | String,
  preserves_non_claims | Bool,
} in
let PromotionPlan = {
  schema | String,
  rail | String,
  artifacts | Array Artifact,
  child_revisions | Array ChildRevision,
  matrix | MatrixBundlePlan,
  current_bundle | MatrixBundlePlan,
  required_non_claims | Array String,
  validation_commands | Array String,
} in
{
  schema = "mc.compat.evidence-promotion-plan.v1",
  rail = "projectile-damage-attribution",
  artifacts = [],
  child_revisions = [],
  matrix = { row = "", action = "plan-only", preserves_non_claims = true },
  current_bundle = { row = "", action = "plan-only", preserves_non_claims = true },
  required_non_claims = ["full Minecraft compatibility", "full CTF/combat correctness"],
  validation_commands = [
    "nix build .#checks.x86_64-linux.mc-compat-acceptance-matrix --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-evidence-manifests --no-link -L",
    "nix run --no-update-lock-file .#cairn -- validate --root .",
  ],
} | PromotionPlan
