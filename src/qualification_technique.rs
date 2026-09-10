// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Frozen qualification-technique manifests and deterministic case identities.
//!
//! Governing requirements: FR-022 and FR-025.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

use crate::contracts::{
    MAX_ARRAY_ITEMS, VerificationError, VerificationErrorCode, jcs_canonicalize, jcs_sha256,
    validate_schema_definition, wire,
};
use crate::qualification_corpus::FrozenQualificationCorpus;
use crate::qualification_oracle::FrozenQualificationOracleManifest;
use crate::qualification_profile::FrozenQualificationProfile;
use crate::shared_reference::ValidatedSharedReference;

/// One owned evidence obligation for which no accepted technique configuration exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationTechniqueGap {
    obligation_id: Box<str>,
    owner: Box<str>,
    reason: Box<str>,
    limitations: Box<[Box<str>]>,
    admission_rules: Box<[Box<str>]>,
}

impl QualificationTechniqueGap {
    /// Returns the exact uncovered evidence obligation.
    #[must_use]
    pub const fn obligation_id(&self) -> &str {
        &self.obligation_id
    }

    /// Returns the owner responsible for closing or accepting the gap.
    #[must_use]
    pub const fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns why no accepted technique configuration is available.
    #[must_use]
    pub const fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns the exact limitations on the missing technique evidence.
    #[must_use]
    pub const fn limitations(&self) -> &[Box<str>] {
        &self.limitations
    }

    /// Returns the exact native source-profile rules that remain unadmitted.
    #[must_use]
    pub const fn admission_rules(&self) -> &[Box<str>] {
        &self.admission_rules
    }
}

/// One immutable, schema-derived qualification-technique manifest.
#[derive(Clone, Debug)]
pub struct FrozenQualificationTechniqueManifest {
    manifest: wire::QualificationTechniqueManifest,
    identity: Box<str>,
}

impl FrozenQualificationTechniqueManifest {
    /// Returns the normalized schema-derived manifest.
    #[must_use]
    pub const fn manifest(&self) -> &wire::QualificationTechniqueManifest {
        &self.manifest
    }

    /// Returns the RFC 8785 identity of the complete normalized manifest.
    #[must_use]
    pub const fn identity(&self) -> &str {
        &self.identity
    }
}

/// One immutable schema-derived next-case or shrink request.
#[derive(Clone, Debug)]
pub struct FrozenQualificationCase {
    request: wire::QualificationCaseRequest,
    identity: Box<str>,
}

impl FrozenQualificationCase {
    /// Returns the normalized schema-derived request.
    #[must_use]
    pub const fn request(&self) -> &wire::QualificationCaseRequest {
        &self.request
    }

    /// Returns the RFC 8785 identity of the complete normalized request.
    #[must_use]
    pub const fn identity(&self) -> &str {
        &self.identity
    }
}

/// Result of freezing all technique configurations for one qualification profile.
#[derive(Clone, Debug)]
pub enum QualificationTechniqueOutcome {
    /// Every evidence obligation has one accepted configuration.
    Complete(Box<FrozenQualificationTechniqueManifest>),
    /// Every unconfigured evidence obligation remains an exact owned gap.
    Incomplete {
        /// The immutable manifest, including its owned gaps.
        manifest: Box<FrozenQualificationTechniqueManifest>,
        /// The owned gaps in obligation-identifier order.
        gaps: Box<[QualificationTechniqueGap]>,
    },
}

