// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Independent acceptance tests for the retained shared-reference packet.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use ix_trace_rs::trace;
use jsonschema::Draft;
use quire_verification::contracts::{
    MAX_JSON_BYTES, VerificationError, VerificationErrorCode, jcs_canonicalize,
};
use quire_verification::shared_reference::{
    ArtifactExpectation, LanguageExpectation, MigrationExpectation, ReferenceExpectation,
    ReferenceSupport, SemanticExpectation, SemanticProfileExpectation, SharedReferenceReader,
    SourceMapExpectation, parse_strict_bounded_json, validate_jcs_correspondence,
    verify_shared_reference_snapshot,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const FIXTURE_DIRECTORY: &str = "fixtures/shared-reference-2-draft";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture_bytes(name: &str) -> Vec<u8> {
    fs::read(root().join(FIXTURE_DIRECTORY).join(name)).expect("retained fixture")
}

fn fixture(name: &str) -> Value {
    serde_json::from_slice(&fixture_bytes(name)).expect("fixture JSON")
}

fn byte_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn support() -> ReferenceSupport {
    let mut wire_versions = BTreeMap::new();
    for identity in [
        "ix.fs05.synthetic-role",
        "ix.fs05.synthetic-disposition",
        "ix.fs05.synthetic-core",
        "text/plain",
        "ix.test.binding",
    ] {
        wire_versions.insert(
            identity.to_owned(),
            ["1".to_owned(), "2".to_owned()].into_iter().collect(),
        );
    }
    ReferenceSupport {
        wire_versions,
        language_editions: BTreeMap::from([(
            "quire".to_owned(),
            ["1".to_owned(), "2".to_owned()].into_iter().collect(),
        )]),
        semantic_profile_versions: BTreeMap::from([(
            "ix.fs05.correspondence-fixture".to_owned(),
            ["1".to_owned()].into_iter().collect(),
        )]),
        features: ["a".to_owned(), "b".to_owned()].into_iter().collect(),
    }
}

fn artifact_expectation(value: &Value, bytes: Option<Vec<u8>>) -> ArtifactExpectation {
    ArtifactExpectation {
        ref_version: value["refVersion"].as_str().expect("refVersion").to_owned(),
        kind: value["kind"].as_str().expect("kind").to_owned(),
        authority: value["authority"].as_str().expect("authority").to_owned(),
        identity: value["identity"].as_str().expect("identity").to_owned(),
        revision_namespace: value["revision"]["namespace"]
            .as_str()
            .expect("revision namespace")
            .to_owned(),
        revision_value: value["revision"]["value"]
            .as_str()
            .expect("revision value")
            .to_owned(),
        digest: value["digest"].as_str().expect("digest").to_owned(),
        wire_identity: value["wire"]["identity"]
            .as_str()
            .expect("wire identity")
            .to_owned(),
        wire_version: value["wire"]["version"]
            .as_str()
            .expect("wire version")
            .to_owned(),
        bytes,
    }
}

fn semantic_expectation(value: &Value) -> SemanticExpectation {
    SemanticExpectation {
        artifact: artifact_expectation(&value["artifact"], Some(fixture_bytes("core.json"))),
        language: None,
        semantic_profile: Some(SemanticProfileExpectation {
            identity: "ix.fs05.correspondence-fixture".to_owned(),
            version: "1".to_owned(),
            definition_bytes: Some(fixture_bytes("profile.json")),
        }),
        required_features: BTreeSet::new(),
        canonical_value: Some(fixture("core.json")),
    }
}

fn assert_code<T: std::fmt::Debug>(
    result: Result<T, VerificationError>,
    expected: VerificationErrorCode,
) {
    assert_eq!(result.expect_err("case must refuse").code(), expected);
}

fn replace(value: &mut Value, pointer: &str, replacement: Value) {
    *value.pointer_mut(pointer).expect("mutation pointer") = replacement;
}

fn artifact_ref(
    ref_version: &str,
    kind: &str,
    identity: &str,
    wire_identity: &str,
    bytes: &[u8],
) -> Value {
    json!({
        "refVersion": ref_version,
        "kind": kind,
        "authority": "ix.test",
        "identity": identity,
        "revision": {"namespace": "fixture", "value": "1"},
        "digest": byte_digest(bytes),
        "wire": {"identity": wire_identity, "version": "1"}
    })
}

#[trace("TC-029", "FR-016-AC-1")]
#[trace("FR-016-AC-2")]
#[trace("FR-016-AC-3")]
#[trace("FR-016-AC-4")]
#[trace("FR-016-AC-5")]
#[trace("FR-016-AC-6")]
#[test]
fn tc_029_accepts_exact_v2_roles_and_refuses_each_selected_failure() {
    let mut at_limit = vec![b' '; MAX_JSON_BYTES - 4];
    at_limit.extend_from_slice(b"null");
    assert_eq!(
        parse_strict_bounded_json(&at_limit).expect("exact byte limit must be accepted"),
        Value::Null
    );
    accepts_exact_roles_and_semantic_reference();
    rejects_duplicate_keys_and_inexact_roles();
    rejects_artifact_identity_failures();
    rejects_unsupported_reference_versions();
    rejects_semantic_wire_and_canonical_failures();
    validates_language_and_profile_support();
    accepts_feature_set_reordering_and_coupled_nulls();
}

fn accepts_exact_roles_and_semantic_reference() {
    let roles = fixture("roles.json");
    let mut reader = SharedReferenceReader::new(support());
    for role in roles.as_array().expect("role cases") {
        let payload = fixture_bytes(role["file"].as_str().expect("role file"));
        let envelope = &role["envelope"];
        let expected = artifact_expectation(&envelope["value"], Some(payload));
        assert_eq!(expected.kind, role["expectedRole"]);
        reader
            .read(
                &serde_json::to_vec(envelope).expect("serialize envelope"),
                &ReferenceExpectation::Artifact(expected),
            )
            .expect("exact role reference");
    }

    let semantic_envelope = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic_envelope["value"]);
    SharedReferenceReader::new(support())
        .read(
            &fixture_bytes("semantic.json"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        )
        .expect("exact semantic reference");
}

fn rejects_duplicate_keys_and_inexact_roles() {
    assert_code(
        parse_strict_bounded_json(&fixture_bytes("raw-duplicate.json")),
        VerificationErrorCode::InvalidWire,
    );
    assert_code(
        parse_strict_bounded_json(br#"{"nested":{"a":1,"\u0061":2}}"#),
        VerificationErrorCode::InvalidWire,
    );
    assert_code(
        parse_strict_bounded_json(br#"{"exact":18446744073709551616}"#),
        VerificationErrorCode::InvalidWire,
    );

    let roles = fixture("roles.json");
    let substitutions = fixture("adverse.json")["roleSubstitutions"]["replaceKindWith"]
        .as_array()
        .expect("role substitutions")
        .clone();
    for role in roles.as_array().expect("role cases") {
        let envelope = &role["envelope"];
        let expected = artifact_expectation(
            &envelope["value"],
            Some(fixture_bytes(role["file"].as_str().expect("role file"))),
        );
        for replacement_kind in &substitutions {
            if replacement_kind == &role["expectedRole"] {
                continue;
            }
            let mut changed = envelope.clone();
            changed["value"]["kind"] = replacement_kind.clone();
            assert_code(
                SharedReferenceReader::new(support()).read(
                    &serde_json::to_vec(&changed).expect("serialize role mutation"),
                    &ReferenceExpectation::Artifact(expected.clone()),
                ),
                VerificationErrorCode::IdentityMismatch,
            );
        }
    }
}

fn rejects_artifact_identity_failures() {
    let roles = fixture("roles.json");
    let first_role = &roles[0]["envelope"];
    let first_payload = fixture_bytes(roles[0]["file"].as_str().expect("role file"));
    let first_expected = artifact_expectation(&first_role["value"], Some(first_payload));

    for (pointer, replacement) in [
        ("/value/authority", json!("ix.other")),
        ("/value/identity", json!("other")),
        ("/value/revision/namespace", json!("other")),
        ("/value/revision/value", json!("2")),
        (
            "/value/digest",
            json!("sha256:0000000000000000000000000000000000000000000000000000000000000000"),
        ),
        ("/value/wire/identity", json!("ix.fs05.synthetic-core")),
        ("/value/wire/version", json!("2")),
    ] {
        let mut changed = first_role.clone();
        replace(&mut changed, pointer, replacement);
        assert_code(
            SharedReferenceReader::new(support()).read(
                &serde_json::to_vec(&changed).expect("serialize identity mutation"),
                &ReferenceExpectation::Artifact(first_expected.clone()),
            ),
            VerificationErrorCode::IdentityMismatch,
        );
    }

    let mut stale = first_expected.clone();
    stale.bytes = None;
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(first_role).expect("serialize stale case"),
            &ReferenceExpectation::Artifact(stale),
        ),
        VerificationErrorCode::Stale,
    );

    let mut conflict_reader = SharedReferenceReader::new(support());
    conflict_reader
        .read(
            &serde_json::to_vec(first_role).expect("serialize first identity"),
            &ReferenceExpectation::Artifact(first_expected.clone()),
        )
        .expect("first immutable identity");
    let mut conflicting = first_role.clone();
    conflicting["value"]["digest"] =
        json!("sha256:0000000000000000000000000000000000000000000000000000000000000000");
    assert_code(
        conflict_reader.read(
            &serde_json::to_vec(&conflicting).expect("serialize conflict"),
            &ReferenceExpectation::Artifact(first_expected.clone()),
        ),
        VerificationErrorCode::IdentityContentConflict,
    );

    let mut rollback_reader = SharedReferenceReader::new(support());
    assert_code(
        rollback_reader.read(
            &serde_json::to_vec(&conflicting).expect("serialize refused identity"),
            &ReferenceExpectation::Artifact(first_expected.clone()),
        ),
        VerificationErrorCode::IdentityMismatch,
    );
    rollback_reader
        .read(
            &serde_json::to_vec(first_role).expect("serialize accepted identity"),
            &ReferenceExpectation::Artifact(first_expected),
        )
        .expect("refused input must not poison reader state");
}

fn rejects_unsupported_reference_versions() {
    let semantic_envelope = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic_envelope["value"]);
    let mut unsupported_envelope = semantic_envelope.clone();
    unsupported_envelope["wireVersion"] = json!("ix.shared-reference/unknown");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unsupported_envelope).expect("serialize unknown envelope"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
    let mut unsupported_wire = semantic_envelope.clone();
    unsupported_wire["value"]["artifact"]["wire"]["version"] = json!("3");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unsupported_wire).expect("serialize unknown wire"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
    let mut unsupported_profile = semantic_envelope.clone();
    unsupported_profile["value"]["semanticProfile"]["version"] = json!("2");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unsupported_profile).expect("serialize unknown profile"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
    let mut unsupported_ref = semantic_envelope.clone();
    unsupported_ref["value"]["artifact"]["refVersion"] = json!("ix.artifact-ref/2-draft");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unsupported_ref).expect("serialize old ref"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
    let mut wrong_version_kind = semantic_envelope.clone();
    wrong_version_kind["kind"] = json!("source-map");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&wrong_version_kind).expect("serialize wrong allocation"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
}

