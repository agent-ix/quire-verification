// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

use std::collections::{BTreeMap, BTreeSet};

use quire_verification::contracts::jcs_sha256;
use quire_verification::qualification_corpus::{
    FrozenQualificationCorpus, QualificationCorpusOutcome, qualify_corpus_manifest,
};
use quire_verification::qualification_oracle::{
    FrozenQualificationOracleManifest, QualificationOracleOutcome, qualify_oracle_manifest,
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

const WIRE_IDENTITY: &str = "quire-verification.qualification-technique-fixture/v1";

pub struct TechniqueFixture {
    pub profile: Box<FrozenQualificationProfile>,
    pub corpus: Box<FrozenQualificationCorpus>,
    pub oracle: Box<FrozenQualificationOracleManifest>,
    pub catalog: Value,
    pub manifest: Value,
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

pub fn reference(kind: &str, identity: &str) -> Value {
    json!({
        "wireVersion": "ix.shared-reference/2-draft",
        "kind": "artifact",
        "value": {
            "refVersion": "ix.artifact-ref/3-draft",
            "kind": kind,
            "authority": "qualification.agent-ix.local",
            "identity": identity,
            "revision": {"namespace": "qualification-technique", "value": "r1"},
            "digest": digest(identity.as_bytes()),
            "wire": {"identity": WIRE_IDENTITY, "version": "1"}
        }
    })
}

fn interface_kind(role: &str) -> &'static str {
    if role == "parser-compiler" {
        "executable-projection"
    } else if role == "production-caller" {
        "invocation"
    } else {
        "source"
    }
}

fn interface(id: &str, role: &str) -> Value {
    json!({
        "interfaceId": id,
        "role": role,
        "owner": format!("owner:{id}"),
        "status": "accepted",
        "interfaceRef": reference(interface_kind(role), &format!("interface:{id}")),
        "reason": null
    })
}

fn profile_value(claim_authority: &Value, technique_id: &str, stopping_rule: &str) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-profile/v1",
        "kind": "QualificationProfile",
        "profileId": "QP-TECHNIQUE-001",
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
            "populationId": "POP-TECHNIQUE-001",
            "description": "Closed technique input population",
            "requiredClasses": ["valid"],
            "selectionRationale": "Exercise one retained property input"
        },
        "semanticGuarantees": [{
            "guaranteeId": "syntax-outcome",
            "stage": "syntax",
            "claim": "Return the authored syntax outcome",
            "claimAuthority": claim_authority
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
            "obligationId": "syntax-property",
            "guaranteeId": "syntax-outcome",
            "techniqueId": technique_id,
            "budgetId": "parse",
            "stoppingRule": stopping_rule,
            "required": true
        }],
        "exclusions": ["No guarantee beyond the selected syntax property"],
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

pub fn proofs(value: &Value) -> Vec<ValidatedSharedReference> {
    let mut envelopes = Vec::new();
    collect_artifacts(value, &mut envelopes);
    let mut reader = SharedReferenceReader::new(ReferenceSupport {
        wire_versions: BTreeMap::from([(
            WIRE_IDENTITY.to_owned(),
            BTreeSet::from(["1".to_owned()]),
        )]),
        ..ReferenceSupport::default()
    });
    envelopes
        .into_iter()
        .map(|envelope| {
            let value = &envelope["value"];
            let identity = value["identity"].as_str().expect("identity");
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
                        bytes: Some(identity.as_bytes().to_vec()),
                    }),
                )
                .expect("strict reference")
        })
        .collect()
}

fn freeze_profile(
    claim_authority: &Value,
    technique_id: &str,
    stopping_rule: &str,
) -> Box<FrozenQualificationProfile> {
    let value = profile_value(claim_authority, technique_id, stopping_rule);
    match freeze_qualification_profile(value.clone(), &proofs(&value)).expect("profile decision") {
        QualificationProfileOutcome::Frozen(profile) => profile,
        QualificationProfileOutcome::Blocked(blockers) => panic!("profile blocked: {blockers:?}"),
    }
}

fn freeze_corpus(profile: &FrozenQualificationProfile) -> Box<FrozenQualificationCorpus> {
    let value = json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-corpus/v1",
        "kind": "QualificationCorpusManifest",
        "manifestId": "QCM-TECHNIQUE-001",
        "manifestRevision": "1",
        "profileIdentity": profile.identity(),
        "populationId": "POP-TECHNIQUE-001",
        "requiredClasses": ["valid"],
        "entries": [{
            "entryId": "valid",
            "artifactRef": reference("source", "input:valid"),
            "classes": ["valid"],
            "origin": "repository",
            "provenance": {"authority": "repository", "revision": "r1", "path": "corpus/valid.quire"},
            "rights": {"owner": "owner:corpus", "disposition": "retained-private", "basis": "Authorized for qualification"},
            "selectionReason": "Covers the valid syntax stratum",
            "lineage": null
        }],
        "gaps": [],
        "exclusions": []
    });
    match qualify_corpus_manifest(value.clone(), profile, &proofs(&value)).expect("corpus decision")
    {
        QualificationCorpusOutcome::Complete(corpus) => corpus,
        QualificationCorpusOutcome::Incomplete { gaps, .. } => panic!("corpus gaps: {gaps:?}"),
    }
}

