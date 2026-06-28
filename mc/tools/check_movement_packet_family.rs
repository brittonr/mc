mod checker_framework;

use std::env;
use std::fs;
use std::process;

use checker_framework::{
    reject_truthy_overclaims, require_clean_child_revision, require_exact, require_ok,
    KeyValueRecord,
};

const SELF_TEST_FLAG: &str = "--self-test";
const ROW_ID_KEY: &str = "row.id";
const ROW_ID: &str = "movement-packet-family";
const SCENARIO_KEY: &str = "scenario.context";
const SCENARIO: &str = "ctf-spawn-team-balance-reset";
const RECEIPT_KEY: &str = "evidence.receipt";
const RECEIPT: &str =
    "docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json";
const CLIENT_LOG_KEY: &str = "evidence.client_log";
const CLIENT_LOG: &str =
    "docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-a.log";
const SERVER_LOG_KEY: &str = "evidence.server_log";
const SERVER_LOG: &str =
    "docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.server.log";
const SOURCE_SNAPSHOT_KEY: &str = "source.snapshot";
const SOURCE_SNAPSHOT: &str =
    "docs/evidence/movement-packet-family-stevenarella-source-2026-06-06.rs";
const PACKET_ROW_KEY: &str = "packet.row";
const PACKET_ROW: &str = "play/serverbound/0x15 Full -> PlayerPositionLook";
const PACKET_MAPPING_KEY: &str = "packet.inventory.mapping_status";
const PACKET_MAPPING: &str = "reviewed_override_no_shape_claim";
const PACKET_PARSER_KEY: &str = "packet.inventory.parser_shape_status";
const PACKET_PARSER: &str = "shape_review_missing";
const PACKET_COVERAGE_KEY: &str = "packet.inventory.coverage_status";
const PACKET_COVERAGE: &str = "scenario_bounded";
const PACKET_EVIDENCE_KEY: &str = "packet.inventory.scenario_evidence";
const PACKET_EVIDENCE: &str = "movement_packet_family";
const CLIENT_REV_KEY: &str = "child.stevenarella.rev";
const CLIENT_STATUS_KEY: &str = "child.stevenarella.status";
const VALENCE_REV_KEY: &str = "child.valence.rev";
const VALENCE_STATUS_KEY: &str = "child.valence.status";
const ACTOR_KEY: &str = "actor.username";
const ACTOR: &str = "compatbota";
const START_KEY: &str = "movement.start";
const START: &str = "0.000,85.000,0.000";
const TARGET_KEY: &str = "movement.target";
const TARGET: &str = "-4.0,84.0,4.0";
const LOOK_KEY: &str = "movement.look";
const LOOK: &str = "yaw=0.0,pitch=0.0";
const ON_GROUND_KEY: &str = "movement.on_ground";
const ON_GROUND: &str = "true";
const TOLERANCE_KEY: &str = "movement.tolerance";
const TOLERANCE: &str = "exact_logged_values";
const REQUIRED_OK_METRICS: &[&str] = &[
    "metric.client.full_position_look_sent",
    "metric.client.portal_entry_logged",
    "metric.server.red_assignment_correlation",
    "metric.packet.full_variant_bound",
];
const REQUIRED_NONCLAIMS: &[&str] = &[
    "nonclaim.movement_physics",
    "nonclaim.collision",
    "nonclaim.anti_cheat",
    "nonclaim.latency_tolerance",
    "nonclaim.malicious_client_resilience",
    "nonclaim.all_movement_variants",
    "nonclaim.full_protocol_compatibility",
    "nonclaim.public_server_safety",
    "nonclaim.production_readiness",
];
const BROAD_OVERCLAIM_KEYS: &[&str] = &[
    "claim.movement_physics",
    "claim.collision",
    "claim.anti_cheat",
    "claim.latency_tolerance",
    "claim.malicious_client_resilience",
    "claim.all_movement_variants",
    "claim.full_protocol_compatibility",
    "claim.public_server_safety",
    "claim.production_readiness",
    "claim.production_ready",
];
const TRUTHY_OVERCLAIM_VALUES: &[&str] = &["true", "yes", "ok", "claimed", "1"];
const INVENTORY_PATH: &str = "docs/evidence/protocol-763-packet-inventory-2026-05-28.tsv";
const INVENTORY_HEADER: &str = "state\tside\twire_id\tvalence_packet\tstevenarella_mapping_status\tstevenarella_internal_id\tparser_shape_status\tscenario_evidence\tcoverage_status\towner\tnext_action";
const INVENTORY_COLUMN_COUNT: usize = 11;
const STATE_COLUMN: usize = 0;
const SIDE_COLUMN: usize = 1;
const WIRE_ID_COLUMN: usize = 2;
const VALENCE_PACKET_COLUMN: usize = 3;
const MAPPING_STATUS_COLUMN: usize = 4;
const INTERNAL_ID_COLUMN: usize = 5;
const PARSER_SHAPE_COLUMN: usize = 6;
const SCENARIO_EVIDENCE_COLUMN: usize = 7;
const COVERAGE_STATUS_COLUMN: usize = 8;
const OWNER_COLUMN: usize = 9;
const NEXT_ACTION_COLUMN: usize = 10;
const INVENTORY_STATE: &str = "play";
const INVENTORY_SIDE: &str = "serverbound";
const INVENTORY_WIRE_ID: &str = "0x15";
const INVENTORY_VALENCE_PACKET: &str = "Full";
const INVENTORY_OWNER: &str = "agent";
const INVENTORY_NEXT_ACTION: &str = "add_parser_shape_fixture_before_broad_promotion";
const INVENTORY_NONE: &str = "none";
const INVENTORY_NON_CLAIM: &str = "non_claim";
const INVENTORY_BROAD_COVERED: &str = "broad_covered";
const NEIGHBOR_MOVEMENT_VARIANTS: &[(&str, &str)] = &[
    ("0x14", "PositionAndOnGround"),
    ("0x16", "LookAndOnGround"),
    ("0x17", "OnGroundOnly"),
    ("0x18", "VehicleMoveC2SPacket"),
];

