// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Strict, bounded consumption of the selected shared-reference packet.
//!
//! The retained schemas, their derived `wire_v1`/`wire_v2` types, the fourteen amendment
//! fixtures, and the byte-identical snapshot guard live in public crate
//! `quire-verification-contracts` (PLAT-861, VER-50), which this crate depends on and
//! re-exports at `contracts`; this module re-exports them unchanged so every call site below
//! is unaffected.
//!
//! Governing requirements: FR-016, NFR-005, TC-029, TC-031, and IT-002.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::de::{DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};

use crate::contracts::{
    MAX_ARRAY_ITEMS, MAX_JSON_BYTES, MAX_JSON_DEPTH, VerificationError, VerificationErrorCode,
    jcs_canonicalize, jcs_sha256,
};

const V1: &str = "ix.shared-reference/1-draft";
const V2: &str = "ix.shared-reference/2-draft";
const ARTIFACT_V1: &str = "ix.artifact-ref/1-draft";
const ARTIFACT_V2: &str = "ix.artifact-ref/2-draft";
const ARTIFACT_V3: &str = "ix.artifact-ref/3-draft";
const DEPTH_MARKER: &str = "shared-reference nesting limit exceeded";
const ARRAY_MARKER: &str = "shared-reference array limit exceeded";
const DUPLICATE_MARKER: &str = "duplicate decoded member name";

/// Verifies the retained two-schema, fourteen-fixture snapshot manifest (VER-50).
pub use crate::contracts::shared_reference::verify_shared_reference_snapshot;
/// Rust wire types generated from the retained version-1 schema.
pub use crate::contracts::shared_reference::wire_v1;
/// Rust wire types generated from the retained version-2 schema.
pub use crate::contracts::shared_reference::wire_v2;

/// Versions and semantic features the local reader can interpret.
#[derive(Clone, Debug, Default)]
pub struct ReferenceSupport {
    /// Supported versions keyed by artifact wire identity.
    pub wire_versions: BTreeMap<String, BTreeSet<String>>,
    /// Supported editions keyed by language identity.
    pub language_editions: BTreeMap<String, BTreeSet<String>>,
    /// Supported versions keyed by semantic-profile identity.
    pub semantic_profile_versions: BTreeMap<String, BTreeSet<String>>,
    /// Supported decoded semantic feature names.
    pub features: BTreeSet<String>,
}

/// Exact locally selected artifact identity and optional resolved bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactExpectation {
    /// Selected artifact-reference wire version.
    pub ref_version: String,
    /// Selected role-specific artifact kind.
    pub kind: String,
    /// Selected authority.
    pub authority: String,
    /// Selected immutable identity.
    pub identity: String,
    /// Selected revision namespace.
    pub revision_namespace: String,
    /// Selected revision value.
    pub revision_value: String,
    /// Selected exact byte digest, retained even when local bytes are stale.
    pub digest: String,
    /// Selected artifact wire identity.
    pub wire_identity: String,
    /// Selected artifact wire version.
    pub wire_version: String,
    /// Exact locally supplied bytes, or `None` when the selected revision is stale.
    pub bytes: Option<Vec<u8>>,
}

/// Exact locally selected language.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageExpectation {
    /// Language identity.
    pub identity: String,
    /// Language edition.
    pub edition: String,
}

/// Exact locally selected semantic profile and its definition bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticProfileExpectation {
    /// Semantic-profile identity.
    pub identity: String,
    /// Semantic-profile version.
    pub version: String,
    /// Exact profile-definition bytes, or `None` when unavailable.
    pub definition_bytes: Option<Vec<u8>>,
}

/// Local inputs required to validate a version-2 semantic reference.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticExpectation {
    /// Exact referenced byte artifact.
    pub artifact: ArtifactExpectation,
    /// Selected language, including an explicit `None` selection.
    pub language: Option<LanguageExpectation>,
    /// Selected semantic profile, including an explicit `None` selection.
    pub semantic_profile: Option<SemanticProfileExpectation>,
    /// Exact required-feature set; order is intentionally irrelevant.
    pub required_features: BTreeSet<String>,
    /// Qualified structured value selected by the semantic profile for JCS.
    pub canonical_value: Option<Value>,
}

/// Local inputs required to validate a retained version-1 source map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceMapExpectation {
    /// Exact original source artifact and bytes.
    pub original: ArtifactExpectation,
    /// Exact extracted body artifact and bytes.
    pub body: ArtifactExpectation,
}

