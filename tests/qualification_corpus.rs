// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-040 qualification-corpus acceptance tests.

use std::collections::{BTreeMap, BTreeSet};

use ix_trace_rs::trace;
use quire_verification::qualification_corpus::{
    QualificationCorpusOutcome, qualify_corpus_manifest,
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
            "revision": {"namespace": "qualification-corpus", "value": "r1"},
            "digest": digest(bytes),
            "wire": {"identity": "quire-verification.qualification-corpus/v1", "version": "1"}
        }
    })
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

fn profile() -> Value {
    let reference = |kind: &str, identity: &str| artifact(kind, identity, identity.as_bytes());
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
        "profileId": "QP-CORPUS-001",
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
            "populationId": "POP-CORPUS-001",
            "description": "Closed syntax population",
            "requiredClasses": ["valid", "malformed", "boundary", "mutation"],
            "selectionRationale": "Exercise each declared syntax population class"
        },
        "semanticGuarantees": [{
            "guaranteeId": "authored-syntax",
            "stage": "syntax",
            "claim": "Return an authored syntax outcome"
        }],
        "environmentRef": reference("environment", "environment"),
        "resourceBudgets": [{
            "budgetId": "parse",
            "phase": "parse",
            "maximumWallClockNanoseconds": 20_000_000_000_u64,
            "maximumParallelism": 1,
            "maximumStdoutBytes": 4096,
            "maximumStderrBytes": 4096
        }],
        "evidenceRequirements": [{
            "obligationId": "syntax",
            "guaranteeId": "authored-syntax",
            "techniqueId": "native-parser",
            "stoppingRule": "each retained input once",
            "required": true
        }],
        "exclusions": ["No guarantee outside the authored syntax claim"],
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

fn proofs(value: &Value, bytes: &BTreeMap<String, Vec<u8>>) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(value, &mut envelopes);
    let support = || ReferenceSupport {
        wire_versions: BTreeMap::from([(
            "quire-verification.qualification-corpus/v1".to_owned(),
            BTreeSet::from(["1".to_owned()]),
        )]),
        ..ReferenceSupport::default()
    };
    let read = |reader: &mut SharedReferenceReader, envelope: &Value, retained: Vec<u8>| {
        let value = &envelope["value"];
        reader
            .read(
                &serde_json::to_vec(envelope).expect("reference JSON"),
                &ReferenceExpectation::Artifact(ArtifactExpectation {
                    ref_version: value["refVersion"].as_str().expect("refVersion").into(),
                    kind: value["kind"].as_str().expect("kind").into(),
                    authority: value["authority"].as_str().expect("authority").into(),
                    identity: value["identity"].as_str().expect("identity").into(),
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
                    bytes: Some(retained),
                }),
            )
            .expect("strict reference")
    };
    let mut reader = SharedReferenceReader::new(support());
    envelopes
        .into_iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let identity = value["identity"].as_str().expect("identity");
            let retained = bytes
                .get(identity)
                .cloned()
                .unwrap_or_else(|| identity.as_bytes().to_vec());
            read(&mut reader, envelope, retained)
        })
        .collect()
}

fn proofs_from_independent_readers(
    value: &Value,
    bytes: &BTreeMap<String, Vec<u8>>,
) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(value, &mut envelopes);
    envelopes
        .into_iter()
        .flat_map(|envelope| {
            let expected_digest = envelope["value"]["digest"].as_str().expect("digest");
            let retained = bytes
                .values()
                .find(|candidate| digest(candidate) == expected_digest)
                .expect("retained bytes by digest")
                .clone();
            proofs(
                envelope,
                &BTreeMap::from([(
                    envelope["value"]["identity"]
                        .as_str()
                        .expect("identity")
                        .to_owned(),
                    retained,
                )]),
            )
        })
        .collect()
}

fn frozen_profile() -> Box<FrozenQualificationProfile> {
    let value = profile();
    match freeze_qualification_profile(value.clone(), &proofs(&value, &BTreeMap::new()))
        .expect("profile decision")
    {
        QualificationProfileOutcome::Frozen(profile) => profile,
        QualificationProfileOutcome::Blocked(blockers) => {
            panic!("profile blocked: {blockers:?}")
        }
    }
}

