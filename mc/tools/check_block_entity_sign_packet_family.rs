#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-block-entity-sign-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

#[path = "checkers/src/checkers/block_entity_sign_packet_family.rs"]
mod block_entity_sign_packet_family;
#[path = "checkers/src/cli.rs"]
mod cli;
#[allow(dead_code)]
#[path = "checkers/src/key_value.rs"]
mod key_value;

fn main() {
    cli::run_key_value_checker(
        &block_entity_sign_packet_family::BlockEntitySignPacketFamilyChecker,
    );
}