fn rejects_semantic_wire_and_canonical_failures() {
    let semantic_envelope = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic_envelope["value"]);
    let mut unknown_role = semantic_envelope.clone();
    unknown_role["value"]["artifact"]["kind"] = json!("unknown-role");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unknown_role).expect("serialize unknown role"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::InvalidWire,
    );
    let mut byte_prefix_in_semantic = semantic_envelope.clone();
    byte_prefix_in_semantic["value"]["canonicalIdentity"]["digest"] =
        json!(format!("sha256:{}", "0".repeat(64)));
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&byte_prefix_in_semantic).expect("serialize wrong digest kind"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::InvalidWire,
    );
    let mut semantic_prefix_in_bytes = semantic_envelope.clone();
    semantic_prefix_in_bytes["value"]["artifact"]["digest"] =
        json!(format!("sha256-jcs:{}", "0".repeat(64)));
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&semantic_prefix_in_bytes).expect("serialize wrong byte digest"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::InvalidWire,
    );

    for (pointer, replacement) in [
        ("/value/canonicalIdentity/domain", json!("unselected.jcs")),
        ("/value/canonicalIdentity/version", json!("rfc8785-v2")),
        ("/value/canonicalIdentity/algorithm", json!("sha512")),
    ] {
        let mut changed = semantic_envelope.clone();
        replace(&mut changed, pointer, replacement);
        assert_code(
            SharedReferenceReader::new(support()).read(
                &serde_json::to_vec(&changed).expect("serialize canonical mutation"),
                &ReferenceExpectation::Semantic(semantic_expected.clone()),
            ),
            VerificationErrorCode::UnsupportedCanonicalization,
        );
    }
    let mut valid_other_domain = semantic_envelope.clone();
    valid_other_domain["value"]["canonicalIdentity"] = json!({
        "domain": "quire.contract.canonical-json",
        "version": "v1",
        "algorithm": "sha256",
        "digest": "0000000000000000000000000000000000000000000000000000000000000000"
    });
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&valid_other_domain).expect("serialize other domain"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnsupportedCanonicalization,
    );

    let mut wrong_canonical_digest = semantic_envelope.clone();
    wrong_canonical_digest["value"]["canonicalIdentity"]["digest"] =
        json!(format!("sha256-jcs:{}", "0".repeat(64)));
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&wrong_canonical_digest).expect("serialize digest mismatch"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::IdentityMismatch,
    );
    let mut unknown_feature = semantic_envelope.clone();
    unknown_feature["value"]["requiredFeatures"] = json!(["unknown-required-feature"]);
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unknown_feature).expect("serialize unknown feature"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::UnknownRequiredFeature,
    );
    let mut duplicate_feature = semantic_envelope.clone();
    duplicate_feature["value"]["requiredFeatures"] = json!(["a", "a"]);
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&duplicate_feature).expect("serialize duplicate feature"),
            &ReferenceExpectation::Semantic(semantic_expected.clone()),
        ),
        VerificationErrorCode::InvalidWire,
    );
}