/// Validates and freezes one technique manifest against exact qualification inputs.
///
/// Set-like members are normalized before hashing. Corpus projections remain ordered.
///
/// # Errors
/// Returns a schema, identity, reference, catalog, coverage, or family refusal when the
/// manifest cannot support exact qualification evidence.
pub fn qualify_technique_manifest(
    mut value: Value,
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    oracle_manifest: &FrozenQualificationOracleManifest,
    catalog: &Value,
    validated_references: &[ValidatedSharedReference],
) -> Result<QualificationTechniqueOutcome, VerificationError> {
    validate_schema_definition("QualificationTechniqueManifest", &value).map_err(|error| {
        invalid_manifest(
            "technique manifest failed schema validation",
            &error.to_string(),
        )
    })?;
    validate_schema_definition("QuoinCatalogFixture", catalog).map_err(|error| {
        unavailable(
            "accepted catalog failed schema validation",
            &error.to_string(),
        )
    })?;
    bind_inputs(&value, profile, corpus, oracle_manifest, catalog)?;
    normalize_set_like_members(&mut value)?;

    if validated_references.len() > MAX_ARRAY_ITEMS {
        return Err(invalid_manifest(
            "strict-reader proof count exceeds the technique-manifest boundary",
            "validated_references",
        ));
    }
    let proofs = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::reference_key)
        .collect::<BTreeMap<_, _>>();
    validate_catalog_source(catalog, &proofs)?;

    let requirements = profile_requirements(profile)?;
    let guarantees = profile_guarantees(profile)?;
    let corpus_entries = corpus_entries(corpus)?;
    let oracle_bindings = oracle_binding_ids(oracle_manifest)?;
    let methods = catalog_methods(catalog)?;
    let configurations = array(&value, "configurations")?;
    let gaps = array(&value, "gaps")?;

    validate_unique_record_ids(configurations, gaps)?;
    validate_configurations(
        configurations,
        &requirements,
        &guarantees,
        &corpus_entries,
        &oracle_bindings,
        &methods,
        &proofs,
    )?;
    let owned_gaps = validate_coverage(configurations, gaps, &requirements, &guarantees)?;

    let identity = jcs_sha256(&value)?.into_boxed_str();
    let manifest = serde_json::from_value(value).map_err(|error| {
        invalid_manifest(
            "QualificationTechniqueManifest schema-derived decoding failed",
            &error.to_string(),
        )
    })?;
    let frozen = Box::new(FrozenQualificationTechniqueManifest { manifest, identity });
    if owned_gaps.is_empty() {
        Ok(QualificationTechniqueOutcome::Complete(frozen))
    } else {
        Ok(QualificationTechniqueOutcome::Incomplete {
            manifest: frozen,
            gaps: owned_gaps.into_boxed_slice(),
        })
    }
}

/// Plans one deterministic next-case or shrink request for a frozen configuration.
///
/// `case_ordinal` is a canonical unsigned decimal value, zero-based and below the
/// selected technique's declared case limit. Prior observations are set-like; the
/// generated-artifact shrink path is ordered and duplicate-preserving.
///
/// # Errors
/// Returns `invalid_qualification_case` when the configuration, ordinal, exact reference,
/// or bounded collection differs from the frozen manifest.
pub fn plan_qualification_case(
    manifest: &FrozenQualificationTechniqueManifest,
    configuration_id: &str,
    case_ordinal: &str,
    prior_observation_refs: &[Value],
    shrink_path: &[Value],
    validated_references: &[ValidatedSharedReference],
) -> Result<FrozenQualificationCase, VerificationError> {
    if prior_observation_refs.len() > MAX_ARRAY_ITEMS || shrink_path.len() > MAX_ARRAY_ITEMS {
        return Err(invalid_case(
            "case history exceeds the qualification collection boundary",
            configuration_id,
        ));
    }
    let ordinal = canonical_u64(case_ordinal).map_err(|()| {
        invalid_case(
            "case ordinal is not canonical unsigned decimal",
            case_ordinal,
        )
    })?;
    let value = serde_json::to_value(manifest.manifest()).map_err(|error| {
        invalid_case(
            "frozen technique manifest could not be serialized",
            &error.to_string(),
        )
    })?;
    let configuration = array(&value, "configurations")?
        .iter()
        .find(|configuration| configuration["configurationId"].as_str() == Some(configuration_id))
        .ok_or_else(|| invalid_case("case names no frozen configuration", configuration_id))?;
    if ordinal >= configuration_case_limit(configuration)? {
        return Err(invalid_case(
            "case ordinal reaches or exceeds the frozen configuration limit",
            case_ordinal,
        ));
    }

    let proofs = validated_references
        .iter()
        .filter_map(ValidatedSharedReference::reference_key)
        .collect::<BTreeMap<_, _>>();
    let mut prior = Vec::with_capacity(prior_observation_refs.len());
    let mut prior_keys = BTreeSet::new();
    for reference in prior_observation_refs {
        validate_bound_reference(reference, "priorObservationRefs", "observation", &proofs)
            .map_err(|error| invalid_case("prior observation is not exact", &error.to_string()))?;
        let key = jcs_canonicalize(reference)?;
        if !prior_keys.insert(key.clone()) {
            return Err(invalid_case(
                "case repeats a prior observation reference",
                configuration_id,
            ));
        }
        prior.push((key, reference.clone()));
    }
    prior.sort_by(|(left, _), (right, _)| left.cmp(right));

    for reference in shrink_path {
        validate_bound_reference(reference, "shrinkPath", "generated-artifact", &proofs)
            .map_err(|error| invalid_case("shrink step is not exact", &error.to_string()))?;
    }

    let request = json!({
        "contractVersion": "quire-verification/e02-draft-2",
        "schemaVersion": "quire-verification.qualification-case-request/v1",
        "kind": "QualificationCaseRequest",
        "manifestIdentity": manifest.identity(),
        "configurationId": configuration_id,
        "configurationIdentity": jcs_sha256(configuration)?,
        "caseOrdinal": case_ordinal,
        "priorObservationRefs": prior.into_iter().map(|(_, value)| value).collect::<Vec<_>>(),
        "shrinkPath": shrink_path,
    });
    validate_schema_definition("QualificationCaseRequest", &request).map_err(|error| {
        invalid_case(
            "qualification case request failed schema validation",
            &error.to_string(),
        )
    })?;
    let identity = jcs_sha256(&request)?.into_boxed_str();
    let request = serde_json::from_value(request).map_err(|error| {
        invalid_case(
            "QualificationCaseRequest schema-derived decoding failed",
            &error.to_string(),
        )
    })?;
    Ok(FrozenQualificationCase { request, identity })
}

