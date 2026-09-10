// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-039 qualification-profile acceptance tests.

use std::collections::{BTreeMap, BTreeSet};

use ix_trace_rs::trace;
use quire_verification::qualification_profile::{
    QualificationProfileOutcome, freeze_qualification_profile, validate_qualification_profile,
};
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn artifact(kind: &str, identity: &str) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "qualification.agent-ix.local",
            "identity": identity,
            "revision": {
                "namespace": "quire-spec-production-profile",
                "value": "accepted-r1"
            },
            "digest": format!("sha256:{:x}", Sha256::digest(identity.as_bytes())),
            "wire": {
                "identity": "quire-verification.qualification-profile/v1",
                "version": "1"
            }
        }
    })
}

fn validated_references(profile: &Value) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(profile, &mut envelopes);
    let support = ReferenceSupport {
        wire_versions: BTreeMap::from([(
            "quire-verification.qualification-profile/v1".to_owned(),
            BTreeSet::from(["1".to_owned()]),
        )]),
        ..ReferenceSupport::default()
    };
    let mut reader = SharedReferenceReader::new(support);
    envelopes
        .into_iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let identity = value["identity"].as_str().expect("artifact identity");
            let expectation = ArtifactExpectation {
                ref_version: value["refVersion"].as_str().expect("refVersion").into(),
                kind: value["kind"].as_str().expect("kind").into(),
                authority: value["authority"].as_str().expect("authority").into(),
                identity: identity.into(),
                revision_namespace: value["revision"]["namespace"]
                    .as_str()
                    .expect("namespace")
                    .into(),
                revision_value: value["revision"]["value"]
                    .as_str()
                    .expect("revision")
                    .into(),
                digest: value["digest"].as_str().expect("digest").into(),
                wire_identity: value["wire"]["identity"]
                    .as_str()
                    .expect("wire identity")
                    .into(),
                wire_version: value["wire"]["version"]
                    .as_str()
                    .expect("wire version")
                    .into(),
                bytes: Some(identity.as_bytes().to_vec()),
            };
            reader
                .read(
                    &serde_json::to_vec(envelope).expect("reference JSON"),
                    &ReferenceExpectation::Artifact(expectation),
                )
                .expect("strict reference")
        })
        .collect()
}