fn accepts_feature_set_reordering_and_coupled_nulls() {
    let semantic_envelope = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic_envelope["value"]);
    let mut reordered = semantic_envelope.clone();
    reordered["value"]["requiredFeatures"] = json!(["b", "a"]);
    let mut reordered_expected = semantic_expected.clone();
    reordered_expected.required_features = ["a".to_owned(), "b".to_owned()].into_iter().collect();
    SharedReferenceReader::new(support())
        .read(
            &serde_json::to_vec(&reordered).expect("serialize reordered features"),
            &ReferenceExpectation::Semantic(reordered_expected),
        )
        .expect("feature reordering is set-equivalent");

    let mut null_profile = semantic_envelope;
    null_profile["value"]["language"] = Value::Null;
    null_profile["value"]["semanticProfile"] = Value::Null;
    null_profile["value"]["requiredFeatures"] = json!([]);
    null_profile["value"]["canonicalIdentity"] = Value::Null;
    let null_expected = SemanticExpectation {
        artifact: artifact_expectation(
            &null_profile["value"]["artifact"],
            Some(fixture_bytes("core.json")),
        ),
        language: None,
        semantic_profile: None,
        required_features: BTreeSet::new(),
        canonical_value: None,
    };
    SharedReferenceReader::new(support())
        .read(
            &serde_json::to_vec(&null_profile).expect("serialize null profile"),
            &ReferenceExpectation::Semantic(null_expected),
        )
        .expect("coupled null profile");
}