fn validate_evidence(evidence: &KeyValueRecord) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    require_exact(evidence, &mut diagnostics, ROW_ID_KEY, ROW_ID);
    require_exact(evidence, &mut diagnostics, SCENARIO_KEY, SCENARIO);
    require_exact(evidence, &mut diagnostics, RECEIPT_KEY, RECEIPT);
    require_exact(evidence, &mut diagnostics, CLIENT_LOG_KEY, CLIENT_LOG);
    require_exact(evidence, &mut diagnostics, SERVER_LOG_KEY, SERVER_LOG);
    require_exact(
        evidence,
        &mut diagnostics,
        SOURCE_SNAPSHOT_KEY,
        SOURCE_SNAPSHOT,
    );
    require_exact(evidence, &mut diagnostics, PACKET_ROW_KEY, PACKET_ROW);
    require_exact(
        evidence,
        &mut diagnostics,
        PACKET_MAPPING_KEY,
        PACKET_MAPPING,
    );
    require_exact(evidence, &mut diagnostics, PACKET_PARSER_KEY, PACKET_PARSER);
    require_exact(
        evidence,
        &mut diagnostics,
        PACKET_COVERAGE_KEY,
        PACKET_COVERAGE,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        PACKET_EVIDENCE_KEY,
        PACKET_EVIDENCE,
    );
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        CLIENT_REV_KEY,
        CLIENT_STATUS_KEY,
    );
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        VALENCE_REV_KEY,
        VALENCE_STATUS_KEY,
    );
    require_exact(evidence, &mut diagnostics, ACTOR_KEY, ACTOR);
    require_exact(evidence, &mut diagnostics, START_KEY, START);
    require_exact(evidence, &mut diagnostics, TARGET_KEY, TARGET);
    require_exact(evidence, &mut diagnostics, LOOK_KEY, LOOK);
    require_exact(evidence, &mut diagnostics, ON_GROUND_KEY, ON_GROUND);
    require_exact(evidence, &mut diagnostics, TOLERANCE_KEY, TOLERANCE);
    for key in REQUIRED_OK_METRICS {
        require_ok(evidence, &mut diagnostics, key);
    }
    for key in REQUIRED_NONCLAIMS {
        require_exact(evidence, &mut diagnostics, key, "true");
    }
    reject_truthy_overclaims(
        evidence,
        &mut diagnostics,
        BROAD_OVERCLAIM_KEYS,
        TRUTHY_OVERCLAIM_VALUES,
    );

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn fields_for_line<'a>(
    line: &'a str,
    line_number: usize,
    diagnostics: &mut Vec<String>,
) -> Vec<&'a str> {
    let fields = line.split('\t').collect::<Vec<_>>();
    if fields.len() != INVENTORY_COLUMN_COUNT {
        diagnostics.push(format!(
            "inventory line {line_number} expected {INVENTORY_COLUMN_COUNT} columns, got {}",
            fields.len()
        ));
    }
    fields
}

