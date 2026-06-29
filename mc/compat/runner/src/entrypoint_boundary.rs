const MAIN_SIGNATURE: &str = "fn main() -> std::process::ExitCode";
const RUNNER_DELEGATE_CALL: &str = "mc_compat_runner::run_main()";
const MODULE_DECLARATION_PREFIX: &str = "mod ";
const CONST_DECLARATION_PREFIX: &str = "const ";
const STRUCT_DECLARATION_PREFIX: &str = "struct ";
const ENUM_DECLARATION_PREFIX: &str = "enum ";
const IMPL_DECLARATION_PREFIX: &str = "impl ";
const MATCH_EXPRESSION_NEEDLE: &str = "match ";
const PROCESS_COMMAND_NEEDLE: &str = "Command::new";
const FILESYSTEM_NEEDLE: &str = "std::fs";
const ENVIRONMENT_NEEDLE: &str = "std::env";
const PRINT_STDOUT_NEEDLE: &str = "println!";
const PRINT_STDERR_NEEDLE: &str = "eprintln!";
const MAX_ENTRYPOINT_NONEMPTY_LINES: usize = 5;
const EXPECTED_DELEGATE_CALLS: usize = 1;

fn entrypoint_boundary_diagnostics(source: &str) -> Vec<String> {
    let mut diagnostics = Vec::new();
    let nonempty_lines: Vec<&str> = source
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if nonempty_lines.len() > MAX_ENTRYPOINT_NONEMPTY_LINES {
        diagnostics.push(format!(
            "entrypoint has {} nonempty lines; expected at most {}",
            nonempty_lines.len(),
            MAX_ENTRYPOINT_NONEMPTY_LINES
        ));
    }

    if !source.contains(MAIN_SIGNATURE) {
        diagnostics.push(format!("entrypoint missing signature `{MAIN_SIGNATURE}`"));
    }

    let delegate_calls = source.matches(RUNNER_DELEGATE_CALL).count();
    if delegate_calls != EXPECTED_DELEGATE_CALLS {
        diagnostics.push(format!(
            "entrypoint delegate call count was {delegate_calls}; expected {EXPECTED_DELEGATE_CALLS}"
        ));
    }

    reject_entrypoint_prefix(
        &mut diagnostics,
        &nonempty_lines,
        MODULE_DECLARATION_PREFIX,
        "module wiring belongs in the library crate root",
    );
    reject_entrypoint_prefix(
        &mut diagnostics,
        &nonempty_lines,
        CONST_DECLARATION_PREFIX,
        "policy constants belong in focused modules",
    );
    reject_entrypoint_prefix(
        &mut diagnostics,
        &nonempty_lines,
        STRUCT_DECLARATION_PREFIX,
        "data types belong in focused modules",
    );
    reject_entrypoint_prefix(
        &mut diagnostics,
        &nonempty_lines,
        ENUM_DECLARATION_PREFIX,
        "mode/backend enums belong in focused modules",
    );
    reject_entrypoint_prefix(
        &mut diagnostics,
        &nonempty_lines,
        IMPL_DECLARATION_PREFIX,
        "behavior implementations belong in focused modules",
    );

    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        MATCH_EXPRESSION_NEEDLE,
        "entrypoint must not own process-exit policy branches",
    );
    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        PROCESS_COMMAND_NEEDLE,
        "entrypoint must not spawn processes",
    );
    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        FILESYSTEM_NEEDLE,
        "entrypoint must not perform filesystem work",
    );
    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        ENVIRONMENT_NEEDLE,
        "entrypoint must not read environment variables",
    );
    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        PRINT_STDOUT_NEEDLE,
        "entrypoint must not write stdout",
    );
    reject_entrypoint_needle(
        &mut diagnostics,
        source,
        PRINT_STDERR_NEEDLE,
        "entrypoint must not write stderr",
    );

    diagnostics
}

fn reject_entrypoint_prefix(
    diagnostics: &mut Vec<String>,
    nonempty_lines: &[&str],
    prefix: &str,
    message: &str,
) {
    if nonempty_lines
        .iter()
        .any(|line| line.trim_start().starts_with(prefix))
    {
        diagnostics.push(message.to_string());
    }
}

fn reject_entrypoint_needle(
    diagnostics: &mut Vec<String>,
    source: &str,
    needle: &str,
    message: &str,
) {
    if source.contains(needle) {
        diagnostics.push(message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OVERGROWN_ENTRYPOINT_FIXTURE: &str = r#"
mod backend_shell;
const DEFAULT_PORT: u16 = 25565;

fn main() -> std::process::ExitCode {
    match std::env::var("MC_COMPAT_MODE") {
        Ok(_) => println!("running"),
        Err(_) => eprintln!("missing"),
    }
    std::process::ExitCode::SUCCESS
}
"#;

    #[test]
    fn current_entrypoint_is_thin_and_delegates_to_runner_library() {
        let diagnostics = entrypoint_boundary_diagnostics(include_str!("main.rs"));

        assert!(
            diagnostics.is_empty(),
            "current entrypoint should satisfy boundary: {diagnostics:?}"
        );
    }

    #[test]
    fn entrypoint_boundary_rejects_policy_logic_and_side_effects() {
        let diagnostics = entrypoint_boundary_diagnostics(OVERGROWN_ENTRYPOINT_FIXTURE);

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic
                    .contains("module wiring belongs in the library crate root")),
            "expected module declaration diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("policy constants")),
            "expected policy constant diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("process-exit policy")),
            "expected branch policy diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("environment variables")),
            "expected environment diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("stdout")),
            "expected stdout diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("stderr")),
            "expected stderr diagnostic: {diagnostics:?}"
        );
    }
}
