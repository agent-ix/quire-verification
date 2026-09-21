// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Requirement-traced acceptance tests for the Rust verification boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use ix_trace_rs::trace;
use quire_verification::bounded_portfolio::{
    run_bounded_portfolio, run_bounded_portfolio_with_adapter,
};
use quire_verification::catalog::{adapt_quoin_advice, adapt_quoin_catalog};
use quire_verification::contracts::{
    MAX_ARRAY_ITEMS, MAX_JSON_BYTES, MAX_JSON_DEPTH, VerificationError, VerificationErrorCode,
    contract_version, input_kinds, jcs_canonicalize, jcs_equal, jcs_sha256, parse_bounded_json,
    validate_contract, validate_plan_value, validate_resource_envelope,
};
use quire_verification::evidence::{assess_evidence, findings_for, scope_differences};
use quire_verification::fixtures::{
    check_quoin_baseline, render_fixture_packet, validate_all_fixtures,
    validate_fixture_result_references,
};
use quire_verification::planner::build_verification_plan as build_plan_from_validated;
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

type ScopeMutation = (&'static str, fn(&mut Value));

fn fixture(name: &str) -> Value {
    let text = match name {
        "portfolio.json" => include_str!("../fixtures/portfolio.json"),
        "results.json" => include_str!("../fixtures/results.json"),
        "quoin-catalog-export.json" => include_str!("../fixtures/quoin-catalog-export.json"),
        "quoin-advice-export.json" => include_str!("../fixtures/quoin-advice-export.json"),
        _ => panic!("unknown test fixture {name}"),
    };
    serde_json::from_str(text).expect("checked-in fixture must be valid JSON")
}

fn built_plan() -> Value {
    build_fixture_plan(&fixture("portfolio.json")["planRequest"]).expect("fixture plan must build")
}

fn build_fixture_plan(request: &Value) -> Result<Value, VerificationError> {
    let references = validated_fixture_references(request)?;
    build_plan_from_validated(request, &references)
}

fn validated_fixture_references(
    request: &Value,
) -> Result<Vec<ValidatedSharedReference>, VerificationError> {
    let mut envelopes = Vec::new();
    for property in request["properties"]
        .as_array()
        .expect("fixture properties")
    {
        let input = &property["input"];
        match input["kind"].as_str().expect("fixture input kind") {
            "state" => envelopes.push(&input["snapshotRef"]),
            "trace" => envelopes.push(&input["traceRef"]),
            "fault" => {
                envelopes.push(&input["faultModelRef"]);
                envelopes.push(&input["mitigationOracleRef"]);
            }
            "manual" => envelopes.push(&input["reviewProcedureRef"]),
            "relational" => {}
            kind => panic!("unexpected fixture input kind {kind}"),
        }
    }
    let mut support = ReferenceSupport::default();
    let expectations = envelopes
        .iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let wire_identity = value["wire"]["identity"]
                .as_str()
                .expect("fixture wire identity")
                .to_owned();
            let wire_version = value["wire"]["version"]
                .as_str()
                .expect("fixture wire version")
                .to_owned();
            support
                .wire_versions
                .entry(wire_identity.clone())
                .or_default()
                .insert(wire_version.clone());
            let identity = value["identity"]
                .as_str()
                .expect("fixture artifact identity")
                .to_owned();
            ArtifactExpectation {
                ref_version: value["refVersion"]
                    .as_str()
                    .expect("ref version")
                    .to_owned(),
                kind: value["kind"].as_str().expect("artifact kind").to_owned(),
                authority: value["authority"].as_str().expect("authority").to_owned(),
                identity: identity.clone(),
                revision_namespace: value["revision"]["namespace"]
                    .as_str()
                    .expect("revision namespace")
                    .to_owned(),
                revision_value: value["revision"]["value"]
                    .as_str()
                    .expect("revision value")
                    .to_owned(),
                digest: value["digest"].as_str().expect("digest").to_owned(),
                wire_identity,
                wire_version,
                bytes: Some(identity.into_bytes()),
            }
        })
        .collect::<Vec<_>>();
    let mut reader = SharedReferenceReader::new(support);
    envelopes
        .iter()
        .zip(expectations)
        .map(|(envelope, expectation)| {
            reader.read(
                &serde_json::to_vec(envelope).expect("fixture reference serialization"),
                &ReferenceExpectation::Artifact(expectation),
            )
        })
        .collect()
}

fn assess_fixture_evidence(packet: &Value) -> Result<Value, VerificationError> {
    let references = validate_fixture_result_references(packet)?;
    assess_evidence(packet, &references)
}

fn find_by<'a>(values: &'a [Value], key: &str, expected: &str) -> &'a Value {
    values
        .iter()
        .find(|value| value.get(key).and_then(Value::as_str) == Some(expected))
        .expect("expected fixture member")
}

fn indexed_by<'a>(values: &'a [Value], field: &str) -> BTreeMap<&'a str, &'a Value> {
    values
        .iter()
        .map(|value| (value[field].as_str().expect("indexed identifier"), value))
        .collect()
}

fn verification_task<'a>(plan: &'a Value, property_id: &str, technique_id: &str) -> &'a Value {
    plan["tasks"]
        .as_array()
        .expect("plan tasks")
        .iter()
        .find(|task| {
            task["kind"] == "verification"
                && task["propertyId"] == property_id
                && task["techniqueId"] == technique_id
        })
        .expect("selected verification task")
}

fn focused_request(property_id: &str, technique_id: &str) -> Value {
    let mut request = fixture("portfolio.json")["planRequest"].clone();
    request["selections"]
        .as_array_mut()
        .expect("selections")
        .retain(|selection| {
            selection["propertyId"] == property_id && selection["techniqueId"] == technique_id
        });
    request
}

#[trace("TC-001", "FR-001-AC-1", "FR-001-AC-2")]
#[test]
fn tc_001_accepts_only_the_five_public_contracts() {
    let portfolio = fixture("portfolio.json");
    let results = fixture("results.json");
    let catalog =
        adapt_quoin_catalog(&fixture("quoin-catalog-export.json")).expect("catalog must adapt");
    let values = [
        catalog["techniques"][0].clone(),
        portfolio["planRequest"]["capabilities"][0].clone(),
        portfolio["planRequest"]["policy"].clone(),
        built_plan(),
        results["results"][0].clone(),
    ];
    for value in values {
        validate_contract(value).expect("public contract must validate");
    }
    assert!(validate_contract(results["inventory"][0].clone()).is_err());
}

#[trace("TC-002", "FR-001-AC-3", "FR-001-AC-4", "FR-001-AC-5")]
#[test]
fn tc_002_rejects_contradictions_task_drift_and_unknown_versions() {
    let results = fixture("results.json");
    let mut contradiction = results["results"][0].clone();
    contradiction["execution"]["status"] = json!("failed");
    assert!(validate_contract(contradiction).is_err());

    let plan = built_plan();
    let verification_index = plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .position(|task| task["kind"] == "verification")
        .expect("verification task");
    for field_name in [
        "kind",
        "priority",
        "missingPrerequisites",
        "runnerRequest",
        "propertyId",
        "techniqueId",
    ] {
        let mut invalid = plan.clone();
        invalid["tasks"][verification_index]
            .as_object_mut()
            .expect("task object")
            .remove(field_name);
        assert!(validate_plan_value(&invalid).is_err(), "{field_name}");
    }
    let setup_index = plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .position(|task| task["kind"] == "manual-prerequisite")
        .expect("setup task");
    for field_name in ["prerequisite", "requiredBy"] {
        let mut invalid = plan.clone();
        invalid["tasks"][setup_index]
            .as_object_mut()
            .expect("task object")
            .remove(field_name);
        assert!(validate_plan_value(&invalid).is_err(), "{field_name}");
    }
    let mut unknown = plan.clone();
    unknown["tasks"][0]["renamedPriority"] = unknown["tasks"][0]["priority"].clone();
    assert!(validate_plan_value(&unknown).is_err());
    let mut wrong_type = plan.clone();
    wrong_type["tasks"][0]["missingPrerequisites"] = json!("none");
    assert!(validate_plan_value(&wrong_type).is_err());
}

#[trace("TC-002", "FR-028-AC-2")]
#[test]
fn tc_002_refuses_unknown_e02_contract_version_without_fallback() {
    let mut unknown_version = fixture("results.json")["results"][0].clone();
    unknown_version["contractVersion"] = json!("quire-verification/e02-unknown");
    assert_eq!(
        validate_contract(unknown_version)
            .expect_err("unknown contract version must fail")
            .code(),
        VerificationErrorCode::UnsupportedContractVersion
    );
}

#[trace("TC-046", "FR-028-AC-1")]
#[test]
fn tc_046_freezes_e02_accepted_interface_identity() {
    let decision = include_str!("../docs/interface-decision.md");
    for required in [
        "accepted private cross-repository Rust interface",
        "`agent-ix/quire-verification`",
        "`quire-verification/e02-draft-2`",
        "`d2fc0d8134c776d2f1a17fd249f016b18a6ba124`",
        "`293e342c6f5e2c69738956ff5165f75761b0364a`",
        "`9237ce50ce19c6d6c937705bfde4b64adcc9d16b`",
        "exact private cross-repository Rust use",
    ] {
        assert!(decision.contains(required), "missing {required}");
    }
}

#[trace("TC-046", "FR-028-AC-3")]
#[test]
fn tc_046_refuses_lossy_cross_repository_result_projections() {
    let result = fixture("results.json")["results"][0].clone();
    validate_contract(result.clone()).expect("complete result must validate");

    for field in [
        "execution",
        "logical",
        "activation",
        "participation",
        "assumptions",
        "adequacy",
        "provenance",
        "nativeArtifacts",
        "oracleContributions",
    ] {
        let mut lossy = result.clone();
        lossy.as_object_mut().expect("result object").remove(field);
        assert_eq!(
            validate_contract(lossy)
                .expect_err("lossy result projection must fail")
                .code(),
            VerificationErrorCode::SchemaViolation,
            "{field}"
        );
    }
}

#[trace("TC-046", "FR-028-AC-4")]
#[test]
fn tc_046_compiles_the_exact_serde_1_0_228_consumer_graph() {
    const QVERIFY_REVISION: &str = "293e342c6f5e2c69738956ff5165f75761b0364a";
    let consumer = tempfile::tempdir().expect("consumer root");
    fs::create_dir(consumer.path().join("src")).expect("consumer source directory");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let qverify_checkout = consumer.path().join("qverify");
    let clone = Command::new("git")
        .args(["clone", "--quiet", "--no-checkout", "--shared"])
        .arg(manifest_dir)
        .arg(&qverify_checkout)
        .output()
        .expect("local qverify clone must launch");
    assert!(
        clone.status.success(),
        "local qverify clone must succeed:\n{}",
        String::from_utf8_lossy(&clone.stderr)
    );
    let checkout = Command::new("git")
        .args(["-C"])
        .arg(&qverify_checkout)
        .args(["checkout", "--quiet", "--detach", QVERIFY_REVISION])
        .output()
        .expect("exact qverify checkout must launch");
    assert!(
        checkout.status.success(),
        "exact qverify checkout must succeed:\n{}",
        String::from_utf8_lossy(&checkout.stderr)
    );
    let revision = Command::new("git")
        .args(["-C"])
        .arg(&qverify_checkout)
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("qverify revision inspection must launch");
    assert!(revision.status.success());
    assert_eq!(
        String::from_utf8_lossy(&revision.stdout).trim(),
        QVERIFY_REVISION
    );
    fs::write(
        consumer.path().join("Cargo.toml"),
        format!(
            "[package]\nname='e02-consumer-fixture'\nversion='0.0.0'\nedition='2024'\nrust-version='1.98.1'\n[dependencies]\nquire-verification={{path={qverify_checkout:?}}}\nserde={{version='=1.0.228',features=['derive']}}\n"
        ),
    )
    .expect("consumer manifest");
    fs::write(
        consumer.path().join("src/main.rs"),
        "fn main(){assert_eq!(quire_verification::contracts::contract_version(),\"quire-verification/e02-draft-2\");}\n",
    )
    .expect("consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--offline", "--manifest-path"])
        .arg(consumer.path().join("Cargo.toml"))
        .env("CARGO_BUILD_JOBS", "2")
        .env("CARGO_TARGET_DIR", consumer.path().join("target"))
        .output()
        .expect("consumer cargo check must launch");
    assert!(
        output.status.success(),
        "exact consumer graph must compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let lock = fs::read_to_string(consumer.path().join("Cargo.lock"))
        .expect("consumer lockfile must be written");
    assert_eq!(
        lock.matches("name = \"serde\"\nversion = \"1.0.228\"")
            .count(),
        1
    );
    assert!(!lock.contains("name = \"serde\"\nversion = \"1.0.229\""));
    assert!(lock.contains(
        "git+https://github.com/agent-ix/engineering-assurance?rev=9237ce50ce19c6d6c937705bfde4b64adcc9d16b#9237ce50ce19c6d6c937705bfde4b64adcc9d16b"
    ));
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!("../contracts/e02-draft-2.schema.json"))
        ),
        "677d5cd1de786bf260acefa0b8f4a4c51bfbb355507415f14ec1af7246ca4111"
    );
}

