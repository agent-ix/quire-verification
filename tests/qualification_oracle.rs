// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-041 qualification-oracle manifest acceptance tests.

use std::collections::{BTreeMap, BTreeSet};

use ix_trace_rs::trace;
use quire_verification::contracts::{VerificationErrorCode, jcs_sha256};
use quire_verification::qualification_corpus::{
    FrozenQualificationCorpus, QualificationCorpusOutcome, qualify_corpus_manifest,
};
use quire_verification::qualification_oracle::{
    QualificationOracleOutcome, qualify_oracle_manifest,
};
use quire_verification::qualification_profile::{
    FrozenQualificationProfile, QualificationProfileOutcome, freeze_qualification_profile,
};
use quire_verification::shared_reference::{
    ArtifactExpectation, ReferenceExpectation, ReferenceSupport, SharedReferenceReader,
    ValidatedSharedReference,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn artifact(kind: &str, identity: &str, bytes: &[u8]) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "qualification.agent-ix.local",
            "identity": identity,
            "revision": {"namespace": "qualification-oracle", "value": "r1"},
            "digest": digest(bytes),
            "wire": {
                "identity": "quire-verification.qualification-oracle-fixture/v1",
                "version": "1"
            }
        }
    })
}

fn reference(kind: &str, identity: &str) -> Value {
    artifact(kind, identity, identity.as_bytes())
}

fn interface_kind(role: &str) -> &'static str {
    match role {
        "parser-compiler" => "executable-projection",
        "production-caller" => "invocation",
        "portable-contracts" | "shared-references" | "engineering-assurance" | "quoin-evidence" => {
            "source"
        }
        _ => panic!("unsupported test interface role"),
    }
}

fn profile_value() -> Value {
    let interface = |id: &str, role: &str| {
        json!({
            "interfaceId": id,
            "role": role,
            "owner": format!("owner:{id}"),
            "status": "accepted",
            "interfaceRef": reference(interface_kind(role), &format!("interface:{id}")),
            "reason": null
        })
    };
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-profile/v1",
        "kind": "QualificationProfile",
        "profileId": "QP-ORACLE-001",
        "profileRevision": "1",
        "candidateRef": reference("executable-projection", "candidate"),
        "dependencyClosureRef": reference("dependency-closure", "closure"),
        "productionCallerRef": reference("invocation", "caller"),
        "commands": [{
            "commandId": "parse",
            "executableRef": reference("executable-projection", "candidate"),
            "arguments": ["parse", "<corpus-item-path>"],
            "workingDirectoryRef": reference("environment", "workspace"),
            "inputBinding": "replace the corpus path",
            "expectedExitCodes": [0, 1],
            "responseKind": "quire-spec.syntax-cli/v1"
        }],
        "inputPopulation": {
            "populationId": "POP-ORACLE-001",
            "description": "Closed oracle test population",
            "requiredClasses": ["malformed", "valid"],
            "selectionRationale": "Exercise acceptance and refusal"
        },
        "semanticGuarantees": [
            {
                "guaranteeId": "syntax-outcome",
                "stage": "syntax",
                "claim": "Return the authored syntax outcome"
            },
            {
                "guaranteeId": "diagnostic-span",
                "stage": "syntax",
                "claim": "Return an exact diagnostic span"
            }
        ],
        "environmentRef": reference("environment", "environment"),
        "resourceBudgets": [{
            "budgetId": "parse",
            "phase": "parse",
            "maximumWallClockNanoseconds": 20_000_000_000_u64,
            "maximumParallelism": 1,
            "maximumStdoutBytes": 4096,
            "maximumStderrBytes": 4096
        }],
        "evidenceRequirements": [
            {
                "obligationId": "syntax",
                "guaranteeId": "syntax-outcome",
                "techniqueId": "oracle-comparison",
                "stoppingRule": "every retained input once",
                "required": true
            },
            {
                "obligationId": "span",
                "guaranteeId": "diagnostic-span",
                "techniqueId": "oracle-comparison",
                "stoppingRule": "every malformed input once",
                "required": true
            }
        ],
        "exclusions": ["No guarantee outside the two named syntax claims"],
        "interfaces": [
            interface("e02", "portable-contracts"),
            interface("shared", "shared-references"),
            interface("compiler", "parser-compiler"),
            interface("caller", "production-caller"),
            interface("assurance", "engineering-assurance"),
            interface("evidence", "quoin-evidence")
        ]
    })
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

