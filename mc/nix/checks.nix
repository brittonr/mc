{
  self,
  pkgs,
  lib,
  cairn,
  octet,
  pinnedProjectileDamageValenceRev,
  scenarioWrapperMetadata,
  srcRoot,
  baselineOutputInventory,
}:
let
  system = pkgs.stdenv.hostPlatform.system;
  allowedNewChecks = [
    "mc-compat-scenario-contracts"
    "mc-flake-output-inventory"
    "mc-octet-monorepo"
    "mc-valence-schedule-hygiene"
  ];
  baseline = builtins.fromJSON (builtins.readFile baselineOutputInventory);
  current = {
    packages = builtins.attrNames self.packages.${system};
    apps = builtins.attrNames self.apps.${system};
    checks = builtins.attrNames self.checks.${system};
    devShells = builtins.attrNames self.devShells.${system};
  };
  expected = {
    packages = baseline.packages.${system};
    apps = baseline.apps.${system};
    checks = baseline.checks.${system} ++ allowedNewChecks;
    devShells = baseline.devShells.${system};
  };
  missingFrom =
    expectedNames: actualNames: builtins.filter (name: !(builtins.elem name actualNames)) expectedNames;
  unexpectedFrom =
    expectedNames: actualNames: builtins.filter (name: !(builtins.elem name expectedNames)) actualNames;
  diff = builtins.mapAttrs (name: expectedNames: {
    missing = missingFrom expectedNames current.${name};
    unexpected = unexpectedFrom expectedNames current.${name};
  }) expected;
  hasCategoryMismatch = categoryDiff: categoryDiff.missing != [ ] || categoryDiff.unexpected != [ ];
  hasMismatch = builtins.any hasCategoryMismatch (builtins.attrValues diff);
  report = {
    inherit
      system
      allowedNewChecks
      current
      expected
      diff
      ;
  };
  reportJson = builtins.toJSON report;