fn validates_language_and_profile_support() {
    let mut semantic = fixture("semantic.json");
    semantic["value"]["language"] = json!({"identity": "quire", "edition": "1"});
    let mut expected = semantic_expectation(&semantic["value"]);
    expected.language = Some(LanguageExpectation {
        identity: "quire".to_owned(),
        edition: "1".to_owned(),
    });
    SharedReferenceReader::new(support())
        .read(
            &serde_json::to_vec(&semantic).expect("serialize language selection"),
            &ReferenceExpectation::Semantic(expected.clone()),
        )
        .expect("supported selected language");

    let mut unknown = semantic.clone();
    unknown["value"]["language"]["edition"] = json!("3");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unknown).expect("serialize unknown language"),
            &ReferenceExpectation::Semantic(expected.clone()),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
    let mut nearby = semantic.clone();
    nearby["value"]["language"]["edition"] = json!("2");
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&nearby).expect("serialize nearby language"),
            &ReferenceExpectation::Semantic(expected.clone()),
        ),
        VerificationErrorCode::IdentityMismatch,
    );

    expected
        .semantic_profile
        .as_mut()
        .expect("selected profile")
        .definition_bytes = Some(b"changed profile definition".to_vec());
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&semantic).expect("serialize profile mismatch"),
            &ReferenceExpectation::Semantic(expected),
        ),
        VerificationErrorCode::IdentityMismatch,
    );
}