fn proofs(value: &Value, retained: &BTreeMap<String, Vec<u8>>) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(value, &mut envelopes);
    let mut reader = SharedReferenceReader::new(ReferenceSupport {
        wire_versions: BTreeMap::from([(
            "quire-verification.qualification-oracle-fixture/v1".to_owned(),
            BTreeSet::from(["1".to_owned()]),
        )]),
        ..ReferenceSupport::default()
    });
    envelopes
        .into_iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let identity = value["identity"].as_str().expect("identity");
            let bytes = retained
                .get(identity)
                .cloned()
                .unwrap_or_else(|| identity.as_bytes().to_vec());
            reader
                .read(
                    &serde_json::to_vec(envelope).expect("reference JSON"),
                    &ReferenceExpectation::Artifact(ArtifactExpectation {
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
                        bytes: Some(bytes),
                    }),
                )
                .expect("strict reference")
        })
        .collect()
}

fn frozen_profile() -> Box<FrozenQualificationProfile> {
    let value = profile_value();
    match freeze_qualification_profile(value.clone(), &proofs(&value, &BTreeMap::new()))
        .expect("profile decision")
    {
        QualificationProfileOutcome::Frozen(profile) => profile,
        QualificationProfileOutcome::Blocked(blockers) => panic!("profile blocked: {blockers:?}"),
    }
}

fn corpus_value(profile: &FrozenQualificationProfile) -> (Value, BTreeMap<String, Vec<u8>>) {
    let valid = b"kind: valid\n".to_vec();
    let malformed = b"kind: [".to_vec();
    let entry = |entry_id: &str, class: &str, identity: &str, bytes: &[u8]| {
        json!({
            "entryId": entry_id,
            "artifactRef": artifact("source", identity, bytes),
            "classes": [class],
            "origin": "repository",
            "provenance": {
                "authority": "agent-ix/quire-specification",
                "revision": "fixture-r1",
                "path": format!("fixtures/{entry_id}.quire")
            },
            "rights": {
                "owner": "agent-ix",
                "disposition": "redistributable",
                "basis": "repository fixture"
            },
            "selectionReason": format!("Represent {class} syntax"),
            "lineage": null
        })
    };
    (
        json!({
            "contractVersion": "quire-verification/e02-draft-2",
            "schemaVersion": "quire-verification.qualification-corpus/v1",
            "kind": "QualificationCorpusManifest",
            "manifestId": "QC-ORACLE-001",
            "manifestRevision": "1",
            "profileIdentity": profile.identity(),
            "populationId": "POP-ORACLE-001",
            "requiredClasses": ["malformed", "valid"],
            "entries": [
                entry("valid", "valid", "input:valid", &valid),
                entry("malformed", "malformed", "input:malformed", &malformed)
            ],
            "gaps": [],
            "exclusions": []
        }),
        BTreeMap::from([
            ("input:valid".to_owned(), valid),
            ("input:malformed".to_owned(), malformed),
        ]),
    )
}

fn frozen_corpus(profile: &FrozenQualificationProfile) -> Box<FrozenQualificationCorpus> {
    let (value, bytes) = corpus_value(profile);
    match qualify_corpus_manifest(value.clone(), profile, &proofs(&value, &bytes))
        .expect("corpus decision")
    {
        QualificationCorpusOutcome::Complete(corpus) => corpus,
        QualificationCorpusOutcome::Incomplete { gaps, .. } => {
            panic!("corpus unexpectedly incomplete: {gaps:?}")
        }
    }
}

