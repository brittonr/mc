use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const PLAN_SCHEMA: &str = "mc.compat.evidence-promotion-plan.v1";
const EXAMPLE_RAIL: &str = "projectile-damage-attribution";
const SELF_TEST_FLAG: &str = "--self-test";
const APPLY_FLAG: &str = "--apply";
const OUT_DIR_FLAG: &str = "--out-dir";
const DEFAULT_OUT_DIR: &str = "target/evidence-promotion/projectile-damage-attribution";
const DIGEST_HEX_LENGTH: usize = 64;
const MIN_REQUIRED_ARTIFACT_CLASSES: usize = 4;
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "full Minecraft compatibility",
    "full CTF/combat correctness",
];
const VALIDATION_COMMANDS: &[&str] = &[
    "nix build .#checks.x86_64-linux.mc-compat-acceptance-matrix --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-current-evidence-bundle --no-link -L",
    "nix build .#checks.x86_64-linux.mc-compat-evidence-manifests --no-link -L",
    "nix run --no-update-lock-file .#cairn -- validate --root .",
];
const MATRIX_PATH: &str = "docs/evidence/protocol-763-acceptance-matrix.md";
const BUNDLE_PATH: &str = "docs/evidence/protocol-763-current-evidence-bundle.md";
const EXAMPLE_PREFIX: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27";
const EXAMPLE_RECEIPT: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.receipt.json";
const EXAMPLE_RUN_LOG: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.run.log";
const EXAMPLE_CLIENT_A_LOG: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbota.log";
const EXAMPLE_CLIENT_B_LOG: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.client-compatbotb.log";
const EXAMPLE_SERVER_LOG: &str =
    "docs/evidence/protocol-763-roi-10-projectile-damage-pinned-2026-05-27.valence.log";
const EXAMPLE_MATRIX_ROW: &str = "Projectile damage attribution";
const EXAMPLE_BUNDLE_ROW: &str = "Projectile damage attribution";
const EXAMPLE_CHILD_REV_FIELD: &str = "valence_rev";
const EXAMPLE_CHILD_REV_VALUE: &str = "e5d18ad04010d92881267ac1ea43922ae91821f5";
const PLAN_FILE_NAME: &str = "promotion-plan.md";
const B3SUM_COMMAND: &str = "b3sum";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ArtifactClass {
    Receipt,
    RunLog,
    ClientLog,
    ServerLog,
}

