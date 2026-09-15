// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Checked-domain model generation, native replay, and domain-valid shrinking.
//!
//! Governing requirements: FR-029, TC-047, and IT-005-SC-01.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::contracts::{VerificationError, VerificationErrorCode, jcs_sha256};

const MAX_MODEL_DOMAIN_DEPTH: usize = 32;

/// Specialized request contract implemented by this provider.
pub const MODEL_TEST_REQUEST_VERSION: &str = "quire.model-test-request/v1";
/// Terminal method-result contract implemented by this provider.
pub const MODEL_METHOD_RESULT_VERSION: &str = "quire.method-result/v1";

/// A value in an exact checked model domain.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum ModelValue {
    /// A bounded integer.
    Integer(i64),
    /// A Boolean value.
    Boolean(bool),
    /// One member of a checked enumeration.
    Label(String),
    /// A bounded homogeneous collection.
    Collection(Vec<ModelValue>),
}

/// A finite source-model domain, independent of Rust implementation types.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CheckedDomain {
    /// An inclusive integer interval.
    Integer {
        /// Inclusive minimum.
        minimum: i64,
        /// Inclusive maximum.
        maximum: i64,
    },
    /// The two Boolean values.
    Boolean,
    /// An explicitly ordered finite enumeration.
    Enumeration {
        /// Distinct values in checked-source order.
        values: Vec<String>,
    },
    /// A homogeneous collection with inclusive length bounds.
    Collection {
        /// Inclusive minimum item count.
        minimum_items: usize,
        /// Inclusive maximum item count.
        maximum_items: usize,
        /// Checked item domain.
        element: Box<CheckedDomain>,
    },
}

/// One named field in a checked record model.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckedField {
    /// Stable checked-source field identifier.
    pub id: String,
    /// Exact finite field domain.
    pub domain: CheckedDomain,
}

/// A closed, replayable predicate over checked record fields.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CheckedPredicate {
    /// The left integer is strictly less than the right integer.
    LessThan {
        /// Left field identifier.
        left: String,
        /// Right field identifier.
        right: String,
    },
    /// The left integer is less than or equal to the right integer.
    LessThanOrEqual {
        /// Left field identifier.
        left: String,
        /// Right field identifier.
        right: String,
    },
    /// Both fields contain the same typed value.
    Equal {
        /// Left field identifier.
        left: String,
        /// Right field identifier.
        right: String,
    },
    /// The fields contain different typed values.
    NotEqual {
        /// Left field identifier.
        left: String,
        /// Right field identifier.
        right: String,
    },
}

/// Exact checked model and all identities needed for native replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CheckedModel {
    /// Immutable checked-package identity.
    pub package_identity: String,
    /// Exact model-testing profile identity.
    pub profile_identity: String,
    /// Immutable semantic-subject identity.
    pub semantic_subject_identity: String,
    /// Exact native runtime/provider identity.
    pub native_runtime_identity: String,
    /// Exact ecosystem lock identity.
    pub ecosystem_lock_identity: String,
    /// Record fields in checked-source order.
    pub fields: Vec<CheckedField>,
    /// Preconditions compiled into domain construction.
    pub preconditions: Vec<CheckedPredicate>,
    /// Cross-field checked relationships.
    pub relationships: Vec<CheckedPredicate>,
    /// Maximum Cartesian candidates admitted during construction.
    pub construction_bound: usize,
}

impl CheckedModel {
    /// Returns whether an assignment inhabits every domain and satisfies every predicate.
    #[must_use]
    pub fn admits(&self, values: &BTreeMap<String, ModelValue>) -> bool {
        values.len() == self.fields.len()
            && self.fields.iter().all(|field| {
                field.domain.validate().is_ok()
                    && values
                        .get(&field.id)
                        .is_some_and(|value| field.domain.contains(value))
            })
            && self
                .preconditions
                .iter()
                .chain(&self.relationships)
                .all(|predicate| predicate.evaluate(values) == Some(true))
    }

    /// Compares two admitted assignments under the pinned well-founded typed order.
    #[must_use]
    pub fn strictly_smaller(
        &self,
        candidate: &BTreeMap<String, ModelValue>,
        current: &BTreeMap<String, ModelValue>,
    ) -> bool {
        if !self.admits(candidate) || !self.admits(current) {
            return false;
        }
        self.order_key(candidate)
            .zip(self.order_key(current))
            .is_some_and(|(candidate, current)| candidate < current)
    }

    fn order_key(&self, values: &BTreeMap<String, ModelValue>) -> Option<Vec<usize>> {
        self.fields
            .iter()
            .map(|field| field.domain.rank(values.get(&field.id)?))
            .collect()
    }
}