fn binding(id: &str, suffix: &str, mode: &str, entries: &Value) -> Value {
    json!({
        "bindingId": id,
        "guaranteeId": "syntax-outcome",
        "oracleRef": reference("oracle", &format!("oracle:{suffix}")),
        "executableRef": reference("executable-projection", &format!("executable:{suffix}")),
        "comparisonProcedureRef": reference(
            "review-procedure",
            &format!("comparison:{suffix}")
        ),
        "inputProjection": {"mode": mode, "entryIds": entries},
        "expectedRelation": {
            "relationId": "same-syntax-outcome",
            "description": "Candidate and oracle return the same syntax outcome",
            "expectedOutcome": "agreed",
            "origin": "oracle"
        },
        "dependence": {
            "class": "independent",
            "evidenceRefs": [
                reference("source", &format!("independence:{suffix}:b")),
                reference("source", &format!("independence:{suffix}:a"))
            ],
            "candidateAncestryRefs": [
                reference("source", &format!("candidate-ancestry:{suffix}:b")),
                reference("source", &format!("candidate-ancestry:{suffix}:a"))
            ],
            "oracleAncestryRefs": [
                reference("source", &format!("oracle-ancestry:{suffix}:b")),
                reference("source", &format!("oracle-ancestry:{suffix}:a"))
            ]
        },
        "limitations": ["Syntax stage only", "No broader semantic claim"],
        "disagreementProcedureRef": reference(
            "review-procedure",
            &format!("disagreement:{suffix}")
        ),
        "executionRequestRef": reference("invocation", &format!("execution:{suffix}")),
        "comparisonRequestRef": reference("invocation", &format!("comparison-request:{suffix}"))
    })
}

fn manifest_value(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-oracles/v1",
        "kind": "QualificationOracleManifest",
        "manifestId": "QO-001",
        "profileIdentity": profile.identity(),
        "corpusIdentity": corpus.identity(),
        "bindings": [
            binding("binding-b", "b", "ordered-batch", &json!(["malformed", "valid"])),
            binding("binding-a", "a", "each-entry", &json!(["valid"]))
        ],
        "gaps": [{
            "gapId": "gap-span",
            "guaranteeId": "diagnostic-span",
            "owner": "qualification-owner",
            "reason": "No independently implemented span oracle is accepted",
            "limitations": ["Span correctness remains unqualified", "Syntax outcome still tested"]
        }]
    })
}

fn fixture() -> (
    Box<FrozenQualificationProfile>,
    Box<FrozenQualificationCorpus>,
    Value,
) {
    let profile = frozen_profile();
    let corpus = frozen_corpus(&profile);
    let manifest = manifest_value(&profile, &corpus);
    (profile, corpus, manifest)
}

fn remove_field(value: &mut Value, pointer: &str) {
    let (parent, field) = pointer.rsplit_once('/').expect("field pointer");
    value
        .pointer_mut(parent)
        .and_then(Value::as_object_mut)
        .expect("object parent")
        .remove(field);
}

fn assert_manifest_refused(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    changed: &Value,
) {
    assert_eq!(
        qualify_oracle_manifest(
            changed.clone(),
            profile,
            corpus,
            &proofs(changed, &BTreeMap::new())
        )
        .expect_err("invalid oracle manifest")
        .code(),
        VerificationErrorCode::InvalidQualificationOracleManifest
    );
}

fn assert_binding_refused(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    changed: &Value,
    message: &str,
) {
    assert_eq!(
        qualify_oracle_manifest(
            changed.clone(),
            profile,
            corpus,
            &proofs(changed, &BTreeMap::new())
        )
        .expect_err(message)
        .code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );
}