/// Local inputs required to select one explicit retained version-1 migration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationExpectation {
    /// Exact historical artifact reference.
    pub old: ArtifactExpectation,
    /// Exact selected current artifact reference.
    pub new: ArtifactExpectation,
    /// Exact accepted binding artifact that records the mapping.
    pub mapping: ArtifactExpectation,
}

/// The expected reference form and its locally supplied validation inputs.
#[derive(Clone, Debug, PartialEq)]
pub enum ReferenceExpectation {
    /// A version-2 byte-artifact envelope.
    Artifact(ArtifactExpectation),
    /// A version-2 semantic envelope.
    Semantic(SemanticExpectation),
    /// A retained version-1 source-map envelope.
    SourceMap(SourceMapExpectation),
}

#[derive(Clone, Debug)]
enum ValidatedReferenceWire {
    Version1 {
        _value: Box<wire_v1::ProposedSharedReferenceEnvelopeStructuralValidationOnly>,
    },
    Version2 {
        _value: Box<wire_v2::SharedArtifactAndSemanticReferenceAmendmentStructuralValidationOnly>,
    },
}

/// An opaque schema-derived reference that passed every selected reader check.
///
/// Callers can retain and pass this value to planning but cannot construct a
/// validation claim without executing [`SharedReferenceReader::read`].
#[derive(Clone, Debug)]
pub struct ValidatedSharedReference {
    _wire: ValidatedReferenceWire,
    canonical_envelope: Box<[u8]>,
    reference_role: Option<Box<str>>,
    artifact_bytes: Option<Box<[u8]>>,
}

impl ValidatedSharedReference {
    fn version1(
        value: &Value,
        wire: wire_v1::ProposedSharedReferenceEnvelopeStructuralValidationOnly,
    ) -> Result<Self, VerificationError> {
        Ok(Self {
            _wire: ValidatedReferenceWire::Version1 {
                _value: Box::new(wire),
            },
            canonical_envelope: jcs_canonicalize(value)?.into_boxed_slice(),
            reference_role: None,
            artifact_bytes: None,
        })
    }

    fn version2(
        value: &Value,
        wire: wire_v2::SharedArtifactAndSemanticReferenceAmendmentStructuralValidationOnly,
        artifact_bytes: Option<Vec<u8>>,
    ) -> Result<Self, VerificationError> {
        let reference_role = (value.get("kind").and_then(Value::as_str) == Some("artifact"))
            .then(|| value.pointer("/value/kind").and_then(Value::as_str))
            .flatten()
            .map(Box::<str>::from);
        Ok(Self {
            _wire: ValidatedReferenceWire::Version2 {
                _value: Box::new(wire),
            },
            canonical_envelope: jcs_canonicalize(value)?.into_boxed_slice(),
            reference_role,
            artifact_bytes: artifact_bytes.map(Vec::into_boxed_slice),
        })
    }

    pub(crate) fn reference_key(&self) -> Option<(&[u8], &str)> {
        self.reference_role
            .as_deref()
            .map(|role| (self.canonical_envelope.as_ref(), role))
    }

    pub(crate) fn artifact_proof(&self) -> Option<(&[u8], &str, &[u8])> {
        self.reference_key().and_then(|(reference, role)| {
            self.artifact_bytes
                .as_deref()
                .map(|bytes| (reference, role, bytes))
        })
    }
}

/// Stateful strict reader that detects immutable identity conflicts across inputs.
#[derive(Debug)]
pub struct SharedReferenceReader {
    support: ReferenceSupport,
    observed_identities: BTreeMap<Vec<u8>, String>,
}

impl SharedReferenceReader {
    /// Creates a reader with the caller's explicit support registry.
    #[must_use]
    pub fn new(support: ReferenceSupport) -> Self {
        Self {
            support,
            observed_identities: BTreeMap::new(),
        }
    }