fn require_field(
    fields: &[&str],
    diagnostics: &mut Vec<String>,
    column: usize,
    column_name: &str,
    expected: &str,
) {
    match fields.get(column) {
        Some(actual) if *actual == expected => {}
        Some(actual) => diagnostics.push(format!(
            "inventory {INVENTORY_WIRE_ID} {column_name} expected {expected}, got {actual}"
        )),
        None => diagnostics.push(format!(
            "inventory {INVENTORY_WIRE_ID} missing column {column_name}"
        )),
    }
}

fn validate_target_inventory_row(fields: &[&str], diagnostics: &mut Vec<String>) {
    require_field(
        fields,
        diagnostics,
        MAPPING_STATUS_COLUMN,
        "mapping_status",
        PACKET_MAPPING,
    );
    require_field(
        fields,
        diagnostics,
        INTERNAL_ID_COLUMN,
        "stevenarella_internal_id",
        "PlayerPositionLook",
    );
    require_field(
        fields,
        diagnostics,
        PARSER_SHAPE_COLUMN,
        "parser_shape_status",
        PACKET_PARSER,
    );
    require_field(
        fields,
        diagnostics,
        SCENARIO_EVIDENCE_COLUMN,
        "scenario_evidence",
        PACKET_EVIDENCE,
    );
    require_field(
        fields,
        diagnostics,
        COVERAGE_STATUS_COLUMN,
        "coverage_status",
        PACKET_COVERAGE,
    );
    require_field(fields, diagnostics, OWNER_COLUMN, "owner", INVENTORY_OWNER);
    require_field(
        fields,
        diagnostics,
        NEXT_ACTION_COLUMN,
        "next_action",
        INVENTORY_NEXT_ACTION,
    );
    if fields
        .get(COVERAGE_STATUS_COLUMN)
        .is_some_and(|status| *status == INVENTORY_BROAD_COVERED)
    {
        diagnostics.push("inventory 0x15 must not be broad_covered".to_string());
    }
}

fn validate_neighbor_movement_row(fields: &[&str], diagnostics: &mut Vec<String>) {
    let Some(wire_id) = fields.get(WIRE_ID_COLUMN) else {
        return;
    };
    if !NEIGHBOR_MOVEMENT_VARIANTS
        .iter()
        .any(|(neighbor_wire, _)| neighbor_wire == wire_id)
    {
        return;
    }
    match fields.get(SCENARIO_EVIDENCE_COLUMN) {
        Some(actual) if *actual == INVENTORY_NONE => {}
        Some(actual) => diagnostics.push(format!(
            "inventory {wire_id} scenario_evidence must remain none, got {actual}"
        )),
        None => diagnostics.push(format!("inventory {wire_id} missing scenario_evidence")),
    }
    match fields.get(COVERAGE_STATUS_COLUMN) {
        Some(actual) if *actual == INVENTORY_NON_CLAIM => {}
        Some(actual) => diagnostics.push(format!(
            "inventory {wire_id} must remain non_claim, got {actual}"
        )),
        None => diagnostics.push(format!("inventory {wire_id} missing coverage_status")),
    }
}