fn frozen_manifest_json(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    manifest: &Value,
) -> Value {
    let frozen = match qualify_oracle_manifest(
        manifest.clone(),
        profile,
        corpus,
        &proofs(manifest, &BTreeMap::new()),
    )
    .expect("valid oracle manifest")
    {
        QualificationOracleOutcome::Complete(manifest)
        | QualificationOracleOutcome::Incomplete { manifest, .. } => manifest,
    };
    serde_json::to_value(frozen.manifest()).expect("frozen manifest serialization")
}

fn frozen_binding<'a>(manifest: &'a Value, binding_id: &str) -> &'a Value {
    manifest["bindings"]
        .as_array()
        .expect("frozen bindings")
        .iter()
        .find(|binding| binding["bindingId"] == binding_id)
        .expect("frozen binding identifier")
}

#[trace("TC-041", "FR-024-AC-1", "FR-024-AC-4")]
#[test]
fn tc_041_freezes_closed_manifest_and_normalizes_only_set_like_members() {
    let (profile, corpus, manifest) = fixture();
    let mut proof = proofs(&manifest, &BTreeMap::new());
    let (identity, frozen, gaps) =
        match qualify_oracle_manifest(manifest.clone(), &profile, &corpus, &proof)
            .expect("oracle decision")
        {
            QualificationOracleOutcome::Complete(_) => panic!("gapped manifest completed"),
            QualificationOracleOutcome::Incomplete { manifest, gaps } => {
                (manifest.identity().to_owned(), manifest, gaps)
            }
        };
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].guarantee_id(), "diagnostic-span");
    assert_eq!(gaps[0].owner(), "qualification-owner");
    assert!(gaps[0].reason().contains("span oracle"));
    assert_eq!(gaps[0].limitations().len(), 2);

    let normalized = serde_json::to_value(frozen.manifest()).expect("manifest serialization");
    assert_eq!(normalized["bindings"][0]["bindingId"], "binding-a");
    assert_eq!(
        normalized["bindings"][1]["inputProjection"]["entryIds"],
        json!(["malformed", "valid"]),
        "ordered corpus projection was normalized as a set"
    );

    let mut permuted = manifest;
    permuted["bindings"]
        .as_array_mut()
        .expect("bindings")
        .reverse();
    for binding in permuted["bindings"].as_array_mut().expect("bindings") {
        binding["limitations"]
            .as_array_mut()
            .expect("limitations")
            .reverse();
        binding["dependence"]["evidenceRefs"]
            .as_array_mut()
            .expect("evidence refs")
            .reverse();
        binding["dependence"]["candidateAncestryRefs"]
            .as_array_mut()
            .expect("candidate ancestry")
            .reverse();
        binding["dependence"]["oracleAncestryRefs"]
            .as_array_mut()
            .expect("oracle ancestry")
            .reverse();
    }
    permuted["gaps"][0]["limitations"]
        .as_array_mut()
        .expect("gap limitations")
        .reverse();
    proof.reverse();
    let permuted_identity = match qualify_oracle_manifest(permuted, &profile, &corpus, &proof)
        .expect("permuted oracle decision")
    {
        QualificationOracleOutcome::Complete(_) => panic!("gapped manifest completed"),
        QualificationOracleOutcome::Incomplete { manifest, .. } => manifest.identity().to_owned(),
    };
    assert_eq!(identity, permuted_identity);
    assert_eq!(identity, jcs_sha256(&normalized).expect("normalized JCS"));
}

