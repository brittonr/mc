#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-inventory-drag-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

#[path = "checkers/src/cli.rs"]
mod cli;
#[path = "checkers/src/checkers/inventory_drag_transactions.rs"]
mod inventory_drag_transactions;
#[allow(dead_code)]
#[path = "checkers/src/key_value.rs"]
mod key_value;

fn main() {
    cli::run_key_value_checker(&inventory_drag_transactions::DragTransactionsChecker);
}
