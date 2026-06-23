use mc_compat_checkers::checkers::inventory_drag_transactions::DragTransactionsChecker;
use mc_compat_checkers::cli::run_key_value_checker;

fn main() {
    run_key_value_checker(&DragTransactionsChecker);
}
