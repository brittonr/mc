use std::env;
use std::fs;
use std::process;

use crate::key_value::{KeyValueChecker, KeyValueRecord};

const SELF_TEST_FLAG: &str = "--self-test";
const SUCCESS_SUFFIX: &str = ": ok";
const SELF_TEST_SUCCESS_MESSAGE: &str = "self-test ok";
const FAILURE_EXIT_CODE: i32 = 1;

fn run_path(checker: &impl KeyValueChecker, path: &str) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|err| format!("read {path}: {err}"))?;
    let evidence = KeyValueRecord::parse(&text)?;
    checker
        .validate(&evidence)
        .map(|()| format!("{path}{SUCCESS_SUFFIX}"))
        .map_err(|diagnostics| diagnostics.join("\n"))
}

fn run_args(checker: &impl KeyValueChecker, args: &[String]) -> Result<String, String> {
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        checker
            .self_test()
            .map(|()| SELF_TEST_SUCCESS_MESSAGE.to_string())
    } else if let Some(path) = args.first() {
        run_path(checker, path)
    } else {
        Err(checker.usage().to_string())
    }
}

pub fn run_key_value_checker(checker: &impl KeyValueChecker) {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match run_args(checker, &args) {
        Ok(message) => println!("{message}"),
        Err(message) => {
            eprintln!("{message}");
            process::exit(FAILURE_EXIT_CODE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_value::ValidationResult;

    const USAGE: &str = "usage: test-checker (--self-test | <evidence.kv>)";

    struct TestChecker;

    impl KeyValueChecker for TestChecker {
        fn usage(&self) -> &'static str {
            USAGE
        }

        fn validate(&self, _evidence: &KeyValueRecord) -> ValidationResult {
            Ok(())
        }

        fn self_test(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn self_test_flag_runs_checker_self_test() {
        let args = vec![SELF_TEST_FLAG.to_string()];
        let message = run_args(&TestChecker, &args).expect("self-test succeeds");
        assert_eq!(message, SELF_TEST_SUCCESS_MESSAGE);
    }

    #[test]
    fn missing_args_return_usage() {
        let args = Vec::new();
        let error = run_args(&TestChecker, &args).unwrap_err();
        assert_eq!(error, USAGE);
    }
}