#[trace("TC-041", "FR-024-AC-1")]
#[test]
fn tc_041_refuses_missing_fields() {
    let (profile, corpus, manifest) = fixture();
    for pointer in [
        "/contractVersion",
        "/schemaVersion",
        "/kind",
        "/manifestId",
        "/profileIdentity",
        "/corpusIdentity",
        "/bindings",
        "/gaps",
        "/bindings/0/bindingId",
        "/bindings/0/guaranteeId",
        "/bindings/0/oracleRef",
        "/bindings/0/executableRef",
        "/bindings/0/comparisonProcedureRef",
        "/bindings/0/inputProjection",
        "/bindings/0/inputProjection/mode",
        "/bindings/0/inputProjection/entryIds",
        "/bindings/0/expectedRelation",
        "/bindings/0/expectedRelation/relationId",
        "/bindings/0/expectedRelation/description",
        "/bindings/0/expectedRelation/expectedOutcome",
        "/bindings/0/expectedRelation/origin",
        "/bindings/0/dependence",
        "/bindings/0/dependence/class",
        "/bindings/0/dependence/evidenceRefs",
        "/bindings/0/dependence/candidateAncestryRefs",
        "/bindings/0/dependence/oracleAncestryRefs",
        "/bindings/0/limitations",
        "/bindings/0/disagreementProcedureRef",
        "/bindings/0/executionRequestRef",
        "/bindings/0/comparisonRequestRef",
        "/gaps/0/gapId",
        "/gaps/0/guaranteeId",
        "/gaps/0/owner",
        "/gaps/0/reason",
        "/gaps/0/limitations",
    ] {
        let mut changed = manifest.clone();
        remove_field(&mut changed, pointer);
        let error = qualify_oracle_manifest(
            changed.clone(),
            &profile,
            &corpus,
            &proofs(&changed, &BTreeMap::new()),
        )
        .expect_err("missing field");
        assert_eq!(
            error.code(),
            VerificationErrorCode::InvalidQualificationOracleManifest,
            "missing field {pointer} used the wrong refusal"
        );
    }
}

#[trace("TC-041", "FR-024-AC-1")]
#[test]
fn tc_041_refuses_duplicate_unknown_and_conflicting_coverage() {
    let (profile, corpus, manifest) = fixture();

    let mut extra = manifest.clone();
    extra["bindings"][0]["unknown"] = json!(true);
    assert_manifest_refused(&profile, &corpus, &extra);

    let mut duplicate_id = manifest.clone();
    duplicate_id["bindings"][1]["bindingId"] = json!("binding-b");
    assert_manifest_refused(&profile, &corpus, &duplicate_id);

    let mut duplicate_gap = manifest.clone();
    let mut second_gap = duplicate_gap["gaps"][0].clone();
    second_gap["gapId"] = json!("gap-span-2");
    duplicate_gap["gaps"]
        .as_array_mut()
        .expect("gaps")
        .push(second_gap);
    assert_manifest_refused(&profile, &corpus, &duplicate_gap);

    for changed in [
        {
            let mut changed = manifest.clone();
            changed["bindings"][0]["limitations"] = json!([]);
            changed
        },
        {
            let mut changed = manifest.clone();
            changed["gaps"][0]["owner"] = json!("");
            changed
        },
        {
            let mut changed = manifest.clone();
            changed["bindings"][0]["inputProjection"]["entryIds"] = json!("valid");
            changed
        },
    ] {
        assert_manifest_refused(&profile, &corpus, &changed);
    }

    for changed in [
        {
            let mut changed = manifest.clone();
            changed["bindings"][0]["guaranteeId"] = json!("unknown");
            changed
        },
        {
            let mut changed = manifest.clone();
            changed["bindings"][0]["guaranteeId"] = json!("diagnostic-span");
            changed
        },
        {
            let mut changed = manifest.clone();
            changed["bindings"]
                .as_array_mut()
                .expect("bindings")
                .clear();
            changed
        },
        {
            let mut changed = manifest.clone();
            changed["bindings"][0]["inputProjection"]["entryIds"][0] = json!("missing");
            changed
        },
    ] {
        assert_manifest_refused(&profile, &corpus, &changed);
    }
}

