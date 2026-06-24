{ self, pkgs }:
{
  valence = {
    type = "app";
    program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.valence}/bin/valence";
    meta.description = "Run the core Valence server tree through the mc flake dev environment.";
  };
  stevenarella = {
    type = "app";
    program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.stevenarella}/bin/stevenarella";
    meta.description = "Run the core Stevenarella client tree through the mc flake dev environment.";
  };
  mc-compat-smoke = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner";
    meta.description = "Run the hardened Stevenarella/Valence compatibility smoke.";
  };
  evidence-manifest-refresh = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.evidence-manifest-refresh
    }/bin/evidence-manifest-refresh";
    meta.description = "Check or refresh docs/evidence BLAKE3 manifests to a deterministic fixpoint.";
  };
  mc-compat-valence-ctf-600s-soak = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-600s-soak
    }/bin/mc-compat-valence-ctf-600s-soak";
    meta.description = "Run the maintained protocol-763 Valence CTF 600s multi-client soak receipt.";
  };
  mc-compat-valence-ctf-blue-600s-soak = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-blue-600s-soak
    }/bin/mc-compat-valence-ctf-blue-600s-soak";
    meta.description = "Run the maintained protocol-763 Valence CTF BLUE-team 600s soak receipt.";
  };
  mc-compat-valence-ctf-inventory-interaction = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-inventory-interaction
    }/bin/mc-compat-valence-ctf-inventory-interaction";
    meta.description = "Run the maintained protocol-763 Valence CTF inventory/drop interaction receipt.";
  };
  mc-compat-valence-inventory-stack-split-merge = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-stack-split-merge
    }/bin/mc-compat-valence-inventory-stack-split-merge";
    meta.description = "Run the maintained protocol-763 Valence CTF inventory stack split/merge receipt.";
  };
  mc-compat-valence-inventory-drag-transactions = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-drag-transactions
    }/bin/mc-compat-valence-inventory-drag-transactions";
    meta.description = "Run the maintained protocol-763 Valence CTF inventory drag transaction receipt.";
  };
  mc-compat-valence-ctf-combat-damage = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-damage
    }/bin/mc-compat-valence-ctf-combat-damage";
    meta.description = "Run the maintained protocol-763 Valence CTF combat damage receipt.";
  };
  mc-compat-valence-ctf-combat-knockback = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-knockback
    }/bin/mc-compat-valence-ctf-combat-knockback";
    meta.description = "Run the maintained protocol-763 Valence CTF combat knockback receipt.";
  };
  mc-compat-valence-ctf-armor-equipment-mitigation = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-armor-equipment-mitigation
    }/bin/mc-compat-valence-ctf-armor-equipment-mitigation";
    meta.description = "Run the maintained protocol-763 Valence CTF armor/equipment mitigation receipt.";
  };
  mc-compat-valence-ctf-equipment-update-observation = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-equipment-update-observation
    }/bin/mc-compat-valence-ctf-equipment-update-observation";
    meta.description = "Run the maintained protocol-763 Valence CTF entity equipment update observation receipt.";
  };
  mc-compat-valence-ctf-projectile-hit = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-hit
    }/bin/mc-compat-valence-ctf-projectile-hit";
    meta.description = "Run the maintained protocol-763 Valence CTF projectile hit receipt.";
  };
  mc-compat-valence-ctf-projectile-damage-attribution = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-damage-attribution
    }/bin/mc-compat-valence-ctf-projectile-damage-attribution";
    meta.description = "Run the maintained protocol-763 Valence CTF projectile damage attribution receipt.";
  };
  mc-compat-combo = self.apps.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-hit;
  mc-compat-valence-ctf-flag-carrier-death-return = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-flag-carrier-death-return
    }/bin/mc-compat-valence-ctf-flag-carrier-death-return";
    meta.description = "Run the maintained protocol-763 Valence CTF flag-carrier death/return receipt.";
  };
  mc-compat-valence-ctf-latency-jitter-inventory = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-latency-jitter-inventory
    }/bin/mc-compat-valence-ctf-latency-jitter-inventory";
    meta.description = "Run the maintained protocol-763 Valence CTF inventory rail with bounded latency/jitter metadata.";
  };
  mc-compat-valence-ctf-reconnect-flag-state = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-reconnect-flag-state
    }/bin/mc-compat-valence-ctf-reconnect-flag-state";
    meta.description = "Run the maintained protocol-763 Valence CTF reconnect flag-state receipt.";
  };
  mc-compat-valence-ctf-invalid-pickup-ownership = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-pickup-ownership
    }/bin/mc-compat-valence-ctf-invalid-pickup-ownership";
    meta.description = "Run the maintained protocol-763 Valence CTF invalid pickup ownership receipt.";
  };
  mc-compat-valence-ctf-invalid-return-drop = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-return-drop
    }/bin/mc-compat-valence-ctf-invalid-return-drop";
    meta.description = "Run the maintained protocol-763 Valence CTF invalid return/drop receipt.";
  };
  mc-compat-valence-ctf-invalid-opponent-base-return-drop = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-opponent-base-return-drop
    }/bin/mc-compat-valence-ctf-invalid-opponent-base-return-drop";
    meta.description = "Run the maintained protocol-763 Valence CTF invalid opponent-base return/drop receipt.";
  };
  mc-compat-valence-ctf-score-limit-win-condition = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-score-limit-win-condition
    }/bin/mc-compat-valence-ctf-score-limit-win-condition";
    meta.description = "Run the maintained protocol-763 Valence CTF score limit win-condition receipt.";
  };
  mc-compat-valence-survival-break-place-pickup = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-break-place-pickup
    }/bin/mc-compat-valence-survival-break-place-pickup";
    meta.description = "Run the maintained protocol-763 Valence survival break/place/pickup receipt.";
  };
  mc-compat-valence-survival-crafting-table = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-table
    }/bin/mc-compat-valence-survival-crafting-table";
    meta.description = "Run the maintained protocol-763 Valence survival crafting-table receipt.";
  };
  mc-compat-valence-survival-crafting-recipe-breadth = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-recipe-breadth
    }/bin/mc-compat-valence-survival-crafting-recipe-breadth";
    meta.description = "Run the maintained protocol-763 Valence survival crafting recipe breadth receipt.";
  };
  mc-compat-valence-survival-furnace-persistence = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-persistence
    }/bin/mc-compat-valence-survival-furnace-persistence";
    meta.description = "Run the maintained protocol-763 Valence survival furnace persistence receipt.";
  };
  mc-compat-valence-survival-furnace-smelting-breadth = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-smelting-breadth
    }/bin/mc-compat-valence-survival-furnace-smelting-breadth";
    meta.description = "Run the maintained protocol-763 Valence survival furnace smelting breadth receipt.";
  };
  mc-compat-valence-survival-hunger-health-cycle = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-hunger-health-cycle
    }/bin/mc-compat-valence-survival-hunger-health-cycle";
    meta.description = "Run the maintained protocol-763 Valence survival hunger-health cycle receipt.";
  };
  mc-compat-mcp-controlled-smoke = {
    type = "app";
    program = "${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-mcp-controlled-smoke
    }/bin/mc-compat-mcp-controlled-smoke";
    meta.description = "Run the deterministic MCP-controlled Stevenarella receipt dry-run.";
  };
  default = self.apps.${pkgs.stdenv.hostPlatform.system}.mc-compat-combo;
}
