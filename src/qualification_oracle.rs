// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Frozen qualification-oracle manifests and declared dependence validation.
//!
//! Governing requirement: FR-024.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use crate::contracts::{
    MAX_ARRAY_ITEMS, VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_sha256,
    validate_schema_definition, wire,
};
use crate::qualification_corpus::FrozenQualificationCorpus;
use crate::qualification_profile::FrozenQualificationProfile;
use crate::shared_reference::ValidatedSharedReference;

/// One owned guarantee for which no executable oracle is available.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationOracleGap {
    guarantee_id: Box<str>,
    owner: Box<str>,
    reason: Box<str>,
    limitations: Box<[Box<str>]>,
}

impl QualificationOracleGap {
    /// Returns the exact uncovered qualification guarantee.
    #[must_use]
    pub const fn guarantee_id(&self) -> &str {
        &self.guarantee_id
    }

    /// Returns the owner responsible for closing or accepting the gap.
    #[must_use]
    pub const fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns why no executable oracle is available.
    #[must_use]
    pub const fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns the exact limits on the missing oracle evidence.
    #[must_use]
    pub const fn limitations(&self) -> &[Box<str>] {
        &self.limitations
    }
}

/// One immutable, schema-derived qualification-oracle manifest.
#[derive(Clone, Debug)]
pub struct FrozenQualificationOracleManifest {
    manifest: wire::QualificationOracleManifest,
    identity: Box<str>,
}

impl FrozenQualificationOracleManifest {
    /// Returns the normalized schema-derived manifest.
    #[must_use]
    pub const fn manifest(&self) -> &wire::QualificationOracleManifest {
        &self.manifest
    }

    /// Returns the RFC 8785 identity of the complete normalized manifest.
    #[must_use]
    pub const fn identity(&self) -> &str {
        &self.identity
    }
}

/// Result of freezing all oracle bindings for one qualification profile.
#[derive(Clone, Debug)]
pub enum QualificationOracleOutcome {
    /// Every guarantee has at least one executable oracle binding.
    Complete(Box<FrozenQualificationOracleManifest>),
    /// Every uncovered guarantee remains an exact owned gap.
    Incomplete {
        /// The immutable manifest, including its owned gaps.
        manifest: Box<FrozenQualificationOracleManifest>,
        /// The owned gaps in guarantee-identifier order.
        gaps: Box<[QualificationOracleGap]>,
    },
}

/// Validates and freezes one oracle manifest against an exact profile and corpus.
///
/// Set-like manifest members are normalized before hashing. The ordered corpus
/// projection remains in caller-declared order.
///
/// # Errors
/// Returns a schema, identity, reference, guarantee-coverage, or dependence-topology
/// refusal when the manifest cannot support qualification evidence.
pub fn qualify_oracle_manifest(
    mut value: Value,
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    validated_references: &[ValidatedSharedReference],
) -> Result<QualificationOracleOutcome, VerificationError> {
    validate_schema_definition("QualificationOracleManifest", &value).map_err(|error| {
        invalid_manifest(
            "oracle manifest failed schema validation",
            &error.to_string(),
        )
    })?;
    bind_profile_and_corpus(&value, profile, corpus)?;
    normalize_set_like_members(&mut value)?;

    if validated_references.len() > MAX_ARRAY_ITEMS {
        return Err(invalid_manifest(
            "strict-reader proof count exceeds the oracle-manifest boundary",
            "validated_references",
        ));
    }

    let proof_index = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::reference_key)
        .collect::<BTreeMap<_, _>>();
    let guarantees = profile_guarantees(profile)?;
    let entries = corpus_entries(corpus)?;
    let bindings = array(&value, "bindings")?;
    let gaps = array(&value, "gaps")?;

    validate_unique_record_ids(bindings, gaps)?;
    validate_bindings(bindings, profile, &guarantees, &entries, &proof_index)?;
    validate_derived_graph(bindings)?;
    let owned_gaps = validate_coverage(bindings, gaps, &guarantees)?;

    let identity = jcs_sha256(&value)?.into_boxed_str();
    let manifest = serde_json::from_value(value).map_err(|error| {
        invalid_manifest(
            "QualificationOracleManifest schema-derived decoding failed",
            &error.to_string(),
        )
    })?;
    let frozen = Box::new(FrozenQualificationOracleManifest { manifest, identity });
    if owned_gaps.is_empty() {
        Ok(QualificationOracleOutcome::Complete(frozen))
    } else {
        Ok(QualificationOracleOutcome::Incomplete {
            manifest: frozen,
            gaps: owned_gaps.into_boxed_slice(),
        })
    }
}