impl ArtifactClass {
    fn as_str(self) -> &'static str {
        match self {
            ArtifactClass::Receipt => "receipt",
            ArtifactClass::RunLog => "run-log",
            ArtifactClass::ClientLog => "client-log",
            ArtifactClass::ServerLog => "server-log",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ArtifactCandidate {
    class: ArtifactClass,
    source: String,
    destination: String,
    blake3: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ChildRevision {
    field: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PromotionInput {
    rail: String,
    artifacts: Vec<ArtifactCandidate>,
    child_revisions: Vec<ChildRevision>,
    matrix_text: String,
    bundle_text: String,
    matrix_row: String,
    bundle_row: String,
    required_non_claims: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PromotionPlan {
    rail: String,
    writes: Vec<ArtifactCandidate>,
    child_revisions: Vec<ChildRevision>,
    matrix_row: String,
    bundle_row: String,
    required_non_claims: Vec<String>,
    validation_commands: Vec<String>,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("evidence promotion self-test passed: {summary}");
                ExitCode::SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                ExitCode::FAILURE
            }
        };
    }

    let apply = args.iter().any(|arg| arg == APPLY_FLAG);
    let out_dir = parse_out_dir(&args).unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
    match run_example_cli(apply, &out_dir) {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            ExitCode::FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("evidence promotion failed: {error}");
    }
}

fn parse_out_dir(args: &[String]) -> Option<PathBuf> {
    args.windows(2)
        .find(|window| window.first().is_some_and(|arg| arg == OUT_DIR_FLAG))
        .and_then(|window| window.get(1))
        .map(PathBuf::from)
}

fn run_example_cli(apply: bool, out_dir: &Path) -> Result<String, Vec<String>> {
    let input = build_example_input(out_dir)?;
    let plan = compute_promotion_plan(&input)?;
    let rendered = render_plan(&plan);
    if apply {
        apply_plan(&plan, out_dir, &rendered)?;
        Ok(format!(
            "applied {} exact writes plus {}",
            plan.writes.len(),
            out_dir.join(PLAN_FILE_NAME).display()
        ))
    } else {
        println!("{rendered}");
        Ok(format!(
            "dry-run planned {} exact writes under {}",
            plan.writes.len(),
            out_dir.display()
        ))
    }
}

fn build_example_input(out_dir: &Path) -> Result<PromotionInput, Vec<String>> {
    let matrix_text = read_text(MATRIX_PATH)?;
    let bundle_text = read_text(BUNDLE_PATH)?;
    let sources = [
        (ArtifactClass::Receipt, EXAMPLE_RECEIPT),
        (ArtifactClass::RunLog, EXAMPLE_RUN_LOG),
        (ArtifactClass::ClientLog, EXAMPLE_CLIENT_A_LOG),
        (ArtifactClass::ClientLog, EXAMPLE_CLIENT_B_LOG),
        (ArtifactClass::ServerLog, EXAMPLE_SERVER_LOG),
    ];
    let mut artifacts = Vec::new();
    for (class, source) in sources {
        artifacts.push(ArtifactCandidate {
            class,
            source: source.to_string(),
            destination: out_dir
                .join(file_name(source)?)
                .to_string_lossy()
                .into_owned(),
            blake3: compute_b3(source)?,
        });
    }
    Ok(PromotionInput {
        rail: EXAMPLE_RAIL.to_string(),
        artifacts,
        child_revisions: vec![ChildRevision {
            field: EXAMPLE_CHILD_REV_FIELD.to_string(),
            value: EXAMPLE_CHILD_REV_VALUE.to_string(),
        }],
        matrix_text,
        bundle_text,
        matrix_row: EXAMPLE_MATRIX_ROW.to_string(),
        bundle_row: EXAMPLE_BUNDLE_ROW.to_string(),
        required_non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| claim.to_string())
            .collect(),
    })
}

fn read_text(path: &str) -> Result<String, Vec<String>> {
    fs::read_to_string(path).map_err(|err| vec![format!("{path}: {err}")])
}

fn file_name(path: &str) -> Result<&str, Vec<String>> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| vec![format!("source path has no UTF-8 file name: {path}")])
}