#[trace("TC-029", "FR-016-AC-3", "FR-016-AC-6")]
#[test]
fn tc_029_validates_source_maps_and_requires_one_explicit_migration() {
    validates_source_maps();
    rejects_unaligned_source_map_spans();
    requires_one_explicit_migration();
}

fn validates_source_maps() {
    let original = b"  alpha\r\n  beta\r\n".to_vec();
    let body = b"alpha\nbeta".to_vec();
    let original_ref = artifact_ref(
        "ix.artifact-ref/2-draft",
        "source",
        "original",
        "text/plain",
        &original,
    );
    let body_ref = artifact_ref(
        "ix.artifact-ref/2-draft",
        "source",
        "body",
        "text/plain",
        &body,
    );
    let source_map = json!({
        "wireVersion": "ix.shared-reference/1-draft",
        "kind": "source-map",
        "value": {
            "mapVersion": "ix.source-map/1-draft",
            "original": original_ref,
            "body": body_ref,
            "region": {"startByte": 0, "endByte": original.len()},
            "layout": {
                "stripIndentation": true,
                "normalizeCrLf": true,
                "dropFinalNewline": true
            },
            "segments": [
                {"original":{"startByte":2,"endByte":7},"body":{"startByte":0,"endByte":5}},
                {"original":{"startByte":8,"endByte":9},"body":{"startByte":5,"endByte":6}},
                {"original":{"startByte":11,"endByte":15},"body":{"startByte":6,"endByte":10}}
            ]
        }
    });
    let map_expected = SourceMapExpectation {
        original: artifact_expectation(&source_map["value"]["original"], Some(original.clone())),
        body: artifact_expectation(&source_map["value"]["body"], Some(body.clone())),
    };
    SharedReferenceReader::new(support())
        .read(
            &serde_json::to_vec(&source_map).expect("serialize source map"),
            &ReferenceExpectation::SourceMap(map_expected.clone()),
        )
        .expect("exact source map");

    for (pointer, replacement) in [
        ("/value/segments/0/body/startByte", 1),
        ("/value/segments/1/original/startByte", 6),
        ("/value/region/endByte", original.len() + 1),
    ] {
        let mut malformed = source_map.clone();
        replace(&mut malformed, pointer, json!(replacement));
        assert_code(
            SharedReferenceReader::new(support()).read(
                &serde_json::to_vec(&malformed).expect("serialize malformed map"),
                &ReferenceExpectation::SourceMap(map_expected.clone()),
            ),
            VerificationErrorCode::MalformedSourceMap,
        );
    }

    let mut historical_artifact = source_map.clone();
    historical_artifact["kind"] = json!("artifact");
    historical_artifact["value"] = historical_artifact["value"]["original"].clone();
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&historical_artifact).expect("serialize historical artifact"),
            &ReferenceExpectation::SourceMap(map_expected),
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
}

