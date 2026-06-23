#![doc = include_str!("../README.md")]
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    clippy::dbg_macro
)]

use std::fs;
use std::process::ExitCode;
use std::sync::Arc;

use args::StresserArgs;
use clap::Parser;
use config::{validate_config, RawStresserConfig, StresserConfig};
use report::{LoadToolReport, CONFIG_ERROR_EXIT_STATUS};
use stresser::{make_session, SessionFailure, SessionParams};
use tokio::sync::Semaphore;

mod args;
mod config;
mod report;
mod stresser;

#[tokio::main]
async fn main() -> ExitCode {
    let raw_config = RawStresserConfig::from(StresserArgs::parse());
    let report_path = raw_config.report_path.clone();
    let target_for_report = raw_config.target_host.clone();
    let scenario_for_report = raw_config.scenario.clone();

    let config = match validate_config(raw_config) {
        Ok(config) => config,
        Err(err) => {
            let report = LoadToolReport::config_failure(
                target_for_report,
                scenario_for_report,
                err.to_string(),
            );
            if let Err(write_err) = emit_report(&report, report_path.as_ref()) {
                eprintln!("failed to emit structured load report: {write_err}");
            }
            return ExitCode::from(CONFIG_ERROR_EXIT_STATUS);
        }
    };

    let report = if config.dry_run {
        LoadToolReport::dry_run(&config)
    } else if config.max_starts.is_some() {
        match run_bounded_sessions(&config).await {
            Ok(()) => LoadToolReport::completed(&config),
            Err(err) => LoadToolReport::session_failure(&config, err.phase(), err.message()),
        }
    } else {
        return run_unbounded_sessions(config).await;
    };

    let exit_status = report.exit_status();
    if let Err(err) = emit_report(&report, config.report_path.as_ref()) {
        eprintln!("failed to emit structured load report: {err}");
    }

    ExitCode::from(exit_status)
}

async fn run_bounded_sessions(config: &StresserConfig) -> Result<(), SessionFailure> {
    let max_starts = config
        .max_starts
        .expect("bounded sessions require max starts");
    let sema = Arc::new(Semaphore::new(config.sessions_count.get()));
    let mut handles = Vec::with_capacity(max_starts.get());

    for session_index in 0..max_starts.get() {
        let permit = sema.clone().acquire_owned().await.expect("semaphore open");
        let params = SessionParams {
            socket_addr: config.target_addr,
            session_name: session_name(&config.name_prefix, session_index),
            read_buffer_size: config.read_buffer_size,
        };

        let session_timeout = config.session_timeout;
        handles.push(tokio::spawn(async move {
            let result = make_session_with_timeout(&params, session_timeout).await;
            drop(permit);
            result
        }));

        tokio::time::sleep(config.spawn_cooldown).await;
    }

    for handle in handles {
        match handle.await {
            Ok(Ok(())) => (),
            Ok(Err(err)) => return Err(err),
            Err(err) => return Err(SessionFailure::task_join(err.to_string())),
        }
    }

    Ok(())
}

async fn run_unbounded_sessions(config: StresserConfig) -> ExitCode {
    let mut session_index = 0;
    let sema = Arc::new(Semaphore::new(config.sessions_count.get()));

    while let Ok(permit) = sema.clone().acquire_owned().await {
        let params = SessionParams {
            socket_addr: config.target_addr,
            session_name: session_name(&config.name_prefix, session_index),
            read_buffer_size: config.read_buffer_size,
        };

        let report_config = config.clone();
        let session_timeout = config.session_timeout;
        tokio::spawn(async move {
            if let Err(err) = make_session_with_timeout(&params, session_timeout).await {
                let report =
                    LoadToolReport::session_failure(&report_config, err.phase(), err.message());
                match serde_json::to_string(&report) {
                    Ok(json) => eprintln!("{json}"),
                    Err(json_err) => {
                        eprintln!("failed to encode structured load report: {json_err}")
                    }
                }
            }

            drop(permit);
        });

        session_index += 1;
        tokio::time::sleep(config.spawn_cooldown).await;
    }

    ExitCode::SUCCESS
}

async fn make_session_with_timeout(
    params: &SessionParams,
    session_timeout: Option<std::time::Duration>,
) -> Result<(), SessionFailure> {
    match session_timeout {
        Some(timeout) => match tokio::time::timeout(timeout, make_session(params)).await {
            Ok(result) => result,
            Err(_) => Err(SessionFailure::timeout(timeout)),
        },
        None => make_session(params).await,
    }
}

fn session_name(prefix: &str, session_index: usize) -> String {
    format!("{prefix}{session_index}")
}

fn emit_report(
    report: &LoadToolReport,
    report_path: Option<&std::path::PathBuf>,
) -> Result<(), String> {
    let mut json = serde_json::to_string_pretty(report).map_err(|err| err.to_string())?;
    json.push('\n');

    match report_path {
        Some(path) => fs::write(path, json).map_err(|err| err.to_string()),
        None => {
            print!("{json}");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_name_appends_index() {
        assert_eq!(session_name("Load", 0), "Load0");
        assert_eq!(session_name("Load", 1), "Load1");
    }
}
