use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::{self, ExitCode};

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_FLAG: &str = "--row";
const EVIDENCE_FLAG: &str = "--evidence";
const SUCCESS: ExitCode = ExitCode::SUCCESS;
const FAILURE: ExitCode = ExitCode::FAILURE;
const KEY_VALUE_SEPARATOR: char = '=';
const SINGLE_ARGUMENT_COUNT: usize = 5;
const PROGRAM_ARGUMENT_INDEX: usize = 1;
const MIN_BATCH_ARGUMENT_COUNT: usize = 2;
const ARG_VALUE_OFFSET: usize = 1;
const ARG_PAIR_WIDTH: usize = 2;
const LINE_NUMBER_OFFSET: usize = 1;
const FIRST_SPEC_INDEX: usize = 0;
const SECOND_SPEC_INDEX: usize = 1;
const OK_VALUE: &str = "ok";
const TRUE_VALUE: &str = "true";
const TRUTHY_OVERCLAIM_VALUES: &[&str] = &["true", "yes", "ok", "claimed", "1"];
const COMMON_EXACT_FIELDS: &[Field] = &[
    Field::new("evidence.mode", "deterministic-fixture"),
    Field::new("promotion.scope", "bounded-row-only"),
    Field::new("packet.inventory.coverage_status", "scenario_bounded"),
    Field::new(
        "packet.inventory.parser_shape_status",
        "shape_review_missing",
    ),
    Field::new(
        "packet.inventory.mapping_status",
        "reviewed_fixture_no_shape_claim",
    ),
];
const COMMON_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Field {
    key: &'static str,
    value: &'static str,
}

impl Field {
    const fn new(key: &'static str, value: &'static str) -> Self {
        Self { key, value }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RowSpec {
    id: &'static str,
    label: &'static str,
    exact_fields: &'static [Field],
    metrics: &'static [&'static str],
    nonclaims: &'static [&'static str],
}

const BLOCK_ENTITY_UPDATE_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/block-entity-update-breadth-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/block-entity-update-breadth-2026-06-06.md",
    ),
    Field::new(
        "packet.row",
        "play/clientbound/0x08 BlockEntityUpdateS2CPacket",
    ),
    Field::new("block_entity.kind", "Chest"),
    Field::new("block_entity.position", "32,64,0"),
    Field::new(
        "block_entity.payload_metric",
        "items=1;custom_name=Compat Chest",
    ),
    Field::new("backend.evidence", "fixture-normalized"),
];
const BLOCK_ENTITY_UPDATE_METRICS: &[&str] = &[
    "metric.packet.block_entity_update_row_bound",
    "metric.fixture.payload_identity",
    "metric.backend.evidence_present",
    "metric.checker.positive_fixture",
    "metric.checker.negative_fixtures",
];
const BLOCK_ENTITY_UPDATE_NONCLAIMS: &[&str] = &[
    "all_block_entities",
    "arbitrary_nbt_parity",
    "persistence_breadth",
    "sign_editing",
];

const CHAT_COMMAND_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/chat-command-containment-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/chat-command-containment-2026-06-06.md",
    ),
    Field::new(
        "packet.row.chat",
        "play/serverbound/0x05 ChatMessageC2SPacket",
    ),
    Field::new(
        "packet.row.command",
        "play/serverbound/0x04 CommandExecutionC2SPacket",
    ),
    Field::new("actor", "compatbot"),
    Field::new("target.scope", "owned-local-fixture"),
    Field::new("payload.identity", "harmless-chat-command-containment"),
    Field::new("redaction.policy", "no-secrets-no-public-addresses"),
];
const CHAT_COMMAND_METRICS: &[&str] = &[
    "metric.client.chat_payload_sent",
    "metric.client.command_payload_sent",
    "metric.server.containment_correlation",
    "metric.redaction.policy_recorded",
    "metric.checker.negative_fixtures",
];
const CHAT_COMMAND_NONCLAIMS: &[&str] = &[
    "chat_signing_security",
    "all_commands",
    "command_permissions",
    "moderation",
    "adversarial_resilience",
];