    /// Reads one exact artifact, semantic, or source-map reference.
    ///
    /// # Errors
    /// Returns a stable refusal before the value can enter planning or matching.
    pub fn read(
        &mut self,
        bytes: &[u8],
        expectation: &ReferenceExpectation,
    ) -> Result<ValidatedSharedReference, VerificationError> {
        let checkpoint = self.observed_identities.clone();
        let result = (|| {
            let value = parse_strict_bounded_json(bytes)?;
            let (version, kind) = classify(&value)?;
            preflight_reference_versions(&value, version, kind)?;

            match (version, kind, expectation) {
                (V2, "artifact", ReferenceExpectation::Artifact(expected)) => {
                    crate::contracts::shared_reference::validate_shared_reference_v2(&value)?;
                    let generated = decode_v2(value.clone())?;
                    self.validate_artifact(value_at(&value, "/value")?, expected)?;
                    ValidatedSharedReference::version2(&value, generated, expected.bytes.clone())
                }
                (V2, "semantic", ReferenceExpectation::Semantic(expected)) => {
                    crate::contracts::shared_reference::validate_shared_reference_v2(&value)?;
                    let generated = decode_v2(value.clone())?;
                    self.validate_semantic(value_at(&value, "/value")?, expected)?;
                    ValidatedSharedReference::version2(&value, generated, None)
                }
                (V1, "source-map", ReferenceExpectation::SourceMap(expected)) => {
                    crate::contracts::shared_reference::validate_shared_reference_v1(&value)?;
                    let generated = decode_v1(value.clone())?;
                    self.validate_source_map(value_at(&value, "/value")?, expected)?;
                    ValidatedSharedReference::version1(&value, generated)
                }
                (V1 | V2, _, _) => Err(refusal(
                    VerificationErrorCode::IdentityMismatch,
                    "shared-reference kind does not match the selected role",
                )
                .with_context("kind", kind)),
                _ => Err(refusal(
                    VerificationErrorCode::UnsupportedReferenceVersion,
                    "unsupported shared-reference version",
                )
                .with_context("wire_version", version)),
            }
        })();
        if result.is_err() {
            self.observed_identities = checkpoint;
        }
        result
    }

    /// Selects exactly one explicit retained version-1 migration.
    ///
    /// Non-applicable, valid candidates are ignored. Zero applicable mappings
    /// refuse as an identity mismatch; multiple applicable mappings refuse as
    /// ambiguous. Reverse and transitive mappings are never inferred.
    ///
    /// # Errors
    /// Returns a stable refusal for malformed candidates, no exact mapping, or ambiguity.
    pub fn resolve_migration(
        &mut self,
        candidates: &[&[u8]],
        expectation: &MigrationExpectation,
    ) -> Result<ValidatedSharedReference, VerificationError> {
        let checkpoint = self.observed_identities.clone();
        let result = (|| {
            let mut selected: Option<(
                Value,
                wire_v1::ProposedSharedReferenceEnvelopeStructuralValidationOnly,
            )> = None;
            for bytes in candidates {
                let value = parse_strict_bounded_json(bytes)?;
                let (version, kind) = classify(&value)?;
                if version != V1 || kind != "migration" {
                    return Err(refusal(
                        VerificationErrorCode::UnsupportedReferenceVersion,
                        "migration candidates must use the retained version-1 allocation",
                    ));
                }
                preflight_reference_versions(&value, version, kind)?;
                crate::contracts::shared_reference::validate_shared_reference_v1(&value)?;
                let generated = decode_v1(value.clone())?;
                let migration = value_at(&value, "/value")?;
                if !migration_applies(migration, expectation)? {
                    continue;
                }
                if selected.is_some() {
                    return Err(refusal(
                        VerificationErrorCode::AmbiguousEquivalence,
                        "more than one exact migration applies",
                    ));
                }
                selected = Some((value, generated));
            }

            let (value, generated) = selected.ok_or_else(|| {
                refusal(
                    VerificationErrorCode::IdentityMismatch,
                    "no explicit migration applies to the selected artifacts",
                )
            })?;
            self.validate_migration(value_at(&value, "/value")?, expectation)?;
            ValidatedSharedReference::version1(&value, generated)
        })();
        if result.is_err() {
            self.observed_identities = checkpoint;
        }
        result
    }

    fn validate_artifact(
        &mut self,
        value: &Value,
        expected: &ArtifactExpectation,
    ) -> Result<(), VerificationError> {
        self.observe_artifact(value)?;
        let wire_identity = string_at(value, "/wire/identity")?;
        let wire_version = string_at(value, "/wire/version")?;
        require_supported_version(
            &self.support.wire_versions,
            wire_identity,
            wire_version,
            "artifact wire",
        )?;
        let fields_match = string_at(value, "/refVersion")? == expected.ref_version
            && string_at(value, "/kind")? == expected.kind
            && string_at(value, "/authority")? == expected.authority
            && string_at(value, "/identity")? == expected.identity
            && string_at(value, "/revision/namespace")? == expected.revision_namespace
            && string_at(value, "/revision/value")? == expected.revision_value
            && string_at(value, "/digest")? == expected.digest
            && wire_identity == expected.wire_identity
            && wire_version == expected.wire_version;
        if !fields_match {
            return Err(refusal(
                VerificationErrorCode::IdentityMismatch,
                "artifact reference does not match the selected identity",
            ));
        }
        let local = expected.bytes.as_deref().ok_or_else(|| {
            refusal(
                VerificationErrorCode::Stale,
                "selected artifact revision is unavailable locally",
            )
        })?;
        if string_at(value, "/digest")? != byte_digest(local) {
            return Err(refusal(
                VerificationErrorCode::IdentityMismatch,
                "artifact byte digest does not match the supplied bytes",
            ));
        }
        Ok(())
    }