impl CheckedDomain {
    fn validate(&self) -> Result<(), VerificationError> {
        let mut pending = vec![(self, 0_usize)];
        while let Some((domain, depth)) = pending.pop() {
            if depth > MAX_MODEL_DOMAIN_DEPTH {
                return Err(model_error(
                    VerificationErrorCode::InvalidModelDomain,
                    "model domain nesting exceeds 32 levels",
                ));
            }
            match domain {
                Self::Integer { minimum, maximum } if minimum > maximum => {
                    return Err(model_error(
                        VerificationErrorCode::InvalidModelDomain,
                        "integer minimum exceeds maximum",
                    ));
                }
                Self::Enumeration { values } if values.is_empty() => {
                    return Err(model_error(
                        VerificationErrorCode::UnsatisfiableModelDomain,
                        "enumeration is empty",
                    ));
                }
                Self::Enumeration { values } => {
                    let unique = values.iter().collect::<BTreeSet<_>>();
                    if unique.len() != values.len() {
                        return Err(model_error(
                            VerificationErrorCode::InvalidModelDomain,
                            "enumeration values are not distinct",
                        ));
                    }
                }
                Self::Collection {
                    minimum_items,
                    maximum_items,
                    element,
                } => {
                    if minimum_items > maximum_items {
                        return Err(model_error(
                            VerificationErrorCode::InvalidModelDomain,
                            "collection minimum exceeds maximum",
                        ));
                    }
                    pending.push((element, depth + 1));
                }
                Self::Integer { .. } | Self::Boolean => {}
            }
        }
        Ok(())
    }

    fn values(&self, limit: usize) -> Result<Vec<ModelValue>, VerificationError> {
        self.validate()?;
        match self {
            Self::Integer { minimum, maximum } => {
                let width = i128::from(*maximum) - i128::from(*minimum) + 1;
                let width = usize::try_from(width).map_err(|_| {
                    model_error(
                        VerificationErrorCode::ModelConstructionBoundExceeded,
                        "integer domain cannot fit the construction bound",
                    )
                })?;
                if width > limit {
                    return Err(model_error(
                        VerificationErrorCode::ModelConstructionBoundExceeded,
                        "integer domain exceeds the construction bound",
                    ));
                }
                (0..width)
                    .map(|offset| {
                        let offset = i64::try_from(offset).map_err(|_| {
                            model_error(
                                VerificationErrorCode::ModelConstructionBoundExceeded,
                                "integer offset is not representable",
                            )
                        })?;
                        minimum
                            .checked_add(offset)
                            .map(ModelValue::Integer)
                            .ok_or_else(|| {
                                model_error(
                                    VerificationErrorCode::InvalidModelDomain,
                                    "integer domain overflowed",
                                )
                            })
                    })
                    .collect()
            }
            Self::Boolean => Ok(vec![ModelValue::Boolean(false), ModelValue::Boolean(true)]),
            Self::Enumeration { values } => {
                Ok(values.iter().cloned().map(ModelValue::Label).collect())
            }
            Self::Collection {
                minimum_items,
                maximum_items,
                element,
            } => {
                if *maximum_items > limit {
                    return Err(model_error(
                        VerificationErrorCode::ModelConstructionBoundExceeded,
                        "collection length exceeds the construction bound",
                    ));
                }
                let elements = element.values(limit)?;
                let mut collections = Vec::new();
                for length in *minimum_items..=*maximum_items {
                    append_sequences(&elements, length, limit, &mut collections)?;
                }
                Ok(collections
                    .into_iter()
                    .map(ModelValue::Collection)
                    .collect())
            }
        }
    }

    fn contains(&self, value: &ModelValue) -> bool {
        match (self, value) {
            (Self::Integer { minimum, maximum }, ModelValue::Integer(candidate)) => {
                candidate >= minimum && candidate <= maximum
            }
            (Self::Boolean, ModelValue::Boolean(_)) => true,
            (Self::Enumeration { values }, ModelValue::Label(candidate)) => {
                values.contains(candidate)
            }
            (
                Self::Collection {
                    minimum_items,
                    maximum_items,
                    element,
                },
                ModelValue::Collection(items),
            ) => {
                items.len() >= *minimum_items
                    && items.len() <= *maximum_items
                    && items.iter().all(|item| element.contains(item))
            }
            _ => false,
        }
    }

    fn rank(&self, value: &ModelValue) -> Option<usize> {
        match (self, value) {
            (Self::Integer { minimum, .. }, ModelValue::Integer(candidate)) => {
                usize::try_from(i128::from(*candidate) - i128::from(*minimum)).ok()
            }
            (Self::Boolean, ModelValue::Boolean(candidate)) => Some(usize::from(*candidate)),
            (Self::Enumeration { values }, ModelValue::Label(candidate)) => {
                values.iter().position(|value| value == candidate)
            }
            (Self::Collection { element, .. }, ModelValue::Collection(items)) => {
                let mut rank = items.len();
                for item in items {
                    rank = rank.checked_mul(31)?.checked_add(element.rank(item)?)?;
                }
                Some(rank)
            }
            _ => None,
        }
    }
}

impl CheckedPredicate {
    fn fields(&self) -> (&str, &str) {
        match self {
            Self::LessThan { left, right }
            | Self::LessThanOrEqual { left, right }
            | Self::Equal { left, right }
            | Self::NotEqual { left, right } => (left, right),
        }
    }