const CHUNK_BIOME_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/chunk-biome-data-packet-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/chunk-biome-data-packet-2026-06-06.md",
    ),
    Field::new(
        "packet.row",
        "play/clientbound/0x0d ChunkBiomeDataS2CPacket",
    ),
    Field::new("protocol.version", "763"),
    Field::new("fixture.source", "reviewed-byte-fixture"),
    Field::new("payload.identity", "single-overworld-column-biome-palette"),
    Field::new("parser.expectation", "byte-preservation-shape-only"),
];
const CHUNK_BIOME_METRICS: &[&str] = &[
    "metric.packet.chunk_biome_data_row_bound",
    "metric.fixture.payload_identity",
    "metric.parser.byte_preservation",
    "metric.protocol.version_recorded",
    "metric.checker.negative_fixtures",
];
const CHUNK_BIOME_NONCLAIMS: &[&str] = &[
    "all_biome_semantics",
    "all_chunk_semantics",
    "all_worldgen_packets",
    "dimension_travel",
    "nether_end_behavior",
];

const CREATIVE_INVENTORY_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/creative-inventory-action-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/creative-inventory-action-2026-06-06.md",
    ),
    Field::new(
        "packet.row",
        "play/serverbound/0x2b CreativeInventoryActionC2SPacket",
    ),
    Field::new("actor", "compatbot"),
    Field::new("game_mode", "creative"),
    Field::new("semantic_slot", "hotbar_0"),
    Field::new("wire_slot", "36"),
    Field::new("item", "minecraft:stone"),
    Field::new("item.count", "64"),
];
const CREATIVE_INVENTORY_METRICS: &[&str] = &[
    "metric.client.creative_action_sent",
    "metric.server.accepted_slot_mutation",
    "metric.final_slot_state",
    "metric.creative_mode_precondition",
    "metric.checker.negative_fixtures",
];
const CREATIVE_INVENTORY_NONCLAIMS: &[&str] = &[
    "all_creative_inventory_semantics",
    "all_slots",
    "all_items",
    "game_mode_transitions",
    "pick_block_behavior",
];

const STATUS_EFFECT_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/entity-status-effect-packets-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/entity-status-effect-packets-2026-06-06.md",
    ),
    Field::new(
        "packet.row.apply",
        "play/clientbound/0x6c EntityStatusEffectS2CPacket",
    ),
    Field::new(
        "packet.row.remove",
        "play/clientbound/0x3f RemoveEntityStatusEffectS2CPacket",
    ),
    Field::new("target.entity", "compatbot"),
    Field::new("effect", "minecraft:speed"),
    Field::new("amplifier", "1"),
    Field::new("duration_ticks", "200"),
];
const STATUS_EFFECT_METRICS: &[&str] = &[
    "metric.client.effect_apply_observed",
    "metric.client.effect_remove_observed",
    "metric.server.effect_correlation",
    "metric.effect_fields_consistent",
    "metric.checker.negative_fixtures",
];
const STATUS_EFFECT_NONCLAIMS: &[&str] = &[
    "all_effects",
    "effect_stacking",
    "particles_ui",
    "gameplay_modifiers",
    "combat_balancing",
    "survival_parity",
];

const RECIPE_BOOK_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/recipe-book-client-settings-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/recipe-book-client-settings-2026-06-06.md",
    ),
    Field::new(
        "packet.row",
        "play/serverbound/0x22 RecipeBookDataC2SPacket",
    ),
    Field::new("actor", "compatbot"),
    Field::new("book", "crafting"),
    Field::new("open", "true"),
    Field::new("filtering", "false"),
];
const RECIPE_BOOK_METRICS: &[&str] = &[
    "metric.client.settings_transition_sent",
    "metric.server.settings_correlation",
    "metric.state_fields_consistent",
    "metric.crafting_rows_unchanged",
    "metric.checker.negative_fixtures",
];
const RECIPE_BOOK_NONCLAIMS: &[&str] = &[
    "recipe_book_ui_behavior",
    "all_recipe_categories",
    "recipe_discovery",
    "all_recipes",
    "full_crafting_coverage",
];