    fn validate_semantic(
        &mut self,
        value: &Value,
        expected: &SemanticExpectation,
    ) -> Result<(), VerificationError> {
        self.validate_artifact(value_at(value, "/artifact")?, &expected.artifact)?;
        validate_language(
            &self.support,
            value.get("language"),
            expected.language.as_ref(),
        )?;

        let actual_profile = value.get("semanticProfile").ok_or_else(invalid_wire)?;
        match (&expected.semantic_profile, actual_profile) {
            (None, Value::Null) => {
                if value.get("canonicalIdentity") != Some(&Value::Null)
                    || value_at(value, "/requiredFeatures")?
                        .as_array()
                        .is_none_or(|features| !features.is_empty())
                    || value.get("language") != Some(&Value::Null)
                    || !expected.required_features.is_empty()
                    || expected.canonical_value.is_some()
                {
                    return Err(refusal(
                        VerificationErrorCode::IdentityMismatch,
                        "null semantic profile selection is inconsistent",
                    ));
                }
                return Ok(());
            }
            (Some(selected), Value::Object(_)) => {
                validate_profile(&self.support, actual_profile, selected)?;
            }
            _ => {
                return Err(refusal(
                    VerificationErrorCode::IdentityMismatch,
                    "semantic profile does not match the local selection",
                ));
            }
        }

        let actual_features = feature_set(value_at(value, "/requiredFeatures")?)?;
        if let Some(feature) = actual_features
            .iter()
            .find(|feature| !self.support.features.contains(*feature))
        {
            return Err(refusal(
                VerificationErrorCode::UnknownRequiredFeature,
                "semantic reference requires an unsupported feature",
            )
            .with_context("feature", feature));
        }
        if actual_features != expected.required_features {
            return Err(refusal(
                VerificationErrorCode::IdentityMismatch,
                "required feature set does not match the local selection",
            ));
        }

        let canonical = value_at(value, "/canonicalIdentity")?;
        if string_at(canonical, "/domain")? != "quire.verification.jcs"
            || string_at(canonical, "/version")? != "rfc8785-v1"
            || string_at(canonical, "/algorithm")? != "sha256"
        {
            return Err(refusal(
                VerificationErrorCode::UnsupportedCanonicalization,
                "semantic reference selects an unsupported canonicalization",
            ));
        }
        let selected_value = expected.canonical_value.as_ref().ok_or_else(|| {
            refusal(
                VerificationErrorCode::Stale,
                "qualified canonical derivation is unavailable locally",
            )
        })?;
        if string_at(canonical, "/digest")? != jcs_sha256(selected_value)? {
            return Err(refusal(
                VerificationErrorCode::IdentityMismatch,
                "canonical identity does not match the selected structured value",
            ));
        }
        Ok(())
    }

    fn validate_source_map(
        &mut self,
        value: &Value,
        expected: &SourceMapExpectation,
    ) -> Result<(), VerificationError> {
        self.validate_artifact(value_at(value, "/original")?, &expected.original)?;
        self.validate_artifact(value_at(value, "/body")?, &expected.body)?;
        let original = expected.original.bytes.as_deref().ok_or_else(|| {
            refusal(
                VerificationErrorCode::Stale,
                "original source is unavailable",
            )
        })?;
        let body =
            expected.body.bytes.as_deref().ok_or_else(|| {
                refusal(VerificationErrorCode::Stale, "source body is unavailable")
            })?;
        validate_source_map_bytes(value, original, body)
    }

    fn validate_migration(
        &mut self,
        value: &Value,
        expected: &MigrationExpectation,
    ) -> Result<(), VerificationError> {
        let old = value_at(value, "/old")?;
        let new = value_at(value, "/new")?;
        let mapping = value_at(value, "/mapping")?;
        self.validate_artifact(old, &expected.old)?;
        self.validate_artifact(new, &expected.new)?;
        self.validate_artifact(mapping, &expected.mapping)?;
        if expected.mapping.kind != "binding"
            || artifact_identity_without_ref(old)? != artifact_identity_without_ref(new)?
        {
            return Err(refusal(
                VerificationErrorCode::IdentityMismatch,
                "migration does not preserve the exact byte artifact",
            ));
        }
        Ok(())
    }