fn requires_one_explicit_migration() {
    let bytes = b"same artifact".to_vec();
    let mapping_bytes = b"explicit migration binding".to_vec();
    let old = artifact_ref(
        "ix.artifact-ref/1-draft",
        "source",
        "migrated",
        "text/plain",
        &bytes,
    );
    let new = artifact_ref(
        "ix.artifact-ref/2-draft",
        "source",
        "migrated",
        "text/plain",
        &bytes,
    );
    let mapping = artifact_ref(
        "ix.artifact-ref/2-draft",
        "binding",
        "migration-binding",
        "ix.test.binding",
        &mapping_bytes,
    );
    let migration = json!({
        "wireVersion": "ix.shared-reference/1-draft",
        "kind": "migration",
        "value": {
            "migrationVersion": "ix.byte-reference-migration/1-draft",
            "old": old,
            "new": new,
            "mapping": mapping,
            "preservedAspects": ["byte-artifact"],
            "reason": "exact encoding-version transition"
        }
    });
    let migration_expected = MigrationExpectation {
        old: artifact_expectation(&migration["value"]["old"], Some(bytes.clone())),
        new: artifact_expectation(&migration["value"]["new"], Some(bytes)),
        mapping: artifact_expectation(&migration["value"]["mapping"], Some(mapping_bytes)),
    };
    let encoded = serde_json::to_vec(&migration).expect("serialize migration");
    assert_code(
        SharedReferenceReader::new(support()).resolve_migration(&[], &migration_expected),
        VerificationErrorCode::IdentityMismatch,
    );
    SharedReferenceReader::new(support())
        .resolve_migration(&[&encoded], &migration_expected)
        .expect("one explicit migration");
    assert_code(
        SharedReferenceReader::new(support())
            .resolve_migration(&[&encoded, &encoded], &migration_expected),
        VerificationErrorCode::AmbiguousEquivalence,
    );

    let mut v2_migration = migration;
    v2_migration["wireVersion"] = json!("ix.shared-reference/2-draft");
    assert_code(
        SharedReferenceReader::new(support()).resolve_migration(
            &[&serde_json::to_vec(&v2_migration).expect("serialize wrong migration version")],
            &migration_expected,
        ),
        VerificationErrorCode::UnsupportedReferenceVersion,
    );
}

fn rejects_unaligned_source_map_spans() {
    let text = "é".as_bytes().to_vec();
    let source_map = json!({
        "wireVersion": "ix.shared-reference/1-draft",
        "kind": "source-map",
        "value": {
            "mapVersion": "ix.source-map/1-draft",
            "original": artifact_ref(
                "ix.artifact-ref/2-draft", "source", "utf8-original", "text/plain", &text
            ),
            "body": artifact_ref(
                "ix.artifact-ref/2-draft", "source", "utf8-body", "text/plain", &text
            ),
            "region": {"startByte": 0, "endByte": text.len()},
            "layout": {
                "stripIndentation": false,
                "normalizeCrLf": false,
                "dropFinalNewline": false
            },
            "segments": [{
                "original":{"startByte":0,"endByte":text.len()},
                "body":{"startByte":0,"endByte":text.len()}
            }]
        }
    });
    let expected = SourceMapExpectation {
        original: artifact_expectation(&source_map["value"]["original"], Some(text.clone())),
        body: artifact_expectation(&source_map["value"]["body"], Some(text)),
    };
    let mut unaligned = source_map;
    unaligned["value"]["segments"][0]["original"]["startByte"] = json!(1);
    assert_code(
        SharedReferenceReader::new(support()).read(
            &serde_json::to_vec(&unaligned).expect("serialize unaligned span"),
            &ReferenceExpectation::SourceMap(expected),
        ),
        VerificationErrorCode::MalformedSourceMap,
    );
}

#[trace("TC-031", "FR-016-AC-5", "FR-016-AC-7")]
#[test]
fn tc_031_executes_the_byte_identical_data_only_snapshot_independently() {
    validates_snapshot_schema_and_canonical_vectors();
    executes_all_upstream_adverse_vectors();
    detects_snapshot_drift_and_producer_coupling();
}