fn replay_run(entry_id: &str, lineage: &Value, output: &[u8]) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "schemaVersion": "quire-verification.qualification-corpus-replay/v1",
        "kind": "QualificationCorpusReplayRun",
        "status": "completed",
        "entryId": entry_id,
        "lineageIdentity": quire_verification::contracts::jcs_sha256(lineage)
            .expect("lineage identity"),
        "outputDigest": digest(output)
    }))
    .expect("run JSON")
}

fn corpus() -> (Value, BTreeMap<String, Vec<u8>>) {
    let real = b"kind: valid\n".to_vec();
    let malformed = b"kind: [".to_vec();
    let generated = b"x".repeat(64);
    let mutation = b"kind valid\n".to_vec();
    let generator_ref = artifact("executable-projection", "generator", b"generator");
    let generated_claim = json!({
        "kind": "generated",
        "generatorRef": generator_ref,
        "seed": "seed-001",
        "invocation": ["generate", "--size", "64"]
    });
    let mutation_claim = json!({
        "kind": "mutation",
        "parentEntryId": "real",
        "seed": "seed-002",
        "transformation": "delete the first colon"
    });
    let generated_run = replay_run("generated", &generated_claim, &generated);
    let mutation_run = replay_run("mutation", &mutation_claim, &mutation);
    let mut generated_lineage = generated_claim;
    generated_lineage
        .as_object_mut()
        .expect("generated lineage")
        .insert(
            "replayRunRef".to_owned(),
            artifact("run-artifact", "run:generated", &generated_run),
        );
    let mut mutation_lineage = mutation_claim;
    mutation_lineage
        .as_object_mut()
        .expect("mutation lineage")
        .insert(
            "replayRunRef".to_owned(),
            artifact("run-artifact", "run:mutation", &mutation_run),
        );
    let mut bytes = BTreeMap::from([
        ("input:real".to_owned(), real.clone()),
        ("input:malformed".to_owned(), malformed.clone()),
        ("input:generated".to_owned(), generated.clone()),
        ("input:mutation".to_owned(), mutation.clone()),
        ("run:generated".to_owned(), generated_run.clone()),
        ("run:mutation".to_owned(), mutation_run.clone()),
    ]);
    bytes.insert("generator".to_owned(), b"generator".to_vec());
    let entry = |id: &str, classes: Value, origin: &str, input: &[u8], lineage: Value| {
        json!({
            "entryId": id,
            "artifactRef": artifact("source", &format!("input:{id}"), input),
            "classes": classes,
            "origin": origin,
            "provenance": {
                "authority": format!("authority:{id}"),
                "revision": "r1",
                "path": format!("corpus/{id}.quire")
            },
            "rights": {
                "owner": format!("owner:{id}"),
                "disposition": "retained-private",
                "basis": "Owner authorized private qualification retention"
            },
            "selectionReason": format!("Covers the {id} population stratum"),
            "lineage": lineage
        })
    };
    let manifest = json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-corpus/v1",
        "kind": "QualificationCorpusManifest",
        "manifestId": "QCM-001",
        "manifestRevision": "1",
        "profileIdentity": frozen_profile().identity(),
        "populationId": "POP-CORPUS-001",
        "requiredClasses": ["valid", "malformed", "boundary", "mutation"],
        "entries": [
            entry("real", json!(["valid"]), "repository", &real, Value::Null),
            entry("malformed", json!(["malformed"]), "conformance", &malformed, Value::Null),
            entry("generated", json!(["boundary"]), "generated", &generated, generated_lineage),
            entry("mutation", json!(["mutation"]), "mutation", &mutation, mutation_lineage)
        ],
        "gaps": [],
        "exclusions": [{
            "exclusionId": "private-outside-population",
            "classId": "boundary",
            "owner": "verification-owner",
            "reason": "A larger private input is outside the selected retained population"
        }]
    });
    (manifest, bytes)
}

fn remove_field(value: &mut Value, pointer: &str) {
    let (parent, field) = pointer.rsplit_once('/').expect("field pointer");
    value
        .pointer_mut(parent)
        .and_then(Value::as_object_mut)
        .expect("object at field pointer")
        .remove(field)
        .expect("field to remove");
}

