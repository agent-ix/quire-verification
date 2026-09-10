// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Frozen, replayable qualification-corpus retention.
//!
//! Governing requirement: FR-023.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::contracts::{
    MAX_ARRAY_ITEMS, VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_sha256,
    validate_schema_definition, wire,
};
use crate::qualification_profile::FrozenQualificationProfile;
use crate::shared_reference::{ValidatedSharedReference, parse_strict_bounded_json};

/// One explicitly owned population gap retained in the corpus denominator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationCorpusGap {
    class_id: Box<str>,
    owner: Box<str>,
    reason: Box<str>,
    disposition: Option<Box<str>>,
}

impl QualificationCorpusGap {
    /// Returns the exact population class without a retained input.
    #[must_use]
    pub const fn class_id(&self) -> &str {
        &self.class_id
    }

    /// Returns the owner responsible for closing or accepting the gap.
    #[must_use]
    pub const fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the retained explanation.
    #[must_use]
    pub const fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns why a referenced candidate was not retained, when one was named.
    #[must_use]
    pub fn disposition(&self) -> Option<&str> {
        self.disposition.as_deref()
    }
}

/// Immutable, schema-derived corpus metadata and its exact retained input bytes.
#[derive(Clone, Debug)]
pub struct FrozenQualificationCorpus {
    manifest: wire::QualificationCorpusManifest,
    retained_bytes: BTreeMap<Box<str>, Box<[u8]>>,
    identity: Box<str>,
}

impl FrozenQualificationCorpus {
    /// Returns the schema-derived corpus manifest.
    #[must_use]
    pub const fn manifest(&self) -> &wire::QualificationCorpusManifest {
        &self.manifest
    }

    /// Returns the exact strict-reader-proved bytes retained for an entry.
    #[must_use]
    pub fn retained_bytes(&self, entry_id: &str) -> Option<&[u8]> {
        self.retained_bytes.get(entry_id).map(AsRef::as_ref)
    }

    /// Returns the RFC 8785 identity of the complete manifest.
    #[must_use]
    pub const fn identity(&self) -> &str {
        &self.identity
    }
}

/// Result of binding a corpus manifest to one frozen qualification profile.
#[derive(Clone, Debug)]
pub enum QualificationCorpusOutcome {
    /// Every required population class has at least one retained input.
    Complete(Box<FrozenQualificationCorpus>),
    /// Missing classes remain explicit, owned members of the denominator.
    Incomplete {
        /// Frozen corpus metadata and every input that was retained.
        corpus: Box<FrozenQualificationCorpus>,
        /// Exactly one owned record for every missing required class.
        gaps: Box<[QualificationCorpusGap]>,
    },
}

/// Validates and freezes one exact retained qualification corpus.
///
/// # Errors
/// Returns a schema, identity, authorization, coverage, replay, or ancestry
/// refusal when the candidate cannot support reproducible qualification.
pub fn qualify_corpus_manifest(
    value: Value,
    profile: &FrozenQualificationProfile,
    validated_references: &[ValidatedSharedReference],
) -> Result<QualificationCorpusOutcome, VerificationError> {
    validate_schema_definition("QualificationCorpusManifest", &value)?;
    bind_profile(&value, profile)?;
    if validated_references.len() > MAX_ARRAY_ITEMS {
        return Err(invalid(
            "strict-reader proof count exceeds the corpus boundary",
            "validated_references",
        ));
    }

    let proof_index = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::artifact_proof)
        .map(|(reference, role, bytes)| (reference, (role, bytes)))
        .collect::<BTreeMap<_, _>>();
    let required = string_set(&value["requiredClasses"], "requiredClasses")?;
    let entries = array(&value, "entries")?;
    let mut entry_ids = BTreeSet::new();
    let mut artifact_identities = BTreeSet::new();
    let mut covered = BTreeSet::new();
    let mut retained_bytes = BTreeMap::new();
    let mut parents = BTreeMap::new();

    for entry in entries {
        let entry_id = string(entry, "entryId")?;
        if !entry_ids.insert(entry_id) {
            return Err(invalid(
                "duplicate qualification-corpus entry identifier",
                entry_id,
            ));
        }
        let artifact_ref = &entry["artifactRef"];
        if !artifact_identities.insert(artifact_identity(artifact_ref)?) {
            return Err(invalid(
                "qualification corpus repeats an input artifact identity",
                entry_id,
            ));
        }
        let canonical = jcs_canonicalize(artifact_ref)?;
        let bytes = require_proof(&canonical, "source", &proof_index, entry_id)?;
        retained_bytes.insert(Box::<str>::from(entry_id), Box::<[u8]>::from(bytes));

        for class in strings(&entry["classes"], "entries.classes")? {
            if !required.contains(class) {
                return Err(invalid(
                    "corpus entry names an unknown population class",
                    class,
                ));
            }
            covered.insert(class);
        }

        match string(entry, "origin")? {
            "repository" | "conformance" => {}
            "generated" => validate_replay(entry, bytes, &proof_index, true)?,
            "mutation" => {
                validate_replay(entry, bytes, &proof_index, false)?;
                parents.insert(entry_id, string(&entry["lineage"], "parentEntryId")?);
            }
            origin => return Err(invalid("unknown qualification-corpus origin", origin)),
        }
    }
    validate_parent_graph(&entry_ids, &parents)?;
    validate_exclusions(&value, &required)?;
    let gaps = validate_gaps(&value, &required, &covered)?;

    let identity = jcs_sha256(&value)?.into_boxed_str();
    let manifest = serde_json::from_value(value).map_err(|error| {
        invalid(
            "QualificationCorpusManifest schema-derived decoding failed",
            &error.to_string(),
        )
    })?;
    let corpus = Box::new(FrozenQualificationCorpus {
        manifest,
        retained_bytes,
        identity,
    });
    if gaps.is_empty() {
        Ok(QualificationCorpusOutcome::Complete(corpus))
    } else {
        Ok(QualificationCorpusOutcome::Incomplete {
            corpus,
            gaps: gaps.into_boxed_slice(),
        })
    }
}