#[trace("TC-003", "FR-002-AC-1", "FR-002-AC-2", "NFR-002-AC-1")]
#[test]
fn tc_003_derives_contract_vocabulary_and_types_from_one_schema() {
    assert_eq!(contract_version(), "quire-verification/e02-draft-2");
    assert_eq!(
        input_kinds(),
        ["state", "relational", "trace", "fault", "manual"]
    );
    // PLAT-861: the schema-derived wire types and vocabulary constants (build.rs `TypeSpace`
    // codegen against contracts/e02-draft-2.schema.json) now live in public crate
    // quire-verification-contracts, which this crate re-exports at `contracts`. NFR-002 is
    // "single contract source": this repository's own build.rs must not also derive them, or
    // there would be two sources instead of one.
    let build_script = include_str!("../build.rs");
    assert_eq!(
        build_script
            .matches("contracts/e02-draft-2.schema.json")
            .count(),
        0
    );
    let manifest = include_str!("../Cargo.toml");
    assert!(manifest.contains(
        "quire-verification-contracts = { version = \"=0.1.0\", git = \"https://github.com/agent-ix/quire-verification-contracts\""
    ));
    assert!(
        !Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/contracts.generated.rs")
            .exists()
    );
}

#[trace("TC-004", "FR-002-AC-3")]
#[test]
fn tc_004_rejects_invalid_plan_graphs() {
    let plan = built_plan();
    let mut duplicate = plan.clone();
    let first = duplicate["tasks"][0].clone();
    duplicate["tasks"]
        .as_array_mut()
        .expect("tasks array")
        .push(first);
    assert_eq!(
        validate_plan_value(&duplicate)
            .expect_err("duplicate must fail")
            .code(),
        VerificationErrorCode::DuplicateTaskId
    );

    let mut unknown = plan.clone();
    unknown["tasks"][0]["dependencies"] = json!(["missing-task"]);
    assert_eq!(
        validate_plan_value(&unknown)
            .expect_err("unknown dependency must fail")
            .code(),
        VerificationErrorCode::UnknownTaskDependency
    );

    let mut cycle = plan;
    let first_id = cycle["tasks"][0]["taskId"].clone();
    let second_id = cycle["tasks"][1]["taskId"].clone();
    cycle["tasks"][0]["dependencies"] = json!([second_id]);
    cycle["tasks"][1]["dependencies"] = json!([first_id]);
    assert_eq!(
        validate_plan_value(&cycle)
            .expect_err("cycle must fail")
            .code(),
        VerificationErrorCode::TaskDependencyCycle
    );
}

#[trace("TC-005", "FR-003-AC-1", "NFR-002-AC-2")]
#[trace("StR-001-VC-3")]
#[test]
fn tc_005_executes_the_schema_against_every_fixture() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    let summary = validate_all_fixtures(&directory).expect("fixtures must validate");
    assert_eq!(
        summary
            .iter()
            .map(|item| item.name.as_str())
            .collect::<Vec<_>>(),
        [
            "portfolio.json",
            "quoin-advice-export.json",
            "quoin-catalog-export.json",
            "results.json"
        ]
    );
    assert!(summary.iter().map(|item| item.contracts).sum::<usize>() > 0);
}

#[trace("TC-006", "FR-003-AC-2", "FR-003-AC-3", "NFR-002-AC-2")]
#[test]
fn tc_006_rejects_an_unhandled_fixture_and_ci_runs_validation_first() {
    let temporary = tempfile::tempdir().expect("temporary fixture directory");
    for name in [
        "portfolio.json",
        "quoin-advice-export.json",
        "quoin-catalog-export.json",
        "results.json",
    ] {
        fs::write(
            temporary.path().join(name),
            serde_json::to_vec(&fixture(name)).expect("serialize fixture"),
        )
        .expect("copy fixture");
    }
    fs::write(temporary.path().join("unhandled.json"), b"{}").expect("write adverse fixture");
    assert_eq!(
        validate_all_fixtures(temporary.path())
            .expect_err("unhandled fixture must fail")
            .code(),
        VerificationErrorCode::UnhandledFixture
    );
    let workflow = include_str!("../.github/workflows/ci.yml");
    assert!(workflow.find("validate-fixtures").is_some_and(|validate| {
        workflow
            .find("cargo test")
            .is_some_and(|test| validate < test)
    }));
    let workflows = fs::read_dir(Path::new(env!("CARGO_MANIFEST_DIR")).join(".github/workflows"))
        .expect("workflow directory");
    for entry in workflows {
        let path = entry.expect("workflow entry").path();
        let is_workflow = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "yml" | "yaml"));
        if !is_workflow {
            continue;
        }
        let configuration = fs::read_to_string(&path).expect("workflow configuration");
        let trigger_section = configuration
            .split_once("\njobs:")
            .map_or(configuration.as_str(), |(prefix, _)| prefix);
        assert!(
            trigger_section.contains("\non:\n  workflow_dispatch:"),
            "{} must declare workflow_dispatch",
            path.display()
        );
        for automatic_trigger in [
            "pull_request:",
            "pull_request_target:",
            "push:",
            "schedule:",
            "workflow_run:",
            "repository_dispatch:",
        ] {
            assert!(
                !trigger_section
                    .lines()
                    .any(|line| line.trim() == automatic_trigger),
                "{} declares automatic trigger {automatic_trigger}",
                path.display()
            );
        }
    }
}

#[trace("TC-007", "FR-004-AC-1")]
#[trace("StR-001-VC-3")]
#[test]
fn tc_007_preserves_complete_quoin_method_records() {
    let source = fixture("quoin-catalog-export.json");
    let adapted = adapt_quoin_catalog(&source).expect("catalog must adapt");
    let source_by_id = indexed_by(
        source["catalog"]["methods"].as_array().expect("methods"),
        "id",
    );
    let adapted_by_id = indexed_by(
        adapted["techniques"].as_array().expect("techniques"),
        "techniqueId",
    );
    assert_eq!(source_by_id.len(), adapted_by_id.len());
    for (id, method) in source_by_id {
        let technique = adapted_by_id.get(&id).expect("adapted method");
        assert_eq!(technique["name"], method["name"]);
        assert_eq!(technique["class"], method["class"]);
        assert_eq!(technique["purpose"], method["definition"]);
        assert_eq!(technique["sourceModule"], method["moduleName"]);
    }
}

#[trace("TC-008", "FR-004-AC-2", "FR-004-AC-3", "NFR-003-AC-1")]
#[test]
fn tc_008_exposes_catalog_conflicts_and_inconclusive_advice() {
    let mut source = fixture("quoin-catalog-export.json");
    source["catalog"]["duplicates"] = json!([{"id": "fuzzing", "modules": ["a", "b"]}]);
    source["catalog"]["unreadable"] = json!([{"moduleRoot": "broken", "reason": "unreadable"}]);
    let adapted = adapt_quoin_catalog(&source).expect("diagnostic catalog must adapt");
    assert_eq!(adapted["qualified"], false);
    assert_eq!(
        adapted["diagnostics"]
            .as_array()
            .expect("diagnostics")
            .len(),
        2
    );

    let advice = json!({"obligation": "property", "recommended": [], "inconclusive": true, "mismatch": false, "uncatalogued": false});
    assert_eq!(
        adapt_quoin_advice(&advice).expect("advice must adapt")["residualApplicability"],
        "unknown"
    );
}