fn validates_snapshot_schema_and_canonical_vectors() {
    verify_shared_reference_snapshot(&root()).expect("snapshot manifest");

    let schema_bytes =
        fs::read(root().join("contracts/shared-reference-2-draft/schema.json")).expect("v2 schema");
    assert_eq!(
        format!("{:x}", Sha256::digest(&schema_bytes)),
        "c277730c339c48c8cc789f2adb93dbbe1d48624a6df39a4043d20cd0cbeca46b"
    );
    let core_schema: Value = fixture("core.schema.json");
    let validator = jsonschema::options()
        .with_draft(Draft::Draft202012)
        .build(&core_schema)
        .expect("core schema compiles locally");
    assert!(validator.is_valid(&fixture("core.json")));

    for vector in fixture("canonical-vectors.json")
        .as_array()
        .expect("canonical vectors")
    {
        let parsed = parse_strict_bounded_json(
            vector["input"]
                .as_str()
                .expect("canonical input")
                .as_bytes(),
        )
        .expect("strict canonical input");
        assert_eq!(
            jcs_canonicalize(&parsed).expect("JCS"),
            vector["canonical"]
                .as_str()
                .expect("canonical result")
                .as_bytes()
        );
    }
}

fn executes_all_upstream_adverse_vectors() {
    let adverse = fixture("adverse.json");
    assert_eq!(
        adverse["structuralMutations"]
            .as_array()
            .expect("structural mutations")
            .len(),
        8
    );
    assert_eq!(
        adverse["semanticMutations"]
            .as_array()
            .expect("semantic mutations")
            .len(),
        4
    );
    assert_eq!(
        adverse["exactReferenceMutations"]["roles"]
            .as_array()
            .expect("exact roles")
            .len(),
        5
    );
    executes_structural_vectors(&adverse);
    executes_semantic_vectors(&adverse);
    executes_exact_reference_vectors(&adverse);
}

fn executes_structural_vectors(adverse: &Value) {
    let semantic = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic["value"]);
    for mutation in adverse["structuralMutations"]
        .as_array()
        .expect("structural mutations")
    {
        let mut changed = semantic.clone();
        replace(
            &mut changed,
            mutation["pointer"].as_str().expect("mutation pointer"),
            mutation["replacement"].clone(),
        );
        let expected = match mutation["expected"].as_str().expect("expected code") {
            "unsupported-version" => VerificationErrorCode::UnsupportedReferenceVersion,
            "unsupported-canonicalization" => VerificationErrorCode::UnsupportedCanonicalization,
            "invalid-wire" => VerificationErrorCode::InvalidWire,
            other => panic!("unknown structural expectation {other}"),
        };
        assert_code(
            SharedReferenceReader::new(support()).read(
                &serde_json::to_vec(&changed).expect("serialize structural vector"),
                &ReferenceExpectation::Semantic(semantic_expected.clone()),
            ),
            expected,
        );
    }
}

fn executes_semantic_vectors(adverse: &Value) {
    let semantic = fixture("semantic.json");
    let semantic_expected = semantic_expectation(&semantic["value"]);
    let core = fixture("core.json");
    validate_jcs_correspondence(
        &fixture_bytes("review-disposition.json"),
        "/coreDigest",
        &core,
    )
    .expect("exact disposition core correspondence");
    for mutation in adverse["semanticMutations"]
        .as_array()
        .expect("semantic mutations")
    {
        let expected = match mutation["expected"].as_str().expect("expected code") {
            "identity-mismatch" => VerificationErrorCode::IdentityMismatch,
            "unknown-required-feature" => VerificationErrorCode::UnknownRequiredFeature,
            "invalid-wire" => VerificationErrorCode::InvalidWire,
            other => panic!("unknown semantic expectation {other}"),
        };
        if mutation["base"] == "semantic.json" {
            let mut changed = semantic.clone();
            replace(
                &mut changed,
                mutation["pointer"].as_str().expect("mutation pointer"),
                mutation["replacement"].clone(),
            );
            assert_code(
                SharedReferenceReader::new(support()).read(
                    &serde_json::to_vec(&changed).expect("serialize semantic vector"),
                    &ReferenceExpectation::Semantic(semantic_expected.clone()),
                ),
                expected,
            );
        } else {
            let mut disposition = fixture("review-disposition.json");
            replace(
                &mut disposition,
                mutation["pointer"].as_str().expect("mutation pointer"),
                mutation["replacement"].clone(),
            );
            assert_code(
                validate_jcs_correspondence(
                    &serde_json::to_vec(&disposition).expect("serialize disposition vector"),
                    "/coreDigest",
                    &core,
                ),
                expected,
            );
        }
    }
}