fn compute_b3(path: &str) -> Result<String, Vec<String>> {
    let output = Command::new(B3SUM_COMMAND)
        .arg(path)
        .output()
        .map_err(|err| vec![format!("failed to run {B3SUM_COMMAND} for {path}: {err}")])?;
    if !output.status.success() {
        return Err(vec![format!(
            "{B3SUM_COMMAND} failed for {path}: {}",
            String::from_utf8_lossy(&output.stderr)
        )]);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .split_whitespace()
        .next()
        .map(str::to_string)
        .ok_or_else(|| vec![format!("{B3SUM_COMMAND} produced no digest for {path}")])
}

fn compute_promotion_plan(input: &PromotionInput) -> Result<PromotionPlan, Vec<String>> {
    let mut errors = Vec::new();
    if input.rail.is_empty() {
        errors.push("rail is required".to_string());
    }
    validate_artifacts(&input.artifacts, &mut errors);
    validate_child_revisions(&input.child_revisions, &mut errors);
    validate_matrix_bundle_text(input, &mut errors);
    if errors.is_empty() {
        Ok(PromotionPlan {
            rail: input.rail.clone(),
            writes: input.artifacts.clone(),
            child_revisions: input.child_revisions.clone(),
            matrix_row: input.matrix_row.clone(),
            bundle_row: input.bundle_row.clone(),
            required_non_claims: input.required_non_claims.clone(),
            validation_commands: VALIDATION_COMMANDS
                .iter()
                .map(|command| command.to_string())
                .collect(),
        })
    } else {
        Err(errors)
    }
}

fn validate_artifacts(artifacts: &[ArtifactCandidate], errors: &mut Vec<String>) {
    let mut class_counts = BTreeMap::<ArtifactClass, usize>::new();
    let mut destinations = BTreeSet::new();
    for artifact in artifacts {
        *class_counts.entry(artifact.class).or_default() += 1;
        if artifact.source.is_empty() {
            errors.push(format!("{} source is required", artifact.class.as_str()));
        }
        if artifact.destination.is_empty() {
            errors.push(format!(
                "{} destination is required",
                artifact.class.as_str()
            ));
        }
        if !destinations.insert(artifact.destination.clone()) {
            errors.push(format!("duplicate destination {}", artifact.destination));
        }
        if !is_b3_digest(&artifact.blake3) {
            errors.push(format!(
                "{} has invalid BLAKE3 digest {}",
                artifact.source, artifact.blake3
            ));
        }
    }
    for required in [
        ArtifactClass::Receipt,
        ArtifactClass::RunLog,
        ArtifactClass::ClientLog,
        ArtifactClass::ServerLog,
    ] {
        if class_counts.get(&required).copied().unwrap_or_default() == usize::MIN {
            errors.push(format!("missing required {} artifact", required.as_str()));
        }
    }
    if class_counts.len() < MIN_REQUIRED_ARTIFACT_CLASSES {
        errors.push(
            "promotion needs receipt, run log, client log, and server log classes".to_string(),
        );
    }
}

fn validate_child_revisions(child_revisions: &[ChildRevision], errors: &mut Vec<String>) {
    if child_revisions.is_empty() {
        errors.push("child revision fields are required".to_string());
    }
    for revision in child_revisions {
        if revision.field.is_empty() || revision.value.is_empty() {
            errors.push("child revision field/value must be nonempty".to_string());
        }
    }
}

fn validate_matrix_bundle_text(input: &PromotionInput, errors: &mut Vec<String>) {
    if input.matrix_row.is_empty() || !input.matrix_text.contains(&input.matrix_row) {
        errors.push(format!("matrix row missing: {}", input.matrix_row));
    }
    if input.bundle_row.is_empty() || !input.bundle_text.contains(&input.bundle_row) {
        errors.push(format!("current bundle row missing: {}", input.bundle_row));
    }
    for claim in &input.required_non_claims {
        if !input.matrix_text.contains(claim) && !input.bundle_text.contains(claim) {
            errors.push(format!("required non-claim text missing: {claim}"));
        }
    }
}

fn is_b3_digest(value: &str) -> bool {
    value.len() == DIGEST_HEX_LENGTH && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn apply_plan(plan: &PromotionPlan, out_dir: &Path, rendered: &str) -> Result<(), Vec<String>> {
    fs::create_dir_all(out_dir).map_err(|err| vec![format!("{}: {err}", out_dir.display())])?;
    for write in &plan.writes {
        fs::copy(&write.source, &write.destination).map_err(|err| {
            vec![format!(
                "copy {} -> {}: {err}",
                write.source, write.destination
            )]
        })?;
    }
    fs::write(out_dir.join(PLAN_FILE_NAME), rendered)
        .map_err(|err| vec![format!("write promotion plan: {err}")])?;
    Ok(())
}

fn render_plan(plan: &PromotionPlan) -> String {
    let mut text = String::new();
    text.push_str(&format!("# Evidence promotion plan: {}\n\n", plan.rail));
    text.push_str(&format!("schema: `{PLAN_SCHEMA}`\n\n"));
    text.push_str("## Exact writes\n\n");
    for write in &plan.writes {
        text.push_str(&format!(
            "- {}: `{}` -> `{}` (BLAKE3 `{}`)\n",
            write.class.as_str(),
            write.source,
            write.destination,
            write.blake3
        ));
    }
    text.push_str("\n## Child revisions\n\n");
    for revision in &plan.child_revisions {
        text.push_str(&format!("- `{}` = `{}`\n", revision.field, revision.value));
    }
    text.push_str("\n## Matrix/current bundle\n\n");
    text.push_str(&format!("- Matrix row: `{}`\n", plan.matrix_row));
    text.push_str(&format!("- Current bundle row: `{}`\n", plan.bundle_row));
    text.push_str("- Action: plan-only; preserve existing non-claims unless a separate reviewed edit updates rows.\n");
    text.push_str("\n## Required non-claims\n\n");
    for claim in &plan.required_non_claims {
        text.push_str(&format!("- {claim}\n"));
    }
    text.push_str("\n## Validation commands\n\n");
    for command in &plan.validation_commands {
        text.push_str(&format!("- `{command}`\n"));
    }
    text.push_str(&format!("\nsource prefix: `{EXAMPLE_PREFIX}`\n"));
    text
}

fn run_self_tests() -> Result<String, Vec<String>> {
    let cases = [
        ("complete", complete_fixture(), true),
        (
            "missing_receipt",
            without_class(ArtifactClass::Receipt),
            false,
        ),
        (
            "missing_run_log",
            without_class(ArtifactClass::RunLog),
            false,
        ),
        (
            "missing_client_log",
            without_class(ArtifactClass::ClientLog),
            false,
        ),
        (
            "missing_server_log",
            without_class(ArtifactClass::ServerLog),
            false,
        ),
        ("bad_blake3", bad_blake3_fixture(), false),
        (
            "missing_child_revision",
            missing_child_revision_fixture(),
            false,
        ),
        (
            "duplicate_destination",
            duplicate_destination_fixture(),
            false,
        ),
        ("weakened_nonclaim", weakened_nonclaim_fixture(), false),
    ];
    let mut errors = Vec::new();
    for (name, input, should_pass) in cases {
        let passed = compute_promotion_plan(&input).is_ok();
        if passed != should_pass {
            errors.push(format!(
                "{name} expected pass={should_pass} got pass={passed}"
            ));
        }
    }
    if errors.is_empty() {
        Ok("positive and negative promotion fixtures exercised".to_string())
    } else {
        Err(errors)
    }
}

fn complete_fixture() -> PromotionInput {
    let digest = "a".repeat(DIGEST_HEX_LENGTH);
    PromotionInput {
        rail: EXAMPLE_RAIL.to_string(),
        artifacts: vec![
            artifact(
                ArtifactClass::Receipt,
                "receipt.json",
                "out/receipt.json",
                &digest,
            ),
            artifact(ArtifactClass::RunLog, "run.log", "out/run.log", &digest),
            artifact(
                ArtifactClass::ClientLog,
                "client.log",
                "out/client.log",
                &digest,
            ),
            artifact(
                ArtifactClass::ServerLog,
                "server.log",
                "out/server.log",
                &digest,
            ),
        ],
        child_revisions: vec![ChildRevision {
            field: EXAMPLE_CHILD_REV_FIELD.to_string(),
            value: EXAMPLE_CHILD_REV_VALUE.to_string(),
        }],
        matrix_text: format!("{EXAMPLE_MATRIX_ROW}\n{}", REQUIRED_NON_CLAIMS.join("\n")),
        bundle_text: EXAMPLE_BUNDLE_ROW.to_string(),
        matrix_row: EXAMPLE_MATRIX_ROW.to_string(),
        bundle_row: EXAMPLE_BUNDLE_ROW.to_string(),
        required_non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| claim.to_string())
            .collect(),
    }
}

fn artifact(
    class: ArtifactClass,
    source: &str,
    destination: &str,
    blake3: &str,
) -> ArtifactCandidate {
    ArtifactCandidate {
        class,
        source: source.to_string(),
        destination: destination.to_string(),
        blake3: blake3.to_string(),
    }
}

fn without_class(class: ArtifactClass) -> PromotionInput {
    let mut input = complete_fixture();
    input.artifacts.retain(|artifact| artifact.class != class);
    input
}

fn bad_blake3_fixture() -> PromotionInput {
    let mut input = complete_fixture();
    if let Some(first) = input.artifacts.first_mut() {
        first.blake3 = "not-b3".to_string();
    }
    input
}

fn missing_child_revision_fixture() -> PromotionInput {
    let mut input = complete_fixture();
    input.child_revisions.clear();
    input
}

fn duplicate_destination_fixture() -> PromotionInput {
    let mut input = complete_fixture();
    if let Some(second) = input.artifacts.get_mut(1) {
        second.destination = "out/receipt.json".to_string();
    }
    input
}

fn weakened_nonclaim_fixture() -> PromotionInput {
    let mut input = complete_fixture();
    input.matrix_text = EXAMPLE_MATRIX_ROW.to_string();
    input.bundle_text = EXAMPLE_BUNDLE_ROW.to_string();
    input
}