fn bind_profile_and_corpus(
    manifest: &Value,
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
) -> Result<(), VerificationError> {
    if manifest["profileIdentity"] != Value::String(profile.identity().to_owned()) {
        return Err(invalid_manifest(
            "oracle manifest names a different frozen profile",
            "profileIdentity",
        ));
    }
    if manifest["corpusIdentity"] != Value::String(corpus.identity().to_owned()) {
        return Err(invalid_manifest(
            "oracle manifest names a different frozen corpus",
            "corpusIdentity",
        ));
    }
    Ok(())
}

fn profile_guarantees(
    profile: &FrozenQualificationProfile,
) -> Result<BTreeSet<String>, VerificationError> {
    let profile = serde_json::to_value(profile.profile()).map_err(|error| {
        invalid_manifest(
            "frozen qualification profile could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&profile, "semanticGuarantees")?
        .iter()
        .map(|guarantee| string(guarantee, "guaranteeId").map(str::to_owned))
        .collect::<Result<_, _>>()
}

fn corpus_entries(
    corpus: &FrozenQualificationCorpus,
) -> Result<BTreeSet<String>, VerificationError> {
    let manifest = serde_json::to_value(corpus.manifest()).map_err(|error| {
        invalid_manifest(
            "frozen qualification corpus could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&manifest, "entries")?
        .iter()
        .map(|entry| string(entry, "entryId").map(str::to_owned))
        .collect::<Result<_, _>>()
}

fn validate_unique_record_ids(bindings: &[Value], gaps: &[Value]) -> Result<(), VerificationError> {
    let mut identifiers = BTreeSet::new();
    for (record, field) in bindings
        .iter()
        .map(|binding| (binding, "bindingId"))
        .chain(gaps.iter().map(|gap| (gap, "gapId")))
    {
        let identifier = string(record, field)?;
        if !identifiers.insert(identifier) {
            return Err(invalid_manifest(
                "duplicate oracle binding or gap identifier",
                identifier,
            ));
        }
    }
    Ok(())
}

type ProofIndex<'a> = BTreeMap<&'a [u8], &'a str>;

fn validate_bindings(
    bindings: &[Value],
    profile: &FrozenQualificationProfile,
    guarantees: &BTreeSet<String>,
    entries: &BTreeSet<String>,
    proofs: &ProofIndex<'_>,
) -> Result<(), VerificationError> {
    let profile_value = serde_json::to_value(profile.profile()).map_err(|error| {
        invalid_manifest(
            "frozen qualification profile could not be serialized",
            &error.to_string(),
        )
    })?;
    let candidate = jcs_canonicalize(&profile_value["candidateRef"])?;
    let identities = bindings
        .iter()
        .map(binding_identities)
        .collect::<Result<Vec<_>, _>>()?;
    let identity_counts = BindingIdentityCounts::new(&identities);

    for (index, binding) in bindings.iter().enumerate() {
        let binding_id = string(binding, "bindingId")?;
        let guarantee_id = string(binding, "guaranteeId")?;
        if !guarantees.contains(guarantee_id) {
            return Err(invalid_manifest(
                "oracle binding names an unknown qualification guarantee",
                guarantee_id,
            ));
        }
        for entry_id in strings(&binding["inputProjection"]["entryIds"], "entryIds")? {
            if !entries.contains(entry_id) {
                return Err(invalid_manifest(
                    "oracle binding projects an unretained corpus entry",
                    entry_id,
                ));
            }
        }
        validate_binding_references(binding, proofs)?;
        validate_binding_dependence(
            binding,
            binding_id,
            &identities[index],
            &identity_counts,
            &candidate,
            proofs,
        )?;
    }
    Ok(())
}

fn validate_binding_references(
    binding: &Value,
    proofs: &ProofIndex<'_>,
) -> Result<(), VerificationError> {
    for (field, role) in [
        ("oracleRef", "oracle"),
        ("executableRef", "executable-projection"),
        ("comparisonProcedureRef", "review-procedure"),
        ("disagreementProcedureRef", "review-procedure"),
        ("executionRequestRef", "invocation"),
        ("comparisonRequestRef", "invocation"),
    ] {
        require_reference(&binding[field], field, role, proofs)?;
    }
    Ok(())
}

fn validate_binding_dependence(
    binding: &Value,
    binding_id: &str,
    identities: &BindingIdentities,
    identity_counts: &BindingIdentityCounts<'_>,
    candidate: &[u8],
    proofs: &ProofIndex<'_>,
) -> Result<(), VerificationError> {
    let dependence_class = string(&binding["dependence"], "class")?;
    let expected_origin = string(&binding["expectedRelation"], "origin")?;
    if dependence_class == "independent" {
        validate_independence_references(binding, proofs)?;
        let candidate_ancestry = canonical_reference_set(
            &binding["dependence"]["candidateAncestryRefs"],
            "candidateAncestryRefs",
        )?;
        let oracle_ancestry = canonical_reference_set(
            &binding["dependence"]["oracleAncestryRefs"],
            "oracleAncestryRefs",
        )?;
        if identities.executable == candidate
            || !candidate_ancestry.is_disjoint(&oracle_ancestry)
            || identity_counts.is_shared(identities)
            || !matches!(expected_origin, "oracle" | "independent-third-party")
        {
            return Err(invalid_binding(
                "shared ancestry, exact references, or expected values cannot be classified as independent",
                binding_id,
            ));
        }
    }
    if matches!(expected_origin, "candidate" | "shared") {
        let has_expected_value_basis = dependence_class == "shared"
            && strings(&binding["dependence"]["bases"], "bases")?.contains(&"expected-value");
        if !has_expected_value_basis {
            return Err(invalid_binding(
                "candidate or shared expected values require shared dependence",
                binding_id,
            ));
        }
    }
    Ok(())
}

fn validate_independence_references(
    binding: &Value,
    proofs: &ProofIndex<'_>,
) -> Result<(), VerificationError> {
    for (field, diagnostic) in [
        ("evidenceRefs", "dependence.evidenceRefs"),
        ("candidateAncestryRefs", "dependence.candidateAncestryRefs"),
        ("oracleAncestryRefs", "dependence.oracleAncestryRefs"),
    ] {
        for reference in array(&binding["dependence"], field)? {
            require_reference(reference, diagnostic, "source", proofs)?;
        }
    }
    Ok(())
}

struct BindingIdentities {
    oracle: Vec<u8>,
    executable: Vec<u8>,
    comparison_procedure: Vec<u8>,
}

struct BindingIdentityCounts<'a> {
    oracle: BTreeMap<&'a [u8], usize>,
    executable: BTreeMap<&'a [u8], usize>,
    comparison_procedure: BTreeMap<&'a [u8], usize>,
}

impl<'a> BindingIdentityCounts<'a> {
    fn new(identities: &'a [BindingIdentities]) -> Self {
        let mut counts = Self {
            oracle: BTreeMap::new(),
            executable: BTreeMap::new(),
            comparison_procedure: BTreeMap::new(),
        };
        for identity in identities {
            increment(&mut counts.oracle, &identity.oracle);
            increment(&mut counts.executable, &identity.executable);
            increment(
                &mut counts.comparison_procedure,
                &identity.comparison_procedure,
            );
        }
        counts
    }

    fn is_shared(&self, identity: &BindingIdentities) -> bool {
        repeated(&self.oracle, &identity.oracle)
            || repeated(&self.executable, &identity.executable)
            || repeated(&self.comparison_procedure, &identity.comparison_procedure)
    }
}

fn increment<'a>(counts: &mut BTreeMap<&'a [u8], usize>, identity: &'a [u8]) {
    *counts.entry(identity).or_default() += 1;
}

fn repeated(counts: &BTreeMap<&[u8], usize>, identity: &[u8]) -> bool {
    counts.get(identity).copied().unwrap_or_default() > 1
}

fn binding_identities(binding: &Value) -> Result<BindingIdentities, VerificationError> {
    Ok(BindingIdentities {
        oracle: jcs_canonicalize(&binding["oracleRef"])?,
        executable: jcs_canonicalize(&binding["executableRef"])?,
        comparison_procedure: jcs_canonicalize(&binding["comparisonProcedureRef"])?,
    })
}

fn canonical_reference_set(
    value: &Value,
    field: &str,
) -> Result<BTreeSet<Vec<u8>>, VerificationError> {
    array_value(value, field)?
        .iter()
        .map(jcs_canonicalize)
        .collect()
}

fn require_reference(
    reference: &Value,
    field: &str,
    expected_role: &str,
    proofs: &ProofIndex<'_>,
) -> Result<(), VerificationError> {
    let canonical = jcs_canonicalize(reference)?;
    let Some(actual_role) = proofs.get(canonical.as_slice()) else {
        return Err(invalid_binding(
            "oracle binding reference was not accepted by the strict reader",
            field,
        )
        .with_context("expected_role", expected_role));
    };
    if *actual_role != expected_role {
        return Err(invalid_binding(
            "oracle binding reference has the wrong validated role",
            field,
        )
        .with_context("actual_role", *actual_role)
        .with_context("expected_role", expected_role));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VisitState {
    Visiting,
    Done,
}

fn validate_derived_graph(bindings: &[Value]) -> Result<(), VerificationError> {
    let binding_ids = bindings
        .iter()
        .map(|binding| string(binding, "bindingId"))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut sources = BTreeMap::new();
    for binding in bindings {
        if binding["dependence"]["class"] == "derived" {
            let binding_id = string(binding, "bindingId")?;
            let source = string(&binding["dependence"], "sourceBindingId")?;
            if binding_id == source || !binding_ids.contains(source) {
                return Err(invalid_manifest(
                    "derived oracle source is absent or self-referential",
                    binding_id,
                ));
            }
            sources.insert(binding_id, source);
        }
    }

    let mut states = BTreeMap::new();
    for start in sources.keys() {
        if states.get(start).copied() == Some(VisitState::Done) {
            continue;
        }
        let mut path = Vec::new();
        let mut cursor = Some(*start);
        while let Some(binding_id) = cursor {
            match states.get(binding_id).copied() {
                None => {
                    states.insert(binding_id, VisitState::Visiting);
                    path.push(binding_id);
                    cursor = sources.get(binding_id).copied();
                }
                Some(VisitState::Visiting) => {
                    return Err(invalid_manifest(
                        "derived oracle binding graph contains a cycle",
                        start,
                    ));
                }
                Some(VisitState::Done) => break,
            }
        }
        for binding_id in path {
            states.insert(binding_id, VisitState::Done);
        }
    }
    Ok(())
}

fn validate_coverage(
    bindings: &[Value],
    gaps: &[Value],
    guarantees: &BTreeSet<String>,
) -> Result<Vec<QualificationOracleGap>, VerificationError> {
    let mut bound = BTreeSet::new();
    for binding in bindings {
        bound.insert(string(binding, "guaranteeId")?);
    }
    let mut gap_values = BTreeMap::new();
    for gap in gaps {
        let guarantee_id = string(gap, "guaranteeId")?;
        if !guarantees.contains(guarantee_id) {
            return Err(invalid_manifest(
                "oracle gap names an unknown qualification guarantee",
                guarantee_id,
            ));
        }
        if bound.contains(guarantee_id) {
            return Err(invalid_manifest(
                "qualification guarantee is both bound and gapped",
                guarantee_id,
            ));
        }
        if gap_values.insert(guarantee_id, gap).is_some() {
            return Err(invalid_manifest(
                "qualification guarantee has more than one oracle gap",
                guarantee_id,
            ));
        }
    }
    let represented = bound
        .iter()
        .copied()
        .chain(gap_values.keys().copied())
        .collect::<BTreeSet<_>>();
    if represented != guarantees.iter().map(String::as_str).collect() {
        return Err(invalid_manifest(
            "every qualification guarantee requires bindings or one owned gap",
            "guarantees",
        ));
    }

    gap_values
        .into_values()
        .map(|gap| {
            Ok(QualificationOracleGap {
                guarantee_id: Box::from(string(gap, "guaranteeId")?),
                owner: Box::from(string(gap, "owner")?),
                reason: Box::from(string(gap, "reason")?),
                limitations: strings(&gap["limitations"], "limitations")?
                    .into_iter()
                    .map(Box::<str>::from)
                    .collect(),
            })
        })
        .collect()
}

fn normalize_set_like_members(value: &mut Value) -> Result<(), VerificationError> {
    for binding in array_mut(value, "bindings")? {
        sort_strings(&mut binding["limitations"], "limitations")?;
        match binding["dependence"]["class"].as_str() {
            Some("independent") => {
                sort_values(
                    &mut binding["dependence"]["evidenceRefs"],
                    "dependence.evidenceRefs",
                )?;
                sort_values(
                    &mut binding["dependence"]["candidateAncestryRefs"],
                    "dependence.candidateAncestryRefs",
                )?;
                sort_values(
                    &mut binding["dependence"]["oracleAncestryRefs"],
                    "dependence.oracleAncestryRefs",
                )?;
            }
            Some("shared") => sort_strings(&mut binding["dependence"]["bases"], "bases")?,
            Some("derived" | "unknown") => {}
            _ => {
                return Err(invalid_manifest(
                    "oracle dependence has an unknown class",
                    "dependence.class",
                ));
            }
        }
    }
    for gap in array_mut(value, "gaps")? {
        sort_strings(&mut gap["limitations"], "limitations")?;
    }
    array_mut(value, "bindings")?
        .sort_by(|left, right| left["bindingId"].as_str().cmp(&right["bindingId"].as_str()));
    array_mut(value, "gaps")?
        .sort_by(|left, right| left["gapId"].as_str().cmp(&right["gapId"].as_str()));
    Ok(())
}

fn sort_strings(value: &mut Value, field: &str) -> Result<(), VerificationError> {
    let values = value
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))?;
    values.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
    Ok(())
}

fn sort_values(value: &mut Value, field: &str) -> Result<(), VerificationError> {
    let values = value
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))?;
    let mut keyed = values
        .drain(..)
        .map(|item| Ok((jcs_canonicalize(&item)?, item)))
        .collect::<Result<Vec<_>, VerificationError>>()?;
    keyed.sort_by(|left, right| left.0.cmp(&right.0));
    values.extend(keyed.into_iter().map(|(_, item)| item));
    Ok(())
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a [Value], VerificationError> {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))
}

fn array_value<'a>(value: &'a Value, field: &str) -> Result<&'a [Value], VerificationError> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))
}

fn array_mut<'a>(
    value: &'a mut Value,
    field: &str,
) -> Result<&'a mut Vec<Value>, VerificationError> {
    value[field]
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str, VerificationError> {
    value[field]
        .as_str()
        .ok_or_else(|| invalid_manifest("schema-required value is not a string", field))
}

fn strings<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, VerificationError> {
    value
        .as_array()
        .ok_or_else(|| invalid_manifest("schema-required value is not an array", field))?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| invalid_manifest("array member is not a string", field))
        })
        .collect()
}

fn invalid_manifest(message: &str, value: &str) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidQualificationOracleManifest,
        message,
    )
    .with_context("value", value)
}

fn invalid_binding(message: &str, value: &str) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidQualificationOracleBinding,
        message,
    )
    .with_context("value", value)
}