    fn evaluate(&self, values: &BTreeMap<String, ModelValue>) -> Option<bool> {
        let (left, right) = self.fields();
        let left = values.get(left)?;
        let right = values.get(right)?;
        match self {
            Self::LessThan { .. } => match (left, right) {
                (ModelValue::Integer(left), ModelValue::Integer(right)) => Some(left < right),
                _ => None,
            },
            Self::LessThanOrEqual { .. } => match (left, right) {
                (ModelValue::Integer(left), ModelValue::Integer(right)) => Some(left <= right),
                _ => None,
            },
            Self::Equal { .. } => Some(left == right),
            Self::NotEqual { .. } => Some(left != right),
        }
    }

    fn is_boundary(&self, values: &BTreeMap<String, ModelValue>) -> bool {
        let (left, right) = self.fields();
        let Some(left) = values.get(left) else {
            return false;
        };
        let Some(right) = values.get(right) else {
            return false;
        };
        match self {
            Self::LessThan { .. } => match (left, right) {
                (ModelValue::Integer(left), ModelValue::Integer(right)) => {
                    left.checked_add(1) == Some(*right)
                }
                _ => false,
            },
            Self::LessThanOrEqual { .. } | Self::Equal { .. } => left == right,
            Self::NotEqual { .. } => match (left, right) {
                (ModelValue::Integer(left), ModelValue::Integer(right)) => {
                    left.abs_diff(*right) == 1
                }
                _ => left != right,
            },
        }
    }
}

/// Deterministic model-test request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModelTestRequest {
    /// Exact specialized request contract version.
    pub contract_version: String,
    /// Checked model and exact replay identities.
    pub model: CheckedModel,
    /// Seed used only to order the already-admitted domain.
    pub seed: u64,
    /// Maximum generated admitted cases and refusal probes.
    pub case_bound: usize,
    /// Maximum proposed shrink replays.
    pub shrink_bound: usize,
}

/// A validated model case that may be sent to native replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdmittedCase {
    /// Content identity bound to the exact checked model.
    pub identity: String,
    /// Checked field assignment.
    pub values: BTreeMap<String, ModelValue>,
    /// Applicable deterministic boundary partitions.
    pub partitions: Vec<String>,
}

impl AdmittedCase {
    /// Validates and constructs one case for an exact checked model.
    ///
    /// # Errors
    /// Returns a typed error when the assignment is outside the admitted domain.
    pub fn new(
        values: BTreeMap<String, ModelValue>,
        partitions: Vec<String>,
        model: &CheckedModel,
    ) -> Result<Self, VerificationError> {
        if !model.admits(&values) {
            return Err(model_error(
                VerificationErrorCode::InvalidModelDomain,
                "case is not admitted by the exact checked model",
            ));
        }
        let identity = content_identity(&(model_identity(model)?, &values, &partitions))?;
        Ok(Self {
            identity,
            values,
            partitions,
        })
    }
}

/// A representable just-outside input that must be refused before native replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RefusalProbe {
    /// Content identity bound to the checked model.
    pub identity: String,
    /// Intentionally out-of-domain assignment.
    pub values: BTreeMap<String, ModelValue>,
    /// Exact just-outside partition name.
    pub partition: String,
}

/// A generated admitted case or a non-executable refusal probe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ModelCase {
    /// Domain-valid case eligible for native replay.
    Admitted(AdmittedCase),
    /// Domain-invalid boundary probe, retained as a typed refusal expectation.
    RefusalProbe(RefusalProbe),
}

/// A deterministic generated case set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedModelCases {
    /// Identity binding the complete request and generated sequence.
    pub identity: String,
    /// Boundary-partitioned cases in deterministic order.
    pub cases: Vec<ModelCase>,
}

/// A semantic native-replay disposition carried in a failure signature.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplayDisposition {
    /// Native execution refuted the scoped clause.
    Refuted,
    /// Native execution observed a scoped violation without claiming proof.
    ObservedViolation,
}

/// Exact semantic failure that every retained shrink must preserve.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FailureSignature {
    /// Stable checked clause identifier.
    pub clause_id: String,
    /// Exact native semantic disposition.
    pub disposition: ReplayDisposition,
}

/// One native replay observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ReplayObservation {
    /// The scoped clause held for this test case only.
    Satisfied {
        /// Stable checked clause identifier.
        clause_id: String,
    },
    /// The scoped clause failed with an exact signature.
    Refuted(FailureSignature),
    /// The native runtime unexpectedly rejected an already-admitted case.
    RejectedPrecondition {
        /// Native precondition identifier.
        precondition_id: String,
    },
    /// Native replay itself failed and made no semantic claim.
    Failed {
        /// Stable provider failure code.
        code: String,
        /// Human-readable diagnostic.
        message: String,
    },
}

/// Exact native-runtime replay boundary used as the semantic oracle.
pub trait NativeReplay {
    /// Replays one domain-valid case against the exact native subject.
    fn replay(&mut self, case: &AdmittedCase) -> ReplayObservation;
}