    fn observe_artifact(&mut self, value: &Value) -> Result<(), VerificationError> {
        let identity = artifact_identity_without_digest(value)?;
        let digest = string_at(value, "/digest")?.to_owned();
        if let Some(previous) = self.observed_identities.get(&identity)
            && previous != &digest
        {
            return Err(refusal(
                VerificationErrorCode::IdentityContentConflict,
                "immutable artifact identity names conflicting content",
            ));
        }
        self.observed_identities.insert(identity, digest);
        Ok(())
    }
}

/// Parses one bounded JSON document while rejecting duplicate decoded object keys.
///
/// The object map is built only after every decoded key in that object is known
/// to be unique. Escape-equivalent spellings therefore cannot overwrite one another.
///
/// # Errors
/// Returns `invalid_wire` for malformed JSON or duplicate keys and preserves
/// the configured document, nesting, and array resource-limit codes.
pub fn parse_strict_bounded_json(bytes: &[u8]) -> Result<Value, VerificationError> {
    if bytes.len() > MAX_JSON_BYTES {
        return Err(refusal(
            VerificationErrorCode::JsonDocumentTooLarge,
            "serialized JSON exceeds 16 MiB",
        ));
    }
    reject_unrepresentable_integer_tokens(bytes)?;
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictSeed { depth: 0 }
        .deserialize(&mut deserializer)
        .map_err(|error| strict_decode_error(&error))?;
    deserializer
        .end()
        .map_err(|error| strict_decode_error(&error))?;
    Ok(value)
}

/// Validates one selected `sha256-jcs:` correspondence field in strict JSON payload bytes.
///
/// This checks the shared JCS identity layer only. The caller remains responsible
/// for validating the role payload's complete owning schema and policy.
///
/// # Errors
/// Returns `invalid_wire` for a missing or malformed digest and
/// `identity_mismatch` when it does not identify the selected structured value.
pub fn validate_jcs_correspondence(
    payload_bytes: &[u8],
    digest_pointer: &str,
    selected_value: &Value,
) -> Result<Value, VerificationError> {
    let payload = parse_strict_bounded_json(payload_bytes)?;
    let digest = string_at(&payload, digest_pointer)?;
    if digest.len() != 75
        || !digest.strip_prefix("sha256-jcs:").is_some_and(|hex| {
            hex.bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        })
    {
        return Err(invalid_wire());
    }
    if digest != jcs_sha256(selected_value)? {
        return Err(refusal(
            VerificationErrorCode::IdentityMismatch,
            "JCS correspondence does not match the selected structured value",
        ));
    }
    Ok(payload)
}

#[derive(Clone, Copy)]
struct StrictSeed {
    depth: usize,
}

impl<'de> DeserializeSeed<'de> for StrictSeed {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        if self.depth > MAX_JSON_DEPTH {
            return Err(serde::de::Error::custom(DEPTH_MARKER));
        }
        deserializer.deserialize_any(StrictVisitor { depth: self.depth })
    }
}

struct StrictVisitor {
    depth: usize,
}

impl<'de> Visitor<'de> for StrictVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one bounded JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        StrictSeed { depth: self.depth }.deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictSeed {
            depth: self.depth + 1,
        })? {
            if values.len() == MAX_ARRAY_ITEMS {
                return Err(serde::de::Error::custom(ARRAY_MARKER));
            }
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut object: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut seen = BTreeSet::new();
        let mut fields = Vec::new();
        while let Some(key) = object.next_key::<String>()? {
            if !seen.insert(key.clone()) {
                return Err(serde::de::Error::custom(format!(
                    "{DUPLICATE_MARKER}: {key:?}"
                )));
            }
            let value = object.next_value_seed(StrictSeed {
                depth: self.depth + 1,
            })?;
            fields.push((key, value));
        }
        Ok(Value::Object(fields.into_iter().collect::<Map<_, _>>()))
    }
}

fn strict_decode_error(error: &serde_json::Error) -> VerificationError {
    let detail = error.to_string();
    let code = if detail.contains(DEPTH_MARKER) {
        VerificationErrorCode::JsonNestingTooDeep
    } else if detail.contains(ARRAY_MARKER) {
        VerificationErrorCode::JsonArrayTooLarge
    } else {
        VerificationErrorCode::InvalidWire
    };
    refusal(code, detail)
}

