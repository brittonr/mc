#!/usr/bin/env -S CARGO_TARGET_DIR=target/check-scoreboard-team-script nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

#[path = "checkers/src/cli.rs"]
mod cli;
#[allow(dead_code)]
#[path = "checkers/src/key_value.rs"]
mod key_value;
#[path = "checkers/src/checkers/scoreboard_team_packet_family.rs"]
mod scoreboard_team_packet_family;

fn main() {
    cli::run_key_value_checker(&scoreboard_team_packet_family::ScoreboardTeamPacketFamilyChecker);
}
