// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Requirement-traced acceptance tests for the Rust verification boundary.

use std::collections::{BTreeMap, BTreeSet};
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
    build_verification_plan(&request).expect("blocked plan")
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
    let near_plan = build_verification_plan(&near_match).expect("near-match plan");
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
    let stale_plan = build_verification_plan(&stale).expect("stale plan");
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

    let plan = build_verification_plan(&request).expect("delimiter-bearing plan");
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
        build_verification_plan(&request)
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
        let plan = build_verification_plan(&request).expect("qualified plan");
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
    let unsupported_plan = build_verification_plan(&unsupported).expect("unsupported plan");
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
        build_verification_plan(&empty)
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
        let plan = build_verification_plan(&request).expect("authored policy plan");
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
    assess_evidence(packet).expect("valid assessment")["obligations"][0]["status"].clone()
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
        assess_evidence(&future)
            .expect_err("future evidence must fail")
            .code(),
        VerificationErrorCode::InvalidTemporalOrder
    );

    let mut reversed = one_result_packet();
    reversed["results"][0]["producedAt"] = json!("2026-09-01T00:00:00Z");
    reversed["results"][0]["validUntil"] = json!("2026-08-31T23:59:59Z");
    assert_eq!(
        assess_evidence(&reversed)
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
        assess_evidence(&missing_age)
            .expect_err("missing maximum age must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut planning_only = fixture("portfolio.json")["planRequest"].clone();
    planning_only["properties"][0]
        .as_object_mut()
        .expect("property object")
        .remove("freshness");
    build_verification_plan(&planning_only)
        .expect("planning property is not yet an evidence obligation");

    let mut unsafe_age = one_result_packet();
    unsafe_age["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(9_007_199_254_740_992_u64);
    assert_eq!(
        assess_evidence(&unsafe_age)
            .expect_err("unsafe integer maximum age must fail")
            .code(),
        VerificationErrorCode::SchemaViolation
    );
    let mut negative_age = one_result_packet();
    negative_age["inventory"][0]["freshness"]["maxAgeSeconds"] = json!(-1);
    assert_eq!(
        assess_evidence(&negative_age)
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
        assess_evidence(&safe_integer_limit)
            .expect_err("unrepresentable duration must fail after schema acceptance")
            .code(),
        VerificationErrorCode::InvalidTimestamp
    );

    let mut malformed = one_result_packet();
    malformed["asOf"] = json!("not-a-timestamp");
    assert_eq!(
        assess_evidence(&malformed)
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
        assess_evidence(&overflow)
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
    let assessment = assess_evidence(&stale_with_findings).expect("stale assessment");
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
#[trace("NFR-004-AC-3", "NFR-004-AC-4", "NFR-004-AC-5", "StR-001-VC-1")]
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
