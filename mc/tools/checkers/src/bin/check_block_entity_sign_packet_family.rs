use mc_compat_checkers::checkers::block_entity_sign_packet_family::BlockEntitySignPacketFamilyChecker;
use mc_compat_checkers::cli::run_key_value_checker;

fn main() {
    run_key_value_checker(&BlockEntitySignPacketFamilyChecker);
}