const RESOURCE_PACK_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/resource-pack-status-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/resource-pack-status-2026-06-06.md",
    ),
    Field::new(
        "packet.row.offer",
        "play/clientbound/0x40 ResourcePackSendS2CPacket",
    ),
    Field::new(
        "packet.row.status",
        "play/serverbound/0x24 ResourcePackStatusC2SPacket",
    ),
    Field::new("actor", "compatbot"),
    Field::new("offer.scope", "owned-local-fixture"),
    Field::new("status", "declined"),
    Field::new("external_fetch", "false"),
    Field::new("redaction.policy", "no-secrets-no-public-addresses"),
];
const RESOURCE_PACK_METRICS: &[&str] = &[
    "metric.server.offer_sent",
    "metric.client.status_response_sent",
    "metric.server.status_correlation",
    "metric.no_external_fetch_guarantee",
    "metric.checker.negative_fixtures",
];
const RESOURCE_PACK_NONCLAIMS: &[&str] = &[
    "asset_download_application",
    "trust_security_validation",
    "all_resource_pack_statuses",
];

const SIGN_EDITOR_FIELDS: &[Field] = &[
    Field::new(
        "evidence.receipt",
        "docs/evidence/sign-editor-open-update-2026-06-06.receipt.json",
    ),
    Field::new(
        "evidence.doc",
        "docs/evidence/sign-editor-open-update-2026-06-06.md",
    ),
    Field::new(
        "packet.row.open",
        "play/clientbound/0x31 SignEditorOpenS2CPacket",
    ),
    Field::new(
        "packet.row.update",
        "play/serverbound/0x2e UpdateSignC2SPacket",
    ),
    Field::new("actor", "compatbot"),
    Field::new("sign.position", "28,64,0"),
    Field::new("sign.initial_state", "blank"),
    Field::new("sign.payload", "MC|Compat|Sign|Edit"),
];
const SIGN_EDITOR_METRICS: &[&str] = &[
    "metric.client.sign_editor_open_observed",
    "metric.client.sign_update_sent",
    "metric.server.sign_update_accepted",
    "metric.sign_payload_consistent",
    "metric.checker.negative_fixtures",
];
const SIGN_EDITOR_NONCLAIMS: &[&str] = &[
    "sign_editing_ui_behavior",
    "all_sign_variants",
    "all_text_formats",
    "arbitrary_nbt_semantics",
    "all_block_entities",
];