fn classify(value: &Value) -> Result<(&str, &str), VerificationError> {
    let version = string_at(value, "/wireVersion")?;
    let kind = string_at(value, "/kind")?;
    match (version, kind) {
        (V2, "artifact" | "semantic") | (V1, "source-map" | "migration") => Ok((version, kind)),
        (V1 | V2, _) => Err(refusal(
            VerificationErrorCode::UnsupportedReferenceVersion,
            "reference kind is not allocated to the selected envelope version",
        )
        .with_context("wire_version", version)
        .with_context("kind", kind)),
        _ => Err(refusal(
            VerificationErrorCode::UnsupportedReferenceVersion,
            "unsupported shared-reference envelope version",
        )
        .with_context("wire_version", version)),
    }
}

fn preflight_reference_versions(
    value: &Value,
    version: &str,
    kind: &str,
) -> Result<(), VerificationError> {
    let selections: &[(&str, &[&str])] = match (version, kind) {
        (V2, "artifact") => &[("/value/refVersion", &[ARTIFACT_V3])],
        (V2, "semantic") => &[("/value/artifact/refVersion", &[ARTIFACT_V3])],
        (V1, "source-map") => &[
            ("/value/original/refVersion", &[ARTIFACT_V2]),
            ("/value/body/refVersion", &[ARTIFACT_V2]),
        ],
        (V1, "migration") => &[
            ("/value/old/refVersion", &[ARTIFACT_V1, ARTIFACT_V2]),
            ("/value/new/refVersion", &[ARTIFACT_V2]),
            ("/value/mapping/refVersion", &[ARTIFACT_V2]),
        ],
        _ => return Err(invalid_wire()),
    };
    for (pointer, allowed) in selections {
        if let Some(selected) = value.pointer(pointer).and_then(Value::as_str)
            && !allowed.contains(&selected)
        {
            return Err(refusal(
                VerificationErrorCode::UnsupportedReferenceVersion,
                "unsupported artifact-reference version",
            )
            .with_context("ref_version", selected));
        }
    }
    if (version, kind) == (V2, "semantic")
        && let Some(canonical) = value.pointer("/value/canonicalIdentity")
        && !canonical.is_null()
    {
        let selection = (
            canonical.get("domain").and_then(Value::as_str),
            canonical.get("version").and_then(Value::as_str),
            canonical.get("algorithm").and_then(Value::as_str),
            canonical.get("digest").and_then(Value::as_str),
        );
        if let (Some(domain), Some(canonical_version), Some(algorithm), Some(_)) = selection
            && (domain != "quire.verification.jcs"
                || canonical_version != "rfc8785-v1"
                || algorithm != "sha256")
        {
            return Err(refusal(
                VerificationErrorCode::UnsupportedCanonicalization,
                "semantic reference selects an unsupported canonicalization",
            ));
        }
    }
    Ok(())
}

fn validate_language(
    support: &ReferenceSupport,
    actual: Option<&Value>,
    expected: Option<&LanguageExpectation>,
) -> Result<(), VerificationError> {
    match (actual, expected) {
        (Some(Value::Null), None) => Ok(()),
        (Some(actual @ Value::Object(_)), Some(expected)) => {
            let identity = string_at(actual, "/identity")?;
            let edition = string_at(actual, "/edition")?;
            require_supported_version(&support.language_editions, identity, edition, "language")?;
            if identity == expected.identity && edition == expected.edition {
                Ok(())
            } else {
                Err(refusal(
                    VerificationErrorCode::IdentityMismatch,
                    "language does not match the local selection",
                ))
            }
        }
        _ => Err(refusal(
            VerificationErrorCode::IdentityMismatch,
            "language does not match the local selection",
        )),
    }
}

fn validate_profile(
    support: &ReferenceSupport,
    actual: &Value,
    expected: &SemanticProfileExpectation,
) -> Result<(), VerificationError> {
    let identity = string_at(actual, "/identity")?;
    let version = string_at(actual, "/version")?;
    require_supported_version(
        &support.semantic_profile_versions,
        identity,
        version,
        "semantic profile",
    )?;
    if identity != expected.identity || version != expected.version {
        return Err(refusal(
            VerificationErrorCode::IdentityMismatch,
            "semantic profile does not match the local selection",
        ));
    }
    let definition = expected.definition_bytes.as_deref().ok_or_else(|| {
        refusal(
            VerificationErrorCode::Stale,
            "semantic-profile definition is unavailable locally",
        )
    })?;
    if string_at(actual, "/definitionDigest")? != byte_digest(definition) {
        return Err(refusal(
            VerificationErrorCode::IdentityMismatch,
            "semantic-profile definition digest does not match local bytes",
        ));
    }
    Ok(())
}