/// Why a proposed shrink was not retained.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShrinkRejection {
    /// The candidate was equal to or larger than the current retained case.
    NotStrictlySmaller,
    /// The candidate was outside the exact checked domain.
    OutsideDomain,
    /// Native replay did not reproduce the exact failure signature.
    ChangedFailure,
    /// Native replay rejected an already-admitted candidate.
    RejectedPrecondition,
    /// Native replay failed.
    ReplayFailed,
    /// The explicit shrink work bound was reached.
    WorkBoundReached,
}

/// One rejected shrink proposal and its typed reason.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RejectedShrink {
    /// Proposed assignment.
    pub values: BTreeMap<String, ModelValue>,
    /// Typed rejection reason.
    pub reason: ShrinkRejection,
}

/// Immutable shrink lineage for one original counterexample.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ShrinkResult {
    /// Original native counterexample.
    pub original: AdmittedCase,
    /// Strictly descending retained path.
    pub retained: Vec<AdmittedCase>,
    /// Native replay references corresponding one-for-one with the retained path.
    pub retained_replays: Vec<ReplayRecord>,
    /// Smallest retained case, or the original when no shrink was retained.
    pub smallest_retained: AdmittedCase,
    /// Proven minimum when every smaller assignment was replayed within the bound.
    pub minimal_counterexample: Option<AdmittedCase>,
    /// Whether the shrink search examined its complete finite candidate set.
    pub complete: bool,
    /// Auditable rejected proposals.
    pub rejected: Vec<RejectedShrink>,
}

/// Executes proposed shrinks through exact native replay.
///
/// # Errors
/// Returns an invalid-model error when the original counterexample or failure
/// signature is not bound to the exact checked model.
pub fn shrink_counterexample<R: NativeReplay>(
    model: &CheckedModel,
    original: &AdmittedCase,
    proposals: &[BTreeMap<String, ModelValue>],
    expected: &FailureSignature,
    shrink_bound: usize,
    replay: &mut R,
) -> Result<ShrinkResult, VerificationError> {
    let reconstructed =
        AdmittedCase::new(original.values.clone(), original.partitions.clone(), model)?;
    if reconstructed.identity != original.identity || expected.clause_id.is_empty() {
        return Err(model_error(
            VerificationErrorCode::InvalidModelDomain,
            "original counterexample identity and failure clause must be valid",
        ));
    }
    let mut current = original.clone();
    let mut retained = Vec::new();
    let mut retained_replays = Vec::new();
    let mut rejected = Vec::new();
    let mut complete = true;
    for (index, values) in proposals.iter().enumerate() {
        if index >= shrink_bound {
            complete = false;
            rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::WorkBoundReached,
            });
            continue;
        }
        if !model.admits(values) {
            rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::OutsideDomain,
            });
            continue;
        }
        if !model.strictly_smaller(values, &current.values) {
            rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::NotStrictlySmaller,
            });
            continue;
        }
        let Ok(candidate) =
            AdmittedCase::new(values.clone(), vec!["shrink:candidate".to_owned()], model)
        else {
            rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::OutsideDomain,
            });
            continue;
        };
        let observation = replay.replay(&candidate);
        match &observation {
            ReplayObservation::Refuted(signature) if signature == expected => {
                retained_replays.push(replay_record(model, &candidate, observation));
                current = candidate.clone();
                retained.push(candidate);
            }
            ReplayObservation::RejectedPrecondition { .. } => rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::RejectedPrecondition,
            }),
            ReplayObservation::Failed { .. } => rejected.push(RejectedShrink {
                values: values.clone(),
                reason: ShrinkRejection::ReplayFailed,
            }),
            ReplayObservation::Satisfied { .. } | ReplayObservation::Refuted(_) => {
                rejected.push(RejectedShrink {
                    values: values.clone(),
                    reason: ShrinkRejection::ChangedFailure,
                });
            }
        }
    }
    Ok(ShrinkResult {
        original: original.clone(),
        retained,
        retained_replays,
        smallest_retained: current.clone(),
        minimal_counterexample: complete.then_some(current),
        complete,
        rejected,
    })
}

/// One replayed case and its observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CaseObservation {
    /// Exact generated case identity.
    pub case_identity: String,
    /// Immutable native replay identity.
    pub replay_identity: String,
    /// Native replay observation.
    pub observation: ReplayObservation,
}

/// Immutable reference to one exact native replay and its observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReplayRecord {
    /// Identity binding runtime, subject, case, and observation.
    pub identity: String,
    /// Replayed case identity.
    pub case_identity: String,
    /// Exact native observation.
    pub observation: ReplayObservation,
}

/// Completed model-test run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModelTestReport {
    /// Exact method-result contract version.
    pub contract_version: String,
    /// Immutable result identity.
    pub identity: String,
    /// Generated-case identity.
    pub generated_cases_identity: String,
    /// Native observations made before completion or first counterexample.
    pub observations: Vec<CaseObservation>,
    /// Counterexample and shrink lineage, when refuted.
    pub counterexample: Option<ShrinkResult>,
}