#[trace("TC-041", "FR-024-AC-2")]
#[test]
fn tc_041_accepts_each_dependence_form() {
    let (profile, corpus, manifest) = fixture();

    let mut shared = manifest.clone();
    shared["bindings"][1]["oracleRef"] = shared["bindings"][0]["oracleRef"].clone();
    for binding in shared["bindings"].as_array_mut().expect("bindings") {
        binding["dependence"] = json!({
            "class": "shared",
            "groupId": "shared-oracle",
            "bases": ["oracle"]
        });
    }
    let frozen = frozen_manifest_json(&profile, &corpus, &shared);
    assert!(
        frozen["bindings"]
            .as_array()
            .expect("shared bindings")
            .iter()
            .all(|binding| binding["dependence"]
                == json!({"class": "shared", "groupId": "shared-oracle", "bases": ["oracle"]}))
    );

    let mut shared_expected = shared.clone();
    shared_expected["bindings"][0]["expectedRelation"]["origin"] = json!("shared");
    shared_expected["bindings"][0]["dependence"]["bases"] = json!(["oracle", "expected-value"]);
    let frozen = frozen_manifest_json(&profile, &corpus, &shared_expected);
    let binding = frozen_binding(&frozen, "binding-b");
    assert_eq!(binding["expectedRelation"]["origin"], "shared");
    assert_eq!(
        binding["dependence"]["bases"],
        json!(["expected-value", "oracle"])
    );

    let mut derived = manifest.clone();
    derived["bindings"][1]["dependence"] =
        json!({"class": "derived", "sourceBindingId": "binding-b"});
    let frozen = frozen_manifest_json(&profile, &corpus, &derived);
    assert_eq!(
        frozen_binding(&frozen, "binding-a")["dependence"],
        json!({"class": "derived", "sourceBindingId": "binding-b"})
    );

    let mut unknown = manifest.clone();
    unknown["bindings"][0]["dependence"] =
        json!({"class": "unknown", "reason": "ancestry has not been established"});
    let frozen = frozen_manifest_json(&profile, &corpus, &unknown);
    assert_eq!(
        frozen_binding(&frozen, "binding-b")["dependence"],
        json!({"class": "unknown", "reason": "ancestry has not been established"})
    );
}

#[trace("TC-041", "FR-024-AC-2")]
#[test]
fn tc_041_refuses_false_independence() {
    let (profile, corpus, manifest) = fixture();

    for field in ["oracleRef", "executableRef", "comparisonProcedureRef"] {
        let mut shared_as_independent = manifest.clone();
        shared_as_independent["bindings"][1][field] =
            shared_as_independent["bindings"][0][field].clone();
        assert_binding_refused(
            &profile,
            &corpus,
            &shared_as_independent,
            "shared exact reference declared independent",
        );
    }

    let mut candidate_as_independent = manifest.clone();
    let profile_json = serde_json::to_value(profile.profile()).expect("profile serialization");
    candidate_as_independent["bindings"][0]["executableRef"] = profile_json["candidateRef"].clone();
    assert_binding_refused(
        &profile,
        &corpus,
        &candidate_as_independent,
        "candidate declared independent",
    );

    let mut no_evidence = manifest.clone();
    no_evidence["bindings"][0]["dependence"]["evidenceRefs"] = json!([]);
    assert_manifest_refused(&profile, &corpus, &no_evidence);

    let mut intersecting_ancestry = manifest.clone();
    intersecting_ancestry["bindings"][0]["dependence"]["oracleAncestryRefs"][0] =
        intersecting_ancestry["bindings"][0]["dependence"]["candidateAncestryRefs"][0].clone();
    assert_binding_refused(
        &profile,
        &corpus,
        &intersecting_ancestry,
        "intersecting ancestry declared independent",
    );

    for origin in ["candidate", "shared", "unknown"] {
        let mut changed = manifest.clone();
        changed["bindings"][0]["expectedRelation"]["origin"] = json!(origin);
        assert_binding_refused(
            &profile,
            &corpus,
            &changed,
            "non-independent expected value declared independent",
        );
    }
}