fn collect_artifacts<'a>(value: &'a Value, artifacts: &mut Vec<&'a Value>) {
    match value {
        Value::Object(fields) => {
            if fields.get("kind").and_then(Value::as_str) == Some("artifact")
                && fields.get("wireVersion").is_some()
            {
                artifacts.push(value);
                return;
            }
            for child in fields.values() {
                collect_artifacts(child, artifacts);
            }
        }
        Value::Array(items) => {
            for child in items {
                collect_artifacts(child, artifacts);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn interface_kind(role: &str) -> &'static str {
    match role {
        "parser-compiler" => "executable-projection",
        "production-caller" => "invocation",
        "portable-contracts" | "shared-references" | "engineering-assurance" | "quoin-evidence" => {
            "source"
        }
        _ => panic!("unsupported qualification interface role"),
    }
}

fn interface(interface_id: &str, role: &str, owner: &str) -> Value {
    json!({
        "interfaceId": interface_id,
        "role": role,
        "owner": owner,
        "status": "accepted",
        "interfaceRef": artifact(interface_kind(role), &format!("interface:{interface_id}")),
        "reason": null
    })
}

fn native_claim_authority() -> Value {
    let rules = [
        "numeric-domain",
        "presence-null-invalid",
        "containment-references",
        "collections",
        "calls-result",
        "source-authority",
        "evaluation-instants",
        "reference-design-gate",
        "temporal-boundary",
        "runtime-dependency",
    ];
    json!({
        "kind": "native-formal-clause",
        "sourceProfileRef": artifact("source", "native-source-profile"),
        "admissionResults": rules.map(|rule| json!({
            "rule": rule,
            "status": "accepted",
            "evidenceRef": artifact("observation", &format!("admission:{rule}"))
        }))
    })
}

fn export_claim_authority() -> Value {
    json!({
        "kind": "export-mapping",
        "mappingProfileRef": artifact("source", "export-mapping-profile"),
        "lossReportRef": artifact("observation", "export-loss-report")
    })
}

fn profile() -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-profile/v1",
        "kind": "QualificationProfile",
        "profileId": "QP-QUIRE-SPEC-SYNTAX-001",
        "profileRevision": "1",
        "candidateRef": artifact("executable-projection", "quire-spec:binary"),
        "dependencyClosureRef": artifact("dependency-closure", "quire-spec:lockfile"),
        "productionCallerRef": artifact("invocation", "quire-spec:direct-cli-caller"),
        "commands": [{
            "commandId": "parse",
            "executableRef": artifact("executable-projection", "quire-spec:binary"),
            "arguments": ["parse", "profile:source", "source-r1", "<corpus-item-path>"],
            "workingDirectoryRef": artifact("environment", "quire-spec:workspace"),
            "inputBinding": "replace <corpus-item-path> with the retained corpus path",
            "expectedExitCodes": [0, 1, 2, 3],
            "responseKind": "quire-spec.syntax-cli/v1"
        }],
        "inputPopulation": {
            "populationId": "QUIRE-SPEC-SYNTAX-POPULATION-001",
            "description": "Selected production syntax inputs",
            "requiredClasses": ["valid", "malformed", "boundary", "mutation"],
            "selectionRationale": "Cover every declared syntax and refusal class in the profile"
        },
        "semanticGuarantees": [{
            "guaranteeId": "syntax-outcome",
            "stage": "syntax",
            "claim": "Return parsed, refused, incomplete, or usage outcomes for the selected population",
            "claimAuthority": {
                "kind": "non-formal",
                "reason": "This bounded profile claims syntax outcomes, not formal-clause semantics"
            }
        }],
        "environmentRef": artifact("environment", "linux:x86_64:rust-1.98.1"),
        "resourceBudgets": [{
            "budgetId": "parse-hot-path",
            "phase": "parse",
            "maximumWallClockNanoseconds": 10_000_000_000_u64,
            "maximumParallelism": 1,
            "maximumStdoutBytes": 16_777_216,
            "maximumStderrBytes": 16_777_216
        }],
        "evidenceRequirements": [{
            "obligationId": "syntax-examples",
            "guaranteeId": "syntax-outcome",
            "techniqueId": "native-parser-direct",
            "budgetId": "parse-hot-path",
            "stoppingRule": "every retained corpus item executes once with its expected outcome",
            "required": true
        }],
        "exclusions": [
            "No model binding, type checking, evaluation, backend, or unlisted environment claim"
        ],
        "interfaces": [
            interface("e02", "portable-contracts", "agent-ix/quire-verification"),
            interface("fs02-fs05", "shared-references", "agent-ix/quire-specification"),
            interface("compiler", "parser-compiler", "agent-ix/quire-spec-language"),
            interface("caller", "production-caller", "verification-owner"),
            interface("assurance", "engineering-assurance", "agent-ix/engineering-assurance"),
            interface("evidence", "quoin-evidence", "agent-ix/quoin")
        ]
    })
}

fn frozen_identity(value: Value) -> String {
    let references = validated_references(&value);
    match freeze_qualification_profile(value, &references).expect("valid profile decision") {
        QualificationProfileOutcome::Frozen(profile) => profile.identity().to_owned(),
        QualificationProfileOutcome::Blocked(blockers) => {
            panic!("profile unexpectedly blocked: {blockers:?}")
        }
    }
}

fn assert_required_dimensions_are_closed(original: &Value) {
    for field in [
        "contractVersion",
        "schemaVersion",
        "kind",
        "profileId",
        "profileRevision",
        "candidateRef",
        "dependencyClosureRef",
        "productionCallerRef",
        "commands",
        "inputPopulation",
        "semanticGuarantees",
        "environmentRef",
        "resourceBudgets",
        "evidenceRequirements",
        "exclusions",
        "interfaces",
    ] {
        let mut missing = original.clone();
        missing
            .as_object_mut()
            .expect("profile object")
            .remove(field);
        assert!(
            validate_qualification_profile(&missing, &validated_references(&missing)).is_err(),
            "missing {field} was accepted"
        );
    }
    for field in [
        "commands",
        "semanticGuarantees",
        "resourceBudgets",
        "evidenceRequirements",
        "exclusions",
        "interfaces",
    ] {
        let mut open = original.clone();
        open[field] = json!([]);
        assert!(
            validate_qualification_profile(&open, &validated_references(&open)).is_err(),
            "empty {field} was accepted"
        );
    }
    let mut extra = original.clone();
    extra["unspecifiedFutureGuarantees"] = json!(true);
    assert!(validate_qualification_profile(&extra, &validated_references(&extra)).is_err());
}

