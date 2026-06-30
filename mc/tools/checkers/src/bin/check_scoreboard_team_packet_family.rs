use mc_compat_checkers::checkers::scoreboard_team_packet_family::ScoreboardTeamPacketFamilyChecker;
use mc_compat_checkers::cli::run_key_value_checker;

fn main() {
    run_key_value_checker(&ScoreboardTeamPacketFamilyChecker);
}