fn validate_inventory_text(text: &str) -> Result<(), Vec<String>> {
    let mut diagnostics = Vec::new();
    let mut lines = text.lines();
    match lines.next() {
        Some(header) if header == INVENTORY_HEADER => {}
        Some(header) => diagnostics.push(format!("inventory header mismatch: {header}")),
        None => diagnostics.push("inventory is empty".to_string()),
    }

    let mut target_count = 0usize;
    for (line_offset, line) in lines.enumerate() {
        let line_number = line_offset + 2;
        if line.trim().is_empty() {
            continue;
        }
        let fields = fields_for_line(line, line_number, &mut diagnostics);
        if fields.len() != INVENTORY_COLUMN_COUNT {
            continue;
        }
        let is_target = fields[STATE_COLUMN] == INVENTORY_STATE
            && fields[SIDE_COLUMN] == INVENTORY_SIDE
            && fields[WIRE_ID_COLUMN] == INVENTORY_WIRE_ID
            && fields[VALENCE_PACKET_COLUMN] == INVENTORY_VALENCE_PACKET;
        if is_target {
            target_count += 1;
            validate_target_inventory_row(&fields, &mut diagnostics);
        }
        if fields[STATE_COLUMN] == INVENTORY_STATE && fields[SIDE_COLUMN] == INVENTORY_SIDE {
            validate_neighbor_movement_row(&fields, &mut diagnostics);
        }
    }

    match target_count {
        0 => diagnostics.push(
            "missing inventory row play/serverbound/0x15 Full -> PlayerPositionLook".to_string(),
        ),
        1 => {}
        count => diagnostics.push(format!(
            "duplicate inventory row play/serverbound/0x15 Full count {count}"
        )),
    }

    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}

fn valid_fixture() -> String {
    [
        "row.id=movement-packet-family",
        "scenario.context=ctf-spawn-team-balance-reset",
        "evidence.receipt=docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.receipt.json",
        "evidence.client_log=docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.client-a.log",
        "evidence.server_log=docs/evidence/protocol-763-ctf-spawn-team-balance-reset-2026-06-01.server.log",
        "source.snapshot=docs/evidence/movement-packet-family-stevenarella-source-2026-06-06.rs",
        "packet.row=play/serverbound/0x15 Full -> PlayerPositionLook",
        "packet.inventory.mapping_status=reviewed_override_no_shape_claim",
        "packet.inventory.parser_shape_status=shape_review_missing",
        "packet.inventory.coverage_status=scenario_bounded",
        "packet.inventory.scenario_evidence=movement_packet_family",
        "child.stevenarella.rev=d9caec597041b3443d894701591752d23772e5ae",
        "child.stevenarella.status=clean",
        "child.valence.rev=f40d6d6d5aeee300bdaeb406c475e2607ac3b6a7",
        "child.valence.status=clean",
        "actor.username=compatbota",
        "movement.start=0.000,85.000,0.000",
        "movement.target=-4.0,84.0,4.0",
        "movement.look=yaw=0.0,pitch=0.0",
        "movement.on_ground=true",
        "movement.tolerance=exact_logged_values",
        "metric.client.full_position_look_sent=ok",
        "metric.client.portal_entry_logged=ok",
        "metric.server.red_assignment_correlation=ok",
        "metric.packet.full_variant_bound=ok",
        "nonclaim.movement_physics=true",
        "nonclaim.collision=true",
        "nonclaim.anti_cheat=true",
        "nonclaim.latency_tolerance=true",
        "nonclaim.malicious_client_resilience=true",
        "nonclaim.all_movement_variants=true",
        "nonclaim.full_protocol_compatibility=true",
        "nonclaim.public_server_safety=true",
        "nonclaim.production_readiness=true",
    ]
    .join("\n")
}