/// Typed refusal made before native execution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModelRefusal {
    /// Exact method-result contract version.
    pub contract_version: String,
    /// Immutable result identity.
    pub identity: String,
    /// Stable refusal code.
    pub code: ModelRefusalCode,
    /// Human-readable diagnostic.
    pub message: String,
}

/// Typed method failure with no semantic verdict.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ModelFailure {
    /// Exact method-result contract version.
    pub contract_version: String,
    /// Immutable result identity.
    pub identity: String,
    /// Stable failure code.
    pub code: ModelFailureCode,
    /// Native-provider code when the provider reported the failure.
    pub provider_code: Option<String>,
    /// Human-readable diagnostic.
    pub message: String,
}

/// Stable reasons model construction can refuse before execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelRefusalCode {
    /// A domain, identity, predicate, or bound is malformed.
    InvalidDomain,
    /// No assignment satisfies the checked model.
    UnsatisfiableDomain,
    /// Domain expansion exceeded its construction bound.
    ConstructionBoundExceeded,
    /// Complete boundary coverage exceeded the case bound.
    CaseBoundExceeded,
}

impl ModelRefusalCode {
    const fn from_error(code: VerificationErrorCode) -> Self {
        match code {
            VerificationErrorCode::UnsatisfiableModelDomain => Self::UnsatisfiableDomain,
            VerificationErrorCode::ModelConstructionBoundExceeded => {
                Self::ConstructionBoundExceeded
            }
            VerificationErrorCode::ModelCaseBoundExceeded => Self::CaseBoundExceeded,
            _ => Self::InvalidDomain,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidDomain => "invalid-domain",
            Self::UnsatisfiableDomain => "unsatisfiable-domain",
            Self::ConstructionBoundExceeded => "construction-bound-exceeded",
            Self::CaseBoundExceeded => "case-bound-exceeded",
        }
    }
}

/// Stable method failures that never imply a semantic verdict.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelFailureCode {
    /// Native replay rejected a case admitted by checked-domain construction.
    RejectedPrecondition,
    /// Native replay reported a provider failure.
    NativeReplayFailed,
    /// Replay of a proposed shrink rejected or failed after the original refutation.
    InvalidShrinkReplay,
    /// The original counterexample or shrink request is malformed.
    InvalidShrink,
    /// An immutable result identity could not be constructed.
    IdentityFailure,
}

/// Terminal model-provider outcome. Non-conclusion is separate from semantics.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum ModelMethodOutcome {
    /// Domain construction and all retained native observations completed.
    Tested(Box<ModelTestReport>),
    /// Checked-domain construction refused before native replay.
    Refused(ModelRefusal),
    /// Native replay or result construction failed without a semantic claim.
    Failed(ModelFailure),
}

impl ModelMethodOutcome {
    /// Returns the immutable terminal result identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        match self {
            Self::Tested(report) => &report.identity,
            Self::Refused(refusal) => &refusal.identity,
            Self::Failed(failure) => &failure.identity,
        }
    }
}

/// Constructs the complete deterministic bounded case sequence.
///
/// # Errors
/// Returns a typed refusal cause for malformed, unsatisfiable, or over-bound domains.
pub fn generate_model_cases(
    request: &ModelTestRequest,
) -> Result<GeneratedModelCases, VerificationError> {
    validate_request(request)?;
    let request_identity = content_identity(request)?;
    let field_values = request
        .model
        .fields
        .iter()
        .map(|field| field.domain.values(request.model.construction_bound))
        .collect::<Result<Vec<_>, _>>()?;
    let mut assignments = vec![BTreeMap::new()];
    for (field, values) in request.model.fields.iter().zip(field_values) {
        let projected = assignments.len().checked_mul(values.len()).ok_or_else(|| {
            model_error(
                VerificationErrorCode::ModelConstructionBoundExceeded,
                "Cartesian domain size overflowed",
            )
        })?;
        if projected > request.model.construction_bound {
            return Err(model_error(
                VerificationErrorCode::ModelConstructionBoundExceeded,
                "Cartesian domain exceeds the construction bound",
            ));
        }
        let mut next = Vec::with_capacity(projected);
        for assignment in assignments {
            for value in &values {
                let mut candidate = assignment.clone();
                candidate.insert(field.id.clone(), value.clone());
                next.push(candidate);
            }
        }
        assignments = next;
    }
    let admitted = assignments
        .into_iter()
        .filter(|assignment| request.model.admits(assignment))
        .collect::<Vec<_>>();
    if admitted.is_empty() {
        return Err(model_error(
            VerificationErrorCode::UnsatisfiableModelDomain,
            "checked preconditions and relationships admit no assignment",
        ));
    }

    let mut admitted_cases = admitted
        .iter()
        .map(|values| {
            let partitions = partitions(&request.model, values);
            AdmittedCase::new(values.clone(), partitions, &request.model)
        })
        .collect::<Result<Vec<_>, _>>()?;
    deterministic_shuffle(&mut admitted_cases, request.seed);
    let mut cases = admitted_cases
        .into_iter()
        .map(ModelCase::Admitted)
        .collect::<Vec<_>>();
    cases.extend(refusal_probes(&request.model, &admitted[0])?);
    if cases.len() > request.case_bound {
        return Err(model_error(
            VerificationErrorCode::ModelCaseBoundExceeded,
            "complete partitioned case set exceeds the case bound",
        ));
    }
    let identity = content_identity(&(request_identity, &cases))?;
    Ok(GeneratedModelCases { identity, cases })
}