fn bind_profile(
    manifest: &Value,
    profile: &FrozenQualificationProfile,
) -> Result<(), VerificationError> {
    let profile_value = serde_json::to_value(profile.profile()).map_err(|error| {
        invalid(
            "frozen qualification profile could not be serialized",
            &error.to_string(),
        )
    })?;
    if manifest["profileIdentity"] != Value::String(profile.identity().to_owned()) {
        return Err(invalid(
            "corpus names a different frozen profile",
            "profileIdentity",
        ));
    }
    if manifest["populationId"] != profile_value["inputPopulation"]["populationId"] {
        return Err(invalid(
            "corpus names a different input population",
            "populationId",
        ));
    }
    if manifest["requiredClasses"] != profile_value["inputPopulation"]["requiredClasses"] {
        return Err(invalid(
            "corpus required classes differ from the frozen population",
            "requiredClasses",
        ));
    }
    Ok(())
}

type ProofIndex<'a> = BTreeMap<&'a [u8], (&'a str, &'a [u8])>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Visiting,
    Done,
}

fn require_proof<'a>(
    canonical: &[u8],
    expected_role: &str,
    proofs: &'a ProofIndex<'a>,
    field: &str,
) -> Result<&'a [u8], VerificationError> {
    let Some((actual_role, bytes)) = proofs.get(canonical) else {
        return Err(invalid(
            "corpus reference lacks an exact strict-reader proof",
            field,
        ));
    };
    if *actual_role != expected_role {
        return Err(
            invalid("corpus reference has the wrong artifact kind", field)
                .with_context("expected_role", expected_role)
                .with_context("actual_role", *actual_role),
        );
    }
    Ok(bytes)
}

fn validate_replay(
    entry: &Value,
    output: &[u8],
    proofs: &ProofIndex<'_>,
    generated: bool,
) -> Result<(), VerificationError> {
    let lineage = &entry["lineage"];
    if generated {
        let generator = jcs_canonicalize(&lineage["generatorRef"])?;
        require_proof(&generator, "executable-projection", proofs, "generatorRef")?;
    }
    let replay = jcs_canonicalize(&lineage["replayRunRef"])?;
    let replay_bytes = require_proof(&replay, "run-artifact", proofs, "replayRunRef")?;
    let run = parse_strict_bounded_json(replay_bytes)?;
    validate_schema_definition("QualificationCorpusReplayRun", &run)?;
    let mut lineage_claim = lineage.clone();
    lineage_claim
        .as_object_mut()
        .ok_or_else(|| invalid("derived lineage is not an object", "lineage"))?
        .remove("replayRunRef");
    let lineage_identity = jcs_sha256(&lineage_claim)?;
    let output_digest = byte_digest(output);
    if run["entryId"] != entry["entryId"]
        || run["lineageIdentity"] != Value::String(lineage_identity)
        || run["outputDigest"] != Value::String(output_digest)
    {
        return Err(invalid(
            "replay run does not prove the exact retained output bytes",
            string(entry, "entryId")?,
        ));
    }
    Ok(())
}