fn fixture_with_replacement(old: &str, new: &str) -> String {
    valid_fixture().replace(old, new)
}

fn valid_inventory_fixture() -> String {
    [
        INVENTORY_HEADER,
        "play\tserverbound\t0x14\tPositionAndOnGround\treviewed_override_no_shape_claim\tPlayerPosition\tshape_review_missing\tnone\tnon_claim\tagent\tadd_parser_shape_fixture_before_broad_promotion",
        "play\tserverbound\t0x15\tFull\treviewed_override_no_shape_claim\tPlayerPositionLook\tshape_review_missing\tmovement_packet_family\tscenario_bounded\tagent\tadd_parser_shape_fixture_before_broad_promotion",
        "play\tserverbound\t0x16\tLookAndOnGround\tfallback_alias_non_claim\tnone\tshape_review_missing\tnone\tnon_claim\tagent\tadd_mapping_parser_fixture_and_live_receipt",
        "play\tserverbound\t0x17\tOnGroundOnly\tfallback_alias_non_claim\tnone\tshape_review_missing\tnone\tnon_claim\tagent\tadd_mapping_parser_fixture_and_live_receipt",
        "play\tserverbound\t0x18\tVehicleMoveC2SPacket\tfallback_alias_non_claim\tnone\tshape_review_missing\tnone\tnon_claim\tagent\tadd_mapping_parser_fixture_and_live_receipt",
    ]
    .join("\n")
}

fn inventory_fixture_with_replacement(old: &str, new: &str) -> String {
    valid_inventory_fixture().replace(old, new)
}