fn require_supported_version(
    supported: &BTreeMap<String, BTreeSet<String>>,
    identity: &str,
    version: &str,
    label: &str,
) -> Result<(), VerificationError> {
    if supported
        .get(identity)
        .is_some_and(|versions| versions.contains(version))
    {
        Ok(())
    } else {
        Err(refusal(
            VerificationErrorCode::UnsupportedReferenceVersion,
            format!("unsupported {label} identity or version"),
        )
        .with_context("identity", identity)
        .with_context("version", version))
    }
}

fn feature_set(value: &Value) -> Result<BTreeSet<String>, VerificationError> {
    value
        .as_array()
        .ok_or_else(invalid_wire)?
        .iter()
        .map(|feature| feature.as_str().map(str::to_owned).ok_or_else(invalid_wire))
        .collect()
}

fn validate_source_map_bytes(
    map: &Value,
    original: &[u8],
    body: &[u8],
) -> Result<(), VerificationError> {
    let original_text = std::str::from_utf8(original).map_err(|_| malformed_source_map())?;
    let body_text = std::str::from_utf8(body).map_err(|_| malformed_source_map())?;
    let region_start = offset_at(map, "/region/startByte")?;
    let region_end = offset_at(map, "/region/endByte")?;
    require_span(original_text, region_start, region_end)?;
    let layout = value_at(map, "/layout")?;
    let strip_indentation = bool_at(layout, "/stripIndentation")?;
    let normalize_cr_lf = bool_at(layout, "/normalizeCrLf")?;
    let drop_final_newline = bool_at(layout, "/dropFinalNewline")?;
    let transformed = apply_layout(
        &original[region_start..region_end],
        strip_indentation,
        normalize_cr_lf,
        drop_final_newline,
    );
    if transformed != body {
        return Err(malformed_source_map());
    }

    let segments = value_at(map, "/segments")?
        .as_array()
        .ok_or_else(malformed_source_map)?;
    if body.is_empty() {
        return if segments.is_empty() {
            Ok(())
        } else {
            Err(malformed_source_map())
        };
    }
    let mut next_body = 0_usize;
    let mut next_original = region_start;
    for segment in segments {
        let original_start = offset_at(segment, "/original/startByte")?;
        let original_end = offset_at(segment, "/original/endByte")?;
        let body_start = offset_at(segment, "/body/startByte")?;
        let body_end = offset_at(segment, "/body/endByte")?;
        require_span(original_text, original_start, original_end)?;
        require_span(body_text, body_start, body_end)?;
        if original_start < region_start
            || original_end > region_end
            || original_start < next_original
            || body_start != next_body
            || original_start == original_end
            || body_start == body_end
            || original[original_start..original_end] != body[body_start..body_end]
        {
            return Err(malformed_source_map());
        }
        next_original = original_end;
        next_body = body_end;
    }
    if next_body != body.len() {
        return Err(malformed_source_map());
    }
    Ok(())
}

fn apply_layout(
    input: &[u8],
    strip_indentation: bool,
    normalize_cr_lf: bool,
    drop_final_newline: bool,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0_usize;
    let mut line_start = true;
    while index < input.len() {
        if strip_indentation && line_start && matches!(input[index], b' ' | b'\t') {
            index += 1;
            continue;
        }
        if normalize_cr_lf && input[index] == b'\r' && input.get(index + 1) == Some(&b'\n') {
            index += 1;
            continue;
        }
        let byte = input[index];
        output.push(byte);
        line_start = byte == b'\n';
        index += 1;
    }
    if drop_final_newline && output.last() == Some(&b'\n') {
        output.pop();
        if output.last() == Some(&b'\r') {
            output.pop();
        }
    }
    output
}

fn require_span(text: &str, start: usize, end: usize) -> Result<(), VerificationError> {
    if start > end
        || end > text.len()
        || !text.is_char_boundary(start)
        || !text.is_char_boundary(end)
    {
        Err(malformed_source_map())
    } else {
        Ok(())
    }
}

fn migration_applies(
    migration: &Value,
    expectation: &MigrationExpectation,
) -> Result<bool, VerificationError> {
    Ok(
        artifact_matches_without_bytes(value_at(migration, "/old")?, &expectation.old)?
            && artifact_matches_without_bytes(value_at(migration, "/new")?, &expectation.new)?,
    )
}