in
{
  mc-flake-output-inventory =
    if hasMismatch then
      throw "flake output inventory mismatch: ${reportJson}"
    else
      pkgs.runCommand "mc-flake-output-inventory" { } ''
        mkdir -p "$out"
        cat > "$out/inventory-parity.json" <<'REPORT'
        ${reportJson}
        REPORT
      '';
  mc-compat-runner = self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner;
  mc-cairn-policy-fresh = pkgs.runCommand "mc-cairn-policy-fresh" { } ''
    cp -R ${srcRoot} repo
    chmod -R u+w repo
    cd repo
    ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
    }/bin/cairn policy export --check > policy-fresh.log
    grep -Fq "policy fresh: cairn-policy/generated/cairn-policy.json" policy-fresh.log
    mkdir -p "$out"
    cp policy-fresh.log "$out/"
  '';
  mc-cairn-policy-stale-detects-drift = pkgs.runCommand "mc-cairn-policy-stale-detects-drift" { } ''
    cp -R ${srcRoot} repo
    chmod -R u+w repo
    cd repo
    substituteInPlace cairn-policy/default.ncl \
      --replace-fail 'name = "cairn-default"' 'name = "cairn-default-stale-probe"'
    if ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
    }/bin/cairn policy export --check > policy-stale.out 2> policy-stale.err; then
      echo "expected stale generated policy check to fail" >&2
      exit 1
    fi
    grep -Fq "generated policy is stale: cairn-policy/generated/cairn-policy.json" policy-stale.err
    mkdir -p "$out"
    cp policy-stale.out policy-stale.err "$out/"
  '';
  mc-compat-layout-boundaries =
    pkgs.runCommand "mc-compat-layout-boundaries"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
          pkgs.nickel
          pkgs.gnugrep
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        nickel export compat/config/component-registry.ncl > ../component-registry-export.json
        nickel export compat/config/component-registry-fixtures/valid.ncl > ../component-registry-fixture-valid.json
        if nickel export compat/config/component-registry-fixtures/missing-owner.ncl > ../component-registry-fixture-missing-owner.out 2> ../component-registry-fixture-missing-owner.err; then
          echo "expected missing-owner fixture to fail" >&2
          exit 1
        fi
        if nickel export compat/config/component-registry-fixtures/unsafe-path.ncl > ../component-registry-fixture-unsafe-path.out 2> ../component-registry-fixture-unsafe-path.err; then
          echo "expected unsafe-path fixture to fail" >&2
          exit 1
        fi
        if nickel export compat/config/component-registry-fixtures/invalid-gate.ncl > ../component-registry-fixture-invalid-gate.out 2> ../component-registry-fixture-invalid-gate.err; then
          echo "expected invalid-gate fixture to fail" >&2
          exit 1
        fi
        rustc --edition=2021 tools/check_component_registry.rs -o ../check-component-registry
        ../check-component-registry --self-test > ../component-registry-self-test.log
        ../check-component-registry --root . > ../component-registry-check.log
        if ../check-component-registry --registry compat/config/component-registry-fixtures/duplicate-role.ncl --root . > ../component-registry-fixture-duplicate-role.out 2> ../component-registry-fixture-duplicate-role.err; then
          echo "expected duplicate-role fixture to fail" >&2
          exit 1
        fi
        grep -Fq "duplicate_role" ../component-registry-fixture-duplicate-role.err
        rustc --edition=2021 tools/check_layout_boundaries.rs -o ../check-layout-boundaries
        ../check-layout-boundaries --self-test > ../layout-boundaries-self-test.log
        ../check-layout-boundaries --root . > ../layout-boundaries-check.log
        mkdir -p "$out"
        cp ../component-registry-export.json ../component-registry-fixture-valid.json ../component-registry-fixture-missing-owner.out ../component-registry-fixture-missing-owner.err ../component-registry-fixture-unsafe-path.out ../component-registry-fixture-unsafe-path.err ../component-registry-fixture-invalid-gate.out ../component-registry-fixture-invalid-gate.err ../component-registry-self-test.log ../component-registry-check.log ../component-registry-fixture-duplicate-role.out ../component-registry-fixture-duplicate-role.err ../layout-boundaries-self-test.log ../layout-boundaries-check.log "$out/"
      '';
  mc-valence-schedule-hygiene =
    pkgs.runCommand "mc-valence-schedule-hygiene"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_valence_schedule_hygiene.rs -o ../check-valence-schedule-hygiene
        ../check-valence-schedule-hygiene --self-test > ../valence-schedule-hygiene-self-test.log
        ../check-valence-schedule-hygiene --root . > ../valence-schedule-hygiene-check.log
        mkdir -p "$out"
        cp ../valence-schedule-hygiene-self-test.log ../valence-schedule-hygiene-check.log "$out/"
      '';
  mc-compat-editable-app-dry-runs =
    pkgs.runCommand "mc-compat-editable-app-dry-runs" { nativeBuildInputs = [ pkgs.gnugrep ]; }
      ''
        mkdir -p fake-root/servers/valence fake-root/clients/stevenarella
        printf '%s\n' '[package]' 'name = "fake-valence"' 'version = "0.0.0"' 'edition = "2021"' > fake-root/servers/valence/Cargo.toml
        printf '%s\n' '[package]' 'name = "fake-stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-root/clients/stevenarella/Cargo.toml

        (
          cd fake-root
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.valence
          }/bin/valence --dry-run --example-arg > ../valence.log
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.stevenarella
          }/bin/stevenarella --dry-run --client-arg > ../stevenarella.log
        )

        grep -Fq 'repo=' valence.log
        grep -Fq '/servers/valence' valence.log
        grep -Fq -- '--example ctf' valence.log
        grep -Fq -- '--example-arg' valence.log
        grep -Fq 'repo=' stevenarella.log
        grep -Fq '/clients/stevenarella' stevenarella.log
        grep -Fq -- '--client-arg' stevenarella.log

        if VALENCE_REPO="$PWD/missing-valence" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.valence
        }/bin/valence --dry-run > missing-valence.log 2>&1; then
          echo 'expected missing Valence checkout to fail' >&2
          exit 1
        fi
        grep -Fq 'missing Valence source tree' missing-valence.log

        if CLIENT_DIR="$PWD/missing-stevenarella" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.stevenarella
        }/bin/stevenarella --dry-run > missing-stevenarella.log 2>&1; then
          echo 'expected missing Stevenarella checkout to fail' >&2
          exit 1
        fi
        grep -Fq 'missing Stevenarella source tree' missing-stevenarella.log

        mkdir -p "$out"
        cp valence.log stevenarella.log missing-valence.log missing-stevenarella.log "$out/"
      '';
  mc-compat-acceptance-matrix =
    pkgs.runCommand "mc-compat-acceptance-matrix"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_acceptance_matrix.rs -o ../check-acceptance-matrix
        ../check-acceptance-matrix --self-test > ../acceptance-matrix-self-test.log
        ../check-acceptance-matrix > ../acceptance-matrix-check.log
        mkdir -p "$out"
        cp ../acceptance-matrix-self-test.log ../acceptance-matrix-check.log "$out/"
      '';
  mc-compat-current-evidence-bundle =
    pkgs.runCommand "mc-compat-current-evidence-bundle"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_current_evidence_bundle.rs -o ../check-current-evidence-bundle
        ../check-current-evidence-bundle --self-test > ../current-evidence-bundle-self-test.log
        ../check-current-evidence-bundle > ../current-evidence-bundle-check.log
        mkdir -p "$out"
        cp ../current-evidence-bundle-self-test.log ../current-evidence-bundle-check.log "$out/"
      '';
  mc-compat-evidence-manifest-refresh =
    pkgs.runCommand "mc-compat-evidence-manifest-refresh"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/refresh_evidence_manifests.rs -o ../refresh-evidence-manifests
        ../refresh-evidence-manifests --self-test > ../evidence-manifest-refresh-self-test.log
        ../refresh-evidence-manifests --check > ../evidence-manifest-refresh-check.log
        mkdir -p "$out"
        cp ../evidence-manifest-refresh-self-test.log ../evidence-manifest-refresh-check.log "$out/"
      '';
  mc-compat-evidence-manifests =
    pkgs.runCommand "mc-compat-evidence-manifests"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/refresh_evidence_manifests.rs -o ../refresh-evidence-manifests
        ../refresh-evidence-manifests --self-test > ../evidence-manifest-refresh-self-test.log
        ../refresh-evidence-manifests --check > ../evidence-manifest-refresh-check.log
        rustc --edition=2021 tools/check_evidence_manifests.rs -o ../check-evidence-manifests
        ../check-evidence-manifests --self-test > ../evidence-manifest-self-test.log
        ../check-evidence-manifests > ../evidence-manifest-check.log
        mkdir -p "$out"
        cp ../evidence-manifest-refresh-self-test.log ../evidence-manifest-refresh-check.log ../evidence-manifest-self-test.log ../evidence-manifest-check.log "$out/"
      '';
  mc-compat-full-survival-gate =
    pkgs.runCommand "mc-compat-full-survival-gate"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_full_survival_compatibility_gate.rs -o ../check-full-survival-compatibility-gate
        ../check-full-survival-compatibility-gate --self-test > ../full-survival-gate-self-test.log
        ../check-full-survival-compatibility-gate > ../full-survival-gate-check.log
        mkdir -p "$out"
        cp ../full-survival-gate-self-test.log ../full-survival-gate-check.log "$out/"
      '';
  mc-compat-survival-aggregate-claim-boundary =
    pkgs.runCommand "mc-compat-survival-aggregate-claim-boundary"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_survival_aggregate_claim_boundary.rs -o ../check-survival-aggregate-claim-boundary
        ../check-survival-aggregate-claim-boundary --self-test > ../survival-aggregate-claim-boundary-self-test.log
        ../check-survival-aggregate-claim-boundary > ../survival-aggregate-claim-boundary-check.log
        mkdir -p "$out"
        cp ../survival-aggregate-claim-boundary-self-test.log ../survival-aggregate-claim-boundary-check.log "$out/"
      '';
  mc-compat-aggregate-claim-gates =
    pkgs.runCommand "mc-compat-aggregate-claim-gates"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_mc_compat_aggregate_claim_gates.rs -o ../check-mc-compat-aggregate-claim-gates
        ../check-mc-compat-aggregate-claim-gates --self-test > ../aggregate-claim-gates-self-test.log
        ../check-mc-compat-aggregate-claim-gates > ../aggregate-claim-gates-check.log
        mkdir -p "$out"
        cp ../aggregate-claim-gates-self-test.log ../aggregate-claim-gates-check.log "$out/"
      '';
  mc-compat-scenario-manifest =
    pkgs.runCommand "mc-compat-scenario-manifest"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
          pkgs.nickel
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        nickel typecheck compat/config/scenario-manifest.ncl > ../scenario-manifest-typecheck.log
        nickel typecheck compat/config/scenario-fallback-budget-baseline.ncl > ../scenario-fallback-budget-baseline-typecheck.log
        rustc --edition=2021 tools/check_scenario_manifest.rs -o ../check-scenario-manifest
        ../check-scenario-manifest --self-test > ../scenario-manifest-self-test.log
        ../check-scenario-manifest > ../scenario-manifest-check.log
        mkdir -p "$out"
        cp ../scenario-manifest-typecheck.log ../scenario-fallback-budget-baseline-typecheck.log ../scenario-manifest-self-test.log ../scenario-manifest-check.log "$out/"
      '';
  mc-compat-generated-harness-surfaces =
    pkgs.runCommand "mc-compat-generated-harness-surfaces"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
          pkgs.nickel
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        nickel typecheck compat/config/scenario-manifest.ncl > ../generated-harness-surfaces-typecheck.log
        nickel typecheck compat/config/scenario-fallback-budget-baseline.ncl > ../generated-fallback-budget-baseline-typecheck.log
        rustc --edition=2021 tools/check_scenario_manifest.rs -o ../check-scenario-manifest
        ../check-scenario-manifest --self-test > ../generated-harness-surfaces-self-test.log
        ../check-scenario-manifest --check-generated-surfaces > ../generated-harness-surfaces-check.log
        printf 'rows=%s\n' '${toString (builtins.length scenarioWrapperMetadata.rows)}' > ../generated-wrapper-metadata-summary.log
        printf '%s\n' ${pkgs.lib.escapeShellArgs scenarioWrapperMetadata.appWrappers} > ../generated-wrapper-metadata-app-wrappers.log
        printf '%s\n' ${pkgs.lib.escapeShellArgs scenarioWrapperMetadata.dryRunChecks} > ../generated-wrapper-metadata-dry-run-checks.log
        mkdir -p "$out"
        cp ../generated-harness-surfaces-typecheck.log ../generated-fallback-budget-baseline-typecheck.log ../generated-harness-surfaces-self-test.log ../generated-harness-surfaces-check.log ../generated-wrapper-metadata-summary.log ../generated-wrapper-metadata-app-wrappers.log ../generated-wrapper-metadata-dry-run-checks.log "$out/"
      '';
  mc-compat-scenario-contracts =
    pkgs.runCommand "mc-compat-scenario-contracts"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
          pkgs.nickel
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        nickel typecheck compat/config/scenario-contracts.ncl > ../scenario-contracts-typecheck.log
        nickel typecheck compat/config/scenario-manifest.ncl > ../scenario-contracts-manifest-typecheck.log
        rustc --edition=2021 tools/check_scenario_contracts.rs -o ../check-scenario-contracts
        ../check-scenario-contracts --self-test > ../scenario-contracts-self-test.log
        ../check-scenario-contracts --check-generated-surfaces > ../scenario-contracts-check.log
        mkdir -p "$out"
        cp ../scenario-contracts-typecheck.log ../scenario-contracts-manifest-typecheck.log ../scenario-contracts-self-test.log ../scenario-contracts-check.log "$out/"
      '';
  mc-compat-evidence-promotion =
    pkgs.runCommand "mc-compat-evidence-promotion"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
          pkgs.nickel
          pkgs.b3sum
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        nickel typecheck compat/config/evidence-promotion-plan.ncl > ../evidence-promotion-typecheck.log
        rustc --edition=2021 tools/promote_evidence.rs -o ../promote-evidence
        ../promote-evidence --self-test > ../evidence-promotion-self-test.log
        ../promote-evidence --out-dir target/evidence-promotion-dry-run > ../evidence-promotion-dry-run.log
        ../promote-evidence --apply --out-dir target/evidence-promotion-apply > ../evidence-promotion-apply.log
        test -f target/evidence-promotion-apply/promotion-plan.md
        mkdir -p "$out"
        cp ../evidence-promotion-typecheck.log ../evidence-promotion-self-test.log ../evidence-promotion-dry-run.log ../evidence-promotion-apply.log "$out/"
      '';
  mc-compat-cairn-task-evidence =
    pkgs.runCommand "mc-compat-cairn-task-evidence"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_cairn_task_evidence.rs -o ../check-cairn-task-evidence
        ../check-cairn-task-evidence --self-test > ../cairn-task-evidence-self-test.log
        ../check-cairn-task-evidence > ../cairn-task-evidence-check.log
        mkdir -p "$out"
        cp ../cairn-task-evidence-self-test.log ../cairn-task-evidence-check.log "$out/"
      '';
  mc-compat-adversarial-network-oracle =
    pkgs.runCommand "mc-compat-adversarial-network-oracle"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_adversarial_network_oracle.rs -o ../check-adversarial-network-oracle
        ../check-adversarial-network-oracle --self-test > ../adversarial-network-oracle-self-test.log
        ../check-adversarial-network-oracle > ../adversarial-network-oracle-check.log
        mkdir -p "$out"
        cp ../adversarial-network-oracle-self-test.log ../adversarial-network-oracle-check.log "$out/"
      '';
  mc-compat-wan-tolerance-bounded-telemetry =
    pkgs.runCommand "mc-compat-wan-tolerance-bounded-telemetry"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_wan_tolerance_bounded_telemetry.rs -o ../check-wan-tolerance-bounded-telemetry
        ../check-wan-tolerance-bounded-telemetry --self-test > ../wan-tolerance-bounded-telemetry-self-test.log
        ../check-wan-tolerance-bounded-telemetry > ../wan-tolerance-bounded-telemetry-check.log
        mkdir -p "$out"
        cp ../wan-tolerance-bounded-telemetry-self-test.log ../wan-tolerance-bounded-telemetry-check.log "$out/"
      '';
  mc-compat-public-server-authorized-safety =
    pkgs.runCommand "mc-compat-public-server-authorized-safety"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_public_server_authorized_safety.rs -o ../check-public-server-authorized-safety
        ../check-public-server-authorized-safety --self-test > ../public-server-authorized-safety-self-test.log
        ../check-public-server-authorized-safety > ../public-server-authorized-safety-check.log
        mkdir -p "$out"
        cp ../public-server-authorized-safety-self-test.log ../public-server-authorized-safety-check.log "$out/"
      '';
  mc-compat-ctf-invalid-pickup-ownership =
    pkgs.runCommand "mc-compat-ctf-invalid-pickup-ownership"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_ctf_invalid_pickup_ownership.rs -o ../check-ctf-invalid-pickup-ownership
        ../check-ctf-invalid-pickup-ownership --self-test > ../ctf-invalid-pickup-ownership-self-test.log
        ../check-ctf-invalid-pickup-ownership > ../ctf-invalid-pickup-ownership-check.log
        mkdir -p "$out"
        cp ../ctf-invalid-pickup-ownership-self-test.log ../ctf-invalid-pickup-ownership-check.log "$out/"
      '';
  mc-compat-ctf-invalid-return-drop =
    pkgs.runCommand "mc-compat-ctf-invalid-return-drop"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_ctf_invalid_return_drop.rs -o ../check-ctf-invalid-return-drop
        ../check-ctf-invalid-return-drop --self-test > ../ctf-invalid-return-drop-self-test.log
        ../check-ctf-invalid-return-drop > ../ctf-invalid-return-drop-check.log
        mkdir -p "$out"
        cp ../ctf-invalid-return-drop-self-test.log ../ctf-invalid-return-drop-check.log "$out/"
      '';
  mc-compat-ctf-invalid-action-breadth =
    pkgs.runCommand "mc-compat-ctf-invalid-action-breadth"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_ctf_invalid_action_breadth.rs -o ../check-ctf-invalid-action-breadth
        ../check-ctf-invalid-action-breadth --self-test > ../ctf-invalid-action-breadth-self-test.log
        ../check-ctf-invalid-action-breadth > ../ctf-invalid-action-breadth-check.log
        mkdir -p "$out"
        cp ../ctf-invalid-action-breadth-self-test.log ../ctf-invalid-action-breadth-check.log "$out/"
      '';
  mc-compat-ctf-score-limit-win-condition =
    pkgs.runCommand "mc-compat-ctf-score-limit-win-condition"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_ctf_score_limit_win_condition.rs -o ../check-ctf-score-limit-win-condition
        ../check-ctf-score-limit-win-condition --self-test > ../ctf-score-limit-win-condition-self-test.log
        ../check-ctf-score-limit-win-condition > ../ctf-score-limit-win-condition-check.log
        mkdir -p "$out"
        cp ../ctf-score-limit-win-condition-self-test.log ../ctf-score-limit-win-condition-check.log "$out/"
      '';
  mc-compat-red-blue-scoring-soak-live-refresh =
    pkgs.runCommand "mc-compat-red-blue-scoring-soak-live-refresh"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_red_blue_scoring_soak_live_refresh.rs -o ../check-red-blue-scoring-soak-live-refresh
        ../check-red-blue-scoring-soak-live-refresh --self-test > ../red-blue-scoring-soak-live-refresh-self-test.log
        ../check-red-blue-scoring-soak-live-refresh > ../red-blue-scoring-soak-live-refresh-check.log
        mkdir -p "$out"
        cp ../red-blue-scoring-soak-live-refresh-self-test.log ../red-blue-scoring-soak-live-refresh-check.log "$out/"
      '';
  mc-compat-armor-loadout-enchantment-status =
    pkgs.runCommand "mc-compat-armor-loadout-enchantment-status"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_armor_loadout_enchantment_status.rs -o ../check-armor-loadout-enchantment-status
        ../check-armor-loadout-enchantment-status --self-test > ../armor-loadout-enchantment-status-self-test.log
        ../check-armor-loadout-enchantment-status > ../armor-loadout-enchantment-status-check.log
        mkdir -p "$out"
        cp ../armor-loadout-enchantment-status-self-test.log ../armor-loadout-enchantment-status-check.log "$out/"
      '';
  mc-compat-equipment-slot-item-expansion =
    pkgs.runCommand "mc-compat-equipment-slot-item-expansion"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_equipment_slot_item_expansion.rs -o ../check-equipment-slot-item-expansion
        ../check-equipment-slot-item-expansion --self-test > ../equipment-slot-item-expansion-self-test.log
        ../check-equipment-slot-item-expansion > ../equipment-slot-item-expansion-check.log
        mkdir -p "$out"
        cp ../equipment-slot-item-expansion-self-test.log ../equipment-slot-item-expansion-check.log "$out/"
      '';
  mc-compat-dry-run = pkgs.runCommand "mc-compat-dry-run" { } ''
    mkdir -p fake-stevenarella
    printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
    ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" --receipt smoke-receipt.json > dry-run.log
    grep -Fq "start Paper server" dry-run.log
    grep -Fq "would run Rust protocol status probe" dry-run.log
    grep -Fq "would run Stevenarella under xvfb-run" dry-run.log
    grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' smoke-receipt.json
    grep -Fq '"legacy_schema": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
    grep -Fq '"cairn_contract": "mc.compat.scenario.receipt.v2"' smoke-receipt.json
    grep -Fq '"name": "smoke"' smoke-receipt.json
    grep -Fq '"required_milestones": ["protocol_detected"]' smoke-receipt.json
    grep -Fq '"octet_producer_surface": "compat/runner/src/main.rs"' smoke-receipt.json
    grep -Fq '"claims_correctness": false' smoke-receipt.json
    grep -Fq '"claims_semantic_equivalence": false' smoke-receipt.json
    grep -Fq '"wayland_socket_inherited": false' smoke-receipt.json
    mkdir -p "$out"
    cp dry-run.log smoke-receipt.json "$out/"
  '';
  mc-compat-multi-client-scenario-dry-run =
    pkgs.runCommand "mc-compat-multi-client-scenario-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --dry-run --server-backend valence --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD --scenario multi-client-load-score --receipt multi-client-receipt.json > multi-client-dry-run.log
        grep -Fq "scenario 'multi-client-load-score'" multi-client-dry-run.log
        grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' multi-client-receipt.json
        grep -Fq '"legacy_schema": "mc.compat.smoke.receipt.v1"' multi-client-receipt.json
        grep -Fq '"name": "multi-client-load-score"' multi-client-receipt.json
        grep -Fq '"multi_client_count"' multi-client-receipt.json
        grep -Fq '"flag_capture"' multi-client-receipt.json
        grep -Fq '"server_client_a_seen"' multi-client-receipt.json
        grep -Fq '"server_client_b_seen"' multi-client-receipt.json
        grep -Fq '"server_flag_or_score"' multi-client-receipt.json
        grep -Fq '"client_server_correlation"' multi-client-receipt.json
        grep -Fq '"triage"' multi-client-receipt.json
        grep -Fq '"first_missing_client_milestone"' multi-client-receipt.json
        grep -Fq '"first_missing_server_milestone"' multi-client-receipt.json
        grep -Fq '"first_forbidden_pattern"' multi-client-receipt.json
        grep -Fq '"suggested_boundary"' multi-client-receipt.json
        grep -Fq '"client_log_paths"' multi-client-receipt.json
        grep -Fq '"server_log_path"' multi-client-receipt.json
        grep -Fq '"compatbota"' multi-client-receipt.json
        grep -Fq '"compatbotb"' multi-client-receipt.json
        grep -Fq '"claims_correctness": false' multi-client-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' multi-client-receipt.json
        mkdir -p "$out"
        cp multi-client-dry-run.log multi-client-receipt.json "$out/"
      '';
  mc-compat-blue-flag-score-dry-run =
    pkgs.runCommand "mc-compat-blue-flag-score-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --dry-run --server-backend valence --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD --scenario blue-flag-score --receipt blue-flag-receipt.json > blue-flag-dry-run.log
        grep -Fq "scenario 'blue-flag-score'" blue-flag-dry-run.log
        grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' blue-flag-receipt.json
        grep -Fq '"name": "blue-flag-score"' blue-flag-receipt.json
        grep -Fq '"team_blue"' blue-flag-receipt.json
        grep -Fq '"score_blue_1"' blue-flag-receipt.json
        grep -Fq '"server_flag_or_score"' blue-flag-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "chat_scoreboard"]' blue-flag-receipt.json
        grep -Fq '"claims_correctness": false' blue-flag-receipt.json
        mkdir -p "$out"
        cp blue-flag-dry-run.log blue-flag-receipt.json "$out/"
      '';
  mc-compat-historical-scenario-dry-runs =
    pkgs.runCommand "mc-compat-historical-scenario-dry-runs" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts logs
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        for scenario in \
          flag-score-repeat \
          survival-chest-persistence \
          survival-hunger-food \
          survival-hunger-health-cycle \
          survival-mob-drop \
          survival-mob-ai-loot-breadth \
          survival-redstone-toggle \
          survival-redstone-circuit-breadth \
          survival-world-persistence-restart \
          survival-world-multichunk-durability \
          survival-crash-recovery-parity \
          survival-block-entity-persistence-parity \
          survival-container-block-entity-breadth \
          survival-biome-dimension-state \
          survival-biome-dimension-travel \
          survival-sign-editing-live
        do
          receipt="$PWD/receipts/$scenario.json"
          log="logs/$scenario-dry-run.log"
          ${
            self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
          }/bin/mc-compat-runner --dry-run --server-backend valence --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD --scenario "$scenario" --receipt "$receipt" > "$log"
          grep -Fq "scenario '$scenario'" "$log"
          grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' "$receipt"
          grep -Fq "\"name\": \"$scenario\"" "$receipt"
          grep -Fq '"dry_run": true' "$receipt"
          grep -Fq '"claims_correctness": false' "$receipt"
          grep -Fq '"claims_semantic_equivalence": false' "$receipt"
        done
        mkdir -p "$out"
        cp -r logs receipts "$out/"
      '';
  mc-compat-valence-ctf-600s-soak-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-600s-soak-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SOAK_RECEIPT="$PWD/receipts/soak-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-600s-soak
        }/bin/mc-compat-valence-ctf-600s-soak --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > soak-dry-run.log
        grep -Fq "scenario 'multi-client-load-score'" soak-dry-run.log
        grep -Fq '"name": "multi-client-load-score"' receipts/soak-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/soak-receipt.json
        grep -Fq '"protocol": 763' receipts/soak-receipt.json
        grep -Fq '"duration_secs": 600' receipts/soak-receipt.json
        grep -Fq '"timeout_secs": 600' receipts/soak-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "chat_scoreboard"]' receipts/soak-receipt.json
        grep -Fq '"compatbota"' receipts/soak-receipt.json
        grep -Fq '"compatbotb"' receipts/soak-receipt.json
        grep -Fq '"server_client_a_seen"' receipts/soak-receipt.json
        grep -Fq '"server_client_b_seen"' receipts/soak-receipt.json
        mkdir -p "$out"
        cp soak-dry-run.log receipts/soak-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-blue-600s-soak-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-blue-600s-soak-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_BLUE_SOAK_RECEIPT="$PWD/receipts/blue-soak-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-blue-600s-soak
        }/bin/mc-compat-valence-ctf-blue-600s-soak --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > blue-soak-dry-run.log
        grep -Fq "scenario 'blue-flag-score'" blue-soak-dry-run.log
        grep -Fq '"name": "blue-flag-score"' receipts/blue-soak-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/blue-soak-receipt.json
        grep -Fq '"protocol": 763' receipts/blue-soak-receipt.json
        grep -Fq '"duration_secs": 600' receipts/blue-soak-receipt.json
        grep -Fq '"timeout_secs": 600' receipts/blue-soak-receipt.json
        grep -Fq '"team_blue"' receipts/blue-soak-receipt.json
        grep -Fq '"score_blue_1"' receipts/blue-soak-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "chat_scoreboard"]' receipts/blue-soak-receipt.json
        grep -Fq '"server_username_seen"' receipts/blue-soak-receipt.json
        grep -Fq '"server_flag_or_score"' receipts/blue-soak-receipt.json
        mkdir -p "$out"
        cp blue-soak-dry-run.log receipts/blue-soak-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-inventory-interaction-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-inventory-interaction-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_INVENTORY_RECEIPT="$PWD/receipts/inventory-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-inventory-interaction
        }/bin/mc-compat-valence-ctf-inventory-interaction --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > inventory-dry-run.log
        grep -Fq "scenario 'inventory-interaction'" inventory-dry-run.log
        grep -Fq '"name": "inventory-interaction"' receipts/inventory-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/inventory-receipt.json
        grep -Fq '"protocol": 763' receipts/inventory-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/inventory-receipt.json
        grep -Fq '"inventory_slot_update"' receipts/inventory-receipt.json
        grep -Fq '"inventory_sword_slot"' receipts/inventory-receipt.json
        grep -Fq '"inventory_wool_slot"' receipts/inventory-receipt.json
        grep -Fq '"inventory_drop_sent"' receipts/inventory-receipt.json
        grep -Fq '"inventory_pickup_seen"' receipts/inventory-receipt.json
        grep -Fq '"inventory_click_sent"' receipts/inventory-receipt.json
        grep -Fq '"inventory_open_container_seen"' receipts/inventory-receipt.json
        grep -Fq '"inventory_container_click_sent"' receipts/inventory-receipt.json
        grep -Fq '"inventory_block_place_sent"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_hotbar_select"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_drop"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_pickup"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_click"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_open_container"' receipts/inventory-receipt.json
        grep -Fq '"server_inventory_container_click"' receipts/inventory-receipt.json
        grep -Fq '"server_block_place"' receipts/inventory-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "inventory_set_slot", "player_action_drop_item", "open_container", "player_window_click", "player_block_placement"]' receipts/inventory-receipt.json
        mkdir -p "$out"
        cp inventory-dry-run.log receipts/inventory-receipt.json "$out/"
      '';
  mc-compat-valence-inventory-stack-split-merge-dry-run =
    pkgs.runCommand "mc-compat-valence-inventory-stack-split-merge-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_INVENTORY_STACK_RECEIPT="$PWD/receipts/inventory-stack-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-stack-split-merge
        }/bin/mc-compat-valence-inventory-stack-split-merge --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > inventory-stack-dry-run.log
        grep -Fq "scenario 'inventory-stack-split-merge'" inventory-stack-dry-run.log
        grep -Fq '"name": "inventory-stack-split-merge"' receipts/inventory-stack-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/inventory-stack-receipt.json
        grep -Fq '"protocol": 763' receipts/inventory-stack-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_initial_slot"' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_split_pickup_sent"' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_split_source_seen"' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_destination_seen"' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_merge_destination_empty_seen"' receipts/inventory-stack-receipt.json
        grep -Fq '"inventory_stack_final_source_seen"' receipts/inventory-stack-receipt.json
        grep -Fq '"server_inventory_stack_split_pickup"' receipts/inventory-stack-receipt.json
        grep -Fq '"server_inventory_stack_split"' receipts/inventory-stack-receipt.json
        grep -Fq '"server_inventory_stack_merge_pickup"' receipts/inventory-stack-receipt.json
        grep -Fq '"server_inventory_stack_merge"' receipts/inventory-stack-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "inventory_set_slot", "player_window_click"]' receipts/inventory-stack-receipt.json
        mkdir -p "$out"
        cp inventory-stack-dry-run.log receipts/inventory-stack-receipt.json "$out/"
      '';
  mc-compat-inventory-stack-split-merge-evidence-check =
    pkgs.runCommand "mc-compat-inventory-stack-split-merge-evidence-check"
      {
        nativeBuildInputs = [
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-checkers
        ];
      }
      ''
        check-inventory-stack-split-merge-evidence --self-test > inventory-stack-split-merge-evidence-self-test.log
        mkdir -p "$out"
        cp inventory-stack-split-merge-evidence-self-test.log "$out/"
      '';
  mc-compat-valence-inventory-drag-transactions-dry-run =
    pkgs.runCommand "mc-compat-valence-inventory-drag-transactions-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email ci@example.invalid
        git -C fake-valence config user.name ci
        printf 'fake valence\n' > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_INVENTORY_DRAG_RECEIPT="$PWD/receipts/inventory-drag-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-drag-transactions
        }/bin/mc-compat-valence-inventory-drag-transactions --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > inventory-drag-dry-run.log
        grep -Fq "scenario 'inventory-drag-transactions'" inventory-drag-dry-run.log
        grep -Fq '"name": "inventory-drag-transactions"' receipts/inventory-drag-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/inventory-drag-receipt.json
        grep -Fq '"protocol": 763' receipts/inventory-drag-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/inventory-drag-receipt.json
        grep -Fq '"backend": "valence"' receipts/inventory-drag-receipt.json
        grep -Fq '"scenario": {' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_initial_slot"' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_pickup_sent"' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_source_empty_seen"' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_target_a_sent"' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_target_b_sent"' receipts/inventory-drag-receipt.json
        grep -Fq '"inventory_drag_final_distribution_seen"' receipts/inventory-drag-receipt.json
        grep -Fq '"server_inventory_drag_pickup"' receipts/inventory-drag-receipt.json
        grep -Fq '"server_inventory_drag_start"' receipts/inventory-drag-receipt.json
        grep -Fq '"server_inventory_drag_target_a"' receipts/inventory-drag-receipt.json
        grep -Fq '"server_inventory_drag_target_b"' receipts/inventory-drag-receipt.json
        grep -Fq '"server_inventory_drag_end"' receipts/inventory-drag-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "inventory_set_slot", "player_window_click"]' receipts/inventory-drag-receipt.json
        mkdir -p "$out"
        cp inventory-drag-dry-run.log receipts/inventory-drag-receipt.json "$out/"
      '';
  mc-compat-inventory-drag-transactions-evidence-check =
    pkgs.runCommand "mc-compat-inventory-drag-transactions-evidence-check"
      {
        nativeBuildInputs = [
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-checkers
        ];
      }
      ''
        check-inventory-drag-transactions-evidence --self-test > inventory-drag-transactions-evidence-self-test.log
        mkdir -p "$out"
        cp inventory-drag-transactions-evidence-self-test.log "$out/"
      '';
  mc-compat-scoreboard-team-packet-family-check =
    pkgs.runCommand "mc-compat-scoreboard-team-packet-family-check"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_scoreboard_team_packet_family.rs -o ../check-scoreboard-team-packet-family
        ../check-scoreboard-team-packet-family --self-test > ../scoreboard-team-packet-family-self-test.log
        ../check-scoreboard-team-packet-family docs/evidence/scoreboard-team-packet-family-2026-06-06.kv > ../scoreboard-team-packet-family-evidence.log
        mkdir -p "$out"
        cp ../scoreboard-team-packet-family-self-test.log ../scoreboard-team-packet-family-evidence.log "$out/"
      '';
  mc-compat-movement-packet-family-check =
    pkgs.runCommand "mc-compat-movement-packet-family-check"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_movement_packet_family.rs -o ../check-movement-packet-family
        ../check-movement-packet-family --self-test > ../movement-packet-family-self-test.log
        ../check-movement-packet-family docs/evidence/movement-packet-family-2026-06-06.kv > ../movement-packet-family-evidence.log
        mkdir -p "$out"
        cp ../movement-packet-family-self-test.log ../movement-packet-family-evidence.log "$out/"
      '';
  mc-compat-targeted-packet-promotions-check =
    pkgs.runCommand "mc-compat-targeted-packet-promotions-check"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_targeted_packet_promotions.rs -o ../check-targeted-packet-promotions
        ../check-targeted-packet-promotions --self-test > ../targeted-packet-promotions-self-test.log
        ../check-targeted-packet-promotions docs/evidence/block-entity-update-breadth-2026-06-06.kv docs/evidence/chat-command-containment-2026-06-06.kv docs/evidence/chunk-biome-data-packet-2026-06-06.kv docs/evidence/creative-inventory-action-2026-06-06.kv docs/evidence/entity-status-effect-packets-2026-06-06.kv docs/evidence/recipe-book-client-settings-2026-06-06.kv docs/evidence/resource-pack-status-2026-06-06.kv docs/evidence/sign-editor-open-update-2026-06-06.kv > ../targeted-packet-promotions-evidence.log
        mkdir -p "$out"
        cp ../targeted-packet-promotions-self-test.log ../targeted-packet-promotions-evidence.log "$out/"
      '';
  mc-compat-server-correlation-receipts =
    pkgs.runCommand "mc-compat-server-correlation-receipts"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_server_correlation_receipts.rs -o ../check-server-correlation-receipts
        ../check-server-correlation-receipts --self-test > ../server-correlation-receipts-self-test.log
        ../check-server-correlation-receipts docs/evidence/server-correlation-resource-pack-fixture-2026-06-18.receipt.json docs/evidence/server-correlation-sign-editor-fixture-2026-06-18.receipt.json > ../server-correlation-receipts-fixtures.log
        if ../check-server-correlation-receipts --promotable docs/evidence/server-correlation-resource-pack-fixture-2026-06-18.receipt.json > ../server-correlation-receipts-promotable-negative.log 2>&1; then
          echo "checker fixture unexpectedly passed promotable mode" >&2
          exit 1
        fi
        mkdir -p "$out"
        cp ../server-correlation-receipts-self-test.log ../server-correlation-receipts-fixtures.log ../server-correlation-receipts-promotable-negative.log "$out/"
      '';
  mc-compat-valence-ctf-combat-damage-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-combat-damage-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_COMBAT_RECEIPT="$PWD/receipts/combat-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-damage
        }/bin/mc-compat-valence-ctf-combat-damage --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > combat-dry-run.log
        grep -Fq "scenario 'combat-damage'" combat-dry-run.log
        grep -Fq '"name": "combat-damage"' receipts/combat-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/combat-receipt.json
        grep -Fq '"protocol": 763' receipts/combat-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/combat-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/combat-receipt.json
        grep -Fq '"multi_client_count"' receipts/combat-receipt.json
        grep -Fq '"team_red"' receipts/combat-receipt.json
        grep -Fq '"team_blue"' receipts/combat-receipt.json
        grep -Fq '"remote_player_spawn"' receipts/combat-receipt.json
        grep -Fq '"combat_attack_sent"' receipts/combat-receipt.json
        grep -Fq '"combat_health_update"' receipts/combat-receipt.json
        grep -Fq '"server_client_a_seen"' receipts/combat-receipt.json
        grep -Fq '"server_client_b_seen"' receipts/combat-receipt.json
        grep -Fq '"server_combat_damage"' receipts/combat-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "use_entity_attack"]' receipts/combat-receipt.json
        mkdir -p "$out"
        cp combat-dry-run.log receipts/combat-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-combat-knockback-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-combat-knockback-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_KNOCKBACK_RECEIPT="$PWD/receipts/knockback-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-knockback
        }/bin/mc-compat-valence-ctf-combat-knockback --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > knockback-dry-run.log
        grep -Fq "scenario 'combat-knockback'" knockback-dry-run.log
        grep -Fq '"name": "combat-knockback"' receipts/knockback-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/knockback-receipt.json
        grep -Fq '"protocol": 763' receipts/knockback-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/knockback-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/knockback-receipt.json
        grep -Fq '"multi_client_count"' receipts/knockback-receipt.json
        grep -Fq '"combat_attack_sent"' receipts/knockback-receipt.json
        grep -Fq '"combat_velocity_update"' receipts/knockback-receipt.json
        grep -Fq '"server_combat_knockback"' receipts/knockback-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "use_entity_attack", "entity_velocity"]' receipts/knockback-receipt.json
        mkdir -p "$out"
        cp knockback-dry-run.log receipts/knockback-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-armor-equipment-mitigation-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-armor-equipment-mitigation-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_ARMOR_MITIGATION_RECEIPT="$PWD/receipts/armor-mitigation-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-armor-equipment-mitigation
        }/bin/mc-compat-valence-ctf-armor-equipment-mitigation --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > armor-mitigation-dry-run.log
        grep -Fq "scenario 'armor-equipment-mitigation'" armor-mitigation-dry-run.log
        grep -Fq '"name": "armor-equipment-mitigation"' receipts/armor-mitigation-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/armor-mitigation-receipt.json
        grep -Fq '"protocol": 763' receipts/armor-mitigation-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/armor-mitigation-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/armor-mitigation-receipt.json
        grep -Fq '"armor_inventory_slot"' receipts/armor-mitigation-receipt.json
        grep -Fq '"combat_health_update"' receipts/armor-mitigation-receipt.json
        grep -Fq '"server_equipment_state"' receipts/armor-mitigation-receipt.json
        grep -Fq '"server_armor_mitigation"' receipts/armor-mitigation-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "inventory_set_slot", "use_entity_attack", "armor_mitigation"]' receipts/armor-mitigation-receipt.json
        grep -Fq '"claims_correctness": false' receipts/armor-mitigation-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/armor-mitigation-receipt.json
        mkdir -p "$out"
        cp armor-mitigation-dry-run.log receipts/armor-mitigation-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-equipment-update-observation-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-equipment-update-observation-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_EQUIPMENT_UPDATE_RECEIPT="$PWD/receipts/equipment-update-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-equipment-update-observation
        }/bin/mc-compat-valence-ctf-equipment-update-observation --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > equipment-update-dry-run.log
        grep -Fq "scenario 'equipment-update-observation'" equipment-update-dry-run.log
        grep -Fq '"name": "equipment-update-observation"' receipts/equipment-update-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/equipment-update-receipt.json
        grep -Fq '"protocol": 763' receipts/equipment-update-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/equipment-update-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/equipment-update-receipt.json
        grep -Fq '"entity_equipment_update"' receipts/equipment-update-receipt.json
        grep -Fq '"server_equipment_update_state"' receipts/equipment-update-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "entity_equipment_update"]' receipts/equipment-update-receipt.json
        grep -Fq '"claims_correctness": false' receipts/equipment-update-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/equipment-update-receipt.json
        mkdir -p "$out"
        cp equipment-update-dry-run.log receipts/equipment-update-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-projectile-hit-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-projectile-hit-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_PROJECTILE_HIT_RECEIPT="$PWD/receipts/projectile-hit-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-hit
        }/bin/mc-compat-valence-ctf-projectile-hit --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > projectile-hit-dry-run.log
        grep -Fq "scenario 'projectile-hit'" projectile-hit-dry-run.log
        grep -Fq '"name": "projectile-hit"' receipts/projectile-hit-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/projectile-hit-receipt.json
        grep -Fq '"protocol": 763' receipts/projectile-hit-receipt.json
        grep -Fq '"timeout_secs": 240' receipts/projectile-hit-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/projectile-hit-receipt.json
        grep -Fq '"projectile_use_sent"' receipts/projectile-hit-receipt.json
        grep -Fq '"projectile_swing_sent"' receipts/projectile-hit-receipt.json
        grep -Fq '"server_projectile_loadout"' receipts/projectile-hit-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "projectile_use_item", "projectile_hit_attribution"]' receipts/projectile-hit-receipt.json
        grep -Fq '"claims_correctness": false' receipts/projectile-hit-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/projectile-hit-receipt.json
        mkdir -p "$out"
        cp projectile-hit-dry-run.log receipts/projectile-hit-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-projectile-damage-attribution-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-projectile-damage-attribution-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        if MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT="$PWD/receipts/head-rejected.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-damage-attribution
        }/bin/mc-compat-valence-ctf-projectile-damage-attribution --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > head-rejected.log 2>&1; then
          echo "projectile damage dry-run unexpectedly accepted VALENCE_REV=HEAD" >&2
          exit 1
        fi
        grep -Fq "requires pinned Valence revision ${pinnedProjectileDamageValenceRev}" head-rejected.log
        MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT="$PWD/receipts/projectile-damage-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-damage-attribution
        }/bin/mc-compat-valence-ctf-projectile-damage-attribution --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev ${pinnedProjectileDamageValenceRev} > projectile-damage-dry-run.log
        grep -Fq "scenario 'projectile-damage-attribution'" projectile-damage-dry-run.log
        grep -Fq '"name": "projectile-damage-attribution"' receipts/projectile-damage-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/projectile-damage-receipt.json
        grep -Fq '"protocol": 763' receipts/projectile-damage-receipt.json
        grep -Fq '"timeout_secs": 240' receipts/projectile-damage-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/projectile-damage-receipt.json
        grep -Fq '"projectile_use_sent"' receipts/projectile-damage-receipt.json
        grep -Fq '"projectile_swing_sent"' receipts/projectile-damage-receipt.json
        grep -Fq '"projectile_damage_update"' receipts/projectile-damage-receipt.json
        grep -Fq '"server_projectile_use"' receipts/projectile-damage-receipt.json
        grep -Fq '"server_projectile_hit"' receipts/projectile-damage-receipt.json
        grep -Fq '"projectile_damage_causality"' receipts/projectile-damage-receipt.json
        grep -Fq '"attacker": "compatbota"' receipts/projectile-damage-receipt.json
        grep -Fq '"victim": "compatbotb"' receipts/projectile-damage-receipt.json
        grep -Fq '"missing_steps": []' receipts/projectile-damage-receipt.json
        grep -Fq '"order_violations": []' receipts/projectile-damage-receipt.json
        grep -Fq '"rev": "${pinnedProjectileDamageValenceRev}"' receipts/projectile-damage-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "projectile_use_item", "projectile_hit_attribution", "health_update"]' receipts/projectile-damage-receipt.json
        grep -Fq '"claims_correctness": false' receipts/projectile-damage-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/projectile-damage-receipt.json
        mkdir -p "$out"
        cp projectile-damage-dry-run.log receipts/projectile-damage-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-flag-carrier-death-return-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-flag-carrier-death-return-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_FLAG_CARRIER_DEATH_RECEIPT="$PWD/receipts/flag-carrier-death-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-flag-carrier-death-return
        }/bin/mc-compat-valence-ctf-flag-carrier-death-return --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > flag-carrier-death-dry-run.log
        grep -Fq "scenario 'flag-carrier-death-return'" flag-carrier-death-dry-run.log
        grep -Fq '"name": "flag-carrier-death-return"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"protocol": 763' receipts/flag-carrier-death-receipt.json
        grep -Fq '"timeout_secs": 150' receipts/flag-carrier-death-receipt.json
        grep -Fq '"usernames": ["compatbota", "compatbotb"]' receipts/flag-carrier-death-receipt.json
        grep -Fq '"multi_client_count"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"team_red"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"team_blue"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"flag_pickup"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"combat_death_observed"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"respawn_request_sent"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"respawn_health_restored"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"server_flag_carrier_death"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"server_flag_return"' receipts/flag-carrier-death-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "flag_pickup", "use_entity_attack", "health_death", "respawn_request"]' receipts/flag-carrier-death-receipt.json
        mkdir -p "$out"
        cp flag-carrier-death-dry-run.log receipts/flag-carrier-death-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-latency-jitter-inventory-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-latency-jitter-inventory-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
                    mkdir -p fake-stevenarella fake-valence receipts
                    printf '%s
        ' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
                    git -C fake-valence init
                    git -C fake-valence config user.email mc-compat@example.invalid
                    git -C fake-valence config user.name mc-compat
                    printf '%s
        ' fake > fake-valence/README.md
                    git -C fake-valence add README.md
                    git -C fake-valence commit -m init
                    MC_COMPAT_LATENCY_JITTER_RECEIPT="$PWD/receipts/latency-jitter-receipt.json" ${
                      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-latency-jitter-inventory
                    }/bin/mc-compat-valence-ctf-latency-jitter-inventory --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > latency-jitter-dry-run.log
                    grep -Fq "scenario 'inventory-interaction'" latency-jitter-dry-run.log
                    grep -Fq '"name": "inventory-interaction"' receipts/latency-jitter-receipt.json
                    grep -Fq '"latency_jitter_tolerance"' receipts/latency-jitter-receipt.json
                    grep -Fq '"selected": true' receipts/latency-jitter-receipt.json
                    grep -Fq '"target_rail": "inventory-interaction"' receipts/latency-jitter-receipt.json
                    grep -Fq '"delay_ms": "80"' receipts/latency-jitter-receipt.json
                    grep -Fq '"jitter_ms": "30"' receipts/latency-jitter-receipt.json
                    grep -Fq '"loss_percent": "0"' receipts/latency-jitter-receipt.json
                    grep -Fq '"privileged_network_mutation_required": false' receipts/latency-jitter-receipt.json
                    grep -Fq '"target_ownership": "owned-local-loopback"' receipts/latency-jitter-receipt.json
                    grep -Fq '"authorization": "owned-local-fixture-approved"' receipts/latency-jitter-receipt.json
                    grep -Fq '"telemetry_samples"' receipts/latency-jitter-receipt.json
                    grep -Fq '"pass_fail_criteria": "inventory_interaction_client_server_milestones"' receipts/latency-jitter-receipt.json
                    grep -Fq '"claims_wan_safety": false' receipts/latency-jitter-receipt.json
                    grep -Fq '"claims_packet_loss_tolerance": false' receipts/latency-jitter-receipt.json
                    grep -Fq '"claims_public_server_safety": false' receipts/latency-jitter-receipt.json
                    grep -Fq '"inventory_slot_update"' receipts/latency-jitter-receipt.json
                    grep -Fq '"server_inventory_click"' receipts/latency-jitter-receipt.json
                    mkdir -p "$out"
                    cp latency-jitter-dry-run.log receipts/latency-jitter-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-reconnect-flag-state-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-reconnect-flag-state-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
                    mkdir -p fake-stevenarella fake-valence receipts
                    printf '%s
        ' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
                    git -C fake-valence init
                    git -C fake-valence config user.email mc-compat@example.invalid
                    git -C fake-valence config user.name mc-compat
                    printf '%s
        ' fake > fake-valence/README.md
                    git -C fake-valence add README.md
                    git -C fake-valence commit -m init
                    MC_COMPAT_RECONNECT_FLAG_STATE_RECEIPT="$PWD/receipts/reconnect-flag-state-receipt.json" ${
                      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-reconnect-flag-state
                    }/bin/mc-compat-valence-ctf-reconnect-flag-state --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > reconnect-flag-state-dry-run.log
                    grep -Fq "scenario 'reconnect-flag-state'" reconnect-flag-state-dry-run.log
                    grep -Fq '"name": "reconnect-flag-state"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"version": "1.20.1"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"protocol": 763' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"timeout_secs": 120' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"reconnect_session"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"flag_disconnect_return"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"reconnect_state_coherent"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"server_flag_disconnect_return"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"server_reconnect_state_coherent"' receipts/reconnect-flag-state-receipt.json
                    grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "flag_pickup", "disconnect_reconnect", "flag_state_reset"]' receipts/reconnect-flag-state-receipt.json
                    mkdir -p "$out"
                    cp reconnect-flag-state-dry-run.log receipts/reconnect-flag-state-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-invalid-pickup-ownership-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-invalid-pickup-ownership-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_INVALID_PICKUP_OWNERSHIP_RECEIPT="$PWD/receipts/ctf-invalid-pickup-ownership-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-pickup-ownership
        }/bin/mc-compat-valence-ctf-invalid-pickup-ownership --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-invalid-pickup-ownership-dry-run.log
        grep -Fq "scenario 'ctf-invalid-pickup-ownership'" ctf-invalid-pickup-ownership-dry-run.log
        grep -Fq '"name": "ctf-invalid-pickup-ownership"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"ctf_invalid_pickup_attempted"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"ctf_invalid_pickup_contained"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"server_invalid_pickup_rejected"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"invalid_action": "own_flag_pickup_without_ownership_transfer"' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "own_flag_pickup_attempt", "invalid_flag_pickup_rejected"]' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-invalid-pickup-ownership-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-invalid-pickup-ownership-receipt.json
        mkdir -p "$out"
        cp ctf-invalid-pickup-ownership-dry-run.log receipts/ctf-invalid-pickup-ownership-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-invalid-return-drop-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-invalid-return-drop-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_INVALID_RETURN_DROP_RECEIPT="$PWD/receipts/ctf-invalid-return-drop-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-return-drop
        }/bin/mc-compat-valence-ctf-invalid-return-drop --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-invalid-return-drop-dry-run.log
        grep -Fq "scenario 'ctf-invalid-return-drop'" ctf-invalid-return-drop-dry-run.log
        grep -Fq '"name": "ctf-invalid-return-drop"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"ctf_invalid_return_drop_attempted"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"ctf_invalid_return_drop_contained"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"server_invalid_return_drop_rejected"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"invalid_action": "own_base_return_without_carrier"' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "own_flag_return_drop_attempt", "invalid_flag_return_drop_rejected"]' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-invalid-return-drop-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-invalid-return-drop-receipt.json
        mkdir -p "$out"
        cp ctf-invalid-return-drop-dry-run.log receipts/ctf-invalid-return-drop-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-invalid-opponent-base-return-drop-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-invalid-opponent-base-return-drop-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_RECEIPT="$PWD/receipts/ctf-invalid-opponent-base-return-drop-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-opponent-base-return-drop
        }/bin/mc-compat-valence-ctf-invalid-opponent-base-return-drop --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-invalid-opponent-base-return-drop-dry-run.log
        grep -Fq "scenario 'ctf-invalid-opponent-base-return-drop'" ctf-invalid-opponent-base-return-drop-dry-run.log
        grep -Fq '"name": "ctf-invalid-opponent-base-return-drop"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"ctf_invalid_opponent_base_return_drop_attempted"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"ctf_invalid_opponent_base_return_drop_contained"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"server_invalid_opponent_base_return_drop_rejected"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"invalid_action": "opponent_base_return_drop_without_carrier"' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "opponent_base_return_drop_attempt", "invalid_opponent_base_return_drop_rejected"]' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-invalid-opponent-base-return-drop-receipt.json
        mkdir -p "$out"
        cp ctf-invalid-opponent-base-return-drop-dry-run.log receipts/ctf-invalid-opponent-base-return-drop-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-score-limit-win-condition-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-score-limit-win-condition-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_SCORE_LIMIT_WIN_CONDITION_RECEIPT="$PWD/receipts/ctf-score-limit-win-condition-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-score-limit-win-condition
        }/bin/mc-compat-valence-ctf-score-limit-win-condition --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-score-limit-win-condition-dry-run.log
        grep -Fq "scenario 'ctf-score-limit-win-condition'" ctf-score-limit-win-condition-dry-run.log
        grep -Fq '"name": "ctf-score-limit-win-condition"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"ctf_score_limit_win_seen"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"server_score_limit_pre_state"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"server_score_limit_final_capture"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"server_score_limit_win_condition"' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "flag_pickup", "flag_capture", "score_limit_win_condition"]' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-score-limit-win-condition-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-score-limit-win-condition-receipt.json
        mkdir -p "$out"
        cp ctf-score-limit-win-condition-dry-run.log receipts/ctf-score-limit-win-condition-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-simultaneous-pickup-capture-race-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-simultaneous-pickup-capture-race-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_SIMULTANEOUS_PICKUP_CAPTURE_RACE_RECEIPT="$PWD/receipts/ctf-simultaneous-pickup-capture-race-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-simultaneous-pickup-capture-race
        }/bin/mc-compat-valence-ctf-simultaneous-pickup-capture-race --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-simultaneous-pickup-capture-race-dry-run.log
        grep -Fq "scenario 'ctf-simultaneous-pickup-capture-race'" ctf-simultaneous-pickup-capture-race-dry-run.log
        grep -Fq '"name": "ctf-simultaneous-pickup-capture-race"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"ctf_race_client_count"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"server_ctf_race_accepted_transition"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"server_ctf_race_rejected_transition"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"server_ctf_race_final_state"' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "flag_pickup", "duplicate_flag_pickup_rejected", "flag_capture", "race_final_state"]' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-simultaneous-pickup-capture-race-receipt.json
        mkdir -p "$out"
        cp ctf-simultaneous-pickup-capture-race-dry-run.log receipts/ctf-simultaneous-pickup-capture-race-receipt.json "$out/"
      '';
  mc-compat-valence-ctf-spawn-team-balance-reset-dry-run =
    pkgs.runCommand "mc-compat-valence-ctf-spawn-team-balance-reset-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_CTF_SPAWN_TEAM_BALANCE_RESET_RECEIPT="$PWD/receipts/ctf-spawn-team-balance-reset-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-spawn-team-balance-reset
        }/bin/mc-compat-valence-ctf-spawn-team-balance-reset --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ctf-spawn-team-balance-reset-dry-run.log
        grep -Fq "scenario 'ctf-spawn-team-balance-reset'" ctf-spawn-team-balance-reset-dry-run.log
        grep -Fq '"name": "ctf-spawn-team-balance-reset"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"protocol": 763' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"ctf_spawn_team_reset_client_count"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"server_ctf_spawn_red_assignment"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"server_ctf_spawn_blue_assignment"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"server_ctf_spawn_team_balance"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"server_ctf_spawn_resource_reset"' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"expected_summary_packets": ["two_client_login", "play_join_game", "team_assignment", "balanced_team_counts", "flag_capture", "resource_reset_state"]' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"claims_correctness": false' receipts/ctf-spawn-team-balance-reset-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/ctf-spawn-team-balance-reset-receipt.json
        mkdir -p "$out"
        cp ctf-spawn-team-balance-reset-dry-run.log receipts/ctf-spawn-team-balance-reset-receipt.json "$out/"
      '';
  mc-compat-valence-movement-packet-family-dry-run =
    pkgs.runCommand "mc-compat-valence-movement-packet-family-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_MOVEMENT_PACKET_FAMILY_RECEIPT="$PWD/receipts/movement-packet-family-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-movement-packet-family
        }/bin/mc-compat-valence-movement-packet-family --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > movement-packet-family-dry-run.log
        grep -Fq "scenario 'ctf-spawn-team-balance-reset'" movement-packet-family-dry-run.log
        grep -Fq '"name": "ctf-spawn-team-balance-reset"' receipts/movement-packet-family-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/movement-packet-family-receipt.json
        grep -Fq '"protocol": 763' receipts/movement-packet-family-receipt.json
        grep -Fq '"ctf_spawn_team_reset_client_count"' receipts/movement-packet-family-receipt.json
        grep -Fq '"server_ctf_spawn_red_assignment"' receipts/movement-packet-family-receipt.json
        grep -Fq '"claims_correctness": false' receipts/movement-packet-family-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/movement-packet-family-receipt.json
        mkdir -p "$out"
        cp movement-packet-family-dry-run.log receipts/movement-packet-family-receipt.json "$out/"
      '';
  mc-compat-valence-survival-break-place-pickup-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-break-place-pickup-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_BREAK_PLACE_PICKUP_RECEIPT="$PWD/receipts/survival-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-break-place-pickup
        }/bin/mc-compat-valence-survival-break-place-pickup --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-dry-run.log
        grep -Fq "scenario 'survival-break-place-pickup'" survival-dry-run.log
        grep -Fq '"name": "survival-break-place-pickup"' receipts/survival-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-receipt.json
        grep -Fq '"survival_break_sent"' receipts/survival-receipt.json
        grep -Fq '"survival_break_update"' receipts/survival-receipt.json
        grep -Fq '"survival_pickup_seen"' receipts/survival-receipt.json
        grep -Fq '"survival_place_sent"' receipts/survival-receipt.json
        grep -Fq '"survival_place_update"' receipts/survival-receipt.json
        grep -Fq '"server_survival_join"' receipts/survival-receipt.json
        grep -Fq '"server_survival_break"' receipts/survival-receipt.json
        grep -Fq '"server_survival_pickup"' receipts/survival-receipt.json
        grep -Fq '"server_survival_place"' receipts/survival-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "player_action_break_block", "block_update", "inventory_pickup", "player_block_placement"]' receipts/survival-receipt.json
        grep -Fq '"claims_correctness": false' receipts/survival-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-receipt.json
        mkdir -p "$out"
        cp survival-dry-run.log receipts/survival-receipt.json "$out/"
      '';
  mc-compat-valence-survival-crafting-table-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-crafting-table-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_CRAFTING_TABLE_RECEIPT="$PWD/receipts/survival-crafting-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-table
        }/bin/mc-compat-valence-survival-crafting-table --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-crafting-dry-run.log
        grep -Fq "scenario 'survival-crafting-table'" survival-crafting-dry-run.log
        grep -Fq '"name": "survival-crafting-table"' receipts/survival-crafting-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-crafting-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-crafting-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-crafting-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_table_open_seen"' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_input_a_sent"' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_input_b_sent"' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_result_seen"' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_result_collected"' receipts/survival-crafting-receipt.json
        grep -Fq '"survival_crafting_inventory_updated"' receipts/survival-crafting-receipt.json
        grep -Fq '"server_survival_crafting_table_open"' receipts/survival-crafting-receipt.json
        grep -Fq '"server_survival_crafting_input_a"' receipts/survival-crafting-receipt.json
        grep -Fq '"server_survival_crafting_input_b"' receipts/survival-crafting-receipt.json
        grep -Fq '"server_survival_crafting_result"' receipts/survival-crafting-receipt.json
        grep -Fq '"server_survival_crafting_collect"' receipts/survival-crafting-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "open_container", "crafting_grid_click", "crafting_result_collect", "inventory_update"]' receipts/survival-crafting-receipt.json
        grep -Fq '"claims_correctness": false' receipts/survival-crafting-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-crafting-receipt.json
        mkdir -p "$out"
        cp survival-crafting-dry-run.log receipts/survival-crafting-receipt.json "$out/"
      '';
  mc-compat-valence-survival-crafting-recipe-breadth-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-crafting-recipe-breadth-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_CRAFTING_RECIPE_BREADTH_RECEIPT="$PWD/receipts/survival-crafting-recipe-breadth-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-recipe-breadth
        }/bin/mc-compat-valence-survival-crafting-recipe-breadth --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-crafting-recipe-breadth-dry-run.log
        grep -Fq "scenario 'survival-crafting-recipe-breadth'" survival-crafting-recipe-breadth-dry-run.log
        grep -Fq '"name": "survival-crafting-recipe-breadth"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"survival_crafting_breadth_shaped_seen"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"survival_crafting_breadth_shapeless_seen"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"survival_crafting_breadth_grid_clear_seen"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"survival_crafting_breadth_invalid_seen"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"survival_crafting_breadth_inventory_updated"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"server_survival_crafting_breadth_shaped"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"server_survival_crafting_breadth_shapeless"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"server_survival_crafting_breadth_grid_clear"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"server_survival_crafting_breadth_invalid_rejected"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"server_survival_crafting_breadth_state"' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "open_container", "shaped_recipe_result", "shapeless_recipe_result", "invalid_recipe_reject", "inventory_update"]' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"claims_correctness": false' receipts/survival-crafting-recipe-breadth-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-crafting-recipe-breadth-receipt.json
        mkdir -p "$out"
        cp survival-crafting-recipe-breadth-dry-run.log receipts/survival-crafting-recipe-breadth-receipt.json "$out/"
      '';
  mc-compat-valence-survival-furnace-persistence-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-furnace-persistence-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_FURNACE_RECEIPT="$PWD/receipts/survival-furnace-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-persistence
        }/bin/mc-compat-valence-survival-furnace-persistence --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-furnace-dry-run.log
        grep -Fq "scenario 'survival-furnace-persistence'" survival-furnace-dry-run.log
        grep -Fq '"name": "survival-furnace-persistence"' receipts/survival-furnace-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-furnace-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-furnace-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-furnace-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_open_seen"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_input_sent"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_fuel_sent"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_burn_progress_seen"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_output_seen"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_output_collected"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_inventory_updated"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_reconnect_sent"' receipts/survival-furnace-receipt.json
        grep -Fq '"survival_furnace_reopen_seen"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_open"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_input"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_fuel"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_burn_progress"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_output_available"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_output_collect"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_reconnect_reopen"' receipts/survival-furnace-receipt.json
        grep -Fq '"server_survival_furnace_state"' receipts/survival-furnace-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "open_container", "furnace_input_click", "furnace_fuel_click", "furnace_output_collect", "disconnect_reconnect"]' receipts/survival-furnace-receipt.json
        paper_fixture=${
          srcRoot + /compat/fixtures/paper-survival/src/main/java/mc/compat/paper/SurvivalFixturePlugin.java
        }
        grep -Fq 'PlayerQuitEvent' "$paper_fixture"
        grep -Fq 'furnaceReconnectJoinSeen.contains(playerId)' "$paper_fixture"
        grep -Fq 'isExpectedFurnaceInput(inventory.getItem(FURNACE_INPUT_SLOT))' "$paper_fixture"
        grep -Fq 'isExpectedFurnaceFuel(inventory.getItem(FURNACE_FUEL_SLOT))' "$paper_fixture"
        grep -Fq 'isEmptyFurnaceOutput(inventory.getItem(FURNACE_OUTPUT_SLOT))' "$paper_fixture"
        grep -Fq '"claims_correctness": false' receipts/survival-furnace-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-furnace-receipt.json
        mkdir -p "$out"
        cp survival-furnace-dry-run.log receipts/survival-furnace-receipt.json "$out/"
      '';
  mc-compat-valence-survival-furnace-smelting-breadth-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-furnace-smelting-breadth-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_RECEIPT="$PWD/receipts/survival-furnace-smelting-breadth-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-smelting-breadth
        }/bin/mc-compat-valence-survival-furnace-smelting-breadth --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-furnace-smelting-breadth-dry-run.log
        grep -Fq "scenario 'survival-furnace-smelting-breadth'" survival-furnace-smelting-breadth-dry-run.log
        grep -Fq '"name": "survival-furnace-smelting-breadth"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_open_seen"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_input_sent"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_fuel_sent"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_burn_progress_seen"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_output_seen"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_output_collected"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_inventory_updated"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"survival_furnace_invalid_fuel_sent"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_open"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_input"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_fuel"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_burn_progress"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_output_available"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_output_collect"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_invalid_fuel_rejected"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"server_survival_furnace_breadth_state"' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "open_container", "furnace_input", "fuel_inserted", "burn_progress", "output_available", "output_collected", "inventory_update", "invalid_fuel_attempt", "invalid_fuel_reject"]' receipts/survival-furnace-smelting-breadth-receipt.json
        paper_fixture=${
          srcRoot + /compat/fixtures/paper-survival/src/main/java/mc/compat/paper/SurvivalFixturePlugin.java
        }
        grep -Fq 'FURNACE_SMELTING_BREADTH_FIXTURE_ENV' "$paper_fixture"
        grep -Fq 'survival_furnace_invalid_fuel_rejected' "$paper_fixture"
        grep -Fq 'survival_furnace_breadth_state' "$paper_fixture"
        grep -Fq 'FURNACE_INVALID_FUEL_OUTCOME' "$paper_fixture"
        grep -Fq '"claims_correctness": false' receipts/survival-furnace-smelting-breadth-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-furnace-smelting-breadth-receipt.json
        mkdir -p "$out"
        cp survival-furnace-smelting-breadth-dry-run.log receipts/survival-furnace-smelting-breadth-receipt.json "$out/"
      '';
  mc-compat-valence-survival-hunger-health-cycle-dry-run =
    pkgs.runCommand "mc-compat-valence-survival-hunger-health-cycle-dry-run"
      { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        MC_COMPAT_SURVIVAL_HUNGER_HEALTH_CYCLE_RECEIPT="$PWD/receipts/survival-hunger-health-cycle-receipt.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-hunger-health-cycle
        }/bin/mc-compat-valence-survival-hunger-health-cycle --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > survival-hunger-health-cycle-dry-run.log
        grep -Fq "scenario 'survival-hunger-health-cycle'" survival-hunger-health-cycle-dry-run.log
        grep -Fq '"name": "survival-hunger-health-cycle"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"example": "survival_compat"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"version": "1.20.1"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"protocol": 763' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"timeout_secs": 120' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"survival_hunger_health_item_seen"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"survival_hunger_health_pre_seen"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"survival_hunger_health_consume_sent"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"survival_hunger_health_recovery_seen"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"survival_hunger_health_inventory_updated"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"server_survival_hunger_health_pre"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"server_survival_hunger_health_consume_start"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"server_survival_hunger_health_consume_finish"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"server_survival_hunger_health_inventory"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"server_survival_hunger_health_state"' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"expected_summary_packets": ["login_success", "play_join_game", "inventory_set_slot", "use_item", "food_update", "inventory_update"]' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"claims_correctness": false' receipts/survival-hunger-health-cycle-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' receipts/survival-hunger-health-cycle-receipt.json
        mkdir -p "$out"
        cp survival-hunger-health-cycle-dry-run.log receipts/survival-hunger-health-cycle-receipt.json "$out/"
      '';
  mc-compat-mcp-controlled-smoke-dry-run =
    pkgs.runCommand "mc-compat-mcp-controlled-smoke-dry-run"
      {
        nativeBuildInputs = [
          pkgs.git
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        mkdir -p fake-stevenarella fake-valence receipts
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        rustc --edition=2021 tools/check_mcp_controlled_compat_rail.rs -o ../check-mcp-controlled-compat-rail
        ../check-mcp-controlled-compat-rail --self-test > ../mcp-controlled-checker-self-test.log
        MC_COMPAT_MCP_CONTROLLED_SMOKE_RECEIPT="$PWD/receipts/mcp-controlled-smoke.json" ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-mcp-controlled-smoke
        }/bin/mc-compat-mcp-controlled-smoke --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > ../mcp-controlled-smoke-dry-run.log
        ../check-mcp-controlled-compat-rail --receipt receipts/mcp-controlled-smoke.json > ../mcp-controlled-receipt-check.log
        grep -Fq "scenario 'mcp-controlled-smoke'" ../mcp-controlled-smoke-dry-run.log
        grep -Fq '"name": "mcp-controlled-smoke"' receipts/mcp-controlled-smoke.json
        grep -Fq '"mcp_control": {' receipts/mcp-controlled-smoke.json
        grep -Fq '"handshake_success": true' receipts/mcp-controlled-smoke.json
        grep -Fq '"stdout_clean": true' receipts/mcp-controlled-smoke.json
        grep -Fq '"status.applied"' receipts/mcp-controlled-smoke.json
        grep -Fq '"stevenarella_child_revision": "dry-run"' receipts/mcp-controlled-smoke.json
        grep -Fq '"frame_artifacts": {' receipts/mcp-controlled-smoke.json
        grep -Fq '"promotion_ready": false' receipts/mcp-controlled-smoke.json
        grep -Fq '"semantic_equivalence"' receipts/mcp-controlled-smoke.json
        mkdir -p "$out"
        cp ../mcp-controlled-checker-self-test.log ../mcp-controlled-smoke-dry-run.log ../mcp-controlled-receipt-check.log receipts/mcp-controlled-smoke.json "$out/"
      '';
  mc-compat-maintained-dry-runs = pkgs.runCommand "mc-compat-maintained-dry-runs" { } ''
    mkdir -p "$out"
    ln -s ${self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-dry-run} "$out/smoke"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-multi-client-scenario-dry-run
    } "$out/multi-client-load-score"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-blue-flag-score-dry-run
    } "$out/blue-flag-score"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-historical-scenario-dry-runs
    } "$out/historical-scenario-dry-runs"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-600s-soak-dry-run
    } "$out/ctf-600s-soak"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-blue-600s-soak-dry-run
    } "$out/ctf-blue-600s-soak"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-inventory-interaction-dry-run
    } "$out/inventory-interaction"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-stack-split-merge-dry-run
    } "$out/inventory-stack-split-merge"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-inventory-drag-transactions-dry-run
    } "$out/inventory-drag-transactions"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-damage-dry-run
    } "$out/combat-damage"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-combat-knockback-dry-run
    } "$out/combat-knockback"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-armor-equipment-mitigation-dry-run
    } "$out/armor-equipment-mitigation"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-equipment-update-observation-dry-run
    } "$out/equipment-update-observation"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-hit-dry-run
    } "$out/projectile-hit"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-projectile-damage-attribution-dry-run
    } "$out/projectile-damage-attribution"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-flag-carrier-death-return-dry-run
    } "$out/flag-carrier-death-return"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-latency-jitter-inventory-dry-run
    } "$out/latency-jitter-inventory"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-reconnect-flag-state-dry-run
    } "$out/reconnect-flag-state"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-pickup-ownership-dry-run
    } "$out/ctf-invalid-pickup-ownership"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-return-drop-dry-run
    } "$out/ctf-invalid-return-drop"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-invalid-opponent-base-return-drop-dry-run
    } "$out/ctf-invalid-opponent-base-return-drop"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-score-limit-win-condition-dry-run
    } "$out/ctf-score-limit-win-condition"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-simultaneous-pickup-capture-race-dry-run
    } "$out/ctf-simultaneous-pickup-capture-race"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-ctf-spawn-team-balance-reset-dry-run
    } "$out/ctf-spawn-team-balance-reset"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-movement-packet-family-dry-run
    } "$out/movement-packet-family-rail"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-break-place-pickup-dry-run
    } "$out/survival-break-place-pickup"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-table-dry-run
    } "$out/survival-crafting-table"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-crafting-recipe-breadth-dry-run
    } "$out/survival-crafting-recipe-breadth"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-persistence-dry-run
    } "$out/survival-furnace-persistence"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-furnace-smelting-breadth-dry-run
    } "$out/survival-furnace-smelting-breadth"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-valence-survival-hunger-health-cycle-dry-run
    } "$out/survival-hunger-health-cycle"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-mcp-controlled-smoke-dry-run
    } "$out/mcp-controlled-smoke"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-bot-probe-dry-run
    } "$out/compat-bot-probe"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-acceptance-matrix
    } "$out/acceptance-matrix"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-current-evidence-bundle
    } "$out/current-evidence-bundle"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-scoreboard-team-packet-family-check
    } "$out/scoreboard-team-packet-family"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-movement-packet-family-check
    } "$out/movement-packet-family"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-server-correlation-receipts
    } "$out/server-correlation-receipts"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-evidence-manifests
    } "$out/evidence-manifests"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-evidence-manifest-refresh
    } "$out/evidence-manifest-refresh"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-full-survival-gate
    } "$out/full-survival-gate"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-survival-aggregate-claim-boundary
    } "$out/survival-aggregate-claim-boundary"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-aggregate-claim-gates
    } "$out/aggregate-claim-gates"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-scenario-manifest
    } "$out/scenario-manifest"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-evidence-promotion
    } "$out/evidence-promotion"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-cairn-task-evidence
    } "$out/cairn-task-evidence"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-adversarial-network-oracle
    } "$out/adversarial-network-oracle"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-wan-tolerance-bounded-telemetry
    } "$out/wan-tolerance-bounded-telemetry"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-public-server-authorized-safety
    } "$out/public-server-authorized-safety"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-ctf-invalid-pickup-ownership
    } "$out/ctf-invalid-pickup-ownership-checker"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-ctf-invalid-return-drop
    } "$out/ctf-invalid-return-drop-checker"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-ctf-invalid-action-breadth
    } "$out/ctf-invalid-action-breadth-checker"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-ctf-score-limit-win-condition
    } "$out/ctf-score-limit-win-condition-checker"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-red-blue-scoring-soak-live-refresh
    } "$out/red-blue-scoring-soak-live-refresh"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-armor-loadout-enchantment-status
    } "$out/armor-loadout-enchantment-status"
    ln -s ${
      self.checks.${pkgs.stdenv.hostPlatform.system}.mc-compat-equipment-slot-item-expansion
    } "$out/equipment-slot-item-expansion"
    cat > "$out/manifest.txt" <<'EOF'
    smoke
    multi-client-load-score
    blue-flag-score
    flag-score-repeat
    survival-chest-persistence
    survival-hunger-food
    survival-hunger-health-cycle
    survival-mob-drop
    survival-mob-ai-loot-breadth
    survival-redstone-toggle
    survival-redstone-circuit-breadth
    survival-world-persistence-restart
    survival-world-multichunk-durability
    survival-crash-recovery-parity
    survival-block-entity-persistence-parity
    survival-container-block-entity-breadth
    survival-biome-dimension-state
    survival-biome-dimension-travel
    survival-sign-editing-live
    ctf-600s-soak
    ctf-blue-600s-soak
    inventory-interaction
    inventory-stack-split-merge
    inventory-drag-transactions
    combat-damage
    combat-knockback
    armor-equipment-mitigation
    equipment-update-observation
    projectile-hit
    projectile-damage-attribution
    flag-carrier-death-return
    latency-jitter-inventory
    reconnect-flag-state
    ctf-invalid-pickup-ownership
    ctf-invalid-return-drop
    ctf-invalid-opponent-base-return-drop
    ctf-score-limit-win-condition
    ctf-simultaneous-pickup-capture-race
    ctf-spawn-team-balance-reset
    movement-packet-family-rail
    red-blue-scoring-soak-live-refresh
    survival-break-place-pickup
    survival-crafting-table
    survival-furnace-persistence
    survival-furnace-smelting-breadth
    survival-hunger-health-cycle
    compat-bot-probe
    acceptance-matrix
    current-evidence-bundle
    scoreboard-team-packet-family
    movement-packet-family
    server-correlation-receipts
    evidence-manifests
    full-survival-gate
    aggregate-claim-gates
    scenario-manifest
    evidence-promotion
    cairn-task-evidence
    adversarial-network-oracle
    wan-tolerance-bounded-telemetry
    public-server-authorized-safety
    ctf-invalid-pickup-ownership-checker
    ctf-invalid-return-drop-checker
    ctf-invalid-action-breadth-checker
    ctf-score-limit-win-condition-checker
    armor-loadout-enchantment-status
    equipment-slot-item-expansion
    EOF
  '';
  mc-compat-bot-probe-dry-run =
    pkgs.runCommand "mc-compat-bot-probe-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --dry-run --server-backend valence --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD --scenario valence-compat-bot-probe --receipt compat-bot-receipt.json > compat-bot-dry-run.log
        grep -Fq "scenario 'valence-compat-bot-probe'" compat-bot-dry-run.log
        grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' compat-bot-receipt.json
        grep -Fq '"name": "valence-compat-bot-probe"' compat-bot-receipt.json
        grep -Fq '"compat_bot_probe"' compat-bot-receipt.json
        grep -Fq '"selected": true' compat-bot-receipt.json
        grep -Fq '"safe_bounded_probe": true' compat-bot-receipt.json
        grep -Fq '"target_address": "127.0.0.1:25565"' compat-bot-receipt.json
        grep -Fq '"owned_local_target_required": true' compat-bot-receipt.json
        grep -Fq '"external_server_load_authorized": false' compat-bot-receipt.json
        grep -Fq '"public_stress_tool": false' compat-bot-receipt.json
        grep -Fq '"planned_clients": 1' compat-bot-receipt.json
        grep -Fq '"max_clients": 1' compat-bot-receipt.json
        grep -Fq '"required_milestones": ["protocol_detected", "join_game", "render_tick"]' compat-bot-receipt.json
        grep -Fq '"claims_correctness": false' compat-bot-receipt.json
        grep -Fq '"claims_semantic_equivalence": false' compat-bot-receipt.json
        mkdir -p "$out"
        cp compat-bot-dry-run.log compat-bot-receipt.json "$out/"
      '';
  mc-compat-open-cairns-dry-run =
    pkgs.runCommand "mc-compat-open-cairns-dry-run" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --dry-run --server-backend valence --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD --scenario reconnect-flag-score --expect-status-description "compat fixture" --expect-status-version "compat-version" --expect-status-sample compatbot,observer --packet-capture-summary --proxy-route velocity-local --proxy-forwarding-mode modern --receipt open-cairns-receipt.json > open-cairns-dry-run.log
        grep -Fq "scenario 'reconnect-flag-score'" open-cairns-dry-run.log
        grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' open-cairns-receipt.json
        grep -Fq '"name": "reconnect-flag-score"' open-cairns-receipt.json
        grep -Fq '"status_response_resource"' open-cairns-receipt.json
        grep -Fq '"resource_owned": true' open-cairns-receipt.json
        grep -Fq '"configured": true' open-cairns-receipt.json
        grep -Fq '"expected_description": "compat fixture"' open-cairns-receipt.json
        grep -Fq '"expected_version_name": "compat-version"' open-cairns-receipt.json
        grep -Fq '"expected_player_sample": ["compatbot", "observer"]' open-cairns-receipt.json
        grep -Fq '"asserted_by_status_probe": true' open-cairns-receipt.json
        grep -Fq '"packet_capture_oracle"' open-cairns-receipt.json
        grep -Fq '"headless_cli": true' open-cairns-receipt.json
        grep -Fq '"redacted_receipt": true' open-cairns-receipt.json
        grep -Fq '"raw_payloads_recorded": false' open-cairns-receipt.json
        grep -Fq '"normalized_fields": ["direction", "state", "packet_id", "decode_status"]' open-cairns-receipt.json
        grep -Fq '"triage_correlation": true' open-cairns-receipt.json
        grep -Fq '"proxy_compat_seam"' open-cairns-receipt.json
        grep -Fq '"route": "velocity-local"' open-cairns-receipt.json
        grep -Fq '"forwarding_mode": "modern"' open-cairns-receipt.json
        grep -Fq '"direct_and_proxied_claims_separated": true' open-cairns-receipt.json
        grep -Fq '"mtls_ported": false' open-cairns-receipt.json
        grep -Fq '"credentials_recorded": false' open-cairns-receipt.json
        grep -Fq '"owned_local_proxy_required": true' open-cairns-receipt.json
        grep -Fq '"gameplay_oracles"' open-cairns-receipt.json
        grep -Fq '"selected_scenario": "reconnect-flag-score"' open-cairns-receipt.json
        grep -Fq '"reconnect_session"' open-cairns-receipt.json
        grep -Fq '"requires_client_and_server_evidence_for_semantic_claims": true' open-cairns-receipt.json
        grep -Fq '"full_ctf_correctness"' open-cairns-receipt.json
        grep -Fq '"broad_minecraft_compatibility"' open-cairns-receipt.json
        grep -Fq '"unbounded_soak"' open-cairns-receipt.json
        mkdir -p "$out"
        cp open-cairns-dry-run.log open-cairns-receipt.json "$out/"
      '';
  mc-compat-compare-receipts = pkgs.runCommand "mc-compat-compare-receipts" { } ''
              write_receipt() {
                backend="$1"
                protocol="$2"
                port="$3"
                path="$4"
                cat > "$path" <<EOF
    {
      "schema": "mc.compat.smoke.receipt.v1",
      "status": "pass",
      "mode": "run",
      "dry_run": false,
      "contract": {
        "claims_correctness": false,
        "claims_semantic_equivalence": false
      },
      "server": {
        "backend": "$backend",
        "version": "1.18.2",
        "protocol": $protocol,
        "port": $port
      },
      "client": {
        "headless_isolation": {
          "xvfb": true,
          "x11_backend": true,
          "software_gl": true,
          "wayland_socket_inherited": false
        },
        "classification": "timeout-success-evidence",
        "matched_success_pattern": "Detected server protocol version"
      },
      "error": null
    }
    EOF
              }
              write_receipt paper 758 25566 paper.json
              write_receipt valence 758 25565 valence.json
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
              }/bin/mc-compat-runner --compare-receipts paper.json valence.json > compare.log
              grep -Fq "receipt comparison passed" compare.log

              write_receipt valence 759 25565 bad-valence.json
              if ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
              }/bin/mc-compat-runner --compare-receipts paper.json bad-valence.json > bad.log 2>&1; then
                echo "expected mismatched protocol comparison to fail" >&2
                cat bad.log >&2
                exit 1
              fi
              grep -Fq "receipt protocol mismatch" bad.log
              mkdir -p "$out"
              cp paper.json valence.json compare.log bad.log "$out/"
  '';
  mc-compat-run-matrix =
    pkgs.runCommand "mc-compat-run-matrix" { nativeBuildInputs = [ pkgs.git ]; }
      ''
        mkdir -p fake-stevenarella fake-valence matrix
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        git -C fake-valence init
        git -C fake-valence config user.email mc-compat@example.invalid
        git -C fake-valence config user.name mc-compat
        printf '%s\n' fake > fake-valence/README.md
        git -C fake-valence add README.md
        git -C fake-valence commit -m init
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --run-matrix --dry-run --receipt-dir "$PWD/matrix" --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/fake-valence" --valence-rev HEAD > matrix.log
        grep -Fq "matrix passed" matrix.log
        grep -Fq '"backend": "paper"' matrix/paper.json
        grep -Fq '"port": 25566' matrix/paper.json
        grep -Fq '"backend": "valence"' matrix/valence.json
        grep -Fq '"port": 25565' matrix/valence.json
        grep -Fq '"classification": "dry-run"' matrix/paper.json
        grep -Fq '"classification": "dry-run"' matrix/valence.json
        mkdir -p "$out"
        cp -r matrix matrix.log "$out/"
      '';
  mc-compat-missing-client = pkgs.runCommand "mc-compat-missing-client" { } ''
    if ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/no-such-stevenarella" > missing.log 2>&1; then
      echo "expected missing Stevenarella checkout to fail" >&2
      cat missing.log >&2
      exit 1
    fi
    grep -Fq "Stevenarella source tree not found" missing.log
    grep -Fq "clients/stevenarella" missing.log
    grep -Fq -- "--client-dir/CLIENT_DIR" missing.log
    mkdir -p "$out"
    cp missing.log "$out/"
  '';
  mc-compat-missing-valence = pkgs.runCommand "mc-compat-missing-valence" { } ''
    mkdir -p fake-stevenarella
    printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
    if ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner --dry-run --client-dir "$PWD/fake-stevenarella" --valence-repo "$PWD/no-such-valence" > missing.log 2>&1; then
      echo "expected missing Valence checkout to fail" >&2
      cat missing.log >&2
      exit 1
    fi
    grep -Fq "Valence source tree not found" missing.log
    grep -Fq "servers/valence" missing.log
    grep -Fq -- "--valence-repo/VALENCE_REPO" missing.log
    mkdir -p "$out"
    cp missing.log "$out/"
  '';
  mc-compat-help = pkgs.runCommand "mc-compat-help" { } ''
    ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner --help > help.log
    grep -Fq -- "--server-backend valence|paper" help.log
    grep -Fq -- "--config PATH" help.log
    grep -Fq -- "--client-dir PATH" help.log
    grep -Fq -- "--receipt PATH" help.log
    grep -Fq -- "--receipt-dir DIR" help.log
    grep -Fq -- "--run-matrix" help.log
    grep -Fq -- "--status-only" help.log
    grep -Fq -- "--status" help.log
    grep -Fq -- "--cleanup" help.log
    grep -Fq -- "--apply" help.log
    grep -Fq -- "--stop" help.log
    grep -Fq -- "--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT" help.log
    grep -Fq -- "--scenario smoke|valence-compat-bot-probe|flag-score-repeat|blue-flag-score|inventory-interaction|inventory-stack-split-merge|inventory-drag-transactions|survival-break-place-pickup|survival-chest-persistence|survival-crafting-table|survival-crafting-recipe-breadth|survival-furnace-persistence|survival-furnace-smelting-breadth|survival-hunger-food|survival-hunger-health-cycle|survival-mob-drop|survival-mob-ai-loot-breadth|survival-redstone-toggle|survival-redstone-circuit-breadth|survival-world-persistence-restart|survival-world-multichunk-durability|survival-crash-recovery-parity|survival-block-entity-persistence-parity|survival-container-block-entity-breadth|survival-biome-dimension-state|survival-biome-dimension-travel|survival-sign-editing-live|mcp-controlled-smoke|combat-damage|combat-knockback|vanilla-combat-reference-parity|vanilla-combat-armor-reference-parity|armor-equipment-mitigation|armor-loadout-enchantment-status-matrix|equipment-update-observation|equipment-slot-item-matrix-expansion|projectile-hit|projectile-damage-attribution|flag-carrier-death-return|reconnect-flag-state|reconnect-flag-score|multi-client-load-score|negative-inventory-stale-state|negative-inventory-invalid-click|negative-custom-payload|negative-reconnect-race|negative-ctf-wrong-score|ctf-invalid-pickup-ownership|ctf-invalid-return-drop|ctf-invalid-opponent-base-return-drop|ctf-score-limit-win-condition|ctf-simultaneous-pickup-capture-race|ctf-spawn-team-balance-reset" help.log
    grep -Fq "MC_COMPAT_SCENARIO" help.log
    grep -Fq -- "--expect-status-description" help.log
    grep -Fq -- "--packet-capture-summary" help.log
    grep -Fq -- "--proxy-route" help.log
    grep -Fq "SMOKE_RECEIPT" help.log
    grep -Fq "SMOKE_RECEIPT_DIR" help.log
    grep -Fq "MC_COMPAT_CONFIG" help.log
    grep -Fq "CLIENT_DIR" help.log
    grep -Fq -- "--valence-repo PATH" help.log
    grep -Fq "no inherited Wayland socket" help.log
    mkdir -p "$out"
    cp help.log "$out/"
  '';
  mc-compat-nickel-config =
    pkgs.runCommand "mc-compat-nickel-config" { nativeBuildInputs = [ pkgs.nickel ]; }
      ''
        nickel typecheck ${srcRoot + /compat/config/default.ncl}
        nickel export ${srcRoot + /compat/config/default.ncl} > exported.json
        cmp exported.json ${srcRoot + /compat/config/generated/default.json}

        mkdir -p fake-stevenarella
        printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
        ${
          self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
        }/bin/mc-compat-runner --config ${
          srcRoot + /compat/config/generated/default.json
        } --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" > config-dry-run.log
        grep -Fq "start Paper server" config-dry-run.log
        grep -Fq "protocol 758" config-dry-run.log
        mkdir -p "$out"
        cp exported.json config-dry-run.log "$out/"
      '';
  mc-compat-receipt-contract = pkgs.runCommand "mc-compat-receipt-contract" { } ''
    mkdir -p fake-stevenarella
    printf '%s\n' '[package]' 'name = "stevenarella"' 'version = "0.0.0"' 'edition = "2021"' > fake-stevenarella/Cargo.toml
    ${
      self.packages.${pkgs.stdenv.hostPlatform.system}.mc-compat-runner
    }/bin/mc-compat-runner --dry-run --server-backend paper --client-dir "$PWD/fake-stevenarella" --receipt smoke-receipt.json > smoke.log
    ${cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn --help > cairn-help.log
    ${
      octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet
    }/bin/cargo-octet fingerprint --check --output-format json ${
      srcRoot + /compat/runner/src/main.rs
    } > octet-fingerprint.json
    grep -Fq '"schema": "mc.compat.scenario.receipt.v2"' smoke-receipt.json
    grep -Fq '"legacy_schema": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
    grep -Fq '"cairn_contract": "mc.compat.scenario.receipt.v2"' smoke-receipt.json
    grep -Fq '"legacy_cairn_contract": "mc.compat.smoke.receipt.v1"' smoke-receipt.json
    grep -Fq '"octet_producer_surface": "compat/runner/src/main.rs"' smoke-receipt.json
    grep -Fq '"headless_isolation"' smoke-receipt.json
    grep -Fq 'agent-receipt validate' cairn-help.log
    grep -Fq '"schema_version": 1' octet-fingerprint.json
    mkdir -p "$out"
    cp smoke.log smoke-receipt.json cairn-help.log octet-fingerprint.json "$out/"
  '';
  mc-compat-valence-evidence =
    pkgs.runCommand "mc-compat-valence-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/mc-compat-valence-smoke-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/mc-compat-valence-smoke-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.smoke.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "run")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("server.backend", receipt["server"]["backend"], "valence")
        assert_eq("server.version", receipt["server"]["version"], "1.18.2")
        assert_eq("server.protocol", receipt["server"]["protocol"], 758)
        assert_eq("client.classification", receipt["client"]["classification"], "timeout-success-evidence")
        assert_eq("client.matched_success_pattern", receipt["client"]["matched_success_pattern"], "Detected server protocol version")
        assert_eq("headless.xvfb", receipt["client"]["headless_isolation"]["xvfb"], True)
        assert_eq("headless.x11_backend", receipt["client"]["headless_isolation"]["x11_backend"], True)
        assert_eq("headless.software_gl", receipt["client"]["headless_isolation"]["software_gl"], True)
        assert_eq("headless.wayland_socket_inherited", receipt["client"]["headless_isolation"]["wayland_socket_inherited"], False)
        assert_eq("contract.claims_correctness", receipt["contract"]["claims_correctness"], False)
        assert_eq("contract.claims_semantic_equivalence", receipt["contract"]["claims_semantic_equivalence"], False)

        required_note_fragments = [
            "Stevenarella → Valence",
            "Receipt status: `pass`",
            "Server status probe: protocol `758`",
            "Client classification: `timeout-success-evidence`",
            "Matched success pattern: `Detected server protocol version`",
            "This is a bounded compatibility smoke receipt.",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  mc-compat-valence-boundary-evidence =
    pkgs.runCommand "mc-compat-valence-boundary-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/mc-compat-valence-boundary-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/mc-compat-valence-boundary-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.valence-boundary.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "boundary_probe")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.minecraft_version", receipt["valence"]["minecraft_version"], "1.20.1")
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.status_probe.observed_protocol", receipt["valence"]["status_probe"]["observed_protocol"], 763)
        assert_eq("valence.live_parkour_smoke.status", receipt["valence"]["live_parkour_smoke"]["status"], "passed")
        assert_eq("valence.live_parkour_smoke.claims_client_compat", receipt["valence"]["live_parkour_smoke"]["claims_client_compat"], False)
        assert_eq("stevenarella.default_protocol", receipt["stevenarella"]["default_protocol"], 758)
        assert_eq("stevenarella.supports_valence_current_protocol", receipt["stevenarella"]["supports_valence_current_protocol"], False)
        assert_eq("boundary.proven_path_protocol", receipt["boundary"]["proven_path_protocol"], 758)
        assert_eq("boundary.current_valence_protocol", receipt["boundary"]["current_valence_protocol"], 763)
        assert_eq("boundary.protocol_gap", receipt["boundary"]["protocol_gap"], 5)
        assert_eq("boundary.update_stevenarella_required_for_current_valence", receipt["boundary"]["update_stevenarella_required_for_current_valence"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_stevenarella_763_support", receipt["contract"]["claims_stevenarella_763_support"], False)

        required_note_fragments = [
            "Current Valence main is **not** the same compatibility target",
            "Valence advertised protocol: `763`",
            "Stevenarella default/highest supported protocol: `758`",
            "Status probe for expected protocol `758`: failed as expected",
            "Stevenarella is updated to support protocol `763`",
            "Valence is pinned/translated back to protocol `758`",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"boundary evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-handshake-evidence =
    pkgs.runCommand "stevenarella-valence-763-handshake-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/stevenarella-valence-763-handshake-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-handshake-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-handshake.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "current_valence_763_handshake_probe")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.status_probe_passed", receipt["valence"]["status_probe_passed"], True)
        assert_eq("stevenarella.claims_full_1_20_1_protocol_support", receipt["stevenarella"]["claims_full_1_20_1_protocol_support"], False)
        assert_eq("client.exit_code", receipt["client_probe"]["exit_code"], 124)
        assert_eq("client.matched_success_pattern", receipt["client_probe"]["matched_success_pattern"], "Detected server protocol version 763")
        assert_eq("verification.unit_tests_status", receipt["verification"]["unit_tests_status"], "pass")
        assert_eq("contract.claims_current_valence_initial_handshake", receipt["contract"]["claims_current_valence_initial_handshake"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

        required_note_fragments = [
            "Detected server protocol version 763",
            "not** full 1.20.1 protocol support",
            "Client log BLAKE3",
            "Receipt BLAKE3",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"handshake evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-packet-boundary-evidence =
    pkgs.runCommand "stevenarella-valence-763-packet-boundary-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-packet-boundary-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-packet-boundary-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-packet-boundary.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "current_valence_763_offline_login_packet_trace")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.connection_mode", receipt["valence"]["connection_mode"], "Offline")
        assert_eq("first packet", receipt["valence"]["first_captured_packets"][0]["semantic"], "SetCompressionS2C")
        assert_eq("second packet", receipt["valence"]["first_captured_packets"][1]["semantic"], "LoginSuccessS2C")
        assert_eq("boundary wire id", receipt["boundary"]["first_mismatch"]["wire_id"], "0x28")
        assert_eq("boundary valence semantic", receipt["boundary"]["first_mismatch"]["valence_763_semantic"], "GameJoinS2C")
        assert_eq("boundary stevenarella semantic", receipt["boundary"]["first_mismatch"]["stevenarella_758_alias_semantic"], "TradeList_WithRestock")
        assert_eq("trace packet count", receipt["trace"]["packet_count"], 20)
        assert_eq("devshell validated", receipt["devshell_fix"]["validated"], True)
        assert_eq("verification.runner build", receipt["verification"]["mc_compat_runner_build_status"], "pass")
        assert_eq("contract.claims_current_valence_login_packet_boundary", receipt["contract"]["claims_current_valence_login_packet_boundary"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

        required_note_fragments = [
            "play/clientbound/0x28",
            "GameJoinS2C",
            "TradeList_WithRestock",
            "Devshell/flake repair",
            "Receipt BLAKE3",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"packet-boundary evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-join-game-patch-evidence =
    pkgs.runCommand "stevenarella-valence-763-join-game-patch-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-join-game-patch-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-join-game-patch-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-join-game-patch.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "protocol_763_join_game_mapping_patch")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("patched wire id", receipt["boundary"]["patched_boundary"]["wire_id"], "0x28")
        assert_eq("patched valence semantic", receipt["boundary"]["patched_boundary"]["valence_763_semantic"], "GameJoinS2C")
        assert_eq("patched internal", receipt["boundary"]["patched_boundary"]["stevenarella_763_internal"], "JoinGame_WorldNames_IsHard_SimDist")
        assert_eq("patched status", receipt["boundary"]["patched_boundary"]["status"], "mapped")
        assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x10")
        assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "CommandTreeS2CPacket")
        assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], "ClearTitles")
        assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
        assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
        assert_eq("contract.claims_protocol_763_join_game_mapping", receipt["contract"]["claims_protocol_763_join_game_mapping"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

        required_note_fragments = [
            "play/clientbound/0x28",
            "JoinGame_WorldNames_IsHard_SimDist",
            "play/clientbound/0x10",
            "CommandTreeS2CPacket",
            "ClearTitles",
            "Receipt BLAKE3",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"join-game patch evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-command-tree-update-evidence =
    pkgs.runCommand "stevenarella-valence-763-command-tree-update-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-command-tree-update-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-command-tree-update-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-command-tree-update.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "protocol_763_command_tree_mapping_update")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("updated wire id", receipt["boundary"]["updated_boundary"]["wire_id"], "0x10")
        assert_eq("updated valence semantic", receipt["boundary"]["updated_boundary"]["valence_763_semantic"], "CommandTreeS2CPacket")
        assert_eq("updated internal", receipt["boundary"]["updated_boundary"]["stevenarella_763_internal"], "DeclareCommands")
        assert_eq("previous inherited semantic", receipt["boundary"]["updated_boundary"]["previous_stevenarella_758_alias_semantic"], "ClearTitles")
        assert_eq("updated status", receipt["boundary"]["updated_boundary"]["status"], "mapped")
        assert_eq("prior mapped wire id", receipt["boundary"]["still_mapped_from_prior_update"]["wire_id"], "0x28")
        assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x64")
        assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "GameMessageS2CPacket")
        assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], "EntityProperties_VarIntVarInt")
        assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
        assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
        assert_eq("contract.claims_protocol_763_command_tree_mapping", receipt["contract"]["claims_protocol_763_command_tree_mapping"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)

        required_note_fragments = [
            "play/clientbound/0x10",
            "CommandTreeS2CPacket",
            "DeclareCommands",
            "play/clientbound/0x64",
            "EntityProperties_VarIntVarInt",
            "Receipt BLAKE3",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"command-tree update evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-game-message-update-evidence =
    pkgs.runCommand "stevenarella-valence-763-game-message-update-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-game-message-update-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-game-message-update-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-game-message-update.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "protocol_763_game_message_mapping_update")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("updated wire id", receipt["boundary"]["updated_boundary"]["wire_id"], "0x64")
        assert_eq("updated valence semantic", receipt["boundary"]["updated_boundary"]["valence_763_semantic"], "GameMessageS2CPacket")
        assert_eq("updated internal", receipt["boundary"]["updated_boundary"]["stevenarella_763_internal"], "ServerMessage_Position")
        assert_eq("previous inherited semantic", receipt["boundary"]["updated_boundary"]["previous_stevenarella_758_alias_semantic"], "EntityProperties_VarIntVarInt")
        assert_eq("updated status", receipt["boundary"]["updated_boundary"]["status"], "mapped")
        assert_eq("prior first mapped wire id", receipt["boundary"]["still_mapped_from_prior_updates"][0]["wire_id"], "0x28")
        assert_eq("prior second mapped wire id", receipt["boundary"]["still_mapped_from_prior_updates"][1]["wire_id"], "0x10")
        assert_eq("next wire id", receipt["boundary"]["next_boundary"]["wire_id"], "0x69")
        assert_eq("next valence semantic", receipt["boundary"]["next_boundary"]["valence_763_semantic"], "AdvancementUpdateS2CPacket")
        assert_eq("next stevenarella semantic", receipt["boundary"]["next_boundary"]["stevenarella_758_alias_semantic"], None)
        assert_eq("next status", receipt["boundary"]["next_boundary"]["status"], "unmapped_by_758_fallback")
        assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
        assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
        assert_eq("contract.claims_protocol_763_game_message_mapping", receipt["contract"]["claims_protocol_763_game_message_mapping"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)

        required_note_fragments = [
            "play/clientbound/0x64",
            "GameMessageS2CPacket",
            "ServerMessage_Position",
            "EntityProperties_VarIntVarInt",
            "play/clientbound/0x69",
            "AdvancementUpdateS2CPacket",
            "Receipt BLAKE3",
        ]
        for fragment in required_note_fragments:
            if fragment not in note:
                raise SystemExit(f"game-message update evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-observed-boundaries-drain-evidence =
    pkgs.runCommand "stevenarella-valence-763-observed-boundaries-drain-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.receipt.json
            }
            note=${
              srcRoot + /docs/evidence/stevenarella-valence-763-observed-boundaries-drain-2026-05-23.md
            }

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-observed-boundaries-drain.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "protocol_763_observed_boundaries_mapping_drain")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c5e89d")
        assert_eq("boundary count", len(receipt["boundaries"]), 14)
        assert_eq("prior boundary count", len(receipt["prior_mapped_boundaries"]), 3)
        assert_eq("trace first unmapped", receipt["verification"]["trace_first_unmapped"], None)
        assert_eq("fmt", receipt["verification"]["cargo_fmt_check"], "pass")
        assert_eq("tests", receipt["verification"]["steven_protocol_tests"], "pass")
        assert_eq("trace", receipt["verification"]["valence_ctf_trace"], "pass")
        assert_eq("headless", receipt["verification"]["headless_probe"], "pass")
        assert_eq("contract.claims_observed_valence_ctf_boundaries_mapped", receipt["contract"]["claims_observed_valence_ctf_boundaries_mapped"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_all_minecraft_1_20_1_packets", receipt["contract"]["claims_all_minecraft_1_20_1_packets"], False)

        required = {
            "0x69": "Advancements",
            "0x58": "ScoreboardObjective",
            "0x51": "ScoreboardDisplay",
            "0x5b": "UpdateScore_VarInt",
            "0x4d": "SetCurrentHotbarSlot",
            "0x14": "WindowSetSlot_State",
            "0x3a": "PlayerInfo",
            "0x57": "UpdateHealth",
            "0x52": "EntityMetadata",
            "0x6a": "EntityProperties_VarIntVarInt",
            "0x1c": "EntityStatus",
            "0x34": "PlayerAbilities",
            "0x6e": "Tags_Nested",
            "0x24": "ChunkData_AndLight",
        }
        actual = {item["wire_id"]: item["stevenarella_763_internal"] for item in receipt["boundaries"]}
        assert_eq("boundary map", actual, required)

        for fragment in [
            "play/clientbound/0x69",
            "AdvancementUpdateS2CPacket",
            "ChunkData_AndLight",
            "Receipt BLAKE3",
            "Does not prove every protocol 763 packet is mapped",
        ]:
            if fragment not in note:
                raise SystemExit(f"drain evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-gameplay-smoke-evidence =
    pkgs.runCommand "stevenarella-valence-763-gameplay-smoke-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-gameplay-smoke-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-gameplay-smoke-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-gameplay-smoke.receipt.v1")
        assert_eq("status", receipt["status"], "pass")
        assert_eq("mode", receipt["mode"], "protocol_763_bounded_stevenarella_valence_ctf_runtime_smoke")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c5e89d")
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("debug status", receipt["artifacts"]["debug_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("debug protocol", receipt["artifacts"]["debug_log"]["Detected server protocol version 763"], 1)
        assert_eq("probe panic count", receipt["artifacts"]["probe_log"]["panicked"], 0)
        assert_eq("debug panic count", receipt["artifacts"]["debug_log"]["panicked"], 0)
        assert_eq("probe unmapped count", receipt["artifacts"]["probe_log"]["unmapped"], 0)
        assert_eq("debug unmapped count", receipt["artifacts"]["debug_log"]["unmapped"], 0)
        assert_eq("first semantic failure", receipt["verification"]["first_semantic_runtime_failure"], None)
        assert_eq("bounded smoke claim", receipt["contract"]["claims_bounded_runtime_smoke_survived_after_protocol_detection"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_in_world_gameplay_success", receipt["contract"]["claims_in_world_gameplay_success"], False)

        for fragment in [
            "Detected server protocol version 763",
            "bounded runtime smoke",
            "exit=124",
            "Does not prove semantic packet parser correctness",
            "Does not prove in-world gameplay success",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"gameplay smoke evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-instrumented-login-milestones-evidence =
    pkgs.runCommand "stevenarella-valence-763-instrumented-login-milestones-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot
              + /docs/evidence/stevenarella-valence-763-instrumented-login-milestones-2026-05-23.receipt.json
            }
            note=${
              srcRoot + /docs/evidence/stevenarella-valence-763-instrumented-login-milestones-2026-05-23.md
            }

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-instrumented-login-milestones.receipt.v1")
        assert_eq("status", receipt["status"], "fail_at_first_play_read_after_login_success")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_login_milestones")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "10e4562")
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=101")
        assert_eq("backtrace status", receipt["artifacts"]["backtrace_status"]["content"], "exit=101")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login compression", receipt["artifacts"]["probe_log"]["login_compression"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["login_success"], 1)
        assert_eq("join game", receipt["artifacts"]["probe_log"]["join_game"], 0)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["first_chunk_data"], 0)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["render_tick_with_player"], 0)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 1)
        assert_eq("first failure", receipt["verification"]["first_observed_runtime_failure"], "UnexpectedEof while reading first play-state packet after login_success")
        assert_eq("login success claim", receipt["contract"]["claims_login_success_reached"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_in_world_gameplay_success", receipt["contract"]["claims_in_world_gameplay_success"], False)

        for fragment in [
            "MC-COMPAT-MILESTONE login_success",
            "UnexpectedEof while reading the first play-state packet",
            "Does not prove in-world gameplay success",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"instrumented login evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-post-login-play-state-evidence =
    pkgs.runCommand "stevenarella-valence-763-post-login-play-state-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-post-login-play-state-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-post-login-play-state-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-post-login-play-state.receipt.v1")
        assert_eq("status", receipt["status"], "advanced_past_login_success_to_join_game_render_tick_next_failure_entity_metadata_parser_panic")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_post_login_play_state")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "51d22f2")
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("backtrace status", receipt["artifacts"]["backtrace_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("join game", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game"], 2)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked"], 1)
        assert_eq("first failure", receipt["verification"]["first_observed_runtime_failure"], "panic in EntityMetadata metadata string parser: FromUtf8Error at protocol/src/protocol/mod.rs:281")
        assert_eq("join game claim", receipt["contract"]["claims_join_game_reached"], True)
        assert_eq("render tick claim", receipt["contract"]["claims_render_tick_with_player_reached"], True)
        assert_eq("unexpected eof claim", receipt["contract"]["claims_unexpected_eof_fixed_for_this_probe"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_chunk_streaming_success", receipt["contract"]["claims_chunk_streaming_success"], False)

        for fragment in [
            "MC-COMPAT-MILESTONE join_game_763_shape",
            "MC-COMPAT-MILESTONE render_tick_with_player",
            "UnexpectedEof",
            "EntityMetadata metadata parser panic",
            "Does not prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"post-login evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-entity-metadata-evidence =
    pkgs.runCommand "stevenarella-valence-763-entity-metadata-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-entity-metadata-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-entity-metadata.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_probe_timeout_after_entity_metadata_parser_fix")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_entity_metadata")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "b2a6358")
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
        assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
        assert_eq("entity metadata claim", receipt["contract"]["claims_entity_metadata_boundary_cleared_for_this_probe"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

        for fragment in [
            "EntityMetadata",
            "MC-COMPAT-MILESTONE first_chunk_data",
            "MC-COMPAT-MILESTONE render_tick_with_player",
            "FromUtf8Error",
            "Does not prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"entity metadata evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-extended-gameplay-evidence =
    pkgs.runCommand "stevenarella-valence-763-extended-gameplay-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-extended-gameplay-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-extended-gameplay-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-extended-gameplay.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_300s_timeout_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_extended_gameplay")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "b2a6358")
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
        assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
        assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
        assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

        for fragment in [
            "bounded 300-second Valence `ctf` probe",
            "MC-COMPAT-MILESTONE first_chunk_data",
            "MC-COMPAT-MILESTONE render_tick_with_player",
            "No next concrete runtime boundary appeared",
            "Does not prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"extended gameplay evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-active-gameplay-evidence =
    pkgs.runCommand "stevenarella-valence-763-active-gameplay-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-active-gameplay-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-active-gameplay-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-active-gameplay.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_180s_timeout_active_movement_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_active_gameplay")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "05a382b")
        assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
        assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("active start", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_start"], 1)
        assert_eq("active jump release", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_jump_release"], 1)
        assert_eq("active turn", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_turn"], 1)
        assert_eq("active stop", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_input_stop"], 1)
        assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
        assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
        assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
        assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
        assert_eq("movement packet claim", receipt["contract"]["claims_active_movement_packet_sent"], True)
        assert_eq("contract.claims_team_selection", receipt["contract"]["claims_team_selection"], False)
        assert_eq("contract.claims_combat_semantics", receipt["contract"]["claims_combat_semantics"], False)
        assert_eq("contract.claims_inventory_semantics", receipt["contract"]["claims_inventory_semantics"], False)
        assert_eq("contract.claims_death_respawn_semantics", receipt["contract"]["claims_death_respawn_semantics"], False)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

        for fragment in [
            "bounded 180-second Valence `ctf` probe",
            "MC_COMPAT_ACTIVE_PROBE=1",
            "MC-COMPAT-MILESTONE active_probe_position_look_sent",
            "No `UnexpectedEof`, `FromUtf8Error`, parser panic",
            "does not prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"active gameplay evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-team-interaction-evidence =
    pkgs.runCommand "stevenarella-valence-763-team-interaction-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-team-interaction-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-team-interaction-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-team-interaction.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_180s_timeout_team_interaction_probe_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_instrumented_stevenarella_valence_ctf_team_interaction")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "ca62c2c")
        assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
        assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
        assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
        assert_eq("team enter", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_enter_red_portal"], 1)
        assert_eq("team hold", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_hold_red_portal"], 1)
        assert_eq("hotbar", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_select_hotbar_slot"], 1)
        assert_eq("use item", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_use_item_sent"], 1)
        assert_eq("team chat not observed", receipt["artifacts"]["probe_log"]["You are on team"], 0)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
        assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
        assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
        assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
        assert_eq("team packet claim", receipt["contract"]["claims_team_probe_packets_sent"], True)
        assert_eq("contract.claims_team_selection", receipt["contract"]["claims_team_selection"], False)
        assert_eq("contract.claims_team_selection_chat_observed", receipt["contract"]["claims_team_selection_chat_observed"], False)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_semantic_correctness", receipt["contract"]["claims_semantic_correctness"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

        for fragment in [
            "bounded 180-second Valence `ctf` probe",
            "MC_COMPAT_TEAM_PROBE=1",
            "MC-COMPAT-MILESTONE team_probe_use_item_sent",
            "`You are on team`: `0`",
            "does not prove team selection semantics",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"team interaction evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-team-selection-evidence =
    pkgs.runCommand "stevenarella-valence-763-team-selection-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-team-selection-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-team-selection-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-team-selection.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_180s_timeout_team_selection_chat_observed_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_stevenarella_valence_ctf_team_selection_semantic_probe")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("valence.committed_changes", receipt["valence"]["committed_changes"], False)
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "4c891eb")
        assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
        assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
        assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
        assert_eq("probe status", receipt["artifacts"]["probe_status"]["content"], "exit=124")
        assert_eq("probe protocol", receipt["artifacts"]["probe_log"]["Detected server protocol version 763"], 1)
        assert_eq("login success", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE login_success"], 1)
        assert_eq("join game shape", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE join_game_763_shape"], 1)
        assert_eq("first chunk", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE first_chunk_data"], 1)
        assert_eq("render tick", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE render_tick_with_player"], 1)
        assert_eq("position look sent", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE active_probe_position_look_sent"], 1)
        assert_eq("team enter", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_enter_red_portal"], 1)
        assert_eq("team hold", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_hold_red_portal"], 1)
        assert_eq("hotbar", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_select_hotbar_slot"], 1)
        assert_eq("use item", receipt["artifacts"]["probe_log"]["MC-COMPAT-MILESTONE team_probe_use_item_sent"], 1)
        assert_eq("team chat observed", receipt["artifacts"]["probe_log"]["You are on team"], 1)
        assert_eq("red team chat observed", receipt["artifacts"]["probe_log"]["You are on team RED"], 1)
        assert_eq("unexpected eof", receipt["artifacts"]["probe_log"]["UnexpectedEof"], 0)
        assert_eq("from utf8", receipt["artifacts"]["probe_log"]["FromUtf8Error"], 0)
        assert_eq("panic count", receipt["artifacts"]["probe_log"]["panicked at"], 0)
        assert_eq("parse failure", receipt["artifacts"]["probe_log"]["failed to parse packet"], 0)
        assert_eq("short read", receipt["artifacts"]["probe_log"]["Failed to read all of packet"], 0)
        assert_eq("bad packet id", receipt["artifacts"]["probe_log"]["bad packet id"], 0)
        assert_eq("disconnect", receipt["artifacts"]["probe_log"]["Disconnect"], 0)
        assert_eq("contract.claims_team_selection_chat_observed", receipt["contract"]["claims_team_selection_chat_observed"], True)
        assert_eq("no runtime failure claim", receipt["contract"]["claims_no_logged_runtime_failure_in_this_probe"], True)
        assert_eq("team packet claim", receipt["contract"]["claims_team_probe_packets_sent"], True)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)

        for fragment in [
            "bounded 180-second headless Stevenarella probe",
            "MC_COMPAT_TEAM_PROBE=1",
            "You are on team RED!",
            "does **not** prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"team selection evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-combat-death-evidence =
    pkgs.runCommand "stevenarella-valence-763-combat-death-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-combat-death-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-combat-death-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-combat-death.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_180s_two_client_combat_probe_damage_and_death_health_observed_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_stevenarella_valence_ctf_combat_death_semantic_probe")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("valence.committed_changes", receipt["valence"]["committed_changes"], False)
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "2804c81")
        assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
        assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
        assert_eq("combat probe env", receipt["stevenarella"]["combat_probe_env"], "MC_COMPAT_COMBAT_PROBE=1")
        assert_eq("probe duration", receipt["probe"]["duration_seconds"], 180)
        assert_eq("red probe status", receipt["artifacts"]["red_probe_status"]["content"], "exit=124")
        assert_eq("blue probe status", receipt["artifacts"]["blue_probe_status"]["content"], "exit=124")
        combined = receipt["artifacts"]["combined_probe_log"]
        assert_eq("protocol detections", combined["Detected server protocol version 763"], 2)
        assert_eq("login successes", combined["MC-COMPAT-MILESTONE login_success"], 2)
        assert_eq("join game shapes", combined["MC-COMPAT-MILESTONE join_game_763_shape"], 2)
        assert_eq("first chunks", combined["MC-COMPAT-MILESTONE first_chunk_data"], 2)
        assert_eq("render ticks", combined["MC-COMPAT-MILESTONE render_tick_with_player"], 2)
        assert_eq("red team chat observed", combined["You are on team RED"], 1)
        assert_eq("blue team chat observed", combined["You are on team BLUE"], 1)
        assert_eq("combat move", combined["MC-COMPAT-MILESTONE combat_probe_move_near_blue_spawn"], 1)
        assert_eq("attacks", combined["MC-COMPAT-MILESTONE combat_probe_attack_sent"], 6)
        assert_eq("health 16", combined["update_health health=16.0"], 1)
        assert_eq("health 12", combined["update_health health=12.0"], 1)
        assert_eq("health 8", combined["update_health health=8.0"], 1)
        assert_eq("health 4", combined["update_health health=4.0"], 1)
        assert_eq("health 0", combined["update_health health=0.0"], 1)
        assert_eq("death observed", combined["combat_probe_death_observed"], 2)
        assert_eq("death message observed", combined["combat_probe_death_message"], 0)
        assert_eq("unexpected eof", combined["UnexpectedEof"], 0)
        assert_eq("from utf8", combined["FromUtf8Error"], 0)
        assert_eq("panic count", combined["panicked at"], 0)
        assert_eq("parse failure", combined["failed to parse packet"], 0)
        assert_eq("short read", combined["Failed to read all of packet"], 0)
        assert_eq("bad packet id", combined["bad packet id"], 0)
        assert_eq("disconnect", combined["Disconnect"], 0)
        assert_eq("claims both teams", receipt["contract"]["claims_both_teams_selected"], True)
        assert_eq("claims attacks", receipt["contract"]["claims_attack_packets_sent"], True)
        assert_eq("claims health decreased", receipt["contract"]["claims_victim_health_decreased"], True)
        assert_eq("claims death health", receipt["contract"]["claims_victim_death_health_observed"], True)
        assert_eq("no death message claim", receipt["contract"]["claims_death_message_observed"], False)
        assert_eq("no respawn claim", receipt["contract"]["claims_respawn_observed"], False)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)
        assert_eq("contract.claims_full_combat_correctness", receipt["contract"]["claims_full_combat_correctness"], False)

        for fragment in [
            "Bounded two-client headless Stevenarella probe",
            "MC_COMPAT_COMBAT_PROBE=1",
            "You are on team BLUE",
            "You are on team RED",
            "update_health health=0.0",
            "does **not** prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"combat/death evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-respawn-evidence =
    pkgs.runCommand "stevenarella-valence-763-respawn-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/stevenarella-valence-763-respawn-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-respawn-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.stevenarella-valence-763-respawn.receipt.v1")
        assert_eq("status", receipt["status"], "bounded_two_client_respawn_probe_request_health_restored_respawn_packet_observed_no_logged_runtime_failure")
        assert_eq("mode", receipt["mode"], "protocol_763_stevenarella_valence_ctf_respawn_semantic_probe")
        assert_eq("dry_run", receipt["dry_run"], False)
        assert_eq("valence.protocol", receipt["valence"]["protocol"], 763)
        assert_eq("valence.example", receipt["valence"]["example"], "ctf")
        assert_eq("valence.committed_changes", receipt["valence"]["committed_changes"], False)
        assert_eq("stevenarella.commit", receipt["stevenarella"]["commit"], "559c7a6")
        assert_eq("active probe env", receipt["stevenarella"]["active_probe_env"], "MC_COMPAT_ACTIVE_PROBE=1")
        assert_eq("team probe env", receipt["stevenarella"]["team_probe_env"], "MC_COMPAT_TEAM_PROBE=1")
        assert_eq("combat probe env", receipt["stevenarella"]["combat_probe_env"], "MC_COMPAT_COMBAT_PROBE=1")
        assert_eq("respawn probe env", receipt["stevenarella"]["respawn_probe_env"], "MC_COMPAT_RESPAWN_PROBE=1")
        assert_eq("probe duration", receipt["probe"]["duration_seconds"], 360)
        assert_eq("red probe status", receipt["artifacts"]["red_probe_status"]["content"], "exit=124")
        assert_eq("blue probe status", receipt["artifacts"]["blue_probe_status"]["content"], "exit=124")
        combined = receipt["artifacts"]["combined_probe_log"]
        assert_eq("protocol detections", combined["Detected server protocol version 763"], 2)
        assert_eq("login successes", combined["MC-COMPAT-MILESTONE login_success"], 2)
        assert_eq("join game shapes", combined["MC-COMPAT-MILESTONE join_game_763_shape"], 2)
        assert_eq("first chunks", combined["MC-COMPAT-MILESTONE first_chunk_data"], 2)
        assert_eq("render ticks", combined["MC-COMPAT-MILESTONE render_tick_with_player"], 2)
        assert_eq("blue team chat observed", combined["You are on team BLUE"], 1)
        assert_eq("red team chat observed", combined["You are on team RED"], 1)
        assert_eq("attacks", combined["MC-COMPAT-MILESTONE combat_probe_attack_sent"], 10)
        assert_eq("health 16", combined["update_health health=16.0"], 2)
        assert_eq("health 12", combined["update_health health=12.0"], 2)
        assert_eq("health 8", combined["update_health health=8.0"], 1)
        assert_eq("health 4", combined["update_health health=4.0"], 1)
        assert_eq("health 0", combined["update_health health=0.0"], 1)
        assert_eq("death observed", combined["combat_probe_death_observed"], 1)
        assert_eq("respawn request", combined["respawn_probe_request_sent"], 1)
        assert_eq("respawn health restored", combined["respawn_probe_health_restored health=20.0"], 1)
        assert_eq("respawn packet", combined["respawn_packet_763_shape"], 1)
        assert_eq("unexpected eof", combined["UnexpectedEof"], 0)
        assert_eq("from utf8", combined["FromUtf8Error"], 0)
        assert_eq("panic count", combined["panicked at"], 0)
        assert_eq("parse failure", combined["failed to parse packet"], 0)
        assert_eq("short read", combined["Failed to read all of packet"], 0)
        assert_eq("bad packet id", combined["bad packet id"], 0)
        assert_eq("disconnect", combined["Disconnect"], 0)
        assert_eq("claims respawn request", receipt["contract"]["claims_respawn_request_sent"], True)
        assert_eq("claims respawn health", receipt["contract"]["claims_respawn_health_restored"], True)
        assert_eq("claims respawn packet", receipt["contract"]["claims_respawn_packet_763_shape_observed"], True)
        assert_eq("no death message claim", receipt["contract"]["claims_death_message_observed"], False)
        assert_eq("contract.claims_current_valence_client_compat", receipt["contract"]["claims_current_valence_client_compat"], False)
        assert_eq("contract.claims_full_stevenarella_763_support", receipt["contract"]["claims_full_stevenarella_763_support"], False)
        assert_eq("contract.claims_stable_in_world_gameplay", receipt["contract"]["claims_stable_in_world_gameplay"], False)
        assert_eq("contract.claims_full_combat_correctness", receipt["contract"]["claims_full_combat_correctness"], False)

        for fragment in [
            "Bounded two-client headless Stevenarella probe",
            "MC_COMPAT_RESPAWN_PROBE=1",
            "respawn_probe_request_sent action_id=0",
            "respawn_probe_health_restored health=20.0",
            "respawn_packet_763_shape",
            "does **not** prove full Minecraft 1.20.1 compatibility",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"respawn evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-inventory-evidence =
    pkgs.runCommand "stevenarella-valence-763-inventory-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/stevenarella-valence-763-inventory-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-inventory-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.evidence.v1")
        assert_eq("name", receipt["name"], "stevenarella-valence-763-inventory")
        assert_eq("result", receipt["result"], "bounded_single_client_ctf_team_inventory_probe_observed_slots_36_37_and_hotbar_no_logged_runtime_failure")
        assert_eq("stevenarella commit", receipt["stevenarella_commit"], "73d6d4b stevenarella: add 763 ctf inventory probe")
        assert_eq("valence commit", receipt["valence_commit"], "c5140b7 valence: add parkour smoke receipts")
        assert_eq("timeout status", receipt["probe"]["timeout_status"], "124")
        assert_eq("bounded timeout", receipt["probe"]["bounded_timeout_is_expected"], True)
        markers = receipt["observations"]["markers"]
        for marker in [
            "detected_763",
            "login_success",
            "join_game",
            "render",
            "team_red",
            "set_slot36",
            "set_slot37",
            "current_hotbar",
        ]:
            assert_eq(f"marker {marker}", markers[marker], True)
        assert_eq("slot36", receipt["observations"]["slot36_nonempty_events"], [["1", "777"]])
        assert_eq("slot37", receipt["observations"]["slot37_stack_events"], [["64", "194"]])
        failures = receipt["observations"]["failure_marker_counts"]
        for marker in ["UnexpectedEof", "FromUtf8Error", "failed to read packet", "Bad packet", "panic", "disconnect"]:
            assert_eq(f"failure marker {marker}", failures[marker], 0)
        claims = receipt["claims"]
        assert_eq("bounded claim", claims["bounded_valence_ctf_team_inventory_slots_observed"], True)
        for claim in [
            "claims_full_inventory_semantics",
            "claims_inventory_click_semantics",
            "claims_item_pickup_or_drop_semantics",
            "claims_flag_capture_or_score_semantics",
            "claims_full_minecraft_1_20_1_compatibility",
            "claims_complete_protocol_763_coverage",
            "claims_stable_gameplay_or_long_soak",
        ]:
            assert_eq(claim, claims[claim], False)

        for fragment in [
            "MC_COMPAT_INVENTORY_PROBE=1",
            "Received chat message: You are on team RED!",
            "inventory_probe_current_hotbar_slot slot=0",
            "inventory_probe_set_slot window=0 state_id=1 slot=36 item=id=777 count=1",
            "inventory_probe_set_slot window=0 state_id=1 slot=37 item=id=194 count=64",
            "This evidence does **not** prove",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"inventory evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-flag-score-evidence =
    pkgs.runCommand "stevenarella-valence-763-flag-score-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${srcRoot + /docs/evidence/stevenarella-valence-763-flag-score-2026-05-23.receipt.json}
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-flag-score-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.evidence.v1")
        assert_eq("name", receipt["name"], "stevenarella-valence-763-flag-score")
        assert_eq("result", receipt["result"], "bounded_single_client_ctf_flag_pickup_capture_score_probe_observed_red_1_blue_0_no_logged_runtime_failure")
        assert_eq("stevenarella commit", receipt["stevenarella_commit"], "656743f stevenarella: add 763 ctf flag probe")
        assert_eq("valence commit", receipt["valence_commit"], "c5140b7 valence: add parkour smoke receipts")
        assert_eq("timeout status", receipt["probe"]["timeout_status"], "124")
        assert_eq("bounded timeout", receipt["probe"]["bounded_timeout_is_expected"], True)
        markers = receipt["observations"]["markers"]
        for marker in [
            "detected_763",
            "login_success",
            "join_game",
            "render",
            "team_red",
            "move_to_blue_flag",
            "dig_blue_flag",
            "have_flag_chat",
            "move_to_red_capture",
            "capture_chat",
            "score_chat",
        ]:
            assert_eq(f"marker {marker}", markers[marker], True)
        assert_eq("score block observed", receipt["observations"]["score_block_observed"], True)
        assert_eq("score block", receipt["observations"]["score_block"], "Scores:\nRED: 1\nBLUE: 0")
        failures = receipt["observations"]["failure_marker_counts"]
        for marker in ["UnexpectedEof", "FromUtf8Error", "failed to read packet", "Bad packet", "panic", "disconnect"]:
            assert_eq(f"failure marker {marker}", failures[marker], 0)
        claims = receipt["claims"]
        assert_eq("bounded claim", claims["bounded_valence_ctf_flag_pickup_capture_score_observed"], True)
        for claim in [
            "claims_full_ctf_semantics",
            "claims_repeatable_scoring_under_load",
            "claims_full_combat_or_inventory_semantics",
            "claims_reconnect_or_soak_stability",
            "claims_full_minecraft_1_20_1_compatibility",
            "claims_complete_protocol_763_coverage",
            "claims_stable_gameplay_or_long_soak",
        ]:
            assert_eq(claim, claims[claim], False)

        for fragment in [
            "MC_COMPAT_FLAG_PROBE=1",
            "Received chat message: You are on team RED!",
            "flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=1",
            "Received chat message: You have the flag!",
            "Received chat message: You captured the flag!",
            "RED: 1",
            "BLUE: 0",
            "This evidence does **not** prove",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"flag-score evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-repeat-flag-score-evidence =
    pkgs.runCommand "stevenarella-valence-763-repeat-flag-score-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-repeat-flag-score-2026-05-23.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-repeat-flag-score-2026-05-23.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.evidence.v1")
        assert_eq("name", receipt["name"], "stevenarella-valence-763-repeat-flag-score")
        assert_eq("result", receipt["result"], "bounded_single_client_ctf_two_flag_capture_score_events_observed_no_logged_runtime_failure")
        assert_eq("stevenarella commit", receipt["stevenarella_commit"], "6be4515 stevenarella: count repeated ctf score events")
        assert_eq("valence commit", receipt["valence_commit"], "c5140b7 valence: add parkour smoke receipts")
        assert_eq("timeout status", receipt["probe"]["timeout_status"], "124")
        assert_eq("bounded timeout", receipt["probe"]["bounded_timeout_is_expected"], True)
        assert_eq("repeat env", receipt["probe"]["environment"]["MC_COMPAT_FLAG_PROBE_REPEAT"], "2")
        markers = receipt["observations"]["markers"]
        for marker in [
            "detected_763",
            "login_success",
            "join_game",
            "render",
            "team_red",
            "cycle_1_move_to_blue_flag",
            "cycle_1_dig_blue_flag",
            "cycle_1_have_flag",
            "cycle_1_capture",
            "cycle_1_score",
            "cycle_2_move_to_blue_flag",
            "cycle_2_dig_blue_flag",
            "cycle_2_have_flag",
            "cycle_2_capture",
            "cycle_2_score",
            "repeat_target_reached",
        ]:
            assert_eq(f"marker {marker}", markers[marker], True)
        assert_eq("have count", receipt["observations"]["flag_have_count"], 2)
        assert_eq("capture count", receipt["observations"]["flag_capture_count"], 2)
        assert_eq("score count", receipt["observations"]["flag_score_count"], 2)
        assert_eq("score block 1", receipt["observations"]["score_blocks"][0], "Scores:\nRED: 1\nBLUE: 0")
        assert_eq("score block 2", receipt["observations"]["score_blocks"][1], "Scores:\nRED: 2\nBLUE: 0")
        failures = receipt["observations"]["failure_marker_counts"]
        for marker in ["UnexpectedEof", "FromUtf8Error", "failed to read packet", "Bad packet", "panic", "disconnect"]:
            assert_eq(f"failure marker {marker}", failures[marker], 0)
        claims = receipt["claims"]
        assert_eq("bounded claim", claims["bounded_valence_ctf_two_flag_capture_score_events_observed"], True)
        for claim in [
            "claims_repeatable_scoring_under_load",
            "claims_full_ctf_semantics",
            "claims_full_combat_or_inventory_semantics",
            "claims_reconnect_or_soak_stability",
            "claims_stable_gameplay_or_long_soak",
            "claims_full_minecraft_1_20_1_compatibility",
            "claims_complete_protocol_763_coverage",
        ]:
            assert_eq(claim, claims[claim], False)

        for fragment in [
            "MC_COMPAT_FLAG_PROBE_REPEAT=2",
            "Received chat message: You are on team RED!",
            "flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=1 cycle=1",
            "flag_probe_dig_blue_flag_sent status=stop_destroy location=46,67,0 sequence=2 cycle=2",
            "flag_probe_score_chat count=2 target=2",
            "flag_probe_repeat_target_reached count=2 target=2",
            "RED: 2",
            "BLUE: 0",
            "This evidence does **not** prove",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"repeat flag-score evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-reconnect-flag-score-evidence =
    pkgs.runCommand "stevenarella-valence-763-reconnect-flag-score-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-reconnect-flag-score-2026-05-24.receipt.json
            }
            note=${srcRoot + /docs/evidence/stevenarella-valence-763-reconnect-flag-score-2026-05-24.md}

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.evidence.v1")
        assert_eq("name", receipt["name"], "stevenarella-valence-763-reconnect-flag-score")
        assert_eq("result", receipt["result"], "bounded_same_username_two_session_reconnect_ctf_flag_score_observed_no_logged_runtime_failure")
        assert_eq("stevenarella commit", receipt["stevenarella_commit"], "6be4515 stevenarella: count repeated ctf score events")
        assert_eq("valence commit", receipt["valence_commit"], "c5140b7 valence: add parkour smoke receipts")
        probe = receipt["probe"]
        assert_eq("same username", probe["same_username"], "steve763rejoin")
        assert_eq("inter-session gap", probe["inter_session_gap_seconds"], 30)
        assert_eq("bounded timeout seconds", probe["bounded_timeout_seconds_per_session"], 150)
        assert_eq("bounded timeout expected", probe["bounded_timeout_is_expected"], True)
        assert_eq("flag probe env", probe["environment"]["MC_COMPAT_FLAG_PROBE"], "1")
        assert_eq("repeat env", probe["environment"]["MC_COMPAT_FLAG_PROBE_REPEAT"], "1")
        observations = receipt["observations"]
        assert_eq("all login join render team", observations["all_sessions_reached_login_join_render_team"], True)
        assert_eq("all sessions scored", observations["all_sessions_reached_score"], True)
        assert_eq("all failures zero", observations["all_failure_marker_counts_zero"], True)
        sessions = observations["sessions"]
        assert_eq("session count", len(sessions), 2)
        assert_eq("first label", sessions[0]["label"], "first")
        assert_eq("second label", sessions[1]["label"], "second")
        for session in sessions:
            assert_eq(f"{session['label']} status", session["timeout_status"], "124")
            markers = session["markers"]
            for marker in [
                "detected_763",
                "login_success",
                "join_game",
                "render",
                "team_red",
                "move_to_blue_flag",
                "dig_blue_flag",
                "have_flag",
                "move_to_red_capture",
                "capture",
                "score",
                "target_reached",
            ]:
                assert_eq(f"{session['label']} marker {marker}", markers[marker], True)
            failures = session["failure_marker_counts"]
            for marker in ["UnexpectedEof", "FromUtf8Error", "failed to read packet", "Bad packet", "panic", "disconnect"]:
                assert_eq(f"{session['label']} failure marker {marker}", failures[marker], 0)
        claims = receipt["claims"]
        assert_eq("reconnect claim", claims["bounded_same_username_reconnect_after_disconnect_can_login_join_select_red_and_score"], True)
        for claim in [
            "claims_long_soak_stability",
            "claims_full_ctf_semantics",
            "claims_full_minecraft_1_20_1_compatibility",
            "claims_complete_protocol_763_coverage",
        ]:
            assert_eq(claim, claims[claim], False)

        for fragment in [
            "same Stevenarella username (`steve763rejoin`)",
            "MC_COMPAT_FLAG_PROBE=1",
            "MC_COMPAT_FLAG_PROBE_REPEAT=1",
            "Inter-session gap: `30s`",
            "Received chat message: You are on team RED!",
            "MC-COMPAT-MILESTONE flag_probe_score_chat",
            "Bounded same-username reconnect/session restart",
            "What this does not prove",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"reconnect flag-score evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  stevenarella-valence-763-reconnect-post-score-soak-evidence =
    pkgs.runCommand "stevenarella-valence-763-reconnect-post-score-soak-evidence"
      {
        nativeBuildInputs = [
          pkgs.b3sum
          pkgs.python3
        ];
      }
      ''
            receipt=${
              srcRoot + /docs/evidence/stevenarella-valence-763-reconnect-post-score-soak-2026-05-24.receipt.json
            }
            note=${
              srcRoot + /docs/evidence/stevenarella-valence-763-reconnect-post-score-soak-2026-05-24.md
            }

            python3 - "$receipt" "$note" <<'PY'
        import json
        import pathlib
        import sys

        receipt_path = pathlib.Path(sys.argv[1])
        note_path = pathlib.Path(sys.argv[2])
        receipt = json.loads(receipt_path.read_text())
        note = note_path.read_text()

        def assert_eq(name, actual, expected):
            if actual != expected:
                raise SystemExit(f"{name}: expected {expected!r}, got {actual!r}")

        assert_eq("schema", receipt["schema"], "mc.compat.evidence.v1")
        assert_eq("name", receipt["name"], "stevenarella-valence-763-reconnect-post-score-soak")
        assert_eq("result", receipt["result"], "bounded_same_username_reconnect_then_600s_second_session_after_score_probe_no_logged_runtime_failure")
        assert_eq("parent before evidence", receipt["parent_commit_before_evidence"], "5cf5ee2 mc: record 763 reconnect flag scoring probe")
        assert_eq("stevenarella commit", receipt["stevenarella_commit"], "6be4515 stevenarella: count repeated ctf score events")
        assert_eq("valence commit", receipt["valence_commit"], "c5140b7 valence: add parkour smoke receipts")
        probe = receipt["probe"]
        assert_eq("same username", probe["same_username"], "steve763soak")
        assert_eq("inter-session gap", probe["inter_session_gap_seconds"], 30)
        assert_eq("bounded timeout expected", probe["bounded_timeout_is_expected"], True)
        assert_eq("flag probe env", probe["environment"]["MC_COMPAT_FLAG_PROBE"], "1")
        assert_eq("repeat env", probe["environment"]["MC_COMPAT_FLAG_PROBE_REPEAT"], "1")
        observations = receipt["observations"]
        assert_eq("all login join render team", observations["all_sessions_reached_login_join_render_team"], True)
        assert_eq("all sessions scored before timeout", observations["all_sessions_reached_score_before_timeout"], True)
        assert_eq("second session timeout", observations["second_session_timeout_seconds"], 600)
        assert_eq("second session scored before 600s timeout", observations["second_session_reached_score_before_600s_timeout"], True)
        assert_eq("all failures zero", observations["all_failure_marker_counts_zero"], True)
        sessions = observations["sessions"]
        assert_eq("session count", len(sessions), 2)
        assert_eq("first label", sessions[0]["label"], "first")
        assert_eq("first bound", sessions[0]["bounded_timeout_seconds"], 150)
        assert_eq("second label", sessions[1]["label"], "second-soak")
        assert_eq("second bound", sessions[1]["bounded_timeout_seconds"], 600)
        for session in sessions:
            assert_eq(f"{session['label']} status", session["timeout_status"], "124")
            assert_eq(f"{session['label']} reached score before timeout", session["client_reached_score_before_timeout"], True)
            markers = session["markers"]
            for marker in [
                "detected_763",
                "login_success",
                "join_game",
                "render",
                "active_position",
                "team_red",
                "move_to_blue_flag",
                "dig_blue_flag",
                "have_flag",
                "move_to_red_capture",
                "capture",
                "score",
                "target_reached",
            ]:
                assert_eq(f"{session['label']} marker {marker}", markers[marker], True)
            failures = session["failure_marker_counts"]
            for marker in ["UnexpectedEof", "FromUtf8Error", "failed to read packet", "Bad packet", "panic", "disconnect"]:
                assert_eq(f"{session['label']} failure marker {marker}", failures[marker], 0)
        claims = receipt["claims"]
        assert_eq("bounded soak claim", claims["bounded_reconnect_then_second_session_survived_until_600s_timeout_after_score_marker"], True)
        for claim in [
            "claims_long_soak_stability",
            "claims_full_ctf_semantics",
            "claims_full_minecraft_1_20_1_compatibility",
            "claims_complete_protocol_763_coverage",
        ]:
            assert_eq(claim, claims[claim], False)

        for fragment in [
            "same Stevenarella username (`steve763soak`)",
            "second session reached login/join/render/team/flag-score milestones before its bounded `600s` timeout",
            "MC_COMPAT_FLAG_PROBE=1",
            "MC_COMPAT_FLAG_PROBE_REPEAT=1",
            "Inter-session gap: `30s`",
            "second soak session bound: `600s`",
            "MC-COMPAT-MILESTONE active_probe_position_look_sent",
            "MC-COMPAT-MILESTONE flag_probe_score_chat",
            "What this does not prove",
            "Receipt BLAKE3",
        ]:
            if fragment not in note:
                raise SystemExit(f"reconnect post-score soak evidence note missing fragment: {fragment}")
        PY

            b3=$(b3sum "$receipt" | cut -d' ' -f1)
            grep -Fq "Receipt BLAKE3: \`$b3\`" "$note"
            mkdir -p "$out"
            cp "$receipt" "$note" "$out/"
            printf '%s\n' "$b3" > "$out/receipt.b3"
      '';
  onixresearch-ssh-tools = pkgs.runCommand "onixresearch-ssh-tools" { } ''
    ${cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn --help > cairn-help.log
    ${
      octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet
    }/bin/cargo-octet --help > cargo-octet-help.log
    grep -Fq "usage: cairn" cairn-help.log
    grep -Fq "Octet operator commands" cargo-octet-help.log
    mkdir -p "$out"
    cp cairn-help.log cargo-octet-help.log "$out/"
  '';
  mc-octet-monorepo =
    pkgs.runCommand "mc-octet-monorepo"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        cp -R ${srcRoot} repo
        chmod -R u+w repo
        cd repo
        rustc --edition=2021 tools/check_octet_monorepo.rs -o check-octet-monorepo
        ./check-octet-monorepo --self-test > self-test.log
        ./check-octet-monorepo --root . --octet-source ${octet} > static.log
        grep -Fq "mc-octet-monorepo: PASS" static.log
        mkdir -p "$out"
        cp self-test.log static.log "$out/"
      '';
}