/// Executes admitted cases against the exact native replay provider.
#[must_use]
pub fn run_model_tests<R: NativeReplay>(
    request: &ModelTestRequest,
    replay: &mut R,
) -> ModelMethodOutcome {
    let generated = match generate_model_cases(request) {
        Ok(generated) => generated,
        Err(error) if error.code() == VerificationErrorCode::CanonicalizationFailed => {
            return model_failure(
                "invalid-request",
                "identity-failure",
                ModelFailureCode::IdentityFailure,
                None,
                error.to_string(),
            );
        }
        Err(error) => {
            let code = ModelRefusalCode::from_error(error.code());
            return ModelMethodOutcome::Refused(ModelRefusal {
                contract_version: MODEL_METHOD_RESULT_VERSION.to_owned(),
                identity: terminal_identity("invalid-request", "refused", code.as_str()),
                code,
                message: error.to_string(),
            });
        }
    };
    let admitted = generated
        .cases
        .iter()
        .filter_map(|case| match case {
            ModelCase::Admitted(case) => Some(case.clone()),
            ModelCase::RefusalProbe(_) => None,
        })
        .collect::<Vec<_>>();
    let mut observations = Vec::new();
    for case in &admitted {
        let observation = replay.replay(case);
        observations.push(CaseObservation {
            case_identity: case.identity.clone(),
            replay_identity: replay_record(&request.model, case, observation.clone()).identity,
            observation: observation.clone(),
        });
        match observation {
            ReplayObservation::Satisfied { .. } => {}
            ReplayObservation::Refuted(signature) => {
                let proposals = shrink_proposals(&request.model, &admitted, case);
                let shrink = match shrink_counterexample(
                    &request.model,
                    case,
                    &proposals,
                    &signature,
                    request.shrink_bound,
                    replay,
                ) {
                    Ok(shrink) => shrink,
                    Err(error) => {
                        return model_failure(
                            &generated.identity,
                            "invalid-shrink",
                            ModelFailureCode::InvalidShrink,
                            None,
                            error.to_string(),
                        );
                    }
                };
                if shrink.rejected.iter().any(|candidate| {
                    matches!(
                        candidate.reason,
                        ShrinkRejection::RejectedPrecondition | ShrinkRejection::ReplayFailed
                    )
                }) {
                    return model_failure(
                        &generated.identity,
                        "invalid-shrink-replay",
                        ModelFailureCode::InvalidShrinkReplay,
                        None,
                        "native replay rejected or failed for a domain-valid shrink".to_owned(),
                    );
                }
                return tested_outcome(&generated.identity, observations, Some(shrink));
            }
            ReplayObservation::RejectedPrecondition { precondition_id } => {
                return model_failure(
                    &generated.identity,
                    "rejected-precondition",
                    ModelFailureCode::RejectedPrecondition,
                    None,
                    format!("native replay rejected admitted case at {precondition_id}"),
                );
            }
            ReplayObservation::Failed { code, message } => {
                return model_failure(
                    &generated.identity,
                    &code,
                    ModelFailureCode::NativeReplayFailed,
                    Some(code.clone()),
                    message,
                );
            }
        }
    }
    tested_outcome(&generated.identity, observations, None)
}

fn validate_request(request: &ModelTestRequest) -> Result<(), VerificationError> {
    if request.contract_version != MODEL_TEST_REQUEST_VERSION {
        return Err(model_error(
            VerificationErrorCode::InvalidModelDomain,
            "model-test request contract version is unsupported",
        ));
    }
    if request.model.package_identity.is_empty()
        || request.model.profile_identity.is_empty()
        || request.model.semantic_subject_identity.is_empty()
        || request.model.native_runtime_identity.is_empty()
        || request.model.ecosystem_lock_identity.is_empty()
    {
        return Err(model_error(
            VerificationErrorCode::InvalidModelDomain,
            "all checked package, profile, subject, runtime, and ecosystem identities are required",
        ));
    }
    if request.model.construction_bound == 0 || request.case_bound == 0 || request.shrink_bound == 0
    {
        return Err(model_error(
            VerificationErrorCode::InvalidModelDomain,
            "construction, case, and shrink bounds must be positive",
        ));
    }
    let predicate_count = request
        .model
        .preconditions
        .len()
        .checked_add(request.model.relationships.len())
        .ok_or_else(|| {
            model_error(
                VerificationErrorCode::ModelConstructionBoundExceeded,
                "predicate count overflowed",
            )
        })?;
    if request.model.fields.len() > request.model.construction_bound
        || predicate_count > request.model.construction_bound
    {
        return Err(model_error(
            VerificationErrorCode::ModelConstructionBoundExceeded,
            "model structure exceeds the construction bound",
        ));
    }
    let mut field_ids = BTreeSet::new();
    for field in &request.model.fields {
        if field.id.is_empty() || !field_ids.insert(field.id.as_str()) {
            return Err(model_error(
                VerificationErrorCode::InvalidModelDomain,
                "field identifiers must be non-empty and distinct",
            ));
        }
        field.domain.validate()?;
    }
    for predicate in request
        .model
        .preconditions
        .iter()
        .chain(&request.model.relationships)
    {
        let (left, right) = predicate.fields();
        if !field_ids.contains(left) || !field_ids.contains(right) {
            return Err(model_error(
                VerificationErrorCode::InvalidModelDomain,
                "predicate names an unknown field",
            ));
        }
        if matches!(
            predicate,
            CheckedPredicate::LessThan { .. } | CheckedPredicate::LessThanOrEqual { .. }
        ) {
            let integer = |id: &str| {
                request
                    .model
                    .fields
                    .iter()
                    .find(|field| field.id == id)
                    .is_some_and(|field| matches!(field.domain, CheckedDomain::Integer { .. }))
            };
            if !integer(left) || !integer(right) {
                return Err(model_error(
                    VerificationErrorCode::InvalidModelDomain,
                    "ordered predicates require integer fields",
                ));
            }
        }
    }
    Ok(())
}