fn artifact_matches_without_bytes(
    value: &Value,
    expected: &ArtifactExpectation,
) -> Result<bool, VerificationError> {
    Ok(string_at(value, "/refVersion")? == expected.ref_version
        && string_at(value, "/kind")? == expected.kind
        && string_at(value, "/authority")? == expected.authority
        && string_at(value, "/identity")? == expected.identity
        && string_at(value, "/revision/namespace")? == expected.revision_namespace
        && string_at(value, "/revision/value")? == expected.revision_value
        && string_at(value, "/digest")? == expected.digest
        && string_at(value, "/wire/identity")? == expected.wire_identity
        && string_at(value, "/wire/version")? == expected.wire_version)
}

fn reject_unrepresentable_integer_tokens(bytes: &[u8]) -> Result<(), VerificationError> {
    let mut index = 0_usize;
    let mut in_string = false;
    let mut escaped = false;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        if byte == b'"' {
            in_string = true;
            index += 1;
            continue;
        }
        if byte == b'-' || byte.is_ascii_digit() {
            let start = index;
            index += 1;
            while index < bytes.len()
                && !matches!(
                    bytes[index],
                    b' ' | b'\t' | b'\r' | b'\n' | b',' | b']' | b'}' | b':'
                )
            {
                index += 1;
            }
            let token = std::str::from_utf8(&bytes[start..index]).map_err(|_| invalid_wire())?;
            if !token.bytes().any(|part| matches!(part, b'.' | b'e' | b'E')) {
                let exact = if token.starts_with('-') {
                    token.parse::<i64>().is_ok()
                } else {
                    token.parse::<u64>().is_ok()
                };
                if !exact {
                    return Err(invalid_wire().with_context("integer", token));
                }
            }
            continue;
        }
        index += 1;
    }
    Ok(())
}

fn artifact_identity_without_digest(value: &Value) -> Result<Vec<u8>, VerificationError> {
    let identity = [
        string_at(value, "/kind")?,
        string_at(value, "/authority")?,
        string_at(value, "/identity")?,
        string_at(value, "/revision/namespace")?,
        string_at(value, "/revision/value")?,
        string_at(value, "/wire/identity")?,
        string_at(value, "/wire/version")?,
    ];
    jcs_canonicalize(&identity)
}

fn artifact_identity_without_ref(value: &Value) -> Result<Vec<u8>, VerificationError> {
    let identity = [
        string_at(value, "/kind")?,
        string_at(value, "/authority")?,
        string_at(value, "/identity")?,
        string_at(value, "/revision/namespace")?,
        string_at(value, "/revision/value")?,
        string_at(value, "/digest")?,
        string_at(value, "/wire/identity")?,
        string_at(value, "/wire/version")?,
    ];
    jcs_canonicalize(&identity)
}

fn byte_digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn string_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, VerificationError> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(invalid_wire)
}

fn bool_at(value: &Value, pointer: &str) -> Result<bool, VerificationError> {
    value
        .pointer(pointer)
        .and_then(Value::as_bool)
        .ok_or_else(invalid_wire)
}

fn offset_at(value: &Value, pointer: &str) -> Result<usize, VerificationError> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .and_then(|offset| usize::try_from(offset).ok())
        .ok_or_else(malformed_source_map)
}

fn value_at<'a>(value: &'a Value, pointer: &str) -> Result<&'a Value, VerificationError> {
    value.pointer(pointer).ok_or_else(invalid_wire)
}

fn decode_v1(
    value: Value,
) -> Result<wire_v1::ProposedSharedReferenceEnvelopeStructuralValidationOnly, VerificationError> {
    serde_json::from_value(value)
        .map_err(|error| refusal(VerificationErrorCode::InvalidWire, error.to_string()))
}

fn decode_v2(
    value: Value,
) -> Result<
    wire_v2::SharedArtifactAndSemanticReferenceAmendmentStructuralValidationOnly,
    VerificationError,
> {
    serde_json::from_value(value)
        .map_err(|error| refusal(VerificationErrorCode::InvalidWire, error.to_string()))
}

fn refusal(code: VerificationErrorCode, message: impl Into<Box<str>>) -> VerificationError {
    VerificationError::new(code, message)
}

fn invalid_wire() -> VerificationError {
    refusal(
        VerificationErrorCode::InvalidWire,
        "shared-reference wire value is malformed",
    )
}

fn malformed_source_map() -> VerificationError {
    refusal(
        VerificationErrorCode::MalformedSourceMap,
        "source map does not preserve exact ordered byte correspondence",
    )
}
