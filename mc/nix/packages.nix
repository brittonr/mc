{
  pkgs,
  lib,
  cairn,
  octet,
  pinnedProjectileDamageValenceRev,
  srcRoot,
}:
let
  sharedTools = import ./shared-tools.nix { inherit pkgs lib; };
  inherit (sharedTools)
    nativeTools
    guiLibs
    pkgConfigPath
    runtimePath
    libraryPath
    editableCargoTools
    cmakePolicyVersionMinimum
    softwareGlEnabled
    xvfbAutoEnabled
    ;
  valence = pkgs.writeShellApplication {
    name = "valence";
    runtimeInputs = editableCargoTools;
    text = ''
      missing_checkout_exit=64
      repo="''${VALENCE_REPO:-$PWD/servers/valence}"
      example="''${VALENCE_EXAMPLE:-ctf}"
      target_dir="''${VALENCE_TARGET_DIR:-$PWD/target/nix-run-valence}"

      if [[ "''${1:-}" == "--help" ]]; then
        cat <<'USAGE'
      Run an editable local Valence example with Nix-provided Rust/native dependencies.

      Usage:
        nix run .#valence -- [example args...]
        nix run .#valence -- --dry-run

      Environment:
        VALENCE_REPO       source tree path; default: $PWD/servers/valence
        VALENCE_EXAMPLE    cargo example name; default: ctf
        VALENCE_TARGET_DIR cargo target dir; default: $PWD/target/nix-run-valence
      USAGE
        exit 0
      fi

      if [[ ! -f "$repo/Cargo.toml" ]]; then
        printf 'missing Valence source tree at %s; set VALENCE_REPO or run from mc root\n' "$repo" >&2
        exit "$missing_checkout_exit"
      fi

      export CARGO_TARGET_DIR="$target_dir"
      export PKG_CONFIG_PATH=${lib.escapeShellArg pkgConfigPath}:"''${PKG_CONFIG_PATH:-}"
      export LIBRARY_PATH=${lib.escapeShellArg libraryPath}:"''${LIBRARY_PATH:-}"
      export LD_LIBRARY_PATH=${lib.escapeShellArg libraryPath}:"''${LD_LIBRARY_PATH:-}"
      export OPENSSL_INCLUDE_DIR=${lib.escapeShellArg "${pkgs.openssl.dev}/include"}
      export OPENSSL_LIB_DIR=${lib.escapeShellArg "${pkgs.openssl.out}/lib"}
      export RUSTC_WRAPPER=""

      if [[ "''${1:-}" == "--dry-run" ]]; then
        printf 'repo=%s\nexample=%s\ntarget_dir=%s\n' "$repo" "$example" "$target_dir"
        printf 'cd %q && cargo run --example %q --' "$repo" "$example"
        shift
        for arg in "$@"; do
          printf ' %q' "$arg"
        done
        printf '\n'
        exit 0
      fi

      mkdir -p "$target_dir"
      cd "$repo"
      exec cargo run --example "$example" -- "$@"
    '';
    meta = {
      description = "Run the core Valence server tree through the mc flake dev environment.";
      mainProgram = "valence";
    };
  };
  stevenarella = pkgs.writeShellApplication {
    name = "stevenarella";
    runtimeInputs = editableCargoTools;
    text = ''
      missing_checkout_exit=64
      repo="''${CLIENT_DIR:-$PWD/clients/stevenarella}"
      target_dir="''${CLIENT_TARGET_DIR:-$PWD/target/nix-run-stevenarella}"

      if [[ "''${1:-}" == "--help" ]]; then
        cat <<'USAGE'
      Run an editable local Stevenarella checkout with Nix-provided Rust/native dependencies.

      Usage:
        nix run .#stevenarella -- [client args...]
        nix run .#stevenarella -- --dry-run

      Environment:
        CLIENT_DIR        source tree path; default: $PWD/clients/stevenarella
        CLIENT_TARGET_DIR cargo target dir; default: $PWD/target/nix-run-stevenarella
      USAGE
        exit 0
      fi

      if [[ ! -f "$repo/Cargo.toml" ]]; then
        printf 'missing Stevenarella source tree at %s; set CLIENT_DIR or run from mc root\n' "$repo" >&2
        exit "$missing_checkout_exit"
      fi

      export CARGO_TARGET_DIR="$target_dir"
      export PKG_CONFIG_PATH=${lib.escapeShellArg pkgConfigPath}:"''${PKG_CONFIG_PATH:-}"
      export LIBRARY_PATH=${lib.escapeShellArg libraryPath}:"''${LIBRARY_PATH:-}"
      export LD_LIBRARY_PATH=${lib.escapeShellArg libraryPath}:"''${LD_LIBRARY_PATH:-}"
      export OPENSSL_INCLUDE_DIR=${lib.escapeShellArg "${pkgs.openssl.dev}/include"}
      export OPENSSL_LIB_DIR=${lib.escapeShellArg "${pkgs.openssl.out}/lib"}
      export RUSTC_WRAPPER=""
      export CMAKE_POLICY_VERSION_MINIMUM=${lib.escapeShellArg cmakePolicyVersionMinimum}
      export WINIT_UNIX_BACKEND=x11
      export LIBGL_ALWAYS_SOFTWARE=${lib.escapeShellArg softwareGlEnabled}

      if [[ "''${1:-}" == "--dry-run" ]]; then
        printf 'repo=%s\ntarget_dir=%s\n' "$repo" "$target_dir"
        printf 'cd %q && cargo run --' "$repo"
        shift
        for arg in "$@"; do
          printf ' %q' "$arg"
        done
        printf '\n'
        exit 0
      fi

      mkdir -p "$target_dir"
      cd "$repo"
      if [[ -z "''${DISPLAY:-}" && "''${STEVENARELLA_XVFB:-${xvfbAutoEnabled}}" == ${lib.escapeShellArg xvfbAutoEnabled} ]]; then
        exec xvfb-run -a cargo run -- "$@"
      fi
      exec cargo run -- "$@"
    '';
    meta = {
      description = "Run the core Stevenarella client tree through the mc flake dev environment.";
      mainProgram = "stevenarella";
    };
  };
  mc-compat-runner = pkgs.rustPlatform.buildRustPackage {
    pname = "mc-compat-runner";
    version = "0.1.0";
    src = srcRoot + /compat/runner;
    cargoLock.lockFile = srcRoot + /compat/runner/Cargo.lock;
    nativeBuildInputs = [ pkgs.makeWrapper ];
    postInstall = ''
      wrapProgram "$out/bin/mc-compat-runner" \
        --prefix PATH : ${lib.escapeShellArg runtimePath} \
        --prefix PKG_CONFIG_PATH : ${lib.escapeShellArg pkgConfigPath} \
        --prefix LIBRARY_PATH : ${lib.escapeShellArg libraryPath} \
        --prefix LD_LIBRARY_PATH : ${lib.escapeShellArg libraryPath} \
        --set OPENSSL_INCLUDE_DIR ${lib.escapeShellArg "${pkgs.openssl.dev}/include"} \
        --set OPENSSL_LIB_DIR ${lib.escapeShellArg "${pkgs.openssl.out}/lib"} \
        --set RUSTC_WRAPPER "" \
        --set CMAKE_POLICY_VERSION_MINIMUM ${lib.escapeShellArg cmakePolicyVersionMinimum} \
        --set WINIT_UNIX_BACKEND x11 \
        --set LIBGL_ALWAYS_SOFTWARE ${lib.escapeShellArg softwareGlEnabled}
    '';
    meta = {
      description = "Hardened Stevenarella/Valence compatibility smoke runner";
      mainProgram = "mc-compat-runner";
    };
  };
  mc-compat-checkers = pkgs.rustPlatform.buildRustPackage {
    pname = "mc-compat-checkers";
    version = "0.1.0";
    src = srcRoot + /tools/checkers;
    cargoLock.lockFile = srcRoot + /tools/checkers/Cargo.lock;
    meta = {
      description = "Rust evidence checker binaries for the mc compatibility workspace";
    };
  };
  mc-compat-valence-ctf-600s-soak = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-600s-soak";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SOAK_RECEIPT:-target/mc-compat-soak/multi-client-load-score-600s.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-600}"

      exec mc-compat-runner scenario run multi-client-load-score "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF 600s multi-client soak receipt.";
      mainProgram = "mc-compat-valence-ctf-600s-soak";
    };
  };
  mc-compat-valence-ctf-blue-600s-soak = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-blue-600s-soak";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_BLUE_SOAK_RECEIPT:-target/mc-compat-blue-soak/blue-flag-score-600s.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-600}"

      exec mc-compat-runner scenario run blue-flag-score "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF BLUE-team 600s soak receipt.";
      mainProgram = "mc-compat-valence-ctf-blue-600s-soak";
    };
  };
  mc-compat-valence-ctf-inventory-interaction = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-inventory-interaction";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_INVENTORY_RECEIPT:-target/mc-compat-inventory/inventory-interaction.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run inventory-interaction "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF inventory/drop interaction receipt.";
      mainProgram = "mc-compat-valence-ctf-inventory-interaction";
    };
  };
  mc-compat-valence-inventory-stack-split-merge = pkgs.writeShellApplication {
    name = "mc-compat-valence-inventory-stack-split-merge";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_INVENTORY_STACK_RECEIPT:-target/mc-compat-inventory-stack/inventory-stack-split-merge.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run inventory-stack-split-merge "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF inventory stack split/merge receipt.";
      mainProgram = "mc-compat-valence-inventory-stack-split-merge";
    };
  };
  mc-compat-valence-inventory-drag-transactions = pkgs.writeShellApplication {
    name = "mc-compat-valence-inventory-drag-transactions";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_INVENTORY_DRAG_RECEIPT:-target/mc-compat-inventory-drag/inventory-drag-transactions.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run inventory-drag-transactions "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF inventory drag transaction receipt.";
      mainProgram = "mc-compat-valence-inventory-drag-transactions";
    };
  };
  mc-compat-valence-ctf-combat-damage = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-combat-damage";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_COMBAT_RECEIPT:-target/mc-compat-combat/combat-damage.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run combat-damage "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF combat damage receipt.";
      mainProgram = "mc-compat-valence-ctf-combat-damage";
    };
  };
  mc-compat-valence-ctf-combat-knockback = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-combat-knockback";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_KNOCKBACK_RECEIPT:-target/mc-compat-knockback/combat-knockback.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run combat-knockback "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF combat knockback receipt.";
      mainProgram = "mc-compat-valence-ctf-combat-knockback";
    };
  };
  mc-compat-valence-ctf-armor-equipment-mitigation = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-armor-equipment-mitigation";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_ARMOR_MITIGATION_RECEIPT:-target/mc-compat-armor-mitigation/armor-equipment-mitigation.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"
      export LIBGL_DRIVERS_PATH="''${LIBGL_DRIVERS_PATH:-${pkgs.mesa}/lib/dri}"
      export GBM_BACKENDS_PATH="''${GBM_BACKENDS_PATH:-${pkgs.mesa}/lib/gbm}"
      export __EGL_VENDOR_LIBRARY_DIRS="''${__EGL_VENDOR_LIBRARY_DIRS:-${pkgs.mesa}/share/glvnd/egl_vendor.d}"

      exec mc-compat-runner scenario run armor-equipment-mitigation "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF armor/equipment mitigation receipt.";
      mainProgram = "mc-compat-valence-ctf-armor-equipment-mitigation";
    };
  };
  mc-compat-valence-ctf-equipment-update-observation = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-equipment-update-observation";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_EQUIPMENT_UPDATE_RECEIPT:-target/mc-compat-equipment-update/equipment-update-observation.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"
      export LIBGL_DRIVERS_PATH="''${LIBGL_DRIVERS_PATH:-${pkgs.mesa}/lib/dri}"
      export GBM_BACKENDS_PATH="''${GBM_BACKENDS_PATH:-${pkgs.mesa}/lib/gbm}"
      export __EGL_VENDOR_LIBRARY_DIRS="''${__EGL_VENDOR_LIBRARY_DIRS:-${pkgs.mesa}/share/glvnd/egl_vendor.d}"

      exec mc-compat-runner scenario run equipment-update-observation "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF entity equipment update observation receipt.";
      mainProgram = "mc-compat-valence-ctf-equipment-update-observation";
    };
  };
  mc-compat-valence-ctf-projectile-hit = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-projectile-hit";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_PROJECTILE_HIT_RECEIPT:-target/mc-compat-projectile-hit/projectile-hit.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-240}"
      export MC_COMPAT_PROJECTILE_PROBE="''${MC_COMPAT_PROJECTILE_PROBE:-1}"
      export LIBGL_DRIVERS_PATH="''${LIBGL_DRIVERS_PATH:-${pkgs.mesa}/lib/dri}"
      export GBM_BACKENDS_PATH="''${GBM_BACKENDS_PATH:-${pkgs.mesa}/lib/gbm}"
      export __EGL_VENDOR_LIBRARY_DIRS="''${__EGL_VENDOR_LIBRARY_DIRS:-${pkgs.mesa}/share/glvnd/egl_vendor.d}"

      exec mc-compat-runner scenario run projectile-hit "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF projectile hit receipt.";
      mainProgram = "mc-compat-valence-ctf-projectile-hit";
    };
  };
  mc-compat-valence-ctf-projectile-damage-attribution = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-projectile-damage-attribution";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_PROJECTILE_DAMAGE_RECEIPT:-target/mc-compat-projectile-damage-attribution/projectile-damage-attribution.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-${pinnedProjectileDamageValenceRev}}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-projectile-damage-pinned}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-projectile-damage-pinned-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-240}"
      export MC_COMPAT_PROJECTILE_PROBE="''${MC_COMPAT_PROJECTILE_PROBE:-1}"
      export LIBGL_DRIVERS_PATH="''${LIBGL_DRIVERS_PATH:-${pkgs.mesa}/lib/dri}"
      export GBM_BACKENDS_PATH="''${GBM_BACKENDS_PATH:-${pkgs.mesa}/lib/gbm}"
      export __EGL_VENDOR_LIBRARY_DIRS="''${__EGL_VENDOR_LIBRARY_DIRS:-${pkgs.mesa}/share/glvnd/egl_vendor.d}"

      exec mc-compat-runner scenario run projectile-damage-attribution "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF projectile damage attribution receipt.";
      mainProgram = "mc-compat-valence-ctf-projectile-damage-attribution";
    };
  };
  mc-compat-valence-ctf-flag-carrier-death-return = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-flag-carrier-death-return";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_FLAG_CARRIER_DEATH_RECEIPT:-target/mc-compat-flag-carrier-death/flag-carrier-death-return.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-150}"

      exec mc-compat-runner scenario run flag-carrier-death-return "$mode" \
        --server-backend valence \
        --receipt "$receipt" \
        "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF flag-carrier death/return receipt.";
      mainProgram = "mc-compat-valence-ctf-flag-carrier-death-return";
    };
  };
  mc-compat-valence-ctf-latency-jitter-inventory = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-latency-jitter-inventory";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_LATENCY_JITTER_RECEIPT:-target/mc-compat-latency-jitter/latency-jitter-inventory.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-180}"
      export MC_COMPAT_LATENCY_JITTER_ENABLED=1
      export MC_COMPAT_LATENCY_JITTER_TARGET_RAIL="inventory-interaction"
      export MC_COMPAT_LATENCY_JITTER_MECHANISM="bounded-client-cadence"
      export MC_COMPAT_LATENCY_MS="''${MC_COMPAT_LATENCY_MS:-80}"
      export MC_COMPAT_JITTER_MS="''${MC_COMPAT_JITTER_MS:-30}"
      export MC_COMPAT_LOSS_PERCENT="''${MC_COMPAT_LOSS_PERCENT:-0}"
      export MC_COMPAT_WAN_TARGET_OWNERSHIP="owned-local-loopback"
      export MC_COMPAT_WAN_AUTHORIZATION="owned-local-fixture-approved"

      exec mc-compat-runner scenario run inventory-interaction "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF inventory rail with bounded latency/jitter metadata.";
      mainProgram = "mc-compat-valence-ctf-latency-jitter-inventory";
    };
  };
  mc-compat-valence-ctf-reconnect-flag-state = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-reconnect-flag-state";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_RECONNECT_FLAG_STATE_RECEIPT:-target/mc-compat-reconnect-flag-state/reconnect-flag-state.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run reconnect-flag-state "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF reconnect flag-state receipt.";
      mainProgram = "mc-compat-valence-ctf-reconnect-flag-state";
    };
  };
  mc-compat-valence-ctf-invalid-pickup-ownership = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-invalid-pickup-ownership";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_INVALID_PICKUP_OWNERSHIP_RECEIPT:-target/mc-compat-ctf-invalid-pickup-ownership/ctf-invalid-pickup-ownership.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-invalid-pickup-ownership "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF invalid pickup ownership receipt.";
      mainProgram = "mc-compat-valence-ctf-invalid-pickup-ownership";
    };
  };
  mc-compat-valence-ctf-invalid-return-drop = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-invalid-return-drop";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_INVALID_RETURN_DROP_RECEIPT:-target/mc-compat-ctf-invalid-return-drop/ctf-invalid-return-drop.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-invalid-return-drop "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF invalid return/drop receipt.";
      mainProgram = "mc-compat-valence-ctf-invalid-return-drop";
    };
  };
  mc-compat-valence-ctf-invalid-opponent-base-return-drop = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-invalid-opponent-base-return-drop";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_RECEIPT:-target/mc-compat-ctf-invalid-opponent-base-return-drop/ctf-invalid-opponent-base-return-drop.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-invalid-opponent-base-return-drop "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF invalid opponent-base return/drop receipt.";
      mainProgram = "mc-compat-valence-ctf-invalid-opponent-base-return-drop";
    };
  };
  mc-compat-valence-ctf-score-limit-win-condition = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-score-limit-win-condition";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_SCORE_LIMIT_WIN_CONDITION_RECEIPT:-target/mc-compat-ctf-score-limit-win-condition/ctf-score-limit-win-condition.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-score-limit-win-condition "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF score limit win-condition receipt.";
      mainProgram = "mc-compat-valence-ctf-score-limit-win-condition";
    };
  };
  mc-compat-valence-ctf-simultaneous-pickup-capture-race = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-simultaneous-pickup-capture-race";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_SIMULTANEOUS_PICKUP_CAPTURE_RACE_RECEIPT:-target/mc-compat-ctf-simultaneous-pickup-capture-race/ctf-simultaneous-pickup-capture-race.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-simultaneous-pickup-capture-race "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF simultaneous pickup/capture race receipt.";
      mainProgram = "mc-compat-valence-ctf-simultaneous-pickup-capture-race";
    };
  };
  mc-compat-valence-ctf-spawn-team-balance-reset = pkgs.writeShellApplication {
    name = "mc-compat-valence-ctf-spawn-team-balance-reset";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_CTF_SPAWN_TEAM_BALANCE_RESET_RECEIPT:-target/mc-compat-ctf-spawn-team-balance-reset/ctf-spawn-team-balance-reset.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-spawn-team-balance-reset "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence CTF spawn/team balance reset receipt.";
      mainProgram = "mc-compat-valence-ctf-spawn-team-balance-reset";
    };
  };
  mc-compat-valence-movement-packet-family = pkgs.writeShellApplication {
    name = "mc-compat-valence-movement-packet-family";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_MOVEMENT_PACKET_FAMILY_RECEIPT:-target/mc-compat-movement-packet-family/movement-packet-family.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-ctf}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-763}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-763-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run ctf-spawn-team-balance-reset "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence movement packet-family receipt.";
      mainProgram = "mc-compat-valence-movement-packet-family";
    };
  };
  mc-compat-valence-survival-break-place-pickup = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-break-place-pickup";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_BREAK_PLACE_PICKUP_RECEIPT:-target/mc-compat-survival-break-place-pickup/survival-break-place-pickup.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-break-place-pickup "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival break/place/pickup receipt.";
      mainProgram = "mc-compat-valence-survival-break-place-pickup";
    };
  };
  mc-compat-valence-survival-crafting-table = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-crafting-table";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_CRAFTING_TABLE_RECEIPT:-target/mc-compat-survival-crafting-table/survival-crafting-table.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival-crafting}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-crafting-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-crafting-table "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival crafting-table receipt.";
      mainProgram = "mc-compat-valence-survival-crafting-table";
    };
  };
  mc-compat-valence-survival-crafting-recipe-breadth = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-crafting-recipe-breadth";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_CRAFTING_RECIPE_BREADTH_RECEIPT:-target/mc-compat-survival-crafting-recipe-breadth/survival-crafting-recipe-breadth.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival-crafting-recipe-breadth}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-crafting-recipe-breadth-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-crafting-recipe-breadth "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival crafting recipe breadth receipt.";
      mainProgram = "mc-compat-valence-survival-crafting-recipe-breadth";
    };
  };
  mc-compat-valence-survival-furnace-persistence = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-furnace-persistence";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_FURNACE_RECEIPT:-target/mc-compat-survival-furnace-persistence/survival-furnace-persistence.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival-furnace}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-furnace-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-furnace-persistence "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival furnace persistence receipt.";
      mainProgram = "mc-compat-valence-survival-furnace-persistence";
    };
  };
  mc-compat-valence-survival-furnace-smelting-breadth = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-furnace-smelting-breadth";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_FURNACE_SMELTING_BREADTH_RECEIPT:-target/mc-compat-survival-furnace-smelting-breadth/survival-furnace-smelting-breadth.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival-furnace-smelting-breadth}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-furnace-smelting-breadth-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-furnace-smelting-breadth "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival furnace smelting breadth receipt.";
      mainProgram = "mc-compat-valence-survival-furnace-smelting-breadth";
    };
  };
  mc-compat-valence-survival-hunger-health-cycle = pkgs.writeShellApplication {
    name = "mc-compat-valence-survival-hunger-health-cycle";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_SURVIVAL_HUNGER_HEALTH_CYCLE_RECEIPT:-target/mc-compat-survival-hunger-health-cycle/survival-hunger-health-cycle.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-survival-hunger-health-cycle}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-survival-hunger-health-cycle-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run survival-hunger-health-cycle "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the maintained protocol-763 Valence survival hunger-health cycle receipt.";
      mainProgram = "mc-compat-valence-survival-hunger-health-cycle";
    };
  };
  mc-compat-mcp-controlled-smoke = pkgs.writeShellApplication {
    name = "mc-compat-mcp-controlled-smoke";
    runtimeInputs = [ mc-compat-runner ];
    text = ''
      mode="--dry-run"
      if [[ "''${1:-}" == "--dry-run" || "''${1:-}" == "--run" ]]; then
        mode="$1"
        shift
      fi

      receipt="''${MC_COMPAT_MCP_CONTROLLED_SMOKE_RECEIPT:-target/mc-compat-mcp-controlled-smoke/mcp-controlled-smoke.json}"
      mkdir -p "$(dirname "$receipt")"

      export SERVER_PROTOCOL="''${SERVER_PROTOCOL:-763}"
      export SERVER_VERSION="''${SERVER_VERSION:-1.20.1}"
      export VALENCE_REV="''${VALENCE_REV:-main}"
      export VALENCE_EXAMPLE="''${VALENCE_EXAMPLE:-survival_compat}"
      export VALENCE_WORKTREE="''${VALENCE_WORKTREE:-/tmp/valence-compat-mcp-controlled-smoke}"
      export VALENCE_TARGET_DIR="''${VALENCE_TARGET_DIR:-/tmp/valence-compat-mcp-controlled-smoke-target}"
      export CLIENT_TIMEOUT="''${CLIENT_TIMEOUT:-120}"

      exec mc-compat-runner scenario run mcp-controlled-smoke "$mode"                 --server-backend valence                 --receipt "$receipt"                 "$@"
    '';
    meta = {
      description = "Run the deterministic MCP-controlled Stevenarella receipt dry-run.";
      mainProgram = "mc-compat-mcp-controlled-smoke";
    };
  };
  evidence-manifest-refresh-bin =
    pkgs.runCommand "evidence-manifest-refresh-bin"
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.gcc
        ];
      }
      ''
        mkdir -p "$out/bin"
        rustc --edition=2021 ${
          srcRoot + /tools/refresh_evidence_manifests.rs
        } -o "$out/bin/evidence-manifest-refresh"
      '';
  evidence-manifest-refresh = pkgs.writeShellApplication {
    name = "evidence-manifest-refresh";
    runtimeInputs = [ pkgs.b3sum ];
    text = ''
      exec ${evidence-manifest-refresh-bin}/bin/evidence-manifest-refresh "$@"
    '';
    meta = {
      description = "Check or refresh docs/evidence BLAKE3 manifests to a deterministic fixpoint.";
      mainProgram = "evidence-manifest-refresh";
    };
  };
