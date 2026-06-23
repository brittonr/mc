use mc_compat_checkers::checkers::inventory_stack_split_merge::StackSplitMergeChecker;
use mc_compat_checkers::cli::run_key_value_checker;

fn main() {
    run_key_value_checker(&StackSplitMergeChecker);
}
