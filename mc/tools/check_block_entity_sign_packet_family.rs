mod checker_framework;

use checker_framework::{
    assert_self_test_fixtures, reject_truthy_overclaims, require_clean_child_revision,
    require_exact, require_ok, run_checker, Checker, KeyValueRecord, ValidationResult, TRUE_VALUE,
};

const ROW_ID_KEY: &str = "row.id";
const ROW_ID: &str = "block-entity-sign-packet-family";
const ACTOR_KEY: &str = "contract.actor";
const ACTOR: &str = "compatbot";
const KIND_KEY: &str = "contract.kind";
const KIND: &str = "Sign";
const POSITION_KEY: &str = "contract.position";
const POSITION: &str = "28,64,0";
const TEXT_PAYLOAD_KEY: &str = "contract.text_payload";
const TEXT_PAYLOAD: &str = "MC|Compat|Sign|Persist";
const PAPER_BACKEND_KEY: &str = "backend.paper";
const PAPER_BACKEND: &str = "paper";
const VALENCE_BACKEND_KEY: &str = "backend.valence";
const VALENCE_BACKEND: &str = "valence";
const BLOCK_ENTITY_PACKET_KEY: &str = "packet.block_entity_update";
const BLOCK_ENTITY_PACKET: &str = "play/clientbound/0x08 BlockEntityUpdateS2CPacket";
const BLOCK_ENTITY_PACKET_COVERAGE_KEY: &str = "packet.block_entity_update.coverage";
const SCENARIO_BOUNDED_COVERAGE: &str = "scenario_bounded";
const BLOCK_ENTITY_PACKET_EVIDENCE_KEY: &str = "packet.block_entity_update.scenario_evidence";
const BLOCK_ENTITY_PACKET_EVIDENCE: &str = "block_entity_sign_packet_family";
const BLOCK_ENTITY_PACKET_PARSER_SHAPE_KEY: &str = "packet.block_entity_update.parser_shape";
const SHAPE_REVIEW_MISSING: &str = "shape_review_missing";
const SIGN_EDITOR_OPEN_PACKET_KEY: &str = "packet.sign_editor_open";
const SIGN_EDITOR_OPEN_PACKET: &str = "play/clientbound/0x31 SignEditorOpenS2CPacket";
const UPDATE_SIGN_PACKET_KEY: &str = "packet.update_sign";
const UPDATE_SIGN_PACKET: &str = "play/serverbound/0x2e UpdateSignC2SPacket";
const SIGN_EDITOR_OPEN_COVERAGE_KEY: &str = "packet.sign_editor_open.coverage";
const UPDATE_SIGN_COVERAGE_KEY: &str = "packet.update_sign.coverage";
const NON_CLAIM_COVERAGE: &str = "non_claim";
const PAPER_RECEIPT_KEY: &str = "receipt.paper";
const PAPER_RECEIPT: &str =
    "docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json";
const VALENCE_RECEIPT_KEY: &str = "receipt.valence";
const VALENCE_RECEIPT: &str =
    "docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json";
const PAPER_EVIDENCE_KEY: &str = "evidence.paper";
const PAPER_EVIDENCE: &str =
    "docs/evidence/survival-block-entity-persistence-paper-2026-06-04.evidence";
const VALENCE_EVIDENCE_KEY: &str = "evidence.valence";
const VALENCE_EVIDENCE: &str =
    "docs/evidence/survival-block-entity-persistence-valence-2026-06-04.evidence";
const SOURCE_ROW_KEY: &str = "source.survival_row";
const SOURCE_ROW: &str = "survival-block-entity-persistence-parity";
const STEVENARELLA_REV_KEY: &str = "child.stevenarella.rev";
const STEVENARELLA_STATUS_KEY: &str = "child.stevenarella.status";
const VALENCE_REV_KEY: &str = "child.valence.rev";
const VALENCE_STATUS_KEY: &str = "child.valence.status";
const PAPER_FIXTURE_B3_KEY: &str = "fixture.paper.source_b3";
const PAPER_FIXTURE_B3: &str = "864fedc1f1f645058b9ca061829d7c42e3da8d088e8cc394ab0c8abf6b2f5150";
const ROW_PARITY_LOG_KEY: &str = "log.row_parity";
const ROW_PARITY_LOG: &str =
    "docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log";