#[trace("TC-041", "FR-024-AC-2")]
#[test]
fn tc_041_refuses_incomplete_or_cross_tagged_dependence_forms() {
    let (profile, corpus, manifest) = fixture();
    for dependence in [
        json!({"class": "shared", "bases": ["oracle"]}),
        json!({"class": "shared", "groupId": "g"}),
        json!({
            "class": "shared",
            "groupId": "g",
            "bases": ["oracle"],
            "evidenceRefs": [reference("source", "foreign")]
        }),
        json!({"class": "derived", "sourceBindingId": "binding-b", "reason": "foreign"}),
        json!({"class": "unknown", "reason": "not established", "sourceBindingId": "binding-b"}),
        json!({
            "class": "independent",
            "evidenceRefs": [reference("source", "evidence")],
            "candidateAncestryRefs": [reference("source", "candidate-ancestry")],
            "oracleAncestryRefs": [reference("source", "oracle-ancestry")],
            "groupId": "foreign"
        }),
    ] {
        let mut changed = manifest.clone();
        changed["bindings"][0]["dependence"] = dependence;
        assert_manifest_refused(&profile, &corpus, &changed);
    }

    for origin in ["candidate", "shared"] {
        let mut changed = manifest.clone();
        changed["bindings"][0]["dependence"] =
            json!({"class": "shared", "groupId": "g", "bases": ["oracle"]});
        changed["bindings"][0]["expectedRelation"]["origin"] = json!(origin);
        assert_binding_refused(
            &profile,
            &corpus,
            &changed,
            "shared expected value omitted the expected-value basis",
        );
    }
}

#[trace("TC-041", "FR-024-AC-2")]
#[test]
fn tc_041_reaches_and_discriminates_the_derived_graph_gate() {
    let (profile, corpus, manifest) = fixture();

    for source in ["binding-b", "missing"] {
        let mut changed = manifest.clone();
        changed["bindings"][0]["dependence"] =
            json!({"class": "derived", "sourceBindingId": source});
        let error = qualify_oracle_manifest(
            changed.clone(),
            &profile,
            &corpus,
            &proofs(&changed, &BTreeMap::new()),
        )
        .expect_err("invalid derived source");
        assert!(
            error.to_string().contains("derived oracle source"),
            "source {source} was refused before the graph gate: {error}"
        );
    }

    let mut cyclic = manifest;
    cyclic["bindings"][0]["dependence"] =
        json!({"class": "derived", "sourceBindingId": "binding-a"});
    cyclic["bindings"][1]["dependence"] =
        json!({"class": "derived", "sourceBindingId": "binding-b"});
    let error = qualify_oracle_manifest(
        cyclic.clone(),
        &profile,
        &corpus,
        &proofs(&cyclic, &BTreeMap::new()),
    )
    .expect_err("cyclic derived bindings");
    assert!(
        error.to_string().contains("cycle"),
        "cycle was refused before the graph gate: {error}"
    );
}

#[trace("TC-041", "FR-024-AC-4")]
#[test]
fn tc_041_requires_exact_strict_reader_proofs_and_frozen_identities() {
    let (profile, corpus, manifest) = fixture();
    let no_proofs = qualify_oracle_manifest(manifest.clone(), &profile, &corpus, &[])
        .expect_err("missing strict-reader proofs");
    assert_eq!(
        no_proofs.code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );

    let proof = proofs(&manifest, &BTreeMap::new());
    let mut changed = manifest.clone();
    changed["bindings"][0]["oracleRef"]["value"]["identity"] = json!("oracle:changed");
    assert_eq!(
        qualify_oracle_manifest(changed, &profile, &corpus, &proof)
            .expect_err("changed oracle reused proofs")
            .code(),
        VerificationErrorCode::InvalidQualificationOracleBinding
    );

    for field in ["profileIdentity", "corpusIdentity"] {
        let mut changed = manifest.clone();
        changed[field] =
            json!("sha256-jcs:0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(
            qualify_oracle_manifest(
                changed.clone(),
                &profile,
                &corpus,
                &proofs(&changed, &BTreeMap::new())
            )
            .expect_err("foreign frozen identity")
            .code(),
            VerificationErrorCode::InvalidQualificationOracleManifest
        );
    }
}