fn freeze_oracle(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
) -> Box<FrozenQualificationOracleManifest> {
    let value = json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-oracles/v1",
        "kind": "QualificationOracleManifest",
        "manifestId": "QOM-TECHNIQUE-001",
        "profileIdentity": profile.identity(),
        "corpusIdentity": corpus.identity(),
        "bindings": [],
        "gaps": [{
            "gapId": "oracle-gap",
            "guaranteeId": "syntax-outcome",
            "owner": "owner:oracle",
            "reason": "This property requires no independent oracle",
            "limitations": ["Only the declared property is assessed"]
        }]
    });
    match qualify_oracle_manifest(value, profile, corpus, &[]).expect("oracle decision") {
        QualificationOracleOutcome::Complete(oracle) => oracle,
        QualificationOracleOutcome::Incomplete { manifest, .. } => manifest,
    }
}

fn accepted_catalog() -> Value {
    let mut catalog: Value =
        serde_json::from_str(include_str!("../../fixtures/quoin-catalog-export.json"))
            .expect("catalog fixture");
    catalog["source"]["sourceRef"] = reference("source", "catalog:accepted-source");
    catalog
}

fn technique_manifest(
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    oracle: &FrozenQualificationOracleManifest,
    catalog: &Value,
    technique_id: &str,
    stopping_rule: &str,
    technique: &Value,
) -> Value {
    json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-techniques/v1",
        "kind": "QualificationTechniqueManifest",
        "manifestId": "QTM-001",
        "profileIdentity": profile.identity(),
        "corpusIdentity": corpus.identity(),
        "oracleManifestIdentity": oracle.identity(),
        "catalogIdentity": jcs_sha256(catalog).expect("catalog identity"),
        "configurations": [{
            "configurationId": "property-syntax",
            "obligationId": "syntax-property",
            "guaranteeId": "syntax-outcome",
            "techniqueId": technique_id,
            "propertyRef": reference("property", "property:syntax"),
            "inputProjection": {"mode": "each-entry", "entryIds": ["valid"]},
            "budgetId": "parse",
            "stoppingRule": stopping_rule,
            "harnessRef": reference("executable-projection", "harness:syntax"),
            "requestTemplateRef": reference("invocation", "request:syntax"),
            "limitations": ["The property covers the retained syntax population"],
            "technique": technique
        }],
        "gaps": []
    })
}

pub fn fixture() -> TechniqueFixture {
    fixture_with(
        "property-based-testing",
        "execute exactly 32 generated cases or stop at the first counterexample",
        &json!({
            "family": "property",
            "generatorRef": reference("executable-projection", "generator:syntax"),
            "shrinkerRef": reference("executable-projection", "shrinker:syntax"),
            "seed": "42",
            "maximumCases": 32,
            "maximumShrinks": 16
        }),
    )
}

pub fn fixture_with(
    technique_id: &str,
    stopping_rule: &str,
    technique: &Value,
) -> TechniqueFixture {
    let profile = freeze_profile(
        &json!({
            "kind": "non-formal",
            "reason": "This fixture exercises syntax outcomes rather than formal clauses"
        }),
        technique_id,
        stopping_rule,
    );
    let corpus = freeze_corpus(&profile);
    let oracle = freeze_oracle(&profile, &corpus);
    let catalog = accepted_catalog();
    let manifest = technique_manifest(
        &profile,
        &corpus,
        &oracle,
        &catalog,
        technique_id,
        stopping_rule,
        technique,
    );
    TechniqueFixture {
        profile,
        corpus,
        oracle,
        catalog,
        manifest,
    }
}

pub fn native_gapped_fixture() -> TechniqueFixture {
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
    let results = rules.map(|rule| {
        if rule == "numeric-domain" {
            json!({
                "rule": rule,
                "status": "gap",
                "owner": "quire-spec-language",
                "reason": "Native numeric admission has not been accepted",
                "limitations": ["No accepted numeric-domain observation is retained"]
            })
        } else {
            json!({
                "rule": rule,
                "status": "accepted",
                "evidenceRef": reference("observation", &format!("admission:{rule}"))
            })
        }
    });
    let stopping_rule = "execute exactly 32 generated cases or stop at the first counterexample";
    let profile = freeze_profile(
        &json!({
            "kind": "native-formal-clause",
            "sourceProfileRef": reference("source", "native-quire-profile"),
            "admissionResults": results
        }),
        "property-based-testing",
        stopping_rule,
    );
    let corpus = freeze_corpus(&profile);
    let oracle = freeze_oracle(&profile, &corpus);
    let catalog = accepted_catalog();
    let mut manifest = technique_manifest(
        &profile,
        &corpus,
        &oracle,
        &catalog,
        "property-based-testing",
        stopping_rule,
        &json!({
            "family": "property",
            "generatorRef": reference("executable-projection", "generator:syntax"),
            "shrinkerRef": reference("executable-projection", "shrinker:syntax"),
            "seed": "42",
            "maximumCases": 32,
            "maximumShrinks": 16
        }),
    );
    manifest["configurations"] = json!([]);
    manifest["gaps"] = json!([{
        "gapId": "native-admission-gap",
        "obligationId": "syntax-property",
        "owner": "quire-spec-language",
        "reason": "The formal property is blocked by native-profile admission",
        "limitations": ["Numeric semantics remain unadmitted"],
        "admissionRules": ["numeric-domain"]
    }]);
    TechniqueFixture {
        profile,
        corpus,
        oracle,
        catalog,
        manifest,
    }
}