const REQUIRED_OK_METRICS: &[&str] = &[
    "metric.paper.sign_payload_observed",
    "metric.valence.sign_payload_observed",
    "metric.paper.post_restart_observed",
    "metric.valence.post_restart_observed",
    "metric.paper.storage_proof",
    "metric.valence.storage_proof",
    "metric.sign_payload_client_observation",
    "metric.paper_valence_row_parity",
    "metric.survival_row_kept_separate",
];
const REQUIRED_NONCLAIMS: &[&str] = &[
    "nonclaim.sign_editor_packets",
    "nonclaim.sign_editing_ui",
    "nonclaim.all_block_entities",
    "nonclaim.arbitrary_nbt",
    "nonclaim.all_sign_text_variants",
    "nonclaim.all_sign_sides",
    "nonclaim.all_block_entity_packet_shapes",
    "nonclaim.broad_parser_shape_coverage",
    "nonclaim.full_protocol_763_compatibility",
    "nonclaim.broad_minecraft_compatibility",
    "nonclaim.public_server_safety",
    "nonclaim.production_readiness",
];
const BROAD_OVERCLAIM_KEYS: &[&str] = &[
    "claim.sign_editor_packets",
    "claim.sign_editing_ui",
    "claim.all_block_entities",
    "claim.arbitrary_nbt",
    "claim.all_sign_text_variants",
    "claim.all_sign_sides",
    "claim.all_block_entity_packet_shapes",
    "claim.broad_parser_shape_coverage",
    "claim.full_protocol_763_compatibility",
    "claim.broad_minecraft_compatibility",
    "claim.public_server_safety",
    "claim.production_readiness",
    "claim.production_ready",
];
const TRUTHY_OVERCLAIM_VALUES: &[&str] = &["true", "yes", "ok", "claimed", "1"];

type Evidence = KeyValueRecord;