fn assert_profile_identities_are_unique() {
    for (collection, changed_field, replacement) in [
        ("commands", "responseKind", json!("quire-spec.other/v1")),
        (
            "semanticGuarantees",
            "claim",
            json!("A different claim with the same identity"),
        ),
        ("resourceBudgets", "phase", json!("format")),
        (
            "evidenceRequirements",
            "techniqueId",
            json!("different-technique"),
        ),
    ] {
        let mut duplicate_id = profile();
        let mut changed_item = duplicate_id[collection][0].clone();
        changed_item[changed_field] = replacement;
        duplicate_id[collection]
            .as_array_mut()
            .expect("profile collection")
            .push(changed_item);
        assert!(
            validate_qualification_profile(&duplicate_id, &validated_references(&duplicate_id))
                .is_err(),
            "duplicate identity in {collection} was accepted"
        );
    }
    let mut duplicate_interface_id = profile();
    duplicate_interface_id["interfaces"][1]["interfaceId"] = json!("e02");
    assert!(
        validate_qualification_profile(
            &duplicate_interface_id,
            &validated_references(&duplicate_interface_id)
        )
        .is_err()
    );
    let mut duplicate_interface_role = profile();
    duplicate_interface_role["interfaces"][1]["role"] = json!("portable-contracts");
    assert!(
        validate_qualification_profile(
            &duplicate_interface_role,
            &validated_references(&duplicate_interface_role)
        )
        .is_err()
    );
}

fn assert_profile_reference_roles_are_typed() {
    for (pointer, wrong_kind) in [
        ("/candidateRef/value/kind", "source"),
        ("/dependencyClosureRef/value/kind", "model-lock"),
        ("/productionCallerRef/value/kind", "run-artifact"),
        ("/commands/0/executableRef/value/kind", "source"),
        ("/commands/0/workingDirectoryRef/value/kind", "source"),
        ("/environmentRef/value/kind", "source"),
    ] {
        let mut wrong_role = profile();
        *wrong_role
            .pointer_mut(pointer)
            .expect("selected profile field") = json!(wrong_kind);
        assert!(
            validate_qualification_profile(&wrong_role, &validated_references(&wrong_role))
                .is_err(),
            "{pointer} admitted {wrong_kind}"
        );
    }

    for (index, role) in [
        "portable-contracts",
        "shared-references",
        "parser-compiler",
        "production-caller",
        "engineering-assurance",
        "quoin-evidence",
    ]
    .into_iter()
    .enumerate()
    {
        let expected = interface_kind(role);
        let wrong_kind = if expected == "source" {
            "run-artifact"
        } else {
            "source"
        };
        let mut wrong_role = profile();
        wrong_role["interfaces"][index]["interfaceRef"]["value"]["kind"] = json!(wrong_kind);
        assert!(
            validate_qualification_profile(&wrong_role, &validated_references(&wrong_role))
                .is_err(),
            "{role} admitted {wrong_kind} instead of {expected}"
        );
    }
}

#[trace("TC-039", "FR-022-AC-1", "FR-022-AC-2")]
#[test]
fn tc_039_freezes_only_a_complete_closed_schema_derived_profile() {
    let original = profile();
    let references = validated_references(&original);
    let frozen =
        match freeze_qualification_profile(original.clone(), &references).expect("valid profile") {
            QualificationProfileOutcome::Frozen(profile) => profile,
            QualificationProfileOutcome::Blocked(blockers) => {
                panic!("complete profile blocked: {blockers:?}")
            }
        };
    assert_eq!(
        serde_json::to_value(frozen.profile()).expect("generated profile serialization"),
        original
    );
    assert!(frozen.identity().starts_with("sha256-jcs:"));
    assert_required_dimensions_are_closed(&original);
    assert_profile_identities_are_unique();

    let mut unknown_guarantee = profile();
    unknown_guarantee["evidenceRequirements"][0]["guaranteeId"] = json!("absent");
    assert!(
        validate_qualification_profile(
            &unknown_guarantee,
            &validated_references(&unknown_guarantee)
        )
        .is_err()
    );

    let mut unknown_budget = profile();
    unknown_budget["evidenceRequirements"][0]["budgetId"] = json!("absent");
    assert!(
        validate_qualification_profile(&unknown_budget, &validated_references(&unknown_budget))
            .is_err()
    );

    assert_profile_reference_roles_are_typed();
}