fn run_self_test() -> Result<(), String> {
    let valid = KeyValueRecord::parse(&valid_fixture())?;
    validate_evidence(&valid).map_err(|diagnostics| diagnostics.join("; "))?;
    validate_inventory_text(&valid_inventory_fixture())
        .map_err(|diagnostics| diagnostics.join("; "))?;

    let negative_fixtures: &[(&str, String, &str)] = &[
        (
            "missing row id",
            fixture_with_replacement("row.id=movement-packet-family\n", ""),
            "missing row.id",
        ),
        (
            "stale child revision",
            fixture_with_replacement(
                "child.stevenarella.rev=d9caec597041b3443d894701591752d23772e5ae",
                "child.stevenarella.rev=unknown",
            ),
            "child.stevenarella.rev must be concrete",
        ),
        (
            "wrong packet variant",
            fixture_with_replacement(
                "Full -> PlayerPositionLook",
                "PositionAndOnGround -> PlayerPosition",
            ),
            "packet.row expected play/serverbound/0x15 Full",
        ),
        (
            "missing movement target",
            fixture_with_replacement("movement.target=-4.0,84.0,4.0\n", ""),
            "missing movement.target",
        ),
        (
            "tolerance mismatch",
            fixture_with_replacement(
                "movement.tolerance=exact_logged_values",
                "movement.tolerance=loose",
            ),
            "movement.tolerance expected exact_logged_values",
        ),
        (
            "missing server correlation",
            fixture_with_replacement(
                "metric.server.red_assignment_correlation=ok",
                "metric.server.red_assignment_correlation=missing",
            ),
            "metric.server.red_assignment_correlation expected ok",
        ),
        (
            "missing nonclaim",
            fixture_with_replacement("nonclaim.anti_cheat=true\n", ""),
            "missing nonclaim.anti_cheat",
        ),
        (
            "physics overclaim",
            format!("{}\nclaim.movement_physics=true", valid_fixture()),
            "broad overclaim claim.movement_physics=true",
        ),
    ];

    for (name, text, expected) in negative_fixtures {
        let evidence = KeyValueRecord::parse(text).map_err(|error| format!("{name}: {error}"))?;
        let Err(diagnostics) = validate_evidence(&evidence) else {
            return Err(format!("negative fixture {name} unexpectedly passed"));
        };
        if !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains(expected))
        {
            return Err(format!(
                "negative fixture {name} missing diagnostic {expected:?}; got {diagnostics:?}"
            ));
        }
    }

    let negative_inventory_fixtures: &[(&str, String, &str)] = &[
        (
            "missing target inventory row",
            inventory_fixture_with_replacement(
                "play\tserverbound\t0x15\tFull\treviewed_override_no_shape_claim\tPlayerPositionLook\tshape_review_missing\tmovement_packet_family\tscenario_bounded\tagent\tadd_parser_shape_fixture_before_broad_promotion\n",
                "",
            ),
            "missing inventory row play/serverbound/0x15 Full",
        ),
        (
            "wrong scenario evidence",
            inventory_fixture_with_replacement("movement_packet_family", "status_login_play_join"),
            "scenario_evidence expected movement_packet_family",
        ),
        (
            "wrong coverage status",
            inventory_fixture_with_replacement("scenario_bounded", "non_claim"),
            "coverage_status expected scenario_bounded",
        ),
        (
            "broad coverage overclaim",
            inventory_fixture_with_replacement("scenario_bounded", "broad_covered"),
            "coverage_status expected scenario_bounded",
        ),
        (
            "neighbor variant promoted",
            inventory_fixture_with_replacement(
                "0x14\tPositionAndOnGround\treviewed_override_no_shape_claim\tPlayerPosition\tshape_review_missing\tnone\tnon_claim",
                "0x14\tPositionAndOnGround\treviewed_override_no_shape_claim\tPlayerPosition\tshape_review_missing\tmovement_packet_family\tscenario_bounded",
            ),
            "inventory 0x14 scenario_evidence must remain none",
        ),
        (
            "duplicate target inventory row",
            format!(
                "{}\nplay\tserverbound\t0x15\tFull\treviewed_override_no_shape_claim\tPlayerPositionLook\tshape_review_missing\tmovement_packet_family\tscenario_bounded\tagent\tadd_parser_shape_fixture_before_broad_promotion",
                valid_inventory_fixture()
            ),
            "duplicate inventory row play/serverbound/0x15 Full",
        ),
    ];

    for (name, text, expected) in negative_inventory_fixtures {
        let Err(diagnostics) = validate_inventory_text(text) else {
            return Err(format!(
                "negative inventory fixture {name} unexpectedly passed"
            ));
        };
        if !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.contains(expected))
        {
            return Err(format!(
                "negative inventory fixture {name} missing diagnostic {expected:?}; got {diagnostics:?}"
            ));
        }
    }

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.iter().any(|arg| arg == SELF_TEST_FLAG) {
        if let Err(error) = run_self_test() {
            eprintln!("movement packet family self-test failed: {error}");
            process::exit(1);
        }
        println!(
            "movement packet family self-test passed: positive and negative fixtures exercised"
        );
        return;
    }

    let path = args
        .get(1)
        .map(String::as_str)
        .unwrap_or("docs/evidence/movement-packet-family-2026-06-06.kv");
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed to read {path}: {error}");
            process::exit(1);
        }
    };
    let evidence = match KeyValueRecord::parse(&text) {
        Ok(evidence) => evidence,
        Err(error) => {
            eprintln!("failed to parse {path}: {error}");
            process::exit(1);
        }
    };
    if let Err(diagnostics) = validate_evidence(&evidence) {
        for diagnostic in diagnostics {
            eprintln!("movement packet family evidence failed: {diagnostic}");
        }
        process::exit(1);
    }

    let inventory_path = args.get(2).map(String::as_str).unwrap_or(INVENTORY_PATH);
    let inventory_text = match fs::read_to_string(inventory_path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("failed to read {inventory_path}: {error}");
            process::exit(1);
        }
    };
    if let Err(diagnostics) = validate_inventory_text(&inventory_text) {
        for diagnostic in diagnostics {
            eprintln!("movement packet family inventory failed: {diagnostic}");
        }
        process::exit(1);
    }
    println!("movement packet family evidence ok: {path}; inventory ok: {inventory_path}");
}