#[derive(Clone, Debug)]
struct RequirementBinding {
    guarantee_id: String,
    technique_id: String,
    budget_id: String,
    stopping_rule: String,
}

#[derive(Clone, Debug)]
struct GuaranteeBinding {
    authority_kind: String,
    admission_gaps: BTreeSet<String>,
}

fn bind_inputs(
    manifest: &Value,
    profile: &FrozenQualificationProfile,
    corpus: &FrozenQualificationCorpus,
    oracle_manifest: &FrozenQualificationOracleManifest,
    catalog: &Value,
) -> Result<(), VerificationError> {
    for (field, expected) in [
        ("profileIdentity", profile.identity()),
        ("corpusIdentity", corpus.identity()),
        ("oracleManifestIdentity", oracle_manifest.identity()),
    ] {
        if manifest[field].as_str() != Some(expected) {
            return Err(invalid_manifest(
                "technique manifest names a different frozen input",
                field,
            ));
        }
    }
    let catalog_identity = jcs_sha256(catalog)?;
    if manifest["catalogIdentity"].as_str() != Some(catalog_identity.as_str()) {
        return Err(invalid_manifest(
            "technique manifest names a different accepted catalog",
            "catalogIdentity",
        ));
    }
    Ok(())
}

fn profile_requirements(
    profile: &FrozenQualificationProfile,
) -> Result<BTreeMap<String, RequirementBinding>, VerificationError> {
    let profile = serde_json::to_value(profile.profile()).map_err(|error| {
        invalid_manifest(
            "frozen qualification profile could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&profile, "evidenceRequirements")?
        .iter()
        .map(|requirement| {
            let obligation_id = string(requirement, "obligationId")?.to_owned();
            Ok((
                obligation_id,
                RequirementBinding {
                    guarantee_id: string(requirement, "guaranteeId")?.to_owned(),
                    technique_id: string(requirement, "techniqueId")?.to_owned(),
                    budget_id: string(requirement, "budgetId")?.to_owned(),
                    stopping_rule: string(requirement, "stoppingRule")?.to_owned(),
                },
            ))
        })
        .collect()
}

fn profile_guarantees(
    profile: &FrozenQualificationProfile,
) -> Result<BTreeMap<String, GuaranteeBinding>, VerificationError> {
    let profile = serde_json::to_value(profile.profile()).map_err(|error| {
        invalid_manifest(
            "frozen qualification profile could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&profile, "semanticGuarantees")?
        .iter()
        .map(|guarantee| {
            let authority = &guarantee["claimAuthority"];
            let authority_kind = string(authority, "kind")?.to_owned();
            let admission_gaps = if authority_kind == "native-formal-clause" {
                array(authority, "admissionResults")?
                    .iter()
                    .filter(|result| result["status"] == "gap")
                    .map(|result| string(result, "rule").map(str::to_owned))
                    .collect::<Result<BTreeSet<_>, _>>()?
            } else {
                BTreeSet::new()
            };
            Ok((
                string(guarantee, "guaranteeId")?.to_owned(),
                GuaranteeBinding {
                    authority_kind,
                    admission_gaps,
                },
            ))
        })
        .collect()
}

fn corpus_entries(
    corpus: &FrozenQualificationCorpus,
) -> Result<BTreeSet<String>, VerificationError> {
    let corpus = serde_json::to_value(corpus.manifest()).map_err(|error| {
        invalid_manifest(
            "frozen qualification corpus could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&corpus, "entries")?
        .iter()
        .map(|entry| string(entry, "entryId").map(str::to_owned))
        .collect()
}

fn oracle_binding_ids(
    manifest: &FrozenQualificationOracleManifest,
) -> Result<BTreeSet<String>, VerificationError> {
    let manifest = serde_json::to_value(manifest.manifest()).map_err(|error| {
        invalid_manifest(
            "frozen qualification oracle manifest could not be serialized",
            &error.to_string(),
        )
    })?;
    array(&manifest, "bindings")?
        .iter()
        .map(|binding| string(binding, "bindingId").map(str::to_owned))
        .collect()
}

fn catalog_methods(catalog: &Value) -> Result<BTreeSet<String>, VerificationError> {
    if !array(&catalog["catalog"], "duplicates")?.is_empty()
        || !array(&catalog["catalog"], "unreadable")?.is_empty()
    {
        return Err(unavailable(
            "accepted catalog has unresolved source diagnostics",
            "catalog",
        ));
    }
    let mut methods = BTreeSet::new();
    for method in array(&catalog["catalog"], "methods")? {
        let method_id = string(method, "id")?;
        if !methods.insert(method_id.to_owned()) {
            return Err(unavailable(
                "accepted catalog repeats a technique identifier",
                method_id,
            ));
        }
    }
    Ok(methods)
}

fn validate_catalog_source(
    catalog: &Value,
    proofs: &BTreeMap<&[u8], &str>,
) -> Result<(), VerificationError> {
    validate_bound_reference(
        &catalog["source"]["sourceRef"],
        "catalog.source.sourceRef",
        "source",
        proofs,
    )
    .map_err(|error| unavailable("catalog source reference is not exact", &error.to_string()))
}

fn validate_configurations(
    configurations: &[Value],
    requirements: &BTreeMap<String, RequirementBinding>,
    guarantees: &BTreeMap<String, GuaranteeBinding>,
    corpus_entries: &BTreeSet<String>,
    oracle_bindings: &BTreeSet<String>,
    methods: &BTreeSet<String>,
    proofs: &BTreeMap<&[u8], &str>,
) -> Result<(), VerificationError> {
    for configuration in configurations {
        let obligation_id = string(configuration, "obligationId")?;
        let Some(requirement) = requirements.get(obligation_id) else {
            return Err(invalid_manifest(
                "technique configuration names an unknown evidence obligation",
                obligation_id,
            ));
        };
        for (field, actual, expected) in [
            (
                "guaranteeId",
                string(configuration, "guaranteeId")?,
                requirement.guarantee_id.as_str(),
            ),
            (
                "techniqueId",
                string(configuration, "techniqueId")?,
                requirement.technique_id.as_str(),
            ),
            (
                "budgetId",
                string(configuration, "budgetId")?,
                requirement.budget_id.as_str(),
            ),
            (
                "stoppingRule",
                string(configuration, "stoppingRule")?,
                requirement.stopping_rule.as_str(),
            ),
        ] {
            if actual != expected {
                return Err(invalid_manifest(
                    "technique configuration differs from its frozen obligation",
                    field,
                ));
            }
        }
        let method = string(configuration, "techniqueId")?;
        if !methods.contains(method) {
            return Err(unavailable(
                "frozen obligation names no accepted catalog method",
                method,
            ));
        }
        let technique = &configuration["technique"];
        let family = string(technique, "family")?;
        if technique_family(method) != Some(family) {
            return Err(invalid_manifest(
                "catalog method and technique family disagree",
                method,
            ));
        }
        let guarantee_id = string(configuration, "guaranteeId")?;
        let guarantee = guarantees.get(guarantee_id).ok_or_else(|| {
            invalid_manifest(
                "technique configuration names an unknown guarantee",
                guarantee_id,
            )
        })?;
        if !guarantee.admission_gaps.is_empty() {
            return Err(invalid_manifest(
                "technique configuration claims an obligation with native admission gaps",
                guarantee_id,
            ));
        }
        if guarantee.authority_kind == "export-mapping" && family == "differential" {
            return Err(unavailable(
                "export mapping cannot use an upstream differential oracle",
                guarantee_id,
            ));
        }
        validate_configuration_references(configuration, proofs)?;
        validate_projection(configuration, corpus_entries)?;
        validate_technique(technique, corpus_entries, oracle_bindings)?;
    }
    Ok(())
}

fn validate_configuration_references(
    configuration: &Value,
    proofs: &BTreeMap<&[u8], &str>,
) -> Result<(), VerificationError> {
    for (field, role) in [
        ("propertyRef", "property"),
        ("harnessRef", "executable-projection"),
        ("requestTemplateRef", "invocation"),
    ] {
        validate_bound_reference(&configuration[field], field, role, proofs)?;
    }
    let technique = &configuration["technique"];
    match string(technique, "family")? {
        "example" => {}
        "property" => {
            validate_bound_reference(
                &technique["generatorRef"],
                "generatorRef",
                "executable-projection",
                proofs,
            )?;
            validate_bound_reference(
                &technique["shrinkerRef"],
                "shrinkerRef",
                "executable-projection",
                proofs,
            )?;
        }
        "metamorphic" => {
            validate_bound_reference(
                &technique["relationProcedureRef"],
                "relationProcedureRef",
                "executable-projection",
                proofs,
            )?;
            validate_bound_reference(
                &technique["transformRef"],
                "transformRef",
                "executable-projection",
                proofs,
            )?;
        }
        "differential" => validate_bound_reference(
            &technique["comparisonProcedureRef"],
            "comparisonProcedureRef",
            "executable-projection",
            proofs,
        )?,
        "fuzz" => {
            validate_bound_reference(
                &technique["targetRef"],
                "targetRef",
                "executable-projection",
                proofs,
            )?;
            validate_bound_reference(
                &technique["minimizerRef"],
                "minimizerRef",
                "executable-projection",
                proofs,
            )?;
        }
        "mutation" => {
            for mutant in array(technique, "mutants")? {
                validate_bound_reference(&mutant["mutantRef"], "mutantRef", "fault-model", proofs)?;
                validate_bound_reference(
                    &mutant["assignedTestRef"],
                    "assignedTestRef",
                    "trace",
                    proofs,
                )?;
            }
        }
        family => {
            return Err(invalid_manifest(
                "technique configuration has an unknown family",
                family,
            ));
        }
    }
    Ok(())
}

fn validate_projection(
    configuration: &Value,
    corpus_entries: &BTreeSet<String>,
) -> Result<(), VerificationError> {
    for entry_id in strings(&configuration["inputProjection"]["entryIds"], "entryIds")? {
        if !corpus_entries.contains(entry_id) {
            return Err(invalid_manifest(
                "technique configuration projects an unretained corpus entry",
                entry_id,
            ));
        }
    }
    Ok(())
}

fn validate_technique(
    technique: &Value,
    corpus_entries: &BTreeSet<String>,
    oracle_bindings: &BTreeSet<String>,
) -> Result<(), VerificationError> {
    match string(technique, "family")? {
        "property" | "metamorphic" | "differential" | "fuzz" => {
            let seed = string(technique, "seed")?;
            canonical_u64(seed).map_err(|()| {
                invalid_manifest(
                    "technique seed exceeds canonical unsigned 64-bit range",
                    seed,
                )
            })?;
        }
        "example" | "mutation" => {}
        family => {
            return Err(invalid_manifest(
                "technique configuration has an unknown family",
                family,
            ));
        }
    }
    if technique["family"] == "differential" {
        for binding_id in strings(&technique["oracleBindingIds"], "oracleBindingIds")? {
            if !oracle_bindings.contains(binding_id) {
                return Err(invalid_manifest(
                    "differential technique names no frozen oracle binding",
                    binding_id,
                ));
            }
        }
    }
    if technique["family"] == "fuzz" {
        for entry_id in strings(&technique["seedCorpusEntryIds"], "seedCorpusEntryIds")? {
            if !corpus_entries.contains(entry_id) {
                return Err(invalid_manifest(
                    "fuzz technique names an unretained seed corpus entry",
                    entry_id,
                ));
            }
        }
    }
    if technique["family"] == "mutation" {
        let mut mutant_ids = BTreeSet::new();
        for mutant in array(technique, "mutants")? {
            let mutant_id = string(mutant, "mutantId")?;
            if !mutant_ids.insert(mutant_id) {
                return Err(invalid_manifest(
                    "mutation technique repeats a mutant identifier",
                    mutant_id,
                ));
            }
        }
    }
    Ok(())
}

fn validate_unique_record_ids(
    configurations: &[Value],
    gaps: &[Value],
) -> Result<(), VerificationError> {
    let mut identifiers = BTreeSet::new();
    for (record, field) in configurations
        .iter()
        .map(|configuration| (configuration, "configurationId"))
        .chain(gaps.iter().map(|gap| (gap, "gapId")))
    {
        let identifier = string(record, field)?;
        if !identifiers.insert(identifier) {
            return Err(invalid_manifest(
                "duplicate technique configuration or gap identifier",
                identifier,
            ));
        }
    }
    Ok(())
}

fn validate_coverage(
    configurations: &[Value],
    gaps: &[Value],
    requirements: &BTreeMap<String, RequirementBinding>,
    guarantees: &BTreeMap<String, GuaranteeBinding>,
) -> Result<Vec<QualificationTechniqueGap>, VerificationError> {
    let mut coverage = BTreeMap::<&str, usize>::new();
    for record in configurations.iter().chain(gaps) {
        let obligation_id = string(record, "obligationId")?;
        if !requirements.contains_key(obligation_id) {
            return Err(invalid_manifest(
                "technique manifest names an unknown evidence obligation",
                obligation_id,
            ));
        }
        *coverage.entry(obligation_id).or_default() += 1;
    }
    for obligation_id in requirements.keys() {
        if coverage.get(obligation_id.as_str()) != Some(&1) {
            return Err(invalid_manifest(
                "evidence obligation lacks exactly one configuration or owned gap",
                obligation_id,
            ));
        }
    }
    for gap in gaps {
        let obligation_id = string(gap, "obligationId")?;
        let requirement = requirements.get(obligation_id).ok_or_else(|| {
            invalid_manifest("technique gap names an unknown obligation", obligation_id)
        })?;
        let guarantee = guarantees.get(&requirement.guarantee_id).ok_or_else(|| {
            invalid_manifest(
                "technique gap names an obligation with an unknown guarantee",
                obligation_id,
            )
        })?;
        let actual = strings(&gap["admissionRules"], "admissionRules")?
            .into_iter()
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        if actual != guarantee.admission_gaps {
            return Err(invalid_manifest(
                "technique gap does not reproduce the frozen native admission gaps",
                obligation_id,
            ));
        }
    }
    gaps.iter().map(gap_from_value).collect()
}

fn gap_from_value(value: &Value) -> Result<QualificationTechniqueGap, VerificationError> {
    let limitations = strings(&value["limitations"], "limitations")?
        .into_iter()
        .map(|item| item.to_owned().into_boxed_str())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let admission_rules = strings(&value["admissionRules"], "admissionRules")?
        .into_iter()
        .map(|item| item.to_owned().into_boxed_str())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    Ok(QualificationTechniqueGap {
        obligation_id: string(value, "obligationId")?.to_owned().into_boxed_str(),
        owner: string(value, "owner")?.to_owned().into_boxed_str(),
        reason: string(value, "reason")?.to_owned().into_boxed_str(),
        limitations,
        admission_rules,
    })
}

fn normalize_set_like_members(value: &mut Value) -> Result<(), VerificationError> {
    sort_records(value, "configurations", "configurationId")?;
    sort_records(value, "gaps", "gapId")?;
    for configuration in value["configurations"]
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("configurations are not an array", "configurations"))?
    {
        sort_strings(configuration, "limitations")?;
        let technique = &mut configuration["technique"];
        match technique["family"].as_str() {
            Some("differential") => sort_strings(technique, "oracleBindingIds")?,
            Some("fuzz") => sort_strings(technique, "seedCorpusEntryIds")?,
            Some("mutation") => sort_records(technique, "mutants", "mutantId")?,
            Some("example" | "property" | "metamorphic") => {}
            Some(family) => {
                return Err(invalid_manifest(
                    "technique configuration has an unknown family",
                    family,
                ));
            }
            None => return Err(invalid_manifest("technique family is absent", "family")),
        }
    }
    for gap in value["gaps"]
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("gaps are not an array", "gaps"))?
    {
        sort_strings(gap, "limitations")?;
        sort_strings(gap, "admissionRules")?;
    }
    Ok(())
}

fn sort_records(value: &mut Value, field: &str, identity: &str) -> Result<(), VerificationError> {
    let values = value[field]
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("set-like member is not an array", field))?;
    values.sort_by(|left, right| left[identity].as_str().cmp(&right[identity].as_str()));
    Ok(())
}

fn sort_strings(value: &mut Value, field: &str) -> Result<(), VerificationError> {
    let values = value[field]
        .as_array_mut()
        .ok_or_else(|| invalid_manifest("set-like string member is not an array", field))?;
    values.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
    Ok(())
}

fn configuration_case_limit(configuration: &Value) -> Result<u64, VerificationError> {
    let technique = &configuration["technique"];
    match string(technique, "family")? {
        "example" => {
            usize_to_u64(strings(&configuration["inputProjection"]["entryIds"], "entryIds")?.len())
        }
        "property" | "metamorphic" | "differential" => technique["maximumCases"]
            .as_u64()
            .ok_or_else(|| invalid_case("maximumCases is not unsigned", "maximumCases")),
        "fuzz" => technique["maximumExecutions"]
            .as_u64()
            .ok_or_else(|| invalid_case("maximumExecutions is not unsigned", "maximumExecutions")),
        "mutation" => usize_to_u64(array(technique, "mutants")?.len()),
        family => Err(invalid_case(
            "case configuration has an unknown family",
            family,
        )),
    }
}

fn usize_to_u64(value: usize) -> Result<u64, VerificationError> {
    u64::try_from(value)
        .map_err(|error| invalid_case("case limit does not fit u64", &error.to_string()))
}

fn canonical_u64(value: &str) -> Result<u64, ()> {
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(());
    }
    value.parse().map_err(|_| ())
}

fn technique_family(method: &str) -> Option<&'static str> {
    match method {
        "bdd-spec-by-example" => Some("example"),
        "property-based-testing" => Some("property"),
        "metamorphic-testing" => Some("metamorphic"),
        "differential-testing" => Some("differential"),
        "fuzzing" => Some("fuzz"),
        "mutation-testing" => Some("mutation"),
        _ => None,
    }
}

fn validate_bound_reference(
    reference: &Value,
    field: &str,
    expected_role: &str,
    validated_references: &BTreeMap<&[u8], &str>,
) -> Result<(), VerificationError> {
    let canonical = jcs_canonicalize(reference)?;
    let Some(actual_role) = validated_references.get(canonical.as_slice()) else {
        return Err(invalid_manifest(
            "technique reference was not accepted by the strict reader",
            field,
        ));
    };
    if *actual_role != expected_role {
        return Err(invalid_manifest(
            "technique reference has the wrong validated role",
            field,
        ));
    }
    Ok(())
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a [Value], VerificationError> {
    value[field]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid_manifest("required member is not an array", field))
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str, VerificationError> {
    value[field]
        .as_str()
        .ok_or_else(|| invalid_manifest("required member is not a string", field))
}

fn strings<'a>(value: &'a Value, field: &str) -> Result<Vec<&'a str>, VerificationError> {
    value
        .as_array()
        .ok_or_else(|| invalid_manifest("required member is not a string array", field))?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| invalid_manifest("string-array member is not a string", field))
        })
        .collect()
}

fn invalid_manifest(message: &str, value: &str) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::InvalidQualificationTechniqueManifest,
        message,
    )
    .with_context("value", value)
}

fn invalid_case(message: &str, value: &str) -> VerificationError {
    VerificationError::new(VerificationErrorCode::InvalidQualificationCase, message)
        .with_context("value", value)
}

fn unavailable(message: &str, value: &str) -> VerificationError {
    VerificationError::new(
        VerificationErrorCode::QualificationTechniqueUnavailable,
        message,
    )
    .with_context("value", value)
}