#[trace("TC-039", "FR-022-AC-1", "FR-022-AC-2", "FR-022-AC-5")]
#[test]
fn tc_039_closes_nonformal_native_and_export_claim_authorities() {
    let nonformal = profile();
    assert!(validate_qualification_profile(&nonformal, &validated_references(&nonformal)).is_ok());

    let mut missing_reason = profile();
    missing_reason["semanticGuarantees"][0]["claimAuthority"] = json!({
        "kind": "non-formal"
    });
    assert!(
        validate_qualification_profile(&missing_reason, &validated_references(&missing_reason))
            .is_err()
    );

    let mut native = profile();
    native["semanticGuarantees"][0]["claimAuthority"] = native_claim_authority();
    assert!(validate_qualification_profile(&native, &validated_references(&native)).is_ok());

    let mut references_without_native_source = native.clone();
    references_without_native_source["semanticGuarantees"][0]["claimAuthority"]["sourceProfileRef"] =
        artifact("source", "unrelated-native-source-profile");
    assert!(
        validate_qualification_profile(
            &native,
            &validated_references(&references_without_native_source)
        )
        .is_err(),
        "native claim authority must be bound to its exact strict-reader proof"
    );

    for index in 0..10 {
        let mut missing = native.clone();
        missing["semanticGuarantees"][0]["claimAuthority"]["admissionResults"]
            .as_array_mut()
            .expect("admission evidence")
            .remove(index);
        assert!(
            validate_qualification_profile(&missing, &validated_references(&missing)).is_err(),
            "native authority accepted with admission index {index} absent"
        );
    }

    let mut duplicate = native.clone();
    duplicate["semanticGuarantees"][0]["claimAuthority"]["admissionResults"][1]["rule"] =
        json!("numeric-domain");
    assert!(validate_qualification_profile(&duplicate, &validated_references(&duplicate)).is_err());

    let mut gapped = native.clone();
    gapped["semanticGuarantees"][0]["claimAuthority"]["admissionResults"][0] = json!({
        "rule": "numeric-domain",
        "status": "gap",
        "owner": "quire-spec-language",
        "reason": "The native numeric profile has not yet been admitted",
        "limitations": ["No accepted bounded-rational evidence is retained"]
    });
    assert!(validate_qualification_profile(&gapped, &validated_references(&gapped)).is_ok());

    let mut wrong_source_role = native.clone();
    wrong_source_role["semanticGuarantees"][0]["claimAuthority"]["sourceProfileRef"]["value"]["kind"] =
        json!("observation");
    assert!(
        validate_qualification_profile(
            &wrong_source_role,
            &validated_references(&wrong_source_role)
        )
        .is_err()
    );

    let mut export = profile();
    export["semanticGuarantees"][0]["claimAuthority"] = export_claim_authority();
    assert!(validate_qualification_profile(&export, &validated_references(&export)).is_ok());

    let mut wrong_loss_role = export.clone();
    wrong_loss_role["semanticGuarantees"][0]["claimAuthority"]["lossReportRef"]["value"]["kind"] =
        json!("source");
    assert!(
        validate_qualification_profile(&wrong_loss_role, &validated_references(&wrong_loss_role))
            .is_err()
    );

    assert!(validate_qualification_profile(&export, &[]).is_err());
}

#[trace("TC-039", "FR-022-AC-2")]
#[test]
fn tc_039_does_not_infer_a_guarantee_from_commands_or_techniques() {
    let mut command_only = profile();
    command_only["semanticGuarantees"] = json!([]);
    command_only["evidenceRequirements"] = json!([]);
    assert!(
        !command_only["commands"]
            .as_array()
            .expect("commands")
            .is_empty()
    );
    assert!(
        validate_qualification_profile(&command_only, &validated_references(&command_only))
            .is_err()
    );

    let mut technique_without_guarantee = profile();
    technique_without_guarantee["semanticGuarantees"] = json!([]);
    assert!(
        !technique_without_guarantee["evidenceRequirements"]
            .as_array()
            .expect("evidence requirements")
            .is_empty()
    );
    assert!(
        validate_qualification_profile(
            &technique_without_guarantee,
            &validated_references(&technique_without_guarantee)
        )
        .is_err()
    );
}

#[trace("TC-039", "FR-022-AC-1")]
#[test]
fn tc_039_accepts_an_owner_supplied_phase_budget_above_ten_seconds() {
    let mut value = profile();
    value["resourceBudgets"][0]["maximumWallClockNanoseconds"] = json!(20_000_000_000_u64);
    let references = validated_references(&value);
    let frozen = match freeze_qualification_profile(value, &references)
        .expect("owner-supplied phase budget is valid")
    {
        QualificationProfileOutcome::Frozen(profile) => profile,
        QualificationProfileOutcome::Blocked(blockers) => {
            panic!("owner-supplied phase budget blocked: {blockers:?}")
        }
    };
    assert_eq!(
        serde_json::to_value(frozen.profile()).expect("generated profile serialization")["resourceBudgets"]
            [0]["maximumWallClockNanoseconds"],
        json!(20_000_000_000_u64)
    );
}