fn append_sequences(
    elements: &[ModelValue],
    length: usize,
    limit: usize,
    output: &mut Vec<Vec<ModelValue>>,
) -> Result<(), VerificationError> {
    if length == 0 {
        output.push(Vec::new());
        return Ok(());
    }
    let mut current = vec![Vec::new()];
    for _ in 0..length {
        let projected = current.len().checked_mul(elements.len()).ok_or_else(|| {
            model_error(
                VerificationErrorCode::ModelConstructionBoundExceeded,
                "collection domain size overflowed",
            )
        })?;
        if projected > limit || output.len().saturating_add(projected) > limit {
            return Err(model_error(
                VerificationErrorCode::ModelConstructionBoundExceeded,
                "collection domain exceeds the construction bound",
            ));
        }
        let mut next = Vec::with_capacity(projected);
        for prefix in current {
            for element in elements {
                let mut sequence = prefix.clone();
                sequence.push(element.clone());
                next.push(sequence);
            }
        }
        current = next;
    }
    output.extend(current);
    Ok(())
}

fn partitions(model: &CheckedModel, values: &BTreeMap<String, ModelValue>) -> Vec<String> {
    let mut result = Vec::new();
    for field in &model.fields {
        let Some(value) = values.get(&field.id) else {
            continue;
        };
        match (&field.domain, value) {
            (CheckedDomain::Integer { minimum, maximum }, ModelValue::Integer(candidate)) => {
                if candidate == minimum {
                    result.push(format!("{}:minimum", field.id));
                }
                if candidate == maximum {
                    result.push(format!("{}:maximum", field.id));
                }
                if minimum == maximum {
                    result.push(format!("{}:singleton", field.id));
                }
            }
            (CheckedDomain::Boolean, ModelValue::Boolean(candidate)) => {
                result.push(format!("{}:{}", field.id, candidate));
            }
            (CheckedDomain::Enumeration { values }, ModelValue::Label(candidate)) => {
                if values.first() == Some(candidate) {
                    result.push(format!("{}:minimum", field.id));
                }
                if values.last() == Some(candidate) {
                    result.push(format!("{}:maximum", field.id));
                }
                if values.len() == 1 {
                    result.push(format!("{}:singleton", field.id));
                }
            }
            (
                CheckedDomain::Collection {
                    minimum_items,
                    maximum_items,
                    ..
                },
                ModelValue::Collection(items),
            ) => {
                if items.is_empty() {
                    result.push(format!("{}:empty", field.id));
                }
                if items.len() == 1 {
                    result.push(format!("{}:singleton", field.id));
                }
                if items.len() == *minimum_items {
                    result.push(format!("{}:minimum", field.id));
                }
                if items.len() == *maximum_items {
                    result.push(format!("{}:maximum", field.id));
                }
            }
            _ => {}
        }
    }
    for (index, relationship) in model.relationships.iter().enumerate() {
        if relationship.is_boundary(values) {
            result.push(format!("relationship:{index}:boundary"));
        }
    }
    result
}