in
{
  inherit
    valence
    stevenarella
    mc-compat-runner
    mc-compat-checkers
    mc-compat-valence-ctf-600s-soak
    mc-compat-valence-ctf-blue-600s-soak
    mc-compat-valence-ctf-inventory-interaction
    mc-compat-valence-inventory-stack-split-merge
    mc-compat-valence-inventory-drag-transactions
    mc-compat-valence-ctf-combat-damage
    mc-compat-valence-ctf-combat-knockback
    mc-compat-valence-ctf-armor-equipment-mitigation
    mc-compat-valence-ctf-equipment-update-observation
    mc-compat-valence-ctf-projectile-hit
    mc-compat-valence-ctf-projectile-damage-attribution
    mc-compat-valence-ctf-flag-carrier-death-return
    mc-compat-valence-ctf-latency-jitter-inventory
    mc-compat-valence-ctf-reconnect-flag-state
    mc-compat-valence-ctf-invalid-pickup-ownership
    mc-compat-valence-ctf-invalid-return-drop
    mc-compat-valence-ctf-invalid-opponent-base-return-drop
    mc-compat-valence-ctf-score-limit-win-condition
    mc-compat-valence-ctf-simultaneous-pickup-capture-race
    mc-compat-valence-ctf-spawn-team-balance-reset
    mc-compat-valence-movement-packet-family
    mc-compat-valence-survival-break-place-pickup
    mc-compat-valence-survival-crafting-table
    mc-compat-valence-survival-crafting-recipe-breadth
    mc-compat-valence-survival-furnace-persistence
    mc-compat-valence-survival-furnace-smelting-breadth
    mc-compat-valence-survival-hunger-health-cycle
    mc-compat-mcp-controlled-smoke
    evidence-manifest-refresh
    ;
  cairn = cairn.packages.${pkgs.stdenv.hostPlatform.system}.cairn;
  cargo-octet = octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet;
  octet = octet.packages.${pkgs.stdenv.hostPlatform.system}.octet;
  default = mc-compat-runner;
}