#[trace("TC-039", "FR-022-AC-3")]
#[test]
fn tc_039_returns_sorted_owned_blockers_without_a_local_substitute() {
    let mut value = profile();
    let interfaces = value["interfaces"].as_array_mut().expect("interfaces");
    for (index, status, reason) in [
        (
            2_usize,
            "provisional",
            "compiler revision is still under review",
        ),
        (
            4_usize,
            "missing",
            "assurance handoff has no accepted revision",
        ),
    ] {
        interfaces[index]["status"] = json!(status);
        interfaces[index]["interfaceRef"] = Value::Null;
        interfaces[index]["reason"] = json!(reason);
    }

    let references = validated_references(&value);
    let blockers = match freeze_qualification_profile(value, &references).expect("blocked decision")
    {
        QualificationProfileOutcome::Frozen(_) => panic!("blocked profile was frozen"),
        QualificationProfileOutcome::Blocked(blockers) => blockers,
    };
    assert_eq!(blockers.len(), 2);
    assert_eq!(blockers[0].interface_id(), "assurance");
    assert_eq!(blockers[0].owner(), "agent-ix/engineering-assurance");
    assert_eq!(blockers[0].status(), "missing");
    assert_eq!(blockers[1].interface_id(), "compiler");
    assert_eq!(blockers[1].owner(), "agent-ix/quire-spec-language");
    assert_eq!(blockers[1].status(), "provisional");

    let mut invalid = profile();
    invalid["interfaces"][0]["interfaceRef"] = Value::Null;
    assert!(validate_qualification_profile(&invalid, &validated_references(&invalid)).is_err());

    let mut substitute = profile();
    substitute["interfaces"][0]["status"] = json!("missing");
    substitute["interfaces"][0]["reason"] = json!("producer revision unavailable");
    assert!(
        validate_qualification_profile(&substitute, &validated_references(&substitute)).is_err()
    );

    let mut unexplained = profile();
    unexplained["interfaces"][0]["status"] = json!("unsupported");
    unexplained["interfaces"][0]["interfaceRef"] = Value::Null;
    assert!(
        validate_qualification_profile(&unexplained, &validated_references(&unexplained)).is_err()
    );

    let unproved = profile();
    assert!(validate_qualification_profile(&unproved, &[]).is_err());

    let mut changed_after_read = profile();
    let prior_proofs = validated_references(&changed_after_read);
    changed_after_read["candidateRef"]["value"]["revision"]["value"] = json!("accepted-r2");
    assert!(validate_qualification_profile(&changed_after_read, &prior_proofs).is_err());
}

#[trace("TC-039", "FR-022-AC-4")]
#[test]
fn tc_039_changes_identity_for_every_bound_claim_dimension() {
    let original = profile();
    let original_identity = frozen_identity(original.clone());
    let mutations = [
        ("profile revision", "/profileRevision", json!("2")),
        (
            "candidate",
            "/candidateRef/value/revision/value",
            json!("accepted-r2"),
        ),
        (
            "dependency closure",
            "/dependencyClosureRef/value/revision/value",
            json!("accepted-r2"),
        ),
        (
            "caller",
            "/productionCallerRef/value/revision/value",
            json!("accepted-r2"),
        ),
        ("command", "/commands/0/arguments/0", json!("format")),
        (
            "population",
            "/inputPopulation/requiredClasses/0",
            json!("unicode"),
        ),
        (
            "guarantee",
            "/semanticGuarantees/0/claim",
            json!("Return exact syntax outcomes and locations"),
        ),
        (
            "claim authority",
            "/semanticGuarantees/0/claimAuthority/reason",
            json!("This profile still excludes formal clauses for another exact reason"),
        ),
        (
            "environment",
            "/environmentRef/value/revision/value",
            json!("accepted-r2"),
        ),
        ("budget", "/resourceBudgets/0/maximumParallelism", json!(2)),
        (
            "evidence",
            "/evidenceRequirements/0/techniqueId",
            json!("native-parser-roundtrip"),
        ),
        (
            "exclusion",
            "/exclusions/0",
            json!("No backend or unlisted environment claim"),
        ),
        (
            "interface",
            "/interfaces/2/interfaceRef/value/revision/value",
            json!("accepted-r2"),
        ),
    ];
    for (dimension, pointer, replacement) in mutations {
        let mut changed = original.clone();
        *changed
            .pointer_mut(pointer)
            .expect("identity-bearing field") = replacement;
        assert_ne!(
            frozen_identity(changed),
            original_identity,
            "{dimension} did not change profile identity"
        );
    }

    assert_eq!(frozen_identity(original), original_identity);
}
