// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Requirement-traced acceptance tests for the Rust verification boundary.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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
};
use quire_verification::planner::build_verification_plan;
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
    build_verification_plan(&fixture("portfolio.json")["planRequest"])
        .expect("fixture plan must build")
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
    for field_name in [
        "kind",
        "priority",
        "missingPrerequisites",
        "runnerRequest",
        "propertyId",
        "techniqueId",
    ] {
        let mut invalid = plan.clone();
        invalid["tasks"][0]
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
    let mut unknown_version = results["results"][0].clone();
    unknown_version["contractVersion"] = json!("quire-verification/e02-unknown");
    assert_eq!(
        validate_contract(unknown_version)
            .expect_err("unknown contract version must fail")
            .code(),
        VerificationErrorCode::UnsupportedContractVersion
    );
}

#[trace("TC-003", "FR-002-AC-1", "FR-002-AC-2", "NFR-002-AC-1")]
#[test]
fn tc_003_derives_contract_vocabulary_and_types_from_one_schema() {
    assert_eq!(contract_version(), "quire-verification/e02-draft-1");
    assert_eq!(
        input_kinds(),
        ["state", "relational", "trace", "fault", "manual"]
    );
    let build_script = include_str!("../build.rs");
    assert_eq!(
        build_script
            .matches("contracts/e02-draft-1.schema.json")
            .count(),
        1
    );
    assert!(build_script.contains("TypeSpace"));
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
}

#[trace("TC-010", "FR-005-AC-3", "FR-005-AC-4", "FR-005-AC-5")]
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
    let plan = build_verification_plan(&request).expect("unsupported plan");
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
    let stale_plan = build_verification_plan(&stale).expect("stale plan");
    assert_eq!(stale_plan["policyRef"]["freshness"], "stale");
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
        build_verification_plan(&request)
            .expect_err("ambiguous capability must fail")
            .code(),
        VerificationErrorCode::AmbiguousCapability
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
        assess_evidence(&packet).expect("ambiguous assessment")["obligations"][0]["status"],
        "ambiguous"
    );
    packet["results"][1]["resultId"] = packet["results"][0]["resultId"].clone();
    assert_eq!(
        assess_evidence(&packet)
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
    let report = assess_evidence(&packet).expect("assessment");
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
    let report = assess_evidence(&packet).expect("assessment");
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
        violated["assessment"]["obligations"][0]["status"],
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
    assert_eq!(failed["assessment"]["obligations"][0]["status"], "failed");
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
#[trace("NFR-004-AC-3", "NFR-004-AC-4", "StR-001-VC-1")]
#[test]
fn tc_022_qualified_path_configuration_is_rust_and_traced() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut pending = ["src", "tests", "test", "scripts"]
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
    assert!(manifest.contains("path = \"vendor/ix-trace-rs\""));
    assert!(manifest.contains("version = \"=0.1.0\""));
    assert!(!include_str!("../Cargo.lock").contains("git+https://github.com/agent-ix/ix-trace-rs"));
    let upstream = include_str!("../vendor/ix-trace-rs/UPSTREAM.md");
    assert!(upstream.contains("2ce4ebf47f726b9d76388220545cd0abda8a5cfb"));
    assert!(
        include_str!("../.ix/modules/quire-verification/manifest.yaml")
            .contains("source_exclude:\n    - \"vendor/**\"")
    );
    assert_eq!(
        format!(
            "{:x}",
            Sha256::digest(include_bytes!("../vendor/ix-trace-rs/src/lib.rs"))
        ),
        "d7be2c00ed45ef0ad9bda683ef6a5b9397cca32483e89a7ac1fb745eb841d1c4"
    );
    let workflow = include_str!("../.github/workflows/ci.yml");
    for gate in [
        "cargo fmt --check",
        "cargo clippy",
        "cargo test",
        "cargo deny check",
    ] {
        assert!(workflow.contains(gate), "{gate}");
    }
}

#[trace("TC-023", "NFR-005-AC-1", "NFR-005-AC-2", "NFR-005-AC-3")]
#[test]
fn tc_023_enforces_json_resource_limits() {
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
        assess_evidence(&results).expect("assessment")
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