fn refusal_probes(
    model: &CheckedModel,
    baseline: &BTreeMap<String, ModelValue>,
) -> Result<Vec<ModelCase>, VerificationError> {
    let mut probes = Vec::new();
    for field in &model.fields {
        let outside = match &field.domain {
            CheckedDomain::Integer { minimum, maximum } => vec![
                minimum
                    .checked_sub(1)
                    .map(|value| ("just-below", ModelValue::Integer(value))),
                maximum
                    .checked_add(1)
                    .map(|value| ("just-above", ModelValue::Integer(value))),
            ],
            CheckedDomain::Collection {
                minimum_items,
                maximum_items,
                element,
            } => {
                let element = element.values(model.construction_bound)?.into_iter().next();
                let mut values = Vec::new();
                if let (Some(length), Some(item)) = (minimum_items.checked_sub(1), element.clone())
                {
                    values.push(Some((
                        "just-below",
                        ModelValue::Collection(vec![item; length]),
                    )));
                }
                if let (Some(length), Some(item)) = (maximum_items.checked_add(1), element)
                    && length <= model.construction_bound
                {
                    values.push(Some((
                        "just-above",
                        ModelValue::Collection(vec![item; length]),
                    )));
                }
                values
            }
            CheckedDomain::Boolean | CheckedDomain::Enumeration { .. } => Vec::new(),
        };
        for (side, value) in outside.into_iter().flatten() {
            let mut values = baseline.clone();
            values.insert(field.id.clone(), value);
            let partition = format!("{}:{side}", field.id);
            let identity = content_identity(&(model_identity(model)?, &values, &partition))?;
            probes.push(ModelCase::RefusalProbe(RefusalProbe {
                identity,
                values,
                partition,
            }));
        }
    }
    Ok(probes)
}

fn deterministic_shuffle<T>(items: &mut [T], seed: u64) {
    let mut state = seed;
    for index in (1..items.len()).rev() {
        state = splitmix64(state);
        let modulus = u64::try_from(index + 1).unwrap_or(u64::MAX);
        let selected = usize::try_from(state % modulus).unwrap_or(0);
        items.swap(index, selected);
    }
}

const fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn compare_assignments(
    model: &CheckedModel,
    left: &BTreeMap<String, ModelValue>,
    right: &BTreeMap<String, ModelValue>,
) -> Ordering {
    model.order_key(left).cmp(&model.order_key(right))
}

fn shrink_proposals(
    model: &CheckedModel,
    admitted: &[AdmittedCase],
    original: &AdmittedCase,
) -> Vec<BTreeMap<String, ModelValue>> {
    let mut proposals = admitted
        .iter()
        .filter(|candidate| model.strictly_smaller(&candidate.values, &original.values))
        .map(|candidate| candidate.values.clone())
        .collect::<Vec<_>>();
    proposals.sort_by(|left, right| compare_assignments(model, right, left));
    proposals
}

fn model_identity(model: &CheckedModel) -> Result<String, VerificationError> {
    content_identity(model)
}

fn content_identity<T: Serialize>(value: &T) -> Result<String, VerificationError> {
    let value = serde_json::to_value(value).map_err(|error| {
        VerificationError::new(
            VerificationErrorCode::CanonicalizationFailed,
            error.to_string(),
        )
    })?;
    jcs_sha256(&value)
}

fn model_error(code: VerificationErrorCode, message: &str) -> VerificationError {
    VerificationError::new(code, message)
}

fn terminal_identity(request_identity: &str, kind: &str, code: &str) -> String {
    content_identity(&(request_identity, kind, code)).unwrap_or_else(|_| {
        let fallback = [request_identity, kind, code].join("\0");
        format!("sha256:{:x}", Sha256::digest(fallback.as_bytes()))
    })
}

fn replay_record(
    model: &CheckedModel,
    case: &AdmittedCase,
    observation: ReplayObservation,
) -> ReplayRecord {
    let identity = content_identity(&(
        &model.native_runtime_identity,
        &model.semantic_subject_identity,
        &case.identity,
        &observation,
    ))
    .unwrap_or_else(|_| {
        terminal_identity(
            &case.identity,
            &model.native_runtime_identity,
            &format!("{observation:?}"),
        )
    });
    ReplayRecord {
        identity,
        case_identity: case.identity.clone(),
        observation,
    }
}

fn tested_outcome(
    generated_cases_identity: &str,
    observations: Vec<CaseObservation>,
    counterexample: Option<ShrinkResult>,
) -> ModelMethodOutcome {
    match content_identity(&(generated_cases_identity, &observations, &counterexample)) {
        Ok(identity) => ModelMethodOutcome::Tested(Box::new(ModelTestReport {
            contract_version: MODEL_METHOD_RESULT_VERSION.to_owned(),
            identity,
            generated_cases_identity: generated_cases_identity.to_owned(),
            observations,
            counterexample,
        })),
        Err(error) => ModelMethodOutcome::Failed(ModelFailure {
            contract_version: MODEL_METHOD_RESULT_VERSION.to_owned(),
            identity: terminal_identity(generated_cases_identity, "failed", "identity-failure"),
            code: ModelFailureCode::IdentityFailure,
            provider_code: None,
            message: error.to_string(),
        }),
    }
}

fn model_failure(
    base_identity: &str,
    identity_code: &str,
    code: ModelFailureCode,
    provider_code: Option<String>,
    message: String,
) -> ModelMethodOutcome {
    ModelMethodOutcome::Failed(ModelFailure {
        contract_version: MODEL_METHOD_RESULT_VERSION.to_owned(),
        identity: terminal_identity(base_identity, "failed", identity_code),
        code,
        provider_code,
        message,
    })
}