const ROW_SPECS: &[RowSpec] = &[
    RowSpec {
        id: "block-entity-update-breadth",
        label: "block-entity update breadth",
        exact_fields: BLOCK_ENTITY_UPDATE_FIELDS,
        metrics: BLOCK_ENTITY_UPDATE_METRICS,
        nonclaims: BLOCK_ENTITY_UPDATE_NONCLAIMS,
    },
    RowSpec {
        id: "chat-command-containment",
        label: "chat/command containment",
        exact_fields: CHAT_COMMAND_FIELDS,
        metrics: CHAT_COMMAND_METRICS,
        nonclaims: CHAT_COMMAND_NONCLAIMS,
    },
    RowSpec {
        id: "chunk-biome-data-packet",
        label: "chunk biome data packet",
        exact_fields: CHUNK_BIOME_FIELDS,
        metrics: CHUNK_BIOME_METRICS,
        nonclaims: CHUNK_BIOME_NONCLAIMS,
    },
    RowSpec {
        id: "creative-inventory-action",
        label: "creative inventory action",
        exact_fields: CREATIVE_INVENTORY_FIELDS,
        metrics: CREATIVE_INVENTORY_METRICS,
        nonclaims: CREATIVE_INVENTORY_NONCLAIMS,
    },
    RowSpec {
        id: "entity-status-effect-packets",
        label: "entity status-effect packets",
        exact_fields: STATUS_EFFECT_FIELDS,
        metrics: STATUS_EFFECT_METRICS,
        nonclaims: STATUS_EFFECT_NONCLAIMS,
    },
    RowSpec {
        id: "recipe-book-client-settings",
        label: "recipe-book client settings",
        exact_fields: RECIPE_BOOK_FIELDS,
        metrics: RECIPE_BOOK_METRICS,
        nonclaims: RECIPE_BOOK_NONCLAIMS,
    },
    RowSpec {
        id: "resource-pack-status",
        label: "resource-pack status",
        exact_fields: RESOURCE_PACK_FIELDS,
        metrics: RESOURCE_PACK_METRICS,
        nonclaims: RESOURCE_PACK_NONCLAIMS,
    },
    RowSpec {
        id: "sign-editor-open-update",
        label: "sign editor open/update",
        exact_fields: SIGN_EDITOR_FIELDS,
        metrics: SIGN_EDITOR_METRICS,
        nonclaims: SIGN_EDITOR_NONCLAIMS,
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Evidence {
    values: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CliConfig {
    Single { row: String, evidence_path: String },
    Batch { evidence_paths: Vec<String> },
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        return match run_self_tests() {
            Ok(summary) => {
                println!("targeted packet promotion self-test ok: {summary}");
                SUCCESS
            }
            Err(errors) => {
                print_errors(&errors);
                FAILURE
            }
        };
    }

    match parse_cli(&args).and_then(|config| run_config(&config)) {
        Ok(summary) => {
            println!("{summary}");
            SUCCESS
        }
        Err(errors) => {
            print_errors(&errors);
            FAILURE
        }
    }
}

fn print_errors(errors: &[String]) {
    for error in errors {
        eprintln!("targeted packet promotion check failed: {error}");
    }
}

fn parse_cli(args: &[String]) -> Result<CliConfig, Vec<String>> {
    if args.len() >= MIN_BATCH_ARGUMENT_COUNT && args[PROGRAM_ARGUMENT_INDEX] != ROW_FLAG {
        if args[PROGRAM_ARGUMENT_INDEX].starts_with('-') {
            return Err(vec![usage()]);
        }
        return Ok(CliConfig::Batch {
            evidence_paths: args[PROGRAM_ARGUMENT_INDEX..].to_vec(),
        });
    }
    if args.len() != SINGLE_ARGUMENT_COUNT {
        return Err(vec![usage()]);
    }

    let mut row = None;
    let mut evidence_path = None;
    let mut index = PROGRAM_ARGUMENT_INDEX;
    while index < args.len() {
        let flag = args[index].as_str();
        let Some(value) = args.get(index + ARG_VALUE_OFFSET) else {
            return Err(vec![usage()]);
        };
        match flag {
            ROW_FLAG => row = Some(value.clone()),
            EVIDENCE_FLAG => evidence_path = Some(value.clone()),
            _ => return Err(vec![format!("unknown argument: {flag}")]),
        }
        index += ARG_PAIR_WIDTH;
    }

    Ok(CliConfig::Single {
        row: row.ok_or_else(|| vec![usage()])?,
        evidence_path: evidence_path.ok_or_else(|| vec![usage()])?,
    })
}

fn usage() -> String {
    format!("usage: check_targeted_packet_promotions {ROW_FLAG} <row-id> {EVIDENCE_FLAG} <kv> | <kv>... | {SELF_TEST_FLAG}")
}

fn run_config(config: &CliConfig) -> Result<String, Vec<String>> {
    match config {
        CliConfig::Single { row, evidence_path } => {
            let spec = validate_path(row, evidence_path)?;
            Ok(format!("{} promotion evidence passed", spec.label))
        }
        CliConfig::Batch { evidence_paths } => {
            let mut labels = Vec::new();
            for evidence_path in evidence_paths {
                let text = fs::read_to_string(evidence_path)
                    .map_err(|error| vec![format!("{evidence_path}: {error}")])?;
                let evidence = Evidence::parse(&text).map_err(|error| vec![error])?;
                let row = evidence
                    .value("row.id")
                    .ok_or_else(|| vec![format!("{evidence_path}: missing row.id")])?;
                let spec = row_spec(row)?;
                validate_evidence(spec, &evidence)?;
                labels.push(spec.label);
            }
            Ok(format!(
                "targeted packet promotion check passed: {} fixtures ({})",
                labels.len(),
                labels.join(", ")
            ))
        }
    }
}

fn validate_path(row: &str, evidence_path: &str) -> Result<RowSpec, Vec<String>> {
    let spec = row_spec(row)?;
    let text = fs::read_to_string(evidence_path)
        .map_err(|error| vec![format!("{evidence_path}: {error}")])?;
    let evidence = Evidence::parse(&text).map_err(|error| vec![error])?;
    validate_evidence(spec, &evidence)?;
    Ok(spec)
}

impl Evidence {
    fn parse(text: &str) -> Result<Self, String> {
        let mut values = BTreeMap::new();
        for (line_index, raw_line) in text.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((key, value)) = line.split_once(KEY_VALUE_SEPARATOR) else {
                return Err(format!(
                    "line {} is not key=value",
                    line_index + LINE_NUMBER_OFFSET
                ));
            };
            let key = key.trim();
            let value = value.trim();
            if key.is_empty() {
                return Err(format!(
                    "line {} has empty key",
                    line_index + LINE_NUMBER_OFFSET
                ));
            }
            if values.insert(key.to_string(), value.to_string()).is_some() {
                return Err(format!("duplicate key {key}"));
            }
        }
        Ok(Self { values })
    }

    fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
}

fn row_spec(row: &str) -> Result<RowSpec, Vec<String>> {
    ROW_SPECS
        .iter()
        .copied()
        .find(|spec| spec.id == row)
        .ok_or_else(|| vec![format!("unknown row {row}")])
}

fn validate_evidence(spec: RowSpec, evidence: &Evidence) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    require_exact(evidence, &mut diagnostics, "row.id", spec.id);
    for field in COMMON_EXACT_FIELDS.iter().chain(spec.exact_fields.iter()) {
        require_exact(evidence, &mut diagnostics, field.key, field.value);
    }
    for metric in spec.metrics {
        require_exact(evidence, &mut diagnostics, metric, OK_VALUE);
    }
    for nonclaim in COMMON_NONCLAIMS.iter().chain(spec.nonclaims.iter()) {
        require_exact(
            evidence,
            &mut diagnostics,
            &format!("nonclaim.{nonclaim}"),
            TRUE_VALUE,
        );
        reject_truthy_overclaim(evidence, &mut diagnostics, nonclaim);
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn require_exact(evidence: &Evidence, diagnostics: &mut Vec<String>, key: &str, expected: &str) {
    match evidence.value(key) {
        Some(actual) if actual == expected => {}
        Some(actual) => diagnostics.push(format!("{key} expected {expected}, got {actual}")),
        None => diagnostics.push(format!("missing {key}")),
    }
}

fn reject_truthy_overclaim(evidence: &Evidence, diagnostics: &mut Vec<String>, claim: &str) {
    let key = format!("claim.{claim}");
    if let Some(value) = evidence.value(&key) {
        if TRUTHY_OVERCLAIM_VALUES
            .iter()
            .any(|truthy| value.eq_ignore_ascii_case(truthy))
        {
            diagnostics.push(format!("broad overclaim {key}={value}"));
        }
    }
}

fn valid_fixture(spec: RowSpec) -> String {
    let mut lines = Vec::new();
    lines.push(format!("row.id={}", spec.id));
    for field in COMMON_EXACT_FIELDS.iter().chain(spec.exact_fields.iter()) {
        lines.push(format!("{}={}", field.key, field.value));
    }
    for metric in spec.metrics {
        lines.push(format!("{metric}={OK_VALUE}"));
    }
    for nonclaim in COMMON_NONCLAIMS.iter().chain(spec.nonclaims.iter()) {
        lines.push(format!("nonclaim.{nonclaim}={TRUE_VALUE}"));
    }
    lines.join("\n")
}

fn fixture_with_replacement(spec: RowSpec, old: &str, new: &str) -> String {
    valid_fixture(spec).replace(old, new)
}

fn run_self_tests() -> Result<String, Vec<String>> {
    for spec in ROW_SPECS {
        let evidence = Evidence::parse(&valid_fixture(*spec)).map_err(|error| vec![error])?;
        validate_evidence(*spec, &evidence)?;
    }

    let temp_dir = env::temp_dir().join(format!(
        "targeted-packet-promotions-self-test-{}",
        process::id()
    ));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .map_err(|error| vec![format!("{}: {error}", temp_dir.display())])?;
    let first_path = temp_dir.join("first.kv");
    let second_path = temp_dir.join("second.kv");
    fs::write(&first_path, valid_fixture(ROW_SPECS[FIRST_SPEC_INDEX]))
        .map_err(|error| vec![format!("{}: {error}", first_path.display())])?;
    fs::write(&second_path, valid_fixture(ROW_SPECS[SECOND_SPEC_INDEX]))
        .map_err(|error| vec![format!("{}: {error}", second_path.display())])?;
    let batch_args = vec![
        "check_targeted_packet_promotions".to_string(),
        first_path.display().to_string(),
        second_path.display().to_string(),
    ];
    let batch_config = parse_cli(&batch_args)?;
    let batch_summary = run_config(&batch_config)?;
    if !batch_summary.contains("2 fixtures") {
        return Err(vec![format!(
            "batch summary missing fixture count: {batch_summary}"
        )]);
    }
    let flag_only_args = vec![
        "check_targeted_packet_promotions".to_string(),
        EVIDENCE_FLAG.to_string(),
    ];
    let flag_only_errors = parse_cli(&flag_only_args).expect_err("flag-only batch should fail");
    if !flag_only_errors
        .iter()
        .any(|error| error.contains("usage:"))
    {
        return Err(vec![format!(
            "flag-only batch missing usage diagnostic: {flag_only_errors:?}"
        )]);
    }
    let _ = fs::remove_dir_all(&temp_dir);

    let spec = ROW_SPECS[FIRST_SPEC_INDEX];
    expect_error(
        "missing row id",
        &fixture_with_replacement(spec, "row.id=block-entity-update-breadth\n", ""),
        spec,
        "missing row.id",
    )?;
    expect_error(
        "wrong packet row",
        &fixture_with_replacement(
            spec,
            "play/clientbound/0x08 BlockEntityUpdateS2CPacket",
            "play/clientbound/0x09 BlockEventS2CPacket",
        ),
        spec,
        "packet.row expected play/clientbound/0x08 BlockEntityUpdateS2CPacket",
    )?;
    expect_error(
        "missing metric",
        &fixture_with_replacement(spec, "metric.backend.evidence_present=ok\n", ""),
        spec,
        "missing metric.backend.evidence_present",
    )?;
    expect_error(
        "missing nonclaim",
        &fixture_with_replacement(spec, "nonclaim.all_block_entities=true\n", ""),
        spec,
        "missing nonclaim.all_block_entities",
    )?;
    expect_error(
        "truthy overclaim",
        &format!("{}\nclaim.all_block_entities=true", valid_fixture(spec)),
        spec,
        "broad overclaim claim.all_block_entities=true",
    )?;
    let duplicate_key = format!("{}\nrow.id=duplicate", valid_fixture(spec));
    match Evidence::parse(&duplicate_key) {
        Ok(_) => return Err(vec!["duplicate key fixture unexpectedly parsed".to_string()]),
        Err(error) if error.contains("duplicate key row.id") => {}
        Err(error) => return Err(vec![format!("unexpected duplicate diagnostic: {error}")]),
    }
    if row_spec("unknown-row").is_ok() {
        return Err(vec!["unknown row unexpectedly resolved".to_string()]);
    }

    Ok(format!(
        "{} positive fixtures and targeted negative fixtures exercised",
        ROW_SPECS.len()
    ))
}

fn expect_error(
    label: &str,
    fixture: &str,
    spec: RowSpec,
    expected: &str,
) -> Result<(), Vec<String>> {
    let evidence = Evidence::parse(fixture).map_err(|error| vec![error])?;
    let errors = validate_evidence(spec, &evidence).expect_err(label);
    if errors.iter().any(|error| error.contains(expected)) {
        Ok(())
    } else {
        Err(vec![format!(
            "{label} missing diagnostic {expected:?}: {errors:?}"
        )])
    }
}