fn validate_parent_graph<'a>(
    entry_ids: &BTreeSet<&'a str>,
    parents: &BTreeMap<&'a str, &'a str>,
) -> Result<(), VerificationError> {
    for (child, parent) in parents {
        if child == parent || !entry_ids.contains(parent) {
            return Err(invalid(
                "mutation parent is absent or self-referential",
                child,
            ));
        }
    }
    let mut states = BTreeMap::new();
    for start in parents.keys() {
        if states.get(start).copied() == Some(VisitState::Done) {
            continue;
        }
        let mut path = Vec::new();
        let mut cursor = Some(*start);
        while let Some(entry_id) = cursor {
            match states.get(entry_id).copied() {
                None => {
                    states.insert(entry_id, VisitState::Visiting);
                    path.push(entry_id);
                    cursor = parents.get(entry_id).copied();
                }
                Some(VisitState::Visiting) => {
                    return Err(invalid("mutation parent graph contains a cycle", start));
                }
                Some(VisitState::Done) => break,
            }
        }
        for entry_id in path {
            states.insert(entry_id, VisitState::Done);
        }
    }
    Ok(())
}

fn validate_gaps(
    manifest: &Value,
    required: &BTreeSet<&str>,
    covered: &BTreeSet<&str>,
) -> Result<Vec<QualificationCorpusGap>, VerificationError> {
    let mut gap_values = BTreeMap::new();
    for gap in array(manifest, "gaps")? {
        let class_id = string(gap, "classId")?;
        if !required.contains(class_id) || covered.contains(class_id) {
            return Err(invalid(
                "gap does not name one missing required class",
                class_id,
            ));
        }
        if gap_values.insert(class_id, gap).is_some() {
            return Err(invalid("corpus repeats a population gap", class_id));
        }
    }
    let missing = required
        .difference(covered)
        .copied()
        .collect::<BTreeSet<_>>();
    if missing != gap_values.keys().copied().collect() {
        return Err(invalid(
            "every missing required class must have exactly one owned gap",
            "gaps",
        ));
    }
    gap_values
        .into_values()
        .map(|gap| {
            let disposition = match (&gap["candidateRef"], &gap["rights"]) {
                (Value::Null, Value::Null) => None,
                (Value::Object(_), Value::Object(_)) => {
                    let value = string(&gap["rights"], "disposition")?;
                    if !matches!(value, "reference-only" | "unauthorized") {
                        return Err(invalid(
                            "unretained candidate has a retention-authorizing disposition",
                            value,
                        ));
                    }
                    Some(Box::from(value))
                }
                _ => {
                    return Err(invalid(
                        "gap candidate reference and rights decision must appear together",
                        string(gap, "classId")?,
                    ));
                }
            };
            Ok(QualificationCorpusGap {
                class_id: Box::from(string(gap, "classId")?),
                owner: Box::from(string(gap, "owner")?),
                reason: Box::from(string(gap, "reason")?),
                disposition,
            })
        })
        .collect()
}

fn validate_exclusions(
    manifest: &Value,
    required: &BTreeSet<&str>,
) -> Result<(), VerificationError> {
    let mut ids = BTreeSet::new();
    for exclusion in array(manifest, "exclusions")? {
        let exclusion_id = string(exclusion, "exclusionId")?;
        if !ids.insert(exclusion_id) {
            return Err(invalid("duplicate corpus exclusion", exclusion_id));
        }
        let class_id = string(exclusion, "classId")?;
        if !required.contains(class_id) {
            return Err(invalid(
                "exclusion names an unknown population class",
                class_id,
            ));
        }
    }
    Ok(())
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a [Value], VerificationError> {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid("schema-required corpus collection is not an array", field))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str, VerificationError> {
    value[field]
        .as_str()
        .ok_or_else(|| invalid("schema-required corpus field is not a string", field))
}

fn strings<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, VerificationError> {
    value
        .as_array()
        .ok_or_else(|| invalid("schema-required corpus field is not an array", field))?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| invalid("corpus collection item is not a string", field))
        })
        .collect()
}

fn string_set<'a>(value: &'a Value, field: &str) -> Result<BTreeSet<&'a str>, VerificationError> {
    Ok(strings(value, field)?.into_iter().collect())
}

fn byte_digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn artifact_identity(value: &Value) -> Result<[&str; 8], VerificationError> {
    let at = |pointer| {
        value
            .pointer(pointer)
            .and_then(Value::as_str)
            .ok_or_else(|| invalid("artifact identity field is not a string", pointer))
    };
    Ok([
        at("/value/refVersion")?,
        at("/value/kind")?,
        at("/value/authority")?,
        at("/value/identity")?,
        at("/value/revision/namespace")?,
        at("/value/revision/value")?,
        at("/value/wire/identity")?,
        at("/value/wire/version")?,
    ])
}

fn invalid(message: &str, value: &str) -> VerificationError {
    VerificationError::new(VerificationErrorCode::InvalidCorpusManifest, message)
        .with_context("value", value)
}