fn executes_exact_reference_vectors(adverse: &Value) {
    for role in adverse["exactReferenceMutations"]["roles"]
        .as_array()
        .expect("exact-reference roles")
    {
        let role = role.as_str().expect("role name");
        let payload = format!("{role} payload").into_bytes();
        let envelope = json!({
            "wireVersion": "ix.shared-reference/2-draft",
            "kind": "artifact",
            "value": artifact_ref(
                "ix.artifact-ref/3-draft",
                role,
                &format!("{role}-1"),
                "text/plain",
                &payload,
            )
        });
        let expected = artifact_expectation(&envelope["value"], Some(payload));
        SharedReferenceReader::new(support())
            .read(
                &serde_json::to_vec(&envelope).expect("serialize exact role"),
                &ReferenceExpectation::Artifact(expected.clone()),
            )
            .expect("exact role");
        for pointer in adverse["exactReferenceMutations"]["pointers"]
            .as_array()
            .expect("exact-reference pointers")
        {
            let pointer = format!("/value{}", pointer.as_str().expect("identity pointer"));
            let replacement = match pointer.as_str() {
                "/value/authority" => json!("ix.other"),
                "/value/identity" | "/value/revision/namespace" => json!("other"),
                "/value/revision/value" | "/value/wire/version" => json!("2"),
                "/value/digest" => json!(format!("sha256:{}", "0".repeat(64))),
                "/value/wire/identity" => json!("ix.fs05.synthetic-core"),
                _ => unreachable!("fixture pointer is selected"),
            };
            let mut changed = envelope.clone();
            replace(&mut changed, &pointer, replacement);
            assert_code(
                SharedReferenceReader::new(support()).read(
                    &serde_json::to_vec(&changed).expect("serialize exact mutation"),
                    &ReferenceExpectation::Artifact(expected.clone()),
                ),
                VerificationErrorCode::IdentityMismatch,
            );
        }
    }
}

fn detects_snapshot_drift_and_producer_coupling() {
    let manifest = include_str!("../contracts/shared-reference-snapshot.sha256");
    assert_eq!(manifest.lines().count(), 16);
    let temporary = tempfile::tempdir().expect("temporary snapshot");
    for line in manifest.lines() {
        let (_, relative) = line.split_once("  ").expect("manifest line");
        let source = root().join(relative);
        let target = temporary.path().join(relative);
        fs::create_dir_all(target.parent().expect("snapshot parent"))
            .expect("create snapshot directory");
        fs::copy(source, target).expect("copy retained file");
    }
    let extra = temporary
        .path()
        .join("fixtures/shared-reference-2-draft/unlisted.json");
    fs::write(&extra, b"{}").expect("write unlisted snapshot file");
    assert_code(
        verify_shared_reference_snapshot(temporary.path()),
        VerificationErrorCode::InvalidWire,
    );
    fs::remove_file(extra).expect("remove unlisted snapshot file");
    fs::write(
        temporary
            .path()
            .join("fixtures/shared-reference-2-draft/trace.json"),
        b"{}",
    )
    .expect("mutate snapshot");
    assert_code(
        verify_shared_reference_snapshot(temporary.path()),
        VerificationErrorCode::IdentityMismatch,
    );

    let build = include_str!("../build.rs");
    assert!(build.contains("shared-reference-1-draft.schema.json"));
    assert!(build.contains("shared-reference-2-draft/schema.json"));
    assert_eq!(build.matches("TypeSpace::new").count(), 1);
    let cargo = include_str!("../Cargo.toml");
    assert!(!cargo.contains("quire-specification"));
    assert!(!cargo.lines().any(|line| line.contains("git =")));
    assert!(snapshot_is_data_only(
        &root().join("fixtures/shared-reference-2-draft")
    ));
}

fn snapshot_is_data_only(directory: &Path) -> bool {
    fs::read_dir(directory)
        .expect("snapshot directory")
        .all(|entry| {
            entry
                .expect("snapshot entry")
                .path()
                .extension()
                .is_some_and(|extension| extension == "json")
        })
}
