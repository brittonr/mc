{
  description = "Rust-on-Rust Minecraft compatibility smoke harness for Stevenarella and Valence";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    cairn.url = "git+ssh://git@github.com/onixresearch/cairn.git";
    octet.url = "git+ssh://git@github.com/OnixResearch/octet.git";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      cairn,
      octet,
    }:
    let
      pinnedProjectileDamageValenceRev = "e5d18ad04010d92881267ac1ea43922ae91821f5";
      scenarioWrapperMetadata = import ./compat/config/generated/scenario-wrapper-metadata.nix;
      eachSystem = f: nixpkgs.lib.genAttrs (import systems) (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = eachSystem (
        pkgs:
        import ./nix/packages.nix {
          inherit
            pkgs
            cairn
            octet
            pinnedProjectileDamageValenceRev
            ;
          lib = pkgs.lib;
          srcRoot = ./.;
        }
      );

      apps = eachSystem (
        pkgs:
        import ./nix/apps.nix {
          inherit self pkgs;
        }
      );

      checks = eachSystem (
        pkgs:
        import ./nix/checks.nix {
          inherit
            self
            pkgs
            cairn
            octet
            pinnedProjectileDamageValenceRev
            scenarioWrapperMetadata
            ;
          lib = pkgs.lib;
          srcRoot = ./.;
          baselineOutputInventory = ./docs/evidence/split-root-flake-modules-baseline-output-inventory.json;
        }
      );

      devShells = eachSystem (
        pkgs:
        import ./nix/devshells.nix {
          inherit
            self
            pkgs
            cairn
            octet
            ;
          lib = pkgs.lib;
        }
      );
    };
}