fn validate_evidence(evidence: &Evidence) -> ValidationResult {
    let mut diagnostics = Vec::new();
    require_exact(evidence, &mut diagnostics, ROW_ID_KEY, ROW_ID);
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        STEVENARELLA_REV_KEY,
        STEVENARELLA_STATUS_KEY,
    );
    require_clean_child_revision(
        evidence,
        &mut diagnostics,
        VALENCE_REV_KEY,
        VALENCE_STATUS_KEY,
    );
    require_exact(evidence, &mut diagnostics, ACTOR_KEY, ACTOR);
    require_exact(evidence, &mut diagnostics, KIND_KEY, KIND);
    require_exact(evidence, &mut diagnostics, POSITION_KEY, POSITION);
    require_exact(evidence, &mut diagnostics, TEXT_PAYLOAD_KEY, TEXT_PAYLOAD);
    require_exact(evidence, &mut diagnostics, PAPER_BACKEND_KEY, PAPER_BACKEND);
    require_exact(
        evidence,
        &mut diagnostics,
        VALENCE_BACKEND_KEY,
        VALENCE_BACKEND,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        BLOCK_ENTITY_PACKET_KEY,
        BLOCK_ENTITY_PACKET,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        BLOCK_ENTITY_PACKET_COVERAGE_KEY,
        SCENARIO_BOUNDED_COVERAGE,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        BLOCK_ENTITY_PACKET_EVIDENCE_KEY,
        BLOCK_ENTITY_PACKET_EVIDENCE,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        BLOCK_ENTITY_PACKET_PARSER_SHAPE_KEY,
        SHAPE_REVIEW_MISSING,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        SIGN_EDITOR_OPEN_PACKET_KEY,
        SIGN_EDITOR_OPEN_PACKET,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        UPDATE_SIGN_PACKET_KEY,
        UPDATE_SIGN_PACKET,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        SIGN_EDITOR_OPEN_COVERAGE_KEY,
        NON_CLAIM_COVERAGE,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        UPDATE_SIGN_COVERAGE_KEY,
        NON_CLAIM_COVERAGE,
    );
    require_exact(evidence, &mut diagnostics, PAPER_RECEIPT_KEY, PAPER_RECEIPT);
    require_exact(
        evidence,
        &mut diagnostics,
        VALENCE_RECEIPT_KEY,
        VALENCE_RECEIPT,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        PAPER_EVIDENCE_KEY,
        PAPER_EVIDENCE,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        VALENCE_EVIDENCE_KEY,
        VALENCE_EVIDENCE,
    );
    require_exact(evidence, &mut diagnostics, SOURCE_ROW_KEY, SOURCE_ROW);
    require_exact(
        evidence,
        &mut diagnostics,
        PAPER_FIXTURE_B3_KEY,
        PAPER_FIXTURE_B3,
    );
    require_exact(
        evidence,
        &mut diagnostics,
        ROW_PARITY_LOG_KEY,
        ROW_PARITY_LOG,
    );
    for key in REQUIRED_OK_METRICS {
        require_ok(evidence, &mut diagnostics, key);
    }
    for key in REQUIRED_NONCLAIMS {
        require_exact(evidence, &mut diagnostics, key, TRUE_VALUE);
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

fn valid_fixture() -> String {
    [
        "row.id=block-entity-sign-packet-family",
        "child.stevenarella.rev=0123456789abcdef0123456789abcdef01234567",
        "child.stevenarella.status=clean",
        "child.valence.rev=89abcdef0123456789abcdef0123456789abcdef",
        "child.valence.status=clean",
        "contract.actor=compatbot",
        "contract.kind=Sign",
        "contract.position=28,64,0",
        "contract.text_payload=MC|Compat|Sign|Persist",
        "backend.paper=paper",
        "backend.valence=valence",
        "packet.block_entity_update=play/clientbound/0x08 BlockEntityUpdateS2CPacket",
        "packet.block_entity_update.coverage=scenario_bounded",
        "packet.block_entity_update.scenario_evidence=block_entity_sign_packet_family",
        "packet.block_entity_update.parser_shape=shape_review_missing",
        "packet.sign_editor_open=play/clientbound/0x31 SignEditorOpenS2CPacket",
        "packet.update_sign=play/serverbound/0x2e UpdateSignC2SPacket",
        "packet.sign_editor_open.coverage=non_claim",
        "packet.update_sign.coverage=non_claim",
        "receipt.paper=docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json",
        "receipt.valence=docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json",
        "evidence.paper=docs/evidence/survival-block-entity-persistence-paper-2026-06-04.evidence",
        "evidence.valence=docs/evidence/survival-block-entity-persistence-valence-2026-06-04.evidence",
        "source.survival_row=survival-block-entity-persistence-parity",
        "fixture.paper.source_b3=864fedc1f1f645058b9ca061829d7c42e3da8d088e8cc394ab0c8abf6b2f5150",
        "log.row_parity=docs/evidence/survival-block-entity-persistence-row-parity-2026-06-04.run.log",
        "metric.paper.sign_payload_observed=ok",
        "metric.valence.sign_payload_observed=ok",
        "metric.paper.post_restart_observed=ok",
        "metric.valence.post_restart_observed=ok",
        "metric.paper.storage_proof=ok",
        "metric.valence.storage_proof=ok",
        "metric.sign_payload_client_observation=ok",
        "metric.paper_valence_row_parity=ok",
        "metric.survival_row_kept_separate=ok",
        "nonclaim.sign_editor_packets=true",
        "nonclaim.sign_editing_ui=true",
        "nonclaim.all_block_entities=true",
        "nonclaim.arbitrary_nbt=true",
        "nonclaim.all_sign_text_variants=true",
        "nonclaim.all_sign_sides=true",
        "nonclaim.all_block_entity_packet_shapes=true",
        "nonclaim.broad_parser_shape_coverage=true",
        "nonclaim.full_protocol_763_compatibility=true",
        "nonclaim.broad_minecraft_compatibility=true",
        "nonclaim.public_server_safety=true",
        "nonclaim.production_readiness=true",
    ]
    .join("\n")
}

fn fixture_with_replacement(old: &str, new: &str) -> String {
    valid_fixture().replace(old, new)
}

fn run_self_test() -> Result<(), String> {
    let negative_fixtures: &[(&str, String, &str)] = &[
        (
            "missing row id",
            fixture_with_replacement("row.id=block-entity-sign-packet-family\n", ""),
            "missing row.id",
        ),
        (
            "missing paper receipt",
            fixture_with_replacement(
                "receipt.paper=docs/evidence/survival-block-entity-persistence-paper-2026-06-04.receipt.json\n",
                "",
            ),
            "missing receipt.paper",
        ),
        (
            "missing valence receipt",
            fixture_with_replacement(
                "receipt.valence=docs/evidence/survival-block-entity-persistence-valence-2026-06-04.receipt.json\n",
                "",
            ),
            "missing receipt.valence",
        ),
        (
            "unknown child revision",
            fixture_with_replacement(
                "child.stevenarella.rev=0123456789abcdef0123456789abcdef01234567",
                "child.stevenarella.rev=unknown",
            ),
            "child.stevenarella.rev must be concrete",
        ),
        (
            "dirty child revision",
            fixture_with_replacement("child.valence.status=clean", "child.valence.status=dirty"),
            "child.valence.status expected clean",
        ),
        (
            "wrong packet row",
            fixture_with_replacement(
                "packet.block_entity_update=play/clientbound/0x08 BlockEntityUpdateS2CPacket",
                "packet.block_entity_update=play/clientbound/0x31 SignEditorOpenS2CPacket",
            ),
            "packet.block_entity_update expected play/clientbound/0x08 BlockEntityUpdateS2CPacket",
        ),
        (
            "wrong kind",
            fixture_with_replacement("contract.kind=Sign", "contract.kind=Chest"),
            "contract.kind expected Sign",
        ),
        (
            "wrong position",
            fixture_with_replacement("contract.position=28,64,0", "contract.position=29,64,0"),
            "contract.position expected 28,64,0",
        ),
        (
            "wrong text payload",
            fixture_with_replacement(
                "contract.text_payload=MC|Compat|Sign|Persist",
                "contract.text_payload=MC|Compat|Sign|Other",
            ),
            "contract.text_payload expected MC|Compat|Sign|Persist",
        ),
        (
            "wrong backend",
            fixture_with_replacement("backend.paper=paper", "backend.paper=valence"),
            "backend.paper expected paper",
        ),
        (
            "missing sign payload observation",
            fixture_with_replacement("metric.sign_payload_client_observation=ok\n", ""),
            "missing metric.sign_payload_client_observation",
        ),
        (
            "unsupported sign edit promotion",
            fixture_with_replacement(
                "packet.sign_editor_open.coverage=non_claim",
                "packet.sign_editor_open.coverage=scenario_bounded",
            ),
            "packet.sign_editor_open.coverage expected non_claim",
        ),
        (
            "broad block entity overclaim",
            format!("{}\nclaim.all_block_entities=true", valid_fixture()),
            "broad overclaim claim.all_block_entities=true",
        ),
    ];

    assert_self_test_fixtures(&valid_fixture(), validate_evidence, negative_fixtures)
}

struct BlockEntitySignPacketFamilyChecker;

impl Checker for BlockEntitySignPacketFamilyChecker {
    fn usage(&self) -> &'static str {
        "usage: check_block_entity_sign_packet_family (--self-test | <evidence.kv>)"
    }

    fn validate(&self, evidence: &KeyValueRecord) -> ValidationResult {
        validate_evidence(evidence)
    }

    fn self_test(&self) -> Result<(), String> {
        run_self_test()
    }
}

fn main() {
    run_checker(&BlockEntitySignPacketFamilyChecker);
}