#[trace("TC-009", "FR-005-AC-1", "FR-005-AC-2")]
#[trace("StR-001-VC-3")]
#[test]
fn tc_009_builds_ready_tasks_across_all_input_kinds() {
    let plan = built_plan();
    let verification = plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .filter(|task| task["kind"] == "verification")
        .collect::<Vec<_>>();
    assert_eq!(verification.len(), 6);
    let kinds = verification
        .iter()
        .map(|task| task["input"]["kind"].as_str().expect("kind"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        kinds,
        ["fault", "manual", "relational", "state", "trace"]
            .into_iter()
            .collect()
    );
    assert_eq!(plan["evidenceOwner"], "Quoin");
    assert_eq!(plan["sufficiencyOwner"], "authored-human-decision");
    let invocable = verification
        .iter()
        .filter(|task| task["invocationAllowed"] == true)
        .collect::<Vec<_>>();
    assert_eq!(invocable.len(), 2);
    assert!(invocable.iter().all(|task| {
        task["applicability"] == "applicable"
            && task["readiness"] == "ready"
            && task["runnerRequest"]["kind"] == "native-producer"
    }));
    let manual = verification
        .iter()
        .find(|task| task["input"]["kind"] == "manual")
        .expect("manual task");
    assert_eq!(manual["readiness"], "ready");
    assert_eq!(manual["invocationAllowed"], false);
    assert_eq!(manual.get("runnerRequest"), Some(&Value::Null));
}

#[trace("TC-010", "FR-005-AC-3", "FR-005-AC-4", "FR-005-AC-5", "FR-010-AC-2")]
#[test]
fn tc_010_exposes_unsupported_blocked_stale_and_fault_work() {
    let source = fixture("portfolio.json");
    let mut request = source["planRequest"].clone();
    request["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .retain(|item| item["techniqueId"] != "fuzzing");
    request["selections"]
        .as_array_mut()
        .expect("selections")
        .retain(|item| item["techniqueId"] == "fuzzing");
    let plan = build_fixture_plan(&request).expect("unsupported plan");
    let task = find_by(
        plan["tasks"].as_array().expect("tasks"),
        "kind",
        "verification",
    );
    assert_eq!(task["readiness"], "unsupported");
    assert_eq!(task["invocationAllowed"], false);

    let original = built_plan();
    let fault = original["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .find(|task| task["techniqueId"] == "fault-injection" && task["kind"] == "verification")
        .expect("fault task");
    assert_eq!(fault["readiness"], "blocked");
    assert_eq!(
        fault["missingPrerequisites"]
            .as_array()
            .expect("prerequisites")
            .len(),
        3
    );

    let mut stale = source["planRequest"].clone();
    stale["policy"]["validUntil"] = json!("2026-09-01T00:00:00Z");
    stale["selections"]
        .as_array_mut()
        .expect("selections")
        .truncate(1);
    let stale_plan = build_fixture_plan(&stale).expect("stale plan");
    assert_eq!(stale_plan["policyRef"]["freshness"], "stale");
}

#[trace("TC-026", "FR-013-AC-1", "FR-013-AC-2")]
#[trace("FR-013-AC-3", "FR-013-AC-4", "FR-013-AC-5")]
#[test]
fn tc_026_coalesces_plan_level_prerequisites() {
    let source = fixture("portfolio.json");
    let plan = plan_with_every_prerequisite_missing(&source);
    assert_plan_level_setup_tasks(&plan);
    assert_near_match_remains_missing(&source);
    assert_stale_policy_remains_missing(&source);
    assert_delimiter_bearing_selection_ids_remain_distinct(&source);
}

#[trace("TC-027", "FR-005-AC-2", "FR-013-AC-2")]
#[trace("FR-014-AC-1", "FR-014-AC-2")]
#[trace("FR-014-AC-3", "FR-014-AC-4")]
#[test]
fn tc_027_authorizes_only_exact_bounded_native_runner_requests() {
    assert_exact_closed_runner_request();
    assert_non_invocable_requests_are_null();
    assert_budget_refusals_are_explicit();
    assert_budget_contract_boundaries();
    assert_fault_request_requires_all_safety_bindings();
}

fn assert_exact_closed_runner_request() {
    let mut request = focused_request("PROP-STATE", "property-based-testing");
    request["properties"][0]["budget"] = json!({"wallClockSeconds": 5, "parallelism": 1});
    request["budget"] = json!({"wallClockSeconds": 10, "parallelism": 1});
    let plan = build_fixture_plan(&request).expect("bounded native plan");
    let task = verification_task(&plan, "PROP-STATE", "property-based-testing");
    let capability = find_by(
        request["capabilities"].as_array().expect("capabilities"),
        "capabilityId",
        "CAP-PROPERTY",
    );
    let property = find_by(
        request["properties"].as_array().expect("properties"),
        "propertyId",
        "PROP-STATE",
    );
    let expected = json!({
        "kind": "native-producer",
        "adapterId": capability["runner"]["adapterId"],
        "adapterVersion": capability["runner"]["adapterVersion"],
        "capabilityId": capability["capabilityId"],
        "environmentPermissions": capability["environmentPermissions"],
        "budget": property["budget"],
        "nativeArtifactInterface": capability["nativeArtifactInterface"],
        "input": property["input"],
    });
    assert_eq!(task["invocationAllowed"], true);
    assert!(jcs_equal(&task["runnerRequest"], &expected).expect("exact runner JCS"));
    assert_eq!(task["runnerRequest"]["budget"]["wallClockSeconds"], 5);
    assert_eq!(plan["evidenceOwner"], "Quoin");
    assert_eq!(plan["sufficiencyOwner"], "authored-human-decision");

    let task_index = plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .position(|candidate| candidate["taskId"] == task["taskId"])
        .expect("runner task index");
    for field_name in [
        "kind",
        "adapterId",
        "adapterVersion",
        "capabilityId",
        "environmentPermissions",
        "budget",
        "nativeArtifactInterface",
        "input",
    ] {
        let mut invalid = plan.clone();
        invalid["tasks"][task_index]["runnerRequest"]
            .as_object_mut()
            .expect("runner request")
            .remove(field_name);
        assert_eq!(
            validate_plan_value(&invalid)
                .expect_err("missing runner field")
                .code(),
            VerificationErrorCode::SchemaViolation,
            "{field_name}"
        );
    }
    let mut unknown = plan.clone();
    unknown["tasks"][task_index]["runnerRequest"]["evidenceStore"] = json!("invented");
    assert_eq!(
        validate_plan_value(&unknown)
            .expect_err("unknown runner field")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut wrong_type = plan.clone();
    wrong_type["tasks"][task_index]["runnerRequest"]["budget"]["parallelism"] = json!("one");
    assert_eq!(
        validate_plan_value(&wrong_type)
            .expect_err("wrong runner field type")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
}

fn assert_non_invocable_requests_are_null() {
    for applicability in ["not-applicable", "unknown"] {
        let mut request = focused_request("PROP-STATE", "property-based-testing");
        request["selections"][0]["applicability"] = json!(applicability);
        assert_native_request_is_null(&request, "PROP-STATE", "property-based-testing");
    }

    let mut blocked = focused_request("PROP-STATE", "property-based-testing");
    blocked["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .retain(|item| item["kind"] != "harness");
    assert_native_request_is_null(&blocked, "PROP-STATE", "property-based-testing");

    let mut unsupported = focused_request("PROP-STATE", "property-based-testing");
    unsupported["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .retain(|item| item["techniqueId"] != "property-based-testing");
    assert_native_request_is_null(&unsupported, "PROP-STATE", "property-based-testing");

    let mut stale = focused_request("PROP-STATE", "property-based-testing");
    stale["policy"]["validUntil"] = json!("2026-09-01T00:00:00Z");
    assert_native_request_is_null(&stale, "PROP-STATE", "property-based-testing");

    let mut unqualified = focused_request("PROP-STATE", "property-based-testing");
    find_by_mut(
        unqualified["capabilities"]
            .as_array_mut()
            .expect("capabilities"),
        "capabilityId",
        "CAP-PROPERTY",
    )["qualification"] = json!("unqualified");
    assert_native_request_is_null(&unqualified, "PROP-STATE", "property-based-testing");

    let manual = focused_request("PROP-MANUAL", "inspection");
    let manual_plan = build_fixture_plan(&manual).expect("ready manual plan");
    let manual_task = verification_task(&manual_plan, "PROP-MANUAL", "inspection");
    assert_eq!(manual_task["readiness"], "ready");
    assert_eq!(manual_task["invocationAllowed"], false);
    assert_eq!(manual_task.get("runnerRequest"), Some(&Value::Null));
}

fn assert_native_request_is_null(request: &Value, property_id: &str, technique_id: &str) {
    let plan = build_fixture_plan(request).expect("non-invocable plan");
    let task = verification_task(&plan, property_id, technique_id);
    assert_eq!(task["invocationAllowed"], false);
    assert_eq!(task.get("runnerRequest"), Some(&Value::Null));
}

fn find_by_mut<'a>(values: &'a mut [Value], key: &str, expected: &str) -> &'a mut Value {
    values
        .iter_mut()
        .find(|value| value.get(key).and_then(Value::as_str) == Some(expected))
        .expect("expected mutable fixture member")
}

fn assert_budget_refusals_are_explicit() {
    let mut absent = focused_request("PROP-STATE", "property-based-testing");
    absent
        .as_object_mut()
        .expect("plan request")
        .remove("budget");
    let absent_plan = build_fixture_plan(&absent).expect("absent-budget plan");
    let absent_task = verification_task(&absent_plan, "PROP-STATE", "property-based-testing");
    let missing = json!({
        "kind": "resource-allocation",
        "id": "budget:CAP-PROPERTY",
        "required": true,
    });
    assert_eq!(absent_task["readiness"], "blocked");
    assert_eq!(absent_task["missingPrerequisites"], json!([missing]));
    assert_eq!(absent_task["invocationAllowed"], false);
    assert_eq!(absent_task.get("runnerRequest"), Some(&Value::Null));
    let setup = find_by(
        absent_plan["tasks"].as_array().expect("tasks"),
        "taskId",
        "setup:19:resource-allocation:19:budget:CAP-PROPERTY",
    );
    assert_eq!(setup["prerequisite"], missing);

    for (budget, limits) in [
        (
            json!({"wallClockSeconds": 6, "parallelism": 1}),
            json!({"maxWallClockSeconds": 5, "maxParallelism": 1}),
        ),
        (
            json!({"wallClockSeconds": 5, "parallelism": 2}),
            json!({"maxWallClockSeconds": 5, "maxParallelism": 1}),
        ),
    ] {
        let mut request = focused_request("PROP-STATE", "property-based-testing");
        request["budget"] = budget.clone();
        find_by_mut(
            request["capabilities"]
                .as_array_mut()
                .expect("capabilities"),
            "capabilityId",
            "CAP-PROPERTY",
        )["resourceLimits"] = limits.clone();
        let plan = build_fixture_plan(&request).expect("over-limit plan");
        let task = verification_task(&plan, "PROP-STATE", "property-based-testing");
        assert_eq!(task["readiness"], "blocked");
        assert_eq!(
            task["gaps"],
            json!([{"kind": "budget-exceeded", "requested": budget, "limits": limits}])
        );
        assert_eq!(task["invocationAllowed"], false);
        assert_eq!(task.get("runnerRequest"), Some(&Value::Null));
    }
}

fn assert_budget_contract_boundaries() {
    assert_budget_input_boundaries();
    assert_budget_refusal_record_boundaries();
}

fn assert_budget_input_boundaries() {
    let native = focused_request("PROP-STATE", "property-based-testing");
    let native_capability_index = native["capabilities"]
        .as_array()
        .expect("capabilities")
        .iter()
        .position(|item| item["capabilityId"] == "CAP-PROPERTY")
        .expect("native capability index");

    let mut missing_limits = native.clone();
    missing_limits["capabilities"][native_capability_index]
        .as_object_mut()
        .expect("native capability")
        .remove("resourceLimits");
    assert_schema_violation(&missing_limits, "native capability without limits");

    for (field, invalid_value) in [
        ("maxWallClockSeconds", json!(0)),
        ("maxWallClockSeconds", json!(11)),
        ("maxParallelism", json!(0)),
    ] {
        let mut invalid = native.clone();
        invalid["capabilities"][native_capability_index]["resourceLimits"][field] = invalid_value;
        assert_schema_violation(&invalid, "invalid native resource limit");
    }

    let mut manual_with_limits = focused_request("PROP-MANUAL", "inspection");
    let manual_capability = find_by_mut(
        manual_with_limits["capabilities"]
            .as_array_mut()
            .expect("capabilities"),
        "capabilityId",
        "CAP-MANUAL",
    );
    manual_capability["resourceLimits"] = json!({"maxWallClockSeconds": 10, "maxParallelism": 1});
    assert_schema_violation(&manual_with_limits, "manual capability with limits");

    for (field, invalid_value) in [
        ("wallClockSeconds", json!(0)),
        ("wallClockSeconds", json!(11)),
        ("parallelism", json!(0)),
    ] {
        let mut invalid = native.clone();
        invalid["budget"][field] = invalid_value;
        assert_schema_violation(&invalid, "invalid plan budget");
    }
    let mut unknown_budget_field = native.clone();
    unknown_budget_field["budget"]["cpuSeconds"] = json!(1);
    assert_schema_violation(&unknown_budget_field, "unknown plan budget field");

    let mut incomplete_property_budget = native.clone();
    incomplete_property_budget["properties"][0]["budget"] =
        json!({"wallClockSeconds": 5, "parallelism": 1});
    incomplete_property_budget["properties"][0]["budget"]
        .as_object_mut()
        .expect("property budget")
        .remove("parallelism");
    assert_schema_violation(&incomplete_property_budget, "incomplete property budget");
    let mut unknown_property_budget_field = native.clone();
    unknown_property_budget_field["properties"][0]["budget"] =
        json!({"wallClockSeconds": 5, "parallelism": 1});
    unknown_property_budget_field["properties"][0]["budget"]["cpuSeconds"] = json!(1);
    assert_schema_violation(
        &unknown_property_budget_field,
        "unknown property budget field",
    );
}

fn assert_budget_refusal_record_boundaries() {
    let native = focused_request("PROP-STATE", "property-based-testing");
    let native_capability_index = native["capabilities"]
        .as_array()
        .expect("capabilities")
        .iter()
        .position(|item| item["capabilityId"] == "CAP-PROPERTY")
        .expect("native capability index");
    let mut over_limit = native.clone();
    over_limit["budget"] = json!({"wallClockSeconds": 6, "parallelism": 1});
    over_limit["capabilities"][native_capability_index]["resourceLimits"] =
        json!({"maxWallClockSeconds": 5, "maxParallelism": 1});
    let over_limit_plan = build_fixture_plan(&over_limit).expect("over-limit plan");
    let over_limit_task =
        verification_task(&over_limit_plan, "PROP-STATE", "property-based-testing");
    let over_limit_task_index = over_limit_plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .position(|item| item["taskId"] == over_limit_task["taskId"])
        .expect("over-limit task index");
    for field in ["requested", "limits"] {
        let mut invalid = over_limit_plan.clone();
        invalid["tasks"][over_limit_task_index]["gaps"][0]
            .as_object_mut()
            .expect("budget gap")
            .remove(field);
        assert_invalid_plan(&invalid, "incomplete budget gap");
    }
    let mut unknown_gap_field = over_limit_plan.clone();
    unknown_gap_field["tasks"][over_limit_task_index]["gaps"][0]["available"] = json!(true);
    assert_invalid_plan(&unknown_gap_field, "unknown budget gap field");
    let mut wrong_gap_type = over_limit_plan.clone();
    wrong_gap_type["tasks"][over_limit_task_index]["gaps"][0]["requested"]["parallelism"] =
        json!("one");
    assert_invalid_plan(&wrong_gap_type, "wrong budget gap field type");

    let mut absent = native;
    absent
        .as_object_mut()
        .expect("plan request")
        .remove("budget");
    let absent_plan = build_fixture_plan(&absent).expect("absent-budget plan");
    let absent_task = verification_task(&absent_plan, "PROP-STATE", "property-based-testing");
    let absent_task_index = absent_plan["tasks"]
        .as_array()
        .expect("tasks")
        .iter()
        .position(|item| item["taskId"] == absent_task["taskId"])
        .expect("absent-budget task index");
    let mut missing_required = absent_plan.clone();
    missing_required["tasks"][absent_task_index]["missingPrerequisites"][0]
        .as_object_mut()
        .expect("budget prerequisite")
        .remove("required");
    assert_invalid_plan(&missing_required, "incomplete budget prerequisite");
    let mut false_required = absent_plan.clone();
    false_required["tasks"][absent_task_index]["missingPrerequisites"][0]["required"] =
        json!(false);
    assert_invalid_plan(&false_required, "optional budget prerequisite");
    let mut unknown_prerequisite_field = absent_plan;
    unknown_prerequisite_field["tasks"][absent_task_index]["missingPrerequisites"][0]["description"] =
        json!("invented");
    assert_invalid_plan(
        &unknown_prerequisite_field,
        "unknown budget prerequisite field",
    );
}

fn assert_schema_violation(request: &Value, context: &str) {
    assert_eq!(
        build_fixture_plan(request).expect_err(context).code(),
        VerificationErrorCode::SchemaViolation,
        "{context}"
    );
}

fn assert_invalid_plan(plan: &Value, context: &str) {
    assert_eq!(
        validate_plan_value(plan).expect_err(context).code(),
        VerificationErrorCode::SchemaViolation,
        "{context}"
    );
}

fn assert_fault_request_requires_all_safety_bindings() {
    let mut request = focused_request("PROP-FAULT", "fault-injection");
    let safety = [
        ("environment-authorization", "non-production-fault-scope"),
        ("injection-binding", "confirmed-injection-site"),
        ("observation-binding", "independent-effect-observation"),
    ];
    request["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .extend(
            safety
                .iter()
                .map(|(kind, id)| json!({"kind": kind, "id": id})),
        );
    let complete = build_fixture_plan(&request).expect("authorized fault plan");
    let complete_task = verification_task(&complete, "PROP-FAULT", "fault-injection");
    assert_eq!(complete_task["readiness"], "ready");
    assert_eq!(complete_task["invocationAllowed"], true);
    assert_eq!(
        complete_task["runnerRequest"]["input"],
        request["properties"][3]["input"]
    );

    for (missing_kind, missing_id) in safety {
        let mut incomplete = request.clone();
        incomplete["availablePrerequisites"]
            .as_array_mut()
            .expect("available prerequisites")
            .retain(|item| item["kind"] != missing_kind || item["id"] != missing_id);
        assert_native_request_is_null(&incomplete, "PROP-FAULT", "fault-injection");
    }
}

#[trace("TC-030", "FR-013-AC-2", "FR-015-AC-3")]
#[trace("FR-014-AC-1", "FR-014-AC-2")]
#[trace("FR-015-AC-5")]
#[test]
fn tc_030_keeps_parser_portfolio_roles_and_runner_eligibility_separate() {
    let request = prepared_parser_portfolio_request();
    let plan = build_fixture_plan(&request).expect("prepared parser portfolio");
    let state = verification_task(&plan, "PROP-STATE", "property-based-testing");
    let relational = verification_task(&plan, "PROP-RELATION", "metamorphic-testing");
    let manual = verification_task(&plan, "PROP-MANUAL", "inspection");
    for task in [state, relational] {
        assert_eq!(task["readiness"], "ready");
        assert_eq!(task["invocationAllowed"], true);
        assert_eq!(task["runnerRequest"]["kind"], "native-producer");
        assert!(
            jcs_equal(&task["input"], &task["runnerRequest"]["input"]).expect("role input JCS")
        );
    }
    assert_eq!(manual["readiness"], "ready");
    assert_eq!(manual["invocationAllowed"], false);
    assert_eq!(manual.get("runnerRequest"), Some(&Value::Null));

    let mut missing_harness = request.clone();
    missing_harness["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .retain(|item| item["kind"] != "harness");
    let missing_plan = build_fixture_plan(&missing_harness).expect("missing harness plan");
    assert_eq!(
        verification_task(&missing_plan, "PROP-STATE", "property-based-testing")["readiness"],
        "blocked"
    );
    assert_eq!(
        verification_task(&missing_plan, "PROP-RELATION", "metamorphic-testing")["invocationAllowed"],
        true
    );

    let mut unsupported = request.clone();
    unsupported["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .retain(|item| item["capabilityId"] != "CAP-RELATION");
    let unsupported_plan = build_fixture_plan(&unsupported).expect("unsupported relation plan");
    assert_eq!(
        verification_task(&unsupported_plan, "PROP-RELATION", "metamorphic-testing")["readiness"],
        "unsupported"
    );

    let mut absent_budget = request.clone();
    absent_budget
        .as_object_mut()
        .expect("plan request")
        .remove("budget");
    let absent_plan = build_fixture_plan(&absent_budget).expect("absent portfolio budget");
    for (property, technique) in [
        ("PROP-STATE", "property-based-testing"),
        ("PROP-RELATION", "metamorphic-testing"),
    ] {
        let task = verification_task(&absent_plan, property, technique);
        assert_eq!(task["readiness"], "blocked");
        assert_eq!(task["invocationAllowed"], false);
        assert_eq!(task.get("runnerRequest"), Some(&Value::Null));
    }
    let absent_manual = verification_task(&absent_plan, "PROP-MANUAL", "inspection");
    assert_eq!(absent_manual["readiness"], "ready");
    assert_eq!(absent_manual["invocationAllowed"], false);
}

fn prepared_parser_portfolio_request() -> Value {
    let mut request = fixture("portfolio.json")["planRequest"].clone();
    request["selections"]
        .as_array_mut()
        .expect("selections")
        .retain(|selection| {
            matches!(
                (
                    selection["propertyId"].as_str(),
                    selection["techniqueId"].as_str()
                ),
                (Some("PROP-STATE"), Some("property-based-testing"))
                    | (Some("PROP-RELATION"), Some("metamorphic-testing"))
                    | (Some("PROP-MANUAL"), Some("inspection"))
            )
        });
    find_by_mut(
        request["capabilities"]
            .as_array_mut()
            .expect("capabilities"),
        "capabilityId",
        "CAP-RELATION",
    )["qualification"] = json!("qualified");
    request["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .extend([
            json!({"kind": "common-domain", "id": "parser-roundtrip-domain"}),
            json!({"kind": "relation-binding", "id": "roundtrip-equivalence"}),
        ]);
    request
}

#[trace("TC-028", "FR-002-AC-4", "NFR-002-AC-3")]
#[trace("FR-015-AC-1", "FR-015-AC-2")]
#[trace("FR-015-AC-3", "FR-015-AC-4")]
#[test]
fn tc_028_preserves_closed_role_inputs_and_requires_strict_reader_outputs() {
    let source = fixture("portfolio.json");
    let request = &source["planRequest"];
    let references = validated_fixture_references(request).expect("fixture references");
    assert_role_inputs_round_trip(request, &references);
    assert_shared_definition_is_external();
    assert_required_input_fields(request, &references);
    assert_invalid_role_inputs_are_refused(request, &references);
}

fn assert_role_inputs_round_trip(request: &Value, references: &[ValidatedSharedReference]) {
    let plan = build_plan_from_validated(request, references).expect("role-correct plan");
    let tasks = plan["tasks"].as_array().expect("plan tasks");
    for property in request["properties"].as_array().expect("properties") {
        let property_id = property["propertyId"].as_str().expect("property id");
        let task = tasks
            .iter()
            .find(|task| {
                task["kind"] == "verification" && task["propertyId"].as_str() == Some(property_id)
            })
            .expect("verification task for property");
        assert!(jcs_equal(&task["input"], &property["input"]).expect("JCS comparison"));
    }
    let manual = tasks
        .iter()
        .find(|task| task["input"]["kind"] == "manual")
        .expect("manual task");
    assert_eq!(
        manual["input"]["requiredDisposition"],
        json!(["accept", "revise", "reject"])
    );
    assert_eq!(manual["invocationAllowed"], false);
    assert_eq!(manual.get("runnerRequest"), Some(&Value::Null));
}

fn assert_shared_definition_is_external() {
    let schema: Value = serde_json::from_str(include_str!("../contracts/e02-draft-2.schema.json"))
        .expect("authoritative schema");
    assert!(schema["$defs"].get("ArtifactRef").is_none());
    assert_eq!(
        schema.pointer("/$defs/SharedArtifactEnvelope/properties/value/$ref"),
        Some(&json!("urn:ix:shared-reference:2-draft#/$defs/ArtifactRef"))
    );
    // PLAT-861: resolving that external $ref at build time (`type_space.add_ref_types`) is
    // now quire-verification-contracts's build.rs, not this repository's — this crate no
    // longer derives E02 wire types at all (see tc_003_derives_contract_vocabulary...).
    let build_script = include_str!("../build.rs");
    assert!(!build_script.contains("type_space.add_ref_types"));
}

fn assert_required_input_fields(request: &Value, references: &[ValidatedSharedReference]) {
    for (property_index, property) in request["properties"]
        .as_array()
        .expect("properties")
        .iter()
        .enumerate()
    {
        for required in property["input"].as_object().expect("closed input").keys() {
            let mut missing = request.clone();
            missing["properties"][property_index]["input"]
                .as_object_mut()
                .expect("input object")
                .remove(required);
            assert_eq!(
                build_plan_from_validated(&missing, references)
                    .expect_err("required input field")
                    .code(),
                VerificationErrorCode::SchemaViolation,
                "input kind {} accepted without {required}",
                property["input"]["kind"]
            );
        }
    }
}

fn assert_invalid_role_inputs_are_refused(
    request: &Value,
    references: &[ValidatedSharedReference],
) {
    let mut aliased = request.clone();
    aliased["properties"][1]["input"]["rightSubject"] =
        aliased["properties"][1]["input"]["leftSubject"].clone();
    assert_eq!(
        build_plan_from_validated(&aliased, references)
            .expect_err("equal relational subjects")
            .code(),
        VerificationErrorCode::InvalidInput
    );

    let reference_pointers = [
        "/properties/0/input/snapshotRef",
        "/properties/2/input/traceRef",
        "/properties/3/input/faultModelRef",
        "/properties/3/input/mitigationOracleRef",
        "/properties/4/input/reviewProcedureRef",
    ];
    for pointer in reference_pointers {
        let mut wrong_version = request.clone();
        wrong_version
            .pointer_mut(&format!("{pointer}/wireVersion"))
            .expect("wire version")
            .clone_from(&json!("ix.shared-reference/1-draft"));
        assert_eq!(
            build_plan_from_validated(&wrong_version, references)
                .expect_err("wrong shared-reference version")
                .code(),
            VerificationErrorCode::SchemaViolation
        );

        let mut wrong_artifact_version = request.clone();
        wrong_artifact_version
            .pointer_mut(&format!("{pointer}/value/refVersion"))
            .expect("artifact reference version")
            .clone_from(&json!("ix.artifact-ref/2-draft"));
        assert_eq!(
            build_plan_from_validated(&wrong_artifact_version, references)
                .expect_err("wrong artifact-reference version")
                .code(),
            VerificationErrorCode::SchemaViolation
        );

        let mut wrong_role = request.clone();
        wrong_role
            .pointer_mut(&format!("{pointer}/value/kind"))
            .expect("artifact role")
            .clone_from(&json!("observation"));
        assert_eq!(
            build_plan_from_validated(&wrong_role, references)
                .expect_err("wrong artifact role")
                .code(),
            VerificationErrorCode::SchemaViolation
        );

        let mut unvalidated = request.clone();
        unvalidated
            .pointer_mut(&format!("{pointer}/value/identity"))
            .expect("artifact identity")
            .clone_from(&json!("artifact:not-read"));
        assert_eq!(
            build_plan_from_validated(&unvalidated, references)
                .expect_err("unvalidated envelope")
                .code(),
            VerificationErrorCode::IdentityMismatch
        );
    }

    let mut cross_variant = request.clone();
    cross_variant["properties"][0]["input"]["relation"] = json!("foreign");
    assert_eq!(
        build_plan_from_validated(&cross_variant, references)
            .expect_err("cross-variant input field")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut unknown = request.clone();
    unknown["properties"][2]["input"]["unknown"] = json!(true);
    assert_eq!(
        build_plan_from_validated(&unknown, references)
            .expect_err("unknown input field")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut duplicate = request.clone();
    duplicate["properties"][4]["input"]["requiredDisposition"] = json!(["accept", "accept"]);
    assert_eq!(
        build_plan_from_validated(&duplicate, references)
            .expect_err("duplicate manual disposition")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut draft_1 = request.clone();
    draft_1["policy"]["contractVersion"] = json!("quire-verification/e02-draft-1");
    assert_eq!(
        build_plan_from_validated(&draft_1, references)
            .expect_err("draft-1 request")
            .code(),
        VerificationErrorCode::UnsupportedContractVersion
    );
}

fn plan_with_every_prerequisite_missing(source: &Value) -> Value {
    let mut request = source["planRequest"].clone();
    let property_capability = request["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .iter_mut()
        .find(|item| item["capabilityId"] == "CAP-PROPERTY")
        .expect("property capability");
    property_capability["prerequisites"]
        .as_array_mut()
        .expect("prerequisites")
        .push(json!({
            "kind": "resource-allocation",
            "id": "cpu-seconds-10",
            "required": true
        }));
    request["availablePrerequisites"] = json!([]);
    build_fixture_plan(&request).expect("blocked plan")
}

fn assert_plan_level_setup_tasks(plan: &Value) {
    let tasks = plan["tasks"].as_array().expect("tasks");
    let setup_ids = tasks
        .iter()
        .filter(|task| task["kind"] == "manual-prerequisite")
        .map(|task| task["taskId"].as_str().expect("setup task id"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        setup_ids,
        [
            "setup:24:capability-qualification:12:CAP-RELATION",
            "setup:24:capability-qualification:9:CAP-TRACE",
            "setup:13:common-domain:23:parser-roundtrip-domain",
            "setup:6:corpus:12:parser-seeds",
            "setup:25:environment-authorization:26:non-production-fault-scope",
            "setup:9:generator:22:parser-state-generator",
            "setup:7:harness:14:parser-harness",
            "setup:17:injection-binding:24:confirmed-injection-site",
            "setup:19:observation-binding:30:independent-effect-observation",
            "setup:19:observation-binding:20:parser-event-binding",
            "setup:16:relation-binding:21:roundtrip-equivalence",
            "setup:19:resource-allocation:14:cpu-seconds-10",
            "setup:8:reviewer:12:domain-owner",
        ]
        .into_iter()
        .collect()
    );
    for setup in tasks
        .iter()
        .filter(|task| task["kind"] == "manual-prerequisite")
    {
        assert!(setup.get("propertyId").is_none());
        assert!(setup.get("techniqueId").is_none());
        assert!(setup.get("input").is_none());
        let required_by = setup["requiredBy"].as_array().expect("requiredBy");
        assert!(!required_by.is_empty());
        assert!(required_by.windows(2).all(|pair| {
            pair[0].as_str().expect("requiredBy identifier")
                < pair[1].as_str().expect("requiredBy identifier")
        }));
        assert_eq!(setup["missingPrerequisites"][0], setup["prerequisite"]);
    }
    let harness_id = "setup:7:harness:14:parser-harness";
    let setup = find_by(tasks, "taskId", harness_id);
    assert_eq!(
        setup["prerequisite"],
        json!({"kind":"harness","id":"parser-harness","required":true})
    );
    assert_eq!(
        setup["requiredBy"],
        json!([
            "verify:10:PROP-STATE:22:property-based-testing",
            "verify:10:PROP-STATE:7:fuzzing"
        ])
    );
    assert_eq!(
        tasks
            .iter()
            .filter(|task| task["taskId"] == harness_id)
            .count(),
        1
    );
    for id in [
        "verify:10:PROP-STATE:7:fuzzing",
        "verify:10:PROP-STATE:22:property-based-testing",
    ] {
        let task = find_by(tasks, "taskId", id);
        assert_eq!(task["readiness"], "blocked");
        assert!(
            task["dependencies"]
                .as_array()
                .expect("dependencies")
                .contains(&json!(harness_id))
        );
    }
}

fn assert_near_match_remains_missing(source: &Value) {
    let mut near_match = source["planRequest"].clone();
    near_match["selections"]
        .as_array_mut()
        .expect("selections")
        .truncate(1);
    near_match["availablePrerequisites"] = json!([
        {"kind": "generator", "id": "parser-state-generator-near"},
        {"kind": "near-generator", "id": "parser-state-generator"},
        {"kind": "harness", "id": "parser-harness"}
    ]);
    near_match["capabilities"][0]["prerequisites"]
        .as_array_mut()
        .expect("prerequisites")
        .push(json!({"kind": "a:b", "id": "c", "required": true}));
    near_match["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .push(json!({"kind": "a", "id": "b:c"}));
    let near_plan = build_fixture_plan(&near_match).expect("near-match plan");
    let near_tasks = near_plan["tasks"].as_array().expect("near-match tasks");
    assert!(
        near_tasks
            .iter()
            .any(|task| { task["taskId"] == "setup:9:generator:22:parser-state-generator" })
    );
    assert!(
        !near_tasks
            .iter()
            .any(|task| task["taskId"] == "setup:7:harness:14:parser-harness")
    );
    assert!(
        near_tasks
            .iter()
            .any(|task| task["taskId"] == "setup:3:a:b:1:c")
    );
}

fn assert_stale_policy_remains_missing(source: &Value) {
    let mut stale = source["planRequest"].clone();
    stale["selections"]
        .as_array_mut()
        .expect("selections")
        .truncate(1);
    stale["asOf"] = json!("2026-10-01T00:00:00Z");
    stale["availablePrerequisites"]
        .as_array_mut()
        .expect("available prerequisites")
        .push(json!({
            "kind": "policy-freshness",
            "id": "POLICY-PARSER-FIXTURE@1"
        }));
    let stale_plan = build_fixture_plan(&stale).expect("stale plan");
    let stale_tasks = stale_plan["tasks"].as_array().expect("stale tasks");
    assert!(
        stale_tasks.iter().any(|task| {
            task["taskId"] == "setup:16:policy-freshness:23:POLICY-PARSER-FIXTURE@1"
        })
    );
    assert_eq!(
        find_by(
            stale_tasks,
            "taskId",
            "verify:10:PROP-STATE:22:property-based-testing"
        )["invocationAllowed"],
        false
    );
}

fn assert_delimiter_bearing_selection_ids_remain_distinct(source: &Value) {
    let mut request = source["planRequest"].clone();
    let property = request["properties"][0].clone();
    let capability = request["capabilities"][0].clone();
    let selection = request["selections"][0].clone();
    request["properties"] = json!([property.clone(), property]);
    request["properties"][0]["propertyId"] = json!("a:b");
    request["properties"][1]["propertyId"] = json!("a");
    request["capabilities"] = json!([capability.clone(), capability]);
    request["capabilities"][0]["capabilityId"] = json!("CAP-C");
    request["capabilities"][0]["techniqueId"] = json!("c");
    request["capabilities"][0]["prerequisites"] = json!([]);
    request["capabilities"][1]["capabilityId"] = json!("CAP-B-C");
    request["capabilities"][1]["techniqueId"] = json!("b:c");
    request["capabilities"][1]["prerequisites"] = json!([]);
    request["selections"] = json!([selection.clone(), selection]);
    request["selections"][0]["propertyId"] = json!("a:b");
    request["selections"][0]["techniqueId"] = json!("c");
    request["selections"][1]["propertyId"] = json!("a");
    request["selections"][1]["techniqueId"] = json!("b:c");

    let plan = build_fixture_plan(&request).expect("delimiter-bearing plan");
    let tasks = plan["tasks"].as_array().expect("tasks");
    assert!(
        tasks
            .iter()
            .any(|task| task["taskId"] == "verify:3:a:b:1:c")
    );
    assert!(
        tasks
            .iter()
            .any(|task| task["taskId"] == "verify:1:a:3:b:c")
    );
}

#[trace("TC-011", "FR-005-AC-6")]
#[test]
fn tc_011_rejects_ambiguous_capabilities() {
    let source = fixture("portfolio.json");
    let mut request = source["planRequest"].clone();
    let mut duplicate = request["capabilities"][0].clone();
    duplicate["capabilityId"] = json!("CAP-PROPERTY-SECOND");
    request["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .push(duplicate);
    request["selections"]
        .as_array_mut()
        .expect("selections")
        .truncate(1);
    assert_eq!(
        build_fixture_plan(&request)
            .expect_err("ambiguous capability must fail")
            .code(),
        VerificationErrorCode::AmbiguousCapability
    );
}

#[trace("TC-024", "FR-010-AC-1", "FR-011-AC-1", "FR-011-AC-2")]
#[test]
fn tc_024_keeps_applicability_readiness_and_classification_independent() {
    let source = fixture("portfolio.json");
    for applicability in ["applicable", "not-applicable", "unknown"] {
        let mut request = source["planRequest"].clone();
        request["selections"]
            .as_array_mut()
            .expect("selections")
            .truncate(1);
        request["selections"][0]["applicability"] = json!(applicability);
        request["properties"][0]["classification"] = json!("safety-critical");
        let plan = build_fixture_plan(&request).expect("qualified plan");
        let task = find_by(
            plan["tasks"].as_array().expect("tasks"),
            "kind",
            "verification",
        );
        assert_eq!(task["applicability"], applicability);
        assert_eq!(task["readiness"], "ready");
        assert_eq!(task["invocationAllowed"], applicability == "applicable");

        let inventory = plan["inventory"].as_array().expect("inventory");
        assert_eq!(
            find_by(inventory, "propertyId", "PROP-STATE")["classification"],
            "safety-critical"
        );
        assert!(inventory.iter().all(|item| {
            item["classification"] == "safety-critical" || item["classification"] == "unknown"
        }));
    }

    let mut unsupported = source["planRequest"].clone();
    unsupported["selections"]
        .as_array_mut()
        .expect("selections")
        .truncate(1);
    unsupported["capabilities"]
        .as_array_mut()
        .expect("capabilities")
        .retain(|item| item["techniqueId"] != "property-based-testing");
    let unsupported_plan = build_fixture_plan(&unsupported).expect("unsupported plan");
    let unsupported_task = find_by(
        unsupported_plan["tasks"].as_array().expect("tasks"),
        "kind",
        "verification",
    );
    assert_eq!(unsupported_task["applicability"], "applicable");
    assert_eq!(unsupported_task["readiness"], "unsupported");
    assert_eq!(unsupported_task["invocationAllowed"], false);

    let mut empty = source["planRequest"].clone();
    empty["properties"][0]["classification"] = json!("");
    assert_eq!(
        build_fixture_plan(&empty)
            .expect_err("empty classification must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
}

#[trace("TC-025", "FR-012-AC-1", "FR-012-AC-2")]
#[test]
fn tc_025_retains_authored_policy_and_rejects_ordinal_scoring() {
    let source = fixture("portfolio.json");
    let mut policy = source["planRequest"]["policy"].clone();
    policy["rules"] = json!([
        {"ruleId": "required-rule", "priority": "required", "explanation": "Authored required reason."},
        {"ruleId": "recommended-rule", "priority": "recommended", "explanation": "Authored recommended reason."},
        {"ruleId": "optional-rule", "priority": "optional", "explanation": "Authored optional reason."}
    ]);
    validate_contract(policy.clone()).expect("authored priorities and explanations must validate");

    for priority in ["required", "recommended", "optional"] {
        let reasons = json!([{"rule": "authored-policy", "value": priority}]);
        let mut request = source["planRequest"].clone();
        request["policy"] = policy.clone();
        request["selections"]
            .as_array_mut()
            .expect("selections")
            .truncate(1);
        request["selections"][0]["priority"] = json!(priority);
        request["selections"][0]["reasons"] = reasons.clone();
        let plan = build_fixture_plan(&request).expect("authored policy plan");
        let task = find_by(
            plan["tasks"].as_array().expect("tasks"),
            "kind",
            "verification",
        );
        assert_eq!(task["priority"], priority);
        assert_eq!(task["reasons"], reasons);
        assert!(task.get("score").is_none());
    }

    policy["rules"][0]["score"] = json!(3);
    assert_eq!(
        validate_contract(policy)
            .expect_err("ordinal score must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
}

#[trace("TC-012", "FR-006-AC-1", "FR-006-AC-2", "NFR-001-AC-2")]
#[test]
fn tc_012_rejects_every_evidence_binding_mismatch() {
    let packet = fixture("results.json");
    let property = &packet["inventory"][0];
    let result = &packet["results"][0];
    assert!(
        scope_differences(property, result)
            .expect("scope comparison")
            .is_empty()
    );
    let mutations: [ScopeMutation; 14] = [
        ("techniqueId", |v| v["techniqueId"] = json!("other")),
        ("propertyRef", |v| {
            v["propertyRef"]["revision"] = json!("other");
        }),
        ("subjectRefs", |v| {
            v["subjectRefs"][0]["revision"] = json!("other");
        }),
        ("profile", |v| v["profile"] = json!("other")),
        ("bindingRefs", |v| {
            v["bindingRefs"][0]["revision"] = json!("other");
        }),
        ("dependencyClosure", |v| {
            v["dependencyClosure"] = json!("other");
        }),
        ("executionScope", |v| {
            v["executionScope"]["revision"] = json!("other");
        }),
        ("planId", |v| v["planId"] = json!("other")),
        ("taskId", |v| v["taskId"] = json!("other")),
        ("runRef", |v| v["runRef"]["revision"] = json!("other")),
        ("producer", |v| v["provenance"]["producer"] = json!("other")),
        ("producerVersion", |v| {
            v["provenance"]["producerVersion"] = json!("other");
        }),
        ("adapter", |v| v["provenance"]["adapter"] = json!("other")),
        ("adapterVersion", |v| {
            v["provenance"]["adapterVersion"] = json!("other");
        }),
    ];
    for (dimension, mutate) in mutations {
        let mut changed = result.clone();
        mutate(&mut changed);
        assert!(
            scope_differences(property, &changed)
                .expect("scope comparison")
                .contains(&dimension),
            "{dimension}"
        );
    }
}

#[trace("TC-013", "FR-006-AC-3", "FR-006-AC-4", "NFR-001-AC-2")]
#[test]
fn tc_013_refuses_ambiguous_and_duplicate_results() {
    let mut packet = fixture("results.json");
    packet["inventory"] = json!([packet["inventory"][0].clone()]);
    let mut second = packet["results"][0].clone();
    second["resultId"] = json!("RESULT-SUPPORTED-SECOND");
    packet["results"] = json!([packet["results"][0].clone(), second]);
    assert_eq!(
        assess_fixture_evidence(&packet).expect("ambiguous assessment")["obligations"][0]["status"],
        "ambiguous"
    );
    packet["results"][1]["resultId"] = packet["results"][0]["resultId"].clone();
    assert_eq!(
        assess_fixture_evidence(&packet)
            .expect_err("duplicate result id must fail")
            .code(),
        VerificationErrorCode::DuplicateResultId
    );
}

#[trace("TC-014", "FR-006-AC-5", "FR-007-AC-1", "NFR-003-AC-1")]
#[trace("StR-001-VC-2", "StR-001-VC-3")]
#[test]
fn tc_014_distinguishes_freshness_and_every_result_state() {
    let packet = fixture("results.json");
    let report = assess_fixture_evidence(&packet).expect("assessment");
    for (property, expected) in packet["expectedStatuses"]
        .as_object()
        .expect("expected statuses")
    {
        let obligation = find_by(
            report["obligations"].as_array().expect("obligations"),
            "propertyId",
            property,
        );
        assert_eq!(&obligation["status"], expected, "{property}");
    }
}

#[trace("TC-015", "FR-007-AC-2")]
#[trace("FR-007-AC-3", "FR-007-AC-4")]
#[trace("FR-007-AC-5", "NFR-001-AC-2", "NFR-003-AC-1")]
#[test]
fn tc_015_retains_independent_findings_native_refs_and_ownership() {
    let mut packet = fixture("results.json");
    let property_id = packet["inventory"][0]["propertyId"]
        .as_str()
        .expect("property identifier")
        .to_owned();
    packet["results"][0]["logical"]["outcome"] = json!("violated");
    packet["results"][0]["participation"] = json!({"status": "partial", "participants": ["parser"], "missingParticipants": ["serializer"]});
    packet["results"][0]["assumptions"] =
        json!([{"assumptionId": "fixture-inputs-synthetic", "status": "not-met"}]);
    let report = assess_fixture_evidence(&packet).expect("assessment");
    assert_eq!(
        report["denominator"],
        packet["inventory"].as_array().expect("inventory").len()
    );
    assert_eq!(
        report["obligations"].as_array().expect("obligations").len(),
        packet["inventory"].as_array().expect("inventory").len()
    );
    assert!(
        report["obligations"]
            .as_array()
            .expect("obligations")
            .iter()
            .any(|item| item["status"] == "missing")
    );
    let obligation = find_by(
        report["obligations"].as_array().expect("obligations"),
        "propertyId",
        &property_id,
    );
    assert_eq!(
        obligation["gaps"]
            .as_array()
            .expect("gaps")
            .iter()
            .map(|gap| gap["kind"].as_str().expect("kind"))
            .collect::<Vec<_>>(),
        [
            "assumption-unmet",
            "logical-violation",
            "partial-participation"
        ]
    );
    assert_eq!(report["evidenceOwner"], "Quoin");
    assert_eq!(report["sufficiencyDecision"], "not-made");
    assert_eq!(
        obligation["nativeArtifacts"][0]["uri"],
        "fixture://native/RESULT-SUPPORTED.json"
    );
}

fn one_result_packet() -> Value {
    let source = fixture("results.json");
    json!({
        "schemaVersion": source["schemaVersion"],
        "asOf": "2026-09-01T01:00:00Z",
        "inventory": [source["inventory"][0].clone()],
        "results": [source["results"][0].clone()],
    })
}

fn assessment_status(packet: &Value) -> Value {
    assess_fixture_evidence(packet).expect("valid assessment")["obligations"][0]["status"].clone()
}

#[trace("TC-032", "FR-018-AC-1", "FR-018-AC-2")]
#[test]
fn tc_032_uses_the_earliest_expiry_and_instant_boundaries() {
    let mut result_limit_first = one_result_packet();
    result_limit_first["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    result_limit_first["results"][0]["validUntil"] = json!("2026-09-01T01:00:00Z");
    result_limit_first["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(7200);
    assert_eq!(assessment_status(&result_limit_first), "supported");
    result_limit_first["asOf"] = json!("2026-09-01T01:00:00.000000001Z");
    assert_eq!(assessment_status(&result_limit_first), "stale");

    let mut obligation_limit_first = one_result_packet();
    obligation_limit_first["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    obligation_limit_first["results"][0]["validUntil"] = json!("2026-09-01T03:00:00Z");
    obligation_limit_first["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(3600);
    obligation_limit_first["asOf"] = json!("2026-09-01T02:00:00+01:00");
    assert_eq!(assessment_status(&obligation_limit_first), "supported");
    obligation_limit_first["asOf"] = json!("2026-09-01T01:00:00.000000001Z");
    assert_eq!(assessment_status(&obligation_limit_first), "stale");

    let mut zero_age = one_result_packet();
    zero_age["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    zero_age["results"][0]
        .as_object_mut()
        .expect("result object")
        .remove("validUntil");
    zero_age["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(0);
    zero_age["asOf"] = json!("2026-09-01T00:00:00Z");
    assert_eq!(assessment_status(&zero_age), "supported");
    zero_age["asOf"] = json!("2026-09-01T00:00:00.000000001Z");
    assert_eq!(assessment_status(&zero_age), "stale");
}

#[trace("TC-032", "FR-018-AC-3", "FR-018-AC-4")]
#[test]
fn tc_032_refuses_invalid_temporal_inputs_and_overflow() {
    let mut future = one_result_packet();
    future["results"][0]["producedAt"] = json!("2026-09-01T01:00:00.000000001Z");
    assert_eq!(
        assess_fixture_evidence(&future)
            .expect_err("future evidence must fail")
            .code(),
        VerificationErrorCode::InvalidTemporalOrder
    );

    let mut reversed = one_result_packet();
    reversed["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    reversed["results"][0]["validUntil"] = json!("2026-08-31T23:59:59Z");
    assert_eq!(
        assess_fixture_evidence(&reversed)
            .expect_err("validity before production must fail")
            .code(),
        VerificationErrorCode::InvalidTemporalOrder
    );

    let mut missing_age = one_result_packet();
    missing_age["inventory"][0]
        .as_object_mut()
        .expect("obligation object")
        .remove("freshness");
    assert_eq!(
        assess_fixture_evidence(&missing_age)
            .expect_err("missing maximum age must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut planning_only = fixture("portfolio.json")["planRequest"].clone();
    planning_only["properties"][0]
        .as_object_mut()
        .expect("property object")
        .remove("freshness");
    build_fixture_plan(&planning_only)
        .expect("planning property is not yet an evidence obligation");

    let mut unsafe_age = one_result_packet();
    unsafe_age["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(9_007_199_254_740_992_u64);
    assert_eq!(
        assess_fixture_evidence(&unsafe_age)
            .expect_err("unsafe integer maximum age must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut negative_age = one_result_packet();
    negative_age["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(-1);
    assert_eq!(
        assess_fixture_evidence(&negative_age)
            .expect_err("negative maximum age must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );

    let mut safe_integer_limit = one_result_packet();
    safe_integer_limit["results"][0]
        .as_object_mut()
        .expect("result object")
        .remove("validUntil");
    safe_integer_limit["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    safe_integer_limit["inventory"][0]["freshness"]["maxAgeSeconds"] =
        json!(9_007_199_254_740_991_u64);
    assert_eq!(
        assess_fixture_evidence(&safe_integer_limit)
            .expect_err("unrepresentable duration must fail after schema acceptance")
            .code(),
        VerificationErrorCode::InvalidTimestamp
    );

    let mut malformed = one_result_packet();
    malformed["asOf"] = json!("not-a-timestamp");
    assert_eq!(
        assess_fixture_evidence(&malformed)
            .expect_err("malformed assessment time must fail")
            .code(),
        VerificationErrorCode::InvalidTimestamp
    );

    let mut overflow = one_result_packet();
    overflow["results"][0]["producedAt"] = json!("9999-12-31T23:59:59Z");
    overflow["results"][0]
        .as_object_mut()
        .expect("result object")
        .remove("validUntil");
    overflow["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(1);
    overflow["asOf"] = json!("9999-12-31T23:59:59Z");
    assert_eq!(
        assess_fixture_evidence(&overflow)
            .expect_err("expiry outside RFC 3339 must fail")
            .code(),
        VerificationErrorCode::InvalidTimestamp
    );
}

#[trace("TC-032", "FR-018-AC-5")]
#[test]
fn tc_032_stale_results_retain_every_independent_finding() {
    let mut stale_with_findings = one_result_packet();
    stale_with_findings["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    stale_with_findings["results"][0]["validUntil"] = json!("2026-09-01T01:00:00Z");
    stale_with_findings["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(7200);
    stale_with_findings["asOf"] = json!("2026-09-01T01:00:00.000000001Z");
    stale_with_findings["results"][0]["logical"]["outcome"] = json!("violated");
    stale_with_findings["results"][0]["participation"] = json!({
        "status": "partial",
        "participants": ["parser"],
        "missingParticipants": ["serializer"]
    });
    stale_with_findings["results"][0]["assumptions"] =
        json!([{"assumptionId": "fixture-inputs-synthetic", "status": "not-met"}]);
    let assessment = assess_fixture_evidence(&stale_with_findings).expect("stale assessment");
    let obligation = &assessment["obligations"][0];
    assert_eq!(obligation["status"], "stale");
    assert_eq!(
        obligation["gaps"]
            .as_array()
            .expect("gaps")
            .iter()
            .map(|gap| gap["kind"].as_str().expect("gap kind"))
            .collect::<Vec<_>>(),
        [
            "freshness-expired",
            "assumption-unmet",
            "logical-violation",
            "partial-participation"
        ]
    );
    assert_eq!(
        obligation["nativeArtifacts"],
        stale_with_findings["results"][0]["nativeArtifacts"]
    );
    for field in [
        "execution",
        "logical",
        "activation",
        "participation",
        "assumptions",
        "adequacy",
        "provenance",
        "nativeArtifacts",
    ] {
        assert_eq!(
            obligation[field], stale_with_findings["results"][0][field],
            "{field}"
        );
    }
}

#[trace("TC-033", "FR-017-AC-1")]
#[test]
fn tc_033_enforces_the_closed_contribution_contract() {
    let source = fixture("results.json");
    let evaluated = source["results"][0].clone();
    validate_contract(evaluated.clone()).expect("evaluated contribution must validate");
    validate_contract(source["results"][1].clone())
        .expect("non-evaluated result may carry an empty contribution array");

    let mut no_array = evaluated.clone();
    no_array
        .as_object_mut()
        .expect("result object")
        .remove("oracleContributions");
    assert!(validate_contract(no_array).is_err());

    let mut empty = evaluated.clone();
    empty["oracleContributions"] = json!([]);
    assert!(validate_contract(empty).is_err());

    for field_name in ["contributionId", "oracleRef", "observationRef", "relation"] {
        let mut missing = evaluated.clone();
        missing["oracleContributions"][0]
            .as_object_mut()
            .expect("contribution object")
            .remove(field_name);
        assert!(validate_contract(missing).is_err(), "{field_name}");
    }

    let mut shared = evaluated.clone();
    shared["oracleContributions"][0]["relation"] = json!("shared");
    assert!(validate_contract(shared.clone()).is_err());
    shared["oracleContributions"][0]["dependenceGroupId"] = json!("GROUP-1");
    validate_contract(shared.clone()).expect("closed shared contribution");
    shared["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "RESULT-SOURCE", "contributionId": "source"});
    assert!(validate_contract(shared).is_err());

    let mut derived = evaluated.clone();
    derived["oracleContributions"][0]["relation"] = json!("derived");
    assert!(validate_contract(derived.clone()).is_err());
    derived["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "RESULT-SOURCE", "contributionId": "source"});
    validate_contract(derived.clone()).expect("closed derived contribution");
    let mut derived_with_group = derived.clone();
    derived_with_group["oracleContributions"][0]["dependenceGroupId"] = json!("GROUP-1");
    assert!(validate_contract(derived_with_group).is_err());
    derived["oracleContributions"][0]["derivedFrom"]["extra"] = json!(true);
    assert!(validate_contract(derived).is_err());

    for relation in ["independent", "unknown"] {
        for forbidden in [
            ("dependenceGroupId", json!("GROUP-1")),
            (
                "derivedFrom",
                json!({"resultId": "RESULT-SOURCE", "contributionId": "source"}),
            ),
        ] {
            let mut conditional = evaluated.clone();
            conditional["oracleContributions"][0]["relation"] = json!(relation);
            conditional["oracleContributions"][0][forbidden.0] = forbidden.1;
            assert!(
                validate_contract(conditional).is_err(),
                "{relation} with {}",
                forbidden.0
            );
        }
    }

    let mut extra = evaluated.clone();
    extra["oracleContributions"][0]["extra"] = json!(true);
    assert!(validate_contract(extra).is_err());
    let mut wrong_type = evaluated;
    wrong_type["oracleContributions"][0]["contributionId"] = json!(1);
    assert!(validate_contract(wrong_type).is_err());
}

#[trace("TC-033", "FR-017-AC-2")]
#[test]
fn tc_033_requires_exact_strict_reader_outputs() {
    let mut packet = one_result_packet();
    packet["asOf"] = json!("2026-09-08T00:00:00Z");
    let references = validate_fixture_result_references(&packet).expect("strict references");
    let baseline = assess_evidence(&packet, &references).expect("baseline assessment");
    assert_eq!(
        assess_evidence(&packet, &[])
            .expect_err("missing validation output must fail")
            .code(),
        VerificationErrorCode::IdentityMismatch
    );

    let mut duplicated = references.clone();
    duplicated.push(references[0].clone());
    assert_eq!(
        assess_evidence(&packet, &duplicated).expect("duplicates are idempotent"),
        baseline
    );

    let mutations: [ScopeMutation; 6] = [
        ("authority", |v| v["authority"] = json!("other")),
        ("identity", |v| v["identity"] = json!("other")),
        ("revision", |v| v["revision"]["value"] = json!("other")),
        ("digest", |v| {
            v["digest"] = json!(format!("sha256:{}", "0".repeat(64)));
        }),
        ("wire identity", |v| v["wire"]["identity"] = json!("other")),
        ("wire version", |v| v["wire"]["version"] = json!("other")),
    ];
    for reference_field in ["oracleRef", "observationRef"] {
        for (dimension, mutate) in mutations {
            let mut changed = packet.clone();
            mutate(&mut changed["results"][0]["oracleContributions"][0][reference_field]["value"]);
            assert_eq!(
                assess_evidence(&changed, &references)
                    .expect_err("changed reference must fail")
                    .code(),
                VerificationErrorCode::IdentityMismatch,
                "{reference_field} {dimension}"
            );
        }
    }

    for (target, source) in [
        ("oracleRef", "observationRef"),
        ("observationRef", "oracleRef"),
    ] {
        let mut wrong_role = packet.clone();
        wrong_role["results"][0]["oracleContributions"][0][target] =
            wrong_role["results"][0]["oracleContributions"][0][source].clone();
        assert_eq!(
            assess_evidence(&wrong_role, &references)
                .expect_err("wrong role must fail")
                .code(),
            VerificationErrorCode::SchemaViolation,
            "{target}"
        );
    }

    for (field, value) in [
        ("wireVersion", "ix.shared-reference/unknown"),
        ("refVersion", "ix.artifact-ref/unknown"),
    ] {
        let mut unsupported = packet.clone();
        if field == "wireVersion" {
            unsupported["results"][0]["oracleContributions"][0]["oracleRef"][field] = json!(value);
        } else {
            unsupported["results"][0]["oracleContributions"][0]["oracleRef"]["value"][field] =
                json!(value);
        }
        assert_eq!(
            assess_evidence(&unsupported, &references)
                .expect_err("unsupported contribution reference version must fail")
                .code(),
            VerificationErrorCode::SchemaViolation,
            "{field}"
        );
    }
}

#[trace("TC-033", "FR-017-AC-3")]
#[test]
fn tc_033_rejects_duplicate_and_invalid_lineage() {
    let base = one_result_packet();
    let primary = base["results"][0]["oracleContributions"][0].clone();

    let mut duplicate = base.clone();
    duplicate["results"][0]["oracleContributions"] = json!([primary.clone(), primary.clone()]);
    let references = validate_fixture_result_references(&duplicate).expect("strict references");
    assert_eq!(
        assess_evidence(&duplicate, &references)
            .expect_err("duplicate contribution id must fail")
            .code(),
        VerificationErrorCode::DuplicateOracleContributionId
    );

    let mut absent = base.clone();
    absent["results"][0]["oracleContributions"][0]["relation"] = json!("derived");
    absent["results"][0]["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "ABSENT", "contributionId": "source"});
    let references = validate_fixture_result_references(&absent).expect("strict references");
    assert_eq!(
        assess_evidence(&absent, &references)
            .expect_err("absent source must fail")
            .code(),
        VerificationErrorCode::InvalidOracleLineage
    );

    let mut self_reference = base.clone();
    self_reference["results"][0]["oracleContributions"][0]["relation"] = json!("derived");
    self_reference["results"][0]["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "RESULT-SUPPORTED", "contributionId": "primary"});
    let references =
        validate_fixture_result_references(&self_reference).expect("strict references");
    assert_eq!(
        assess_evidence(&self_reference, &references)
            .expect_err("self source must fail")
            .code(),
        VerificationErrorCode::InvalidOracleLineage
    );

    let mut cycle = base;
    let mut first = primary.clone();
    first["contributionId"] = json!("first");
    first["relation"] = json!("derived");
    first["derivedFrom"] = json!({"resultId": "RESULT-SUPPORTED", "contributionId": "second"});
    let mut second = primary;
    second["contributionId"] = json!("second");
    second["relation"] = json!("derived");
    second["derivedFrom"] = json!({"resultId": "RESULT-SUPPORTED", "contributionId": "first"});
    cycle["results"][0]["oracleContributions"] = json!([first, second]);
    let references = validate_fixture_result_references(&cycle).expect("strict references");
    assert_eq!(
        assess_evidence(&cycle, &references)
            .expect_err("cycle must fail")
            .code(),
        VerificationErrorCode::InvalidOracleLineage
    );
}

fn configured_dependence_packet() -> Value {
    let mut packet = fixture("results.json");
    let results = packet["results"].as_array_mut().expect("results");
    find_by_mut(results, "resultId", "RESULT-UNTRIGGERED")["oracleContributions"][0]["relation"] =
        json!("shared");
    find_by_mut(results, "resultId", "RESULT-UNTRIGGERED")["oracleContributions"][0]["dependenceGroupId"] =
        json!("singleton");
    find_by_mut(results, "resultId", "RESULT-PARTIAL")["oracleContributions"][0]["relation"] =
        json!("shared");
    find_by_mut(results, "resultId", "RESULT-PARTIAL")["oracleContributions"][0]["dependenceGroupId"] =
        json!("paired");
    find_by_mut(results, "resultId", "RESULT-ASSUMPTION-UNMET")["oracleContributions"][0]["relation"] =
        json!("shared");
    find_by_mut(results, "resultId", "RESULT-ASSUMPTION-UNMET")["oracleContributions"][0]["dependenceGroupId"] =
        json!("paired");
    find_by_mut(results, "resultId", "RESULT-STALE")["oracleContributions"][0]["relation"] =
        json!("derived");
    find_by_mut(results, "resultId", "RESULT-STALE")["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "RESULT-SUPPORTED", "contributionId": "primary"});
    find_by_mut(results, "resultId", "RESULT-VIOLATED")["oracleContributions"][0]["relation"] =
        json!("derived");
    find_by_mut(results, "resultId", "RESULT-VIOLATED")["oracleContributions"][0]["derivedFrom"] =
        json!({"resultId": "RESULT-SCOPE-MISMATCH", "contributionId": "primary"});
    find_by_mut(results, "resultId", "RESULT-SUPPORTED")["oracleContributions"][0]["relation"] =
        json!("unknown");
    let mut secondary =
        find_by_mut(results, "resultId", "RESULT-PARTIAL")["oracleContributions"][0].clone();
    secondary["contributionId"] = json!("secondary");
    secondary["relation"] = json!("independent");
    secondary
        .as_object_mut()
        .expect("secondary contribution")
        .remove("dependenceGroupId");
    find_by_mut(results, "resultId", "RESULT-PARTIAL")["oracleContributions"]
        .as_array_mut()
        .expect("contributions")
        .insert(0, secondary);
    packet
}

fn assert_dependence_findings(report: &Value) {
    let findings = report["oracleDependenceFindings"]
        .as_array()
        .expect("finding array");
    assert_eq!(findings.len(), 4);
    let correlated = findings
        .iter()
        .find(|finding| finding["kind"] == "correlated-oracle")
        .expect("correlation finding");
    assert_eq!(
        correlated["bases"],
        json!(["declared-group", "derived", "observation", "oracle"])
    );
    assert_eq!(
        correlated["members"],
        json!([
            {"resultId": "RESULT-ASSUMPTION-UNMET", "contributionId": "primary"},
            {"resultId": "RESULT-PARTIAL", "contributionId": "primary"},
            {"resultId": "RESULT-PARTIAL", "contributionId": "secondary"},
            {"resultId": "RESULT-STALE", "contributionId": "primary"},
            {"resultId": "RESULT-SUPPORTED", "contributionId": "primary"},
            {"resultId": "RESULT-UNTRIGGERED", "contributionId": "primary"},
            {"resultId": "RESULT-VIOLATED", "contributionId": "primary"}
        ])
    );
    let unknowns = findings
        .iter()
        .filter_map(|finding| {
            finding.get("reason").and_then(Value::as_str).map(|reason| {
                (
                    finding["member"]["resultId"]
                        .as_str()
                        .expect("unknown result id"),
                    finding["member"]["contributionId"]
                        .as_str()
                        .expect("unknown contribution id"),
                    reason,
                )
            })
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        unknowns,
        BTreeSet::from([
            ("RESULT-SUPPORTED", "primary", "declared-unknown"),
            ("RESULT-UNTRIGGERED", "primary", "shared-group-unresolved"),
            ("RESULT-VIOLATED", "primary", "source-not-assessed")
        ])
    );
    for finding in findings {
        let keys = finding
            .as_object()
            .expect("finding object")
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if finding["kind"] == "correlated-oracle" {
            assert_eq!(keys, BTreeSet::from(["bases", "kind", "members"]));
            for member in finding["members"].as_array().expect("members") {
                assert_eq!(
                    member
                        .as_object()
                        .expect("member")
                        .keys()
                        .map(String::as_str)
                        .collect::<BTreeSet<_>>(),
                    BTreeSet::from(["contributionId", "resultId"])
                );
            }
        } else {
            assert_eq!(keys, BTreeSet::from(["kind", "member", "reason"]));
        }
    }
    let canonical_findings = findings
        .iter()
        .map(|finding| jcs_canonicalize(finding).expect("canonical finding"))
        .collect::<Vec<_>>();
    assert!(canonical_findings.windows(2).all(|pair| pair[0] <= pair[1]));
}

fn assert_retained_contributions_and_claims(report: &Value) {
    let partial = find_by(
        report["obligations"].as_array().expect("obligations"),
        "resultId",
        "RESULT-PARTIAL",
    );
    assert_eq!(
        partial["oracleContributions"]
            .as_array()
            .expect("retained contributions")
            .iter()
            .map(|value| value["contributionId"].as_str().expect("contribution id"))
            .collect::<Vec<_>>(),
        ["primary", "secondary"]
    );
    assert_eq!(report["evidenceOwner"], "Quoin");
    assert_eq!(report["sufficiencyDecision"], "not-made");
    let serialized = serde_json::to_string(&report).expect("assessment JSON");
    assert!(!serialized.contains("\"score\""));
    assert!(!serialized.contains("assuranceLevel"));
}

fn assert_dependence_permutation_invariance(packet: &Value, report: &Value) {
    let mut permuted = packet.clone();
    permuted["inventory"]
        .as_array_mut()
        .expect("inventory")
        .reverse();
    permuted["results"]
        .as_array_mut()
        .expect("results")
        .reverse();
    for result in permuted["results"].as_array_mut().expect("results") {
        result["oracleContributions"]
            .as_array_mut()
            .expect("contributions")
            .reverse();
    }
    let mut permuted_references =
        validate_fixture_result_references(&permuted).expect("strict references");
    permuted_references.reverse();
    assert_eq!(
        jcs_canonicalize(&report).expect("baseline JCS"),
        jcs_canonicalize(
            &assess_evidence(&permuted, &permuted_references).expect("permuted assessment")
        )
        .expect("permuted JCS")
    );
}

fn assert_dependence_selection_boundaries() {
    let mut ambiguous = one_result_packet();
    ambiguous["asOf"] = json!("2026-09-08T00:00:00Z");
    let mut second = ambiguous["results"][0].clone();
    second["resultId"] = json!("RESULT-AMBIGUOUS");
    ambiguous["results"]
        .as_array_mut()
        .expect("results")
        .push(second);
    let references = validate_fixture_result_references(&ambiguous).expect("strict references");
    let ambiguous_report = assess_evidence(&ambiguous, &references).expect("ambiguous assessment");
    assert_eq!(ambiguous_report["obligations"][0]["status"], "ambiguous");
    assert_eq!(ambiguous_report["oracleDependenceFindings"], json!([]));

    let source = fixture("results.json");
    let supported_obligation = find_by(
        source["inventory"].as_array().expect("inventory"),
        "propertyId",
        "OBL-STATE-PROPERTY",
    )
    .clone();
    let mut repeated_obligation = supported_obligation.clone();
    repeated_obligation["propertyId"] = json!("OBL-REUSED");
    let partial_obligation = find_by(
        source["inventory"].as_array().expect("inventory"),
        "propertyId",
        "OBL-PARTIAL",
    )
    .clone();
    let supported_result = find_by(
        source["results"].as_array().expect("results"),
        "resultId",
        "RESULT-SUPPORTED",
    )
    .clone();
    let partial_result = find_by(
        source["results"].as_array().expect("results"),
        "resultId",
        "RESULT-PARTIAL",
    )
    .clone();
    let reused = json!({
        "asOf": source["asOf"],
        "inventory": [supported_obligation, repeated_obligation, partial_obligation],
        "results": [supported_result, partial_result],
    });
    let references = validate_fixture_result_references(&reused).expect("strict references");
    let reused_report = assess_evidence(&reused, &references).expect("reused result assessment");
    assert_eq!(
        reused_report["obligations"]
            .as_array()
            .expect("obligations")
            .len(),
        3
    );
    assert_eq!(
        reused_report["oracleDependenceFindings"][0]["members"],
        json!([
            {"resultId": "RESULT-PARTIAL", "contributionId": "primary"},
            {"resultId": "RESULT-SUPPORTED", "contributionId": "primary"}
        ])
    );
}

#[trace("TC-033", "FR-017-AC-4", "FR-017-AC-5")]
#[trace("FR-017-AC-6", "FR-017-AC-7")]
#[test]
fn tc_033_retains_dependence_deterministically_without_aggregation() {
    let packet = configured_dependence_packet();
    let references = validate_fixture_result_references(&packet).expect("strict references");
    let report = assess_evidence(&packet, &references).expect("dependence assessment");
    assert_dependence_findings(&report);
    assert_retained_contributions_and_claims(&report);
    assert_dependence_permutation_invariance(&packet, &report);
    assert_dependence_selection_boundaries();
}

#[trace("TC-016", "FR-008-AC-1", "FR-008-AC-2", "NFR-001-AC-1")]
#[test]
fn tc_016_matches_rfc_8785_canonicalization() {
    let value = json!({"z": 0, "numbers": [333_333_333.333_333_3_f64, 1e30_f64, 4.5_f64, 2e-3_f64, 1e-27_f64], "a": 1});
    assert_eq!(
        String::from_utf8(jcs_canonicalize(&value).expect("canonical JSON")).expect("UTF-8"),
        "{\"a\":1,\"numbers\":[333333333.3333333,1e+30,4.5,0.002,1e-27],\"z\":0}"
    );
    assert!(jcs_equal(&json!({"b": 2, "a": 1}), &json!({"a": 1, "b": 2})).expect("JCS equality"));
    assert!(!jcs_equal(&json!([1, 2]), &json!([2, 1])).expect("JCS equality"));
}

#[trace("TC-017", "FR-008-AC-3", "FR-008-AC-4")]
#[test]
fn tc_017_rejects_non_json_inputs_and_prefixes_digests() {
    assert!(parse_bounded_json(br#"{"bad":"\ud800"}"#).is_err());
    assert!(parse_bounded_json(b"{").is_err());
    assert!(jcs_canonicalize(&f64::NAN).is_err());
    let digest = jcs_sha256(&json!({"a": 1})).expect("digest");
    assert!(digest.starts_with("sha256-jcs:"));
    assert_eq!(digest.len(), "sha256-jcs:".len() + 64);
}

#[trace("TC-018", "FR-009-AC-1", "FR-009-AC-2", "NFR-001-AC-1")]
#[test]
fn tc_018_catalog_parity_is_keyed_and_permutation_invariant() {
    let catalog = fixture("quoin-catalog-export.json");
    let report =
        run_bounded_portfolio_with_adapter("revision", "2026-09-07T12:00:00Z", &catalog, |value| {
            let mut adapted = adapt_quoin_catalog(value)?;
            adapted["techniques"]
                .as_array_mut()
                .expect("techniques")
                .reverse();
            Ok(adapted)
        })
        .expect("portfolio");
    assert_eq!(report["results"][0]["logical"]["outcome"], "satisfied");
    assert_eq!(report["results"][1]["logical"]["outcome"], "satisfied");
}

#[trace("TC-019", "FR-009-AC-3", "FR-009-AC-4")]
#[test]
fn tc_019_returns_violated_and_failed_technique_results() {
    let catalog = fixture("quoin-catalog-export.json");
    let violated =
        run_bounded_portfolio_with_adapter("revision", "2026-09-07T12:00:00Z", &catalog, |value| {
            let mut adapted = adapt_quoin_catalog(value)?;
            adapted["techniques"][0]["purpose"] = json!("changed");
            Ok(adapted)
        })
        .expect("violated portfolio report");
    assert_eq!(violated["results"][0]["execution"]["status"], "completed");
    assert_eq!(violated["results"][0]["logical"]["outcome"], "violated");
    assert_eq!(
        find_by(
            violated["assessment"]["obligations"]
                .as_array()
                .expect("obligations"),
            "propertyId",
            "LOCAL-CATALOG-PARITY",
        )["status"],
        "violated"
    );

    let failed =
        run_bounded_portfolio_with_adapter("revision", "2026-09-07T12:00:00Z", &catalog, |_| {
            Err(VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "injected adapter failure",
            ))
        })
        .expect("failed portfolio report");
    assert_eq!(failed["results"][0]["execution"]["status"], "failed");
    assert_eq!(failed["results"][0]["logical"]["outcome"], "not-evaluated");
    assert_eq!(
        find_by(
            failed["assessment"]["obligations"]
                .as_array()
                .expect("obligations"),
            "propertyId",
            "LOCAL-CATALOG-PARITY",
        )["status"],
        "failed"
    );
}

#[trace("TC-020", "FR-009-AC-5", "FR-009-AC-6", "NFR-003-AC-2")]
#[trace("StR-001-VC-3")]
#[test]
fn tc_020_retains_pending_human_work_and_claim_limits() {
    let report = run_bounded_portfolio(
        "revision",
        "2026-09-07T12:00:00Z",
        &fixture("quoin-catalog-export.json"),
    )
    .expect("portfolio");
    assert_eq!(report["results"][2]["execution"]["status"], "not-run");
    assert!(
        report["claimLimit"]
            .as_str()
            .expect("claim limit")
            .contains("no external method, parser, fault, or human review")
    );
    assert_eq!(report["assessment"]["sufficiencyDecision"], "not-made");
}

#[trace("TC-021", "FR-004-AC-1", "FR-009-AC-1")]
#[test]
fn tc_021_baseline_checker_accepts_the_pinned_quoin_contract() {
    let fixture = fixture("quoin-catalog-export.json");
    assert_eq!(
        check_quoin_baseline(&fixture["catalog"], &fixture).expect("baseline parity"),
        33
    );
}

#[trace("TC-022", "NFR-004-AC-1", "NFR-004-AC-2")]
#[trace("NFR-004-AC-3", "NFR-004-AC-4", "NFR-004-AC-5")]
#[trace("NFR-004-AC-6")]
#[trace("StR-001-VC-1")]
#[test]
fn tc_022_qualified_path_configuration_is_rust_and_traced() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut pending = ["src", "tests", "test", "scripts", "fuzz"]
        .into_iter()
        .map(|path| root.join(path))
        .filter(|path| path.exists())
        .collect::<Vec<_>>();
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).expect("governed source directory") {
            let entry = entry.expect("source entry");
            if entry.file_type().expect("file type").is_dir() {
                pending.push(entry.path());
            } else {
                assert!(!matches!(
                    entry.path().extension().and_then(|value| value.to_str()),
                    Some("js" | "mjs" | "ts" | "mts")
                ));
            }
        }
    }
    assert!(!root.join("package.json").exists());
    assert!(!root.join("package-lock.json").exists());
    assert!(include_str!("../src/lib.rs").contains("#![forbid(unsafe_code)]"));
    let manifest = include_str!("../Cargo.toml");
    assert!(manifest.contains("git = \"https://github.com/agent-ix/ix-trace-rs\""));
    assert!(manifest.contains("rev = \"2ce4ebf47f726b9d76388220545cd0abda8a5cfb\""));
    assert!(manifest.contains("version = \"=0.1.0\""));
    let workspace_lock = include_str!("../Cargo.lock");
    assert!(workspace_lock.contains("git+https://github.com/agent-ix/ix-trace-rs"));
    assert!(workspace_lock.contains("name = \"quire-verification-fuzz\""));
    assert_eq!(workspace_lock.matches("name = \"jsonschema\"\n").count(), 1);
    assert!(workspace_lock.contains("2ce4ebf47f726b9d76388220545cd0abda8a5cfb"));
    let rust_requirement = include_str!("../spec/non-functional/NFR-004-rust-qualified-path.md");
    assert!(rust_requirement.contains("first-party traceability population"));
    assert!(rust_requirement.contains("Dependency sources resolved by Cargo"));
    assert!(rust_requirement.contains("| Rust quality gates passing | 5 of 5 | 5 of 5 |"));
    let trace_test = include_str!("../spec/test-cases/TC-022-rust-qualified-path.md");
    assert!(trace_test.contains("repository-owned Rust under `src/`, `tests/`, and `fuzz/`"));
    assert!(trace_test.contains("governed by that dependency's matrix"));
    let workflow = include_str!("../.github/workflows/ci.yml");
    for gate in [
        "cargo fmt --all -- --check",
        "cargo clippy",
        "cargo test",
        "cargo deny check",
        "cargo fuzz build",
    ] {
        assert!(workflow.contains(gate), "{gate}");
    }
    assert!(manifest.contains("members = [\"fuzz\"]"));
    let fuzz_manifest = include_str!("../fuzz/Cargo.toml");
    assert!(fuzz_manifest.contains("[lints]\nworkspace = true"));
    assert!(!root.join("fuzz/Cargo.lock").exists());
    let contributor_guidance = include_str!("../AGENTS.md");
    assert!(contributor_guidance.contains("cargo test --workspace --all-features --locked"));
    assert!(!contributor_guidance.contains("There is no compiler build/test command yet"));
    let license_decision = include_str!("../LICENSE-DECISION.md");
    for authoritative_record in ["`Cargo.toml`", "`Cargo.lock`", "`docs/artifact-rights.md`"] {
        assert!(
            license_decision.contains(authoritative_record),
            "{authoritative_record}"
        );
    }
    for removed_dependency_claim in [
        "The implementation uses Ajv",
        "uses Ajv and ajv-formats",
        "transitive dependency set recorded in\n`package-lock.json`",
        "canonicalize\nunder Apache-2.0 terms",
    ] {
        assert!(
            !license_decision.contains(removed_dependency_claim),
            "{removed_dependency_claim}"
        );
    }
}

#[trace("TC-023", "NFR-005-AC-1", "NFR-005-AC-2", "NFR-005-AC-3")]
#[test]
fn tc_023_enforces_json_resource_limits() {
    let mut at_limit = vec![b' '; MAX_JSON_BYTES - 4];
    at_limit.extend_from_slice(b"null");
    assert_eq!(
        parse_bounded_json(&at_limit).expect("exact byte limit must be accepted"),
        Value::Null
    );
    let oversized = vec![b' '; MAX_JSON_BYTES + 1];
    assert_eq!(
        parse_bounded_json(&oversized)
            .expect_err("oversized bytes must fail")
            .code(),
        VerificationErrorCode::JsonDocumentTooLarge
    );
    let mut nested = Value::Null;
    for _ in 0..=MAX_JSON_DEPTH {
        nested = json!([nested]);
    }
    assert_eq!(
        validate_resource_envelope(&nested)
            .expect_err("deep value must fail")
            .code(),
        VerificationErrorCode::JsonNestingTooDeep
    );
    let large_array = Value::Array(vec![Value::Null; MAX_ARRAY_ITEMS + 1]);
    assert_eq!(
        validate_resource_envelope(&large_array)
            .expect_err("large array must fail")
            .code(),
        VerificationErrorCode::JsonArrayTooLarge
    );
}

#[trace("TC-005", "FR-003-AC-1")]
#[test]
fn rendered_fixture_packet_is_reproducible() {
    let portfolio = fixture("portfolio.json");
    let results = fixture("results.json");
    let rendered = render_fixture_packet(&portfolio, &results).expect("rendered packet");
    assert_eq!(rendered["plan"], built_plan());
    assert_eq!(
        rendered["assessment"],
        assess_fixture_evidence(&results).expect("assessment")
    );
}

#[trace("TC-015", "FR-007-AC-2")]
#[test]
fn finding_helper_orders_simultaneous_failures_canonically() {
    let mut result = fixture("results.json")["results"][0].clone();
    result["logical"]["outcome"] = json!("violated");
    result["assumptions"] = json!([{"assumptionId": "x", "status": "not-met"}]);
    let kinds = findings_for(&result)
        .expect("findings")
        .into_iter()
        .map(|item| item["kind"].as_str().expect("kind").to_owned())
        .collect::<Vec<_>>();
    assert_eq!(kinds, ["assumption-unmet", "logical-violation"]);
}