#[trace("TC-040", "FR-023-AC-1", "FR-023-AC-4")]
#[test]
fn tc_040_qualifies_exact_authorized_bytes_and_replayed_lineage() {
    let profile = frozen_profile();
    let (manifest, bytes) = corpus();
    let proof = proofs(&manifest, &bytes);
    let qualified = match qualify_corpus_manifest(manifest.clone(), &profile, &proof)
        .expect("corpus decision")
    {
        QualificationCorpusOutcome::Complete(corpus) => corpus,
        QualificationCorpusOutcome::Incomplete { gaps, .. } => {
            panic!("complete corpus returned gaps: {gaps:?}")
        }
    };
    assert_eq!(
        serde_json::to_value(qualified.manifest()).expect("manifest serialization"),
        manifest
    );
    assert_eq!(
        qualified.retained_bytes("real"),
        Some(b"kind: valid\n".as_slice())
    );
    assert!(qualified.identity().starts_with("sha256-jcs:"));
}

#[trace("TC-040", "FR-023-AC-2")]
#[test]
fn tc_040_retains_exact_owned_gaps_in_the_population_denominator() {
    let profile = frozen_profile();
    let (mut manifest, mut bytes) = corpus();
    manifest["entries"]
        .as_array_mut()
        .expect("entries")
        .remove(2);
    bytes.remove("input:generated");
    bytes.remove("run:generated");
    bytes.remove("generator");
    manifest["gaps"] = json!([{
        "classId": "boundary",
        "owner": "corpus-owner",
        "reason": "No authorized boundary artifact is retained"
    }]);
    let proof = proofs(&manifest, &bytes);
    let gaps = match qualify_corpus_manifest(manifest, &profile, &proof).expect("corpus decision") {
        QualificationCorpusOutcome::Complete(_) => panic!("gapped corpus qualified"),
        QualificationCorpusOutcome::Incomplete { gaps, .. } => gaps,
    };
    assert_eq!(gaps.len(), 1);
    assert_eq!(gaps[0].class_id(), "boundary");
    assert_eq!(gaps[0].owner(), "corpus-owner");
}

#[trace("TC-040", "FR-023-AC-3")]
#[test]
fn tc_040_refuses_identity_class_rights_and_reference_drift() {
    let profile = frozen_profile();
    for (name, mutate) in [
        (
            "profile",
            (
                "/profileIdentity",
                json!(
                    "sha256-jcs:0000000000000000000000000000000000000000000000000000000000000000"
                ),
            ),
        ),
        ("population", ("/populationId", json!("other"))),
        ("classes", ("/requiredClasses/0", json!("other"))),
        (
            "unknown entry class",
            ("/entries/0/classes/0", json!("other")),
        ),
        (
            "rights",
            ("/entries/0/rights/disposition", json!("reference-only")),
        ),
        (
            "kind",
            ("/entries/0/artifactRef/value/kind", json!("snapshot")),
        ),
    ] {
        let (mut manifest, bytes) = corpus();
        *manifest.pointer_mut(mutate.0).expect("mutation pointer") = mutate.1;
        assert!(
            qualify_corpus_manifest(manifest.clone(), &profile, &proofs(&manifest, &bytes))
                .is_err(),
            "{name} drift was accepted"
        );
    }

    let (mut duplicate, bytes) = corpus();
    duplicate["entries"][1]["entryId"] = json!("real");
    assert!(
        qualify_corpus_manifest(duplicate.clone(), &profile, &proofs(&duplicate, &bytes)).is_err()
    );

    let (mut duplicate_artifact, bytes) = corpus();
    duplicate_artifact["entries"][1]["artifactRef"] =
        duplicate_artifact["entries"][0]["artifactRef"].clone();
    assert!(
        qualify_corpus_manifest(
            duplicate_artifact.clone(),
            &profile,
            &proofs(&duplicate_artifact, &bytes)
        )
        .is_err()
    );

    let (mut conflicting_identity, bytes) = corpus();
    conflicting_identity["entries"][1]["artifactRef"]["value"]["identity"] = json!("input:real");
    let independent_proofs = proofs_from_independent_readers(&conflicting_identity, &bytes);
    assert!(
        qualify_corpus_manifest(conflicting_identity, &profile, &independent_proofs).is_err(),
        "one immutable input identity admitted conflicting content"
    );

    let (manifest, bytes) = corpus();
    let mut proof = proofs(&manifest, &bytes);
    proof.pop();
    assert!(qualify_corpus_manifest(manifest, &profile, &proof).is_err());

    let (mut lineage_drift, bytes) = corpus();
    lineage_drift["entries"][2]["lineage"]["seed"] = json!("different-seed");
    assert!(
        qualify_corpus_manifest(
            lineage_drift.clone(),
            &profile,
            &proofs(&lineage_drift, &bytes)
        )
        .is_err(),
        "a replay run admitted lineage inputs it did not execute"
    );
}

