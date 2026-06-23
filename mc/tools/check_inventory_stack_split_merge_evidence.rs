#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-inventory-stack-split-merge-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

#[path = "checkers/src/cli.rs"]
mod cli;
#[path = "checkers/src/checkers/inventory_stack_split_merge.rs"]
mod inventory_stack_split_merge;
#[allow(dead_code)]
#[path = "checkers/src/key_value.rs"]
mod key_value;

fn main() {
    cli::run_key_value_checker(&inventory_stack_split_merge::StackSplitMergeChecker);
}