#[trace("TC-041", "FR-024-AC-4")]
#[test]
fn tc_041_changes_identity_for_each_portable_manifest_dimension() {
    let (profile, corpus, manifest) = fixture();
    let baseline = jcs_sha256(&frozen_manifest_json(&profile, &corpus, &manifest))
        .expect("baseline manifest identity");

    for (pointer, replacement) in [
        ("/manifestId", json!("QO-002")),
        ("/bindings/0/bindingId", json!("binding-b-2")),
        (
            "/bindings/0/oracleRef",
            reference("oracle", "oracle:changed"),
        ),
        (
            "/bindings/0/executableRef",
            reference("executable-projection", "executable:changed"),
        ),
        (
            "/bindings/0/comparisonProcedureRef",
            reference("review-procedure", "comparison:changed"),
        ),
        (
            "/bindings/0/disagreementProcedureRef",
            reference("review-procedure", "disagreement:changed"),
        ),
        (
            "/bindings/0/executionRequestRef",
            reference("invocation", "execution:changed"),
        ),
        (
            "/bindings/0/comparisonRequestRef",
            reference("invocation", "comparison-request:changed"),
        ),
        ("/bindings/0/inputProjection/mode", json!("each-entry")),
        ("/bindings/0/inputProjection/entryIds", json!(["valid"])),
        (
            "/bindings/0/expectedRelation/relationId",
            json!("changed-relation"),
        ),
        (
            "/bindings/0/expectedRelation/description",
            json!("Changed exact comparison relation"),
        ),
        (
            "/bindings/0/expectedRelation/origin",
            json!("independent-third-party"),
        ),
        (
            "/bindings/0/dependence/evidenceRefs/0",
            reference("source", "independence:changed"),
        ),
        (
            "/bindings/0/dependence/candidateAncestryRefs/0",
            reference("source", "candidate-ancestry:changed"),
        ),
        (
            "/bindings/0/dependence/oracleAncestryRefs/0",
            reference("source", "oracle-ancestry:changed"),
        ),
        ("/bindings/0/limitations/0", json!("Changed limitation")),
        ("/gaps/0/gapId", json!("gap-span-2")),
        ("/gaps/0/owner", json!("changed-owner")),
        ("/gaps/0/reason", json!("Changed no-oracle reason")),
        ("/gaps/0/limitations/0", json!("Changed gap limitation")),
    ] {
        let mut changed = manifest.clone();
        *changed
            .pointer_mut(pointer)
            .expect("identity mutation path") = replacement;
        let identity = jcs_sha256(&frozen_manifest_json(&profile, &corpus, &changed))
            .expect("changed manifest identity");
        assert_ne!(baseline, identity, "identity ignored {pointer}");
    }
}

#[trace("TC-041", "FR-024-AC-1")]
#[test]
fn tc_041_reports_complete_only_when_every_guarantee_has_a_binding() {
    let (profile, corpus, mut manifest) = fixture();
    let mut span = binding("binding-span", "span", "each-entry", &json!(["malformed"]));
    span["guaranteeId"] = json!("diagnostic-span");
    manifest["bindings"]
        .as_array_mut()
        .expect("bindings")
        .push(span);
    manifest["gaps"] = json!([]);
    let result = qualify_oracle_manifest(
        manifest.clone(),
        &profile,
        &corpus,
        &proofs(&manifest, &BTreeMap::new()),
    )
    .expect("complete oracle decision");
    assert!(matches!(result, QualificationOracleOutcome::Complete(_)));
}