#[trace("TC-040", "FR-023-AC-1", "FR-023-AC-3")]
#[test]
fn tc_040_refuses_missing_dimensions_silent_class_loss_and_derived_relabeling() {
    let profile = frozen_profile();
    for pointer in [
        "/entries/0/classes",
        "/entries/0/provenance/path",
        "/entries/0/rights/disposition",
        "/entries/0/selectionReason",
        "/entries/2/lineage/generatorRef",
        "/entries/2/lineage/seed",
        "/entries/2/lineage/invocation",
        "/entries/2/lineage/replayRunRef",
        "/entries/3/lineage/parentEntryId",
        "/entries/3/lineage/seed",
        "/entries/3/lineage/transformation",
        "/entries/3/lineage/replayRunRef",
        "/exclusions/0/reason",
    ] {
        let (mut manifest, bytes) = corpus();
        remove_field(&mut manifest, pointer);
        assert!(
            qualify_corpus_manifest(manifest.clone(), &profile, &proofs(&manifest, &bytes))
                .is_err(),
            "missing field {pointer} was accepted"
        );
    }

    let (mut missing_class, bytes) = corpus();
    missing_class["entries"]
        .as_array_mut()
        .expect("entries")
        .remove(2);
    assert!(
        qualify_corpus_manifest(
            missing_class.clone(),
            &profile,
            &proofs(&missing_class, &bytes)
        )
        .is_err()
    );

    let (mut independent_derived, bytes) = corpus();
    independent_derived["entries"][2]["origin"] = json!("repository");
    assert!(
        qualify_corpus_manifest(
            independent_derived.clone(),
            &profile,
            &proofs(&independent_derived, &bytes)
        )
        .is_err(),
        "generated input was relabeled as independent"
    );
}

#[trace("TC-040", "FR-023-AC-3", "FR-023-AC-4")]
#[test]
fn tc_040_refuses_broken_parent_graphs_and_replay_output_drift() {
    let profile = frozen_profile();
    for parent in ["mutation", "missing"] {
        let (mut manifest, bytes) = corpus();
        manifest["entries"][3]["lineage"]["parentEntryId"] = json!(parent);
        assert!(
            qualify_corpus_manifest(manifest.clone(), &profile, &proofs(&manifest, &bytes))
                .is_err()
        );
    }

    let (mut cyclic, bytes) = corpus();
    cyclic["entries"][0]["origin"] = json!("mutation");
    cyclic["entries"][0]["lineage"] = json!({
        "kind": "mutation",
        "parentEntryId": "mutation",
        "seed": "seed-cycle",
        "transformation": "cycle",
        "replayRunRef": cyclic["entries"][3]["lineage"]["replayRunRef"].clone()
    });
    assert!(qualify_corpus_manifest(cyclic.clone(), &profile, &proofs(&cyclic, &bytes)).is_err());

    let (mut manifest, mut bytes) = corpus();
    let mut wrong_run: Value =
        serde_json::from_slice(&bytes["run:generated"]).expect("retained generated replay run");
    wrong_run["outputDigest"] = json!(digest(b"other"));
    let wrong_run = (
        "run:generated".to_owned(),
        serde_json::to_vec(&wrong_run).expect("run JSON"),
    );
    bytes.insert(wrong_run.0, wrong_run.1.clone());
    manifest["entries"][2]["lineage"]["replayRunRef"]["value"]["digest"] =
        json!(digest(&wrong_run.1));
    assert!(proofs(&manifest, &bytes).len() > 1);
    assert!(
        qualify_corpus_manifest(manifest.clone(), &profile, &proofs(&manifest, &bytes)).is_err()
    );
}
