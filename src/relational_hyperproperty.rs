// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Finite relational and hyperproperty evaluation over explicit trace identities.
//!
//! Governing requirements: FR-030, TC-048, and IT-005-SC-02.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::contracts::{VerificationError, VerificationErrorCode, jcs_sha256};

/// Exact relational request contract implemented by this provider.
pub const RELATIONAL_REQUEST_VERSION: &str = "quire.relational-request/v1";
/// Exact relational result contract emitted by this provider.
pub const RELATIONAL_RESULT_VERSION: &str = "quire.relational-result/v1";

/// Exact trace identity. Display names are never used for correlation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct TraceIdentity(String);

impl TraceIdentity {
    /// Constructs one nonempty exact trace identity.
    ///
    /// # Errors
    /// Returns the crate's stable input error for an empty identity.
    pub fn new(value: impl Into<String>) -> Result<Self, VerificationError> {
        let value = value.into();
        if value.is_empty() {
            return Err(VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "trace identity must not be empty",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the exact immutable trace identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A typed value retained at a trace position or in an assumption.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum RelationalValue {
    /// Integer value.
    Integer(i64),
    /// Boolean value.
    Boolean(bool),
    /// Label value.
    Label(String),
}

/// One identified trace point.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TracePoint {
    /// Exact event identity at this position.
    pub event_identity: String,
    /// Explicit typed values visible to relational expressions.
    pub values: BTreeMap<String, RelationalValue>,
}

/// One execution trace from a declared population.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Trace {
    /// Exact trace identity used for all quantification and alignment.
    pub identity: TraceIdentity,
    /// Human-readable local label; deliberately excluded from correlation semantics.
    pub local_name: String,
    /// Typed attributes used by declassification and environment assumptions.
    pub attributes: BTreeMap<String, RelationalValue>,
    /// Ordered finite trace points.
    pub points: Vec<TracePoint>,
}

/// A closed population from which a quantified trace variable may draw.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TracePopulation {
    /// Exact declared population identity.
    pub identity: String,
    /// Exact closure artifact identity, absent when closure is not available.
    pub closure_identity: Option<String>,
    /// Distinct exact traces in the population.
    pub traces: Vec<Trace>,
}

/// Ordered quantifier kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Quantifier {
    /// Every eligible distinct trace must satisfy the remaining prefix.
    ForAll,
    /// One eligible distinct trace must satisfy the remaining prefix.
    Exists,
}

/// One variable in the preserved ordered quantifier prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct QuantifiedTrace {
    /// Stable trace-variable identity within this request.
    pub variable: String,
    /// Quantifier evaluated at this exact prefix position.
    pub quantifier: Quantifier,
    /// Exact population identity from which the variable draws.
    pub population_identity: String,
}

/// Typed comparison operator for explicitly aligned values.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonOperator {
    /// Typed equality.
    Equal,
    /// Typed inequality.
    NotEqual,
    /// Typed less-than comparison.
    LessThan,
    /// Typed less-than-or-equal comparison.
    LessThanOrEqual,
}

/// A typed value selected from one quantified trace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraceValueRef {
    /// Bound quantified variable.
    pub variable: String,
    /// Explicit source-model field name.
    pub field: String,
}

/// Closed relational expression evaluated at every aligned pair of positions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RelationalExpression {
    /// Compare one typed field from each of two distinct quantified traces.
    Compare {
        /// Left trace value.
        left: TraceValueRef,
        /// Typed operator.
        operator: ComparisonOperator,
        /// Right trace value.
        right: TraceValueRef,
    },
}

/// How an authored alignment establishes correlation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AlignmentKind {
    /// The request explicitly maps ordered positions.
    ExplicitPosition,
    /// Every mapped pair must carry the same exact event identity.
    ExactEventIdentity,
}

/// One explicit pair of trace positions.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AlignedPosition {
    /// Position in the alignment's left trace.
    pub left: usize,
    /// Position in the alignment's right trace.
    pub right: usize,
}

/// A total, typed alignment between two exact trace identities.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TraceAlignment {
    /// Immutable authored alignment identity.
    pub identity: String,
    /// Exact left trace identity.
    pub left_trace: TraceIdentity,
    /// Exact right trace identity.
    pub right_trace: TraceIdentity,
    /// Typed correspondence mode.
    pub kind: AlignmentKind,
    /// Explicit total position mapping.
    pub pairs: Vec<AlignedPosition>,
}

/// Assumption category retained in result identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceAssumptionKind {
    /// Environment restriction applied before quantification.
    Environment,
    /// Declassification restriction applied before quantification.
    Declassification,
}

/// An exact attribute filter applied to a declared population before quantification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PopulationAssumption {
    /// Immutable authored assumption identity.
    pub identity: String,
    /// Assumption category.
    pub kind: TraceAssumptionKind,
    /// Population restricted by this assumption.
    pub population_identity: String,
    /// Typed trace attribute name.
    pub attribute: String,
    /// Required exact attribute value.
    pub equals: RelationalValue,
}

/// Exact dependence classification supplied by the native/oracle boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum OracleDependence {
    /// The supplied oracle contribution is declared independent.
    Independent,
    /// The contribution shares this declared dependence group.
    Shared {
        /// Exact shared group identity.
        group_identity: String,
    },
    /// The contribution is derived from named exact contributions.
    Derived {
        /// Exact source contribution identities.
        source_identities: Vec<String>,
    },
    /// Dependence is not known.
    Unknown,
}

/// Complete finite relational evaluation request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RelationalRequest {
    /// Exact request contract version.
    pub contract_version: String,
    /// Exact first-party provider implementation identity.
    pub provider_identity: String,
    /// Exact checked multi-execution semantic-subject identity.
    pub semantic_subject_identity: String,
    /// Maximum finite leaf assignments the provider may evaluate.
    pub finite_assignment_bound: usize,
    /// Closed declared populations.
    pub populations: Vec<TracePopulation>,
    /// Ordered quantifier prefix.
    pub prefix: Vec<QuantifiedTrace>,
    /// Typed cross-execution expression.
    pub expression: RelationalExpression,
    /// Authored exact alignments.
    pub alignments: Vec<TraceAlignment>,
    /// Declassification/environment restrictions.
    pub assumptions: Vec<PopulationAssumption>,
    /// Exact retained oracle-dependence classification.
    pub oracle_dependence: OracleDependence,
}

/// Exact paired or trace-set witness retained with a semantic result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RelationalWitness {
    /// Decisive exact variable-to-trace assignment.
    pub decisive_assignment: BTreeMap<String, TraceIdentity>,
    /// Exact alignment identities used by the decisive assignment.
    pub alignment_identities: Vec<String>,
    /// Every leaf assignment evaluated before the terminal result.
    pub evaluated_assignments: Vec<BTreeMap<String, TraceIdentity>>,
}

/// A terminal semantic finite relational result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RelationalResult {
    /// Exact result contract version.
    pub contract_version: String,
    /// Immutable result identity.
    pub identity: String,
    /// Immutable request identity.
    pub request_identity: String,
    /// Exact witness or counterexample.
    pub witness: RelationalWitness,
    /// Retained without any independence promotion.
    pub oracle_dependence: OracleDependence,
}

/// Stable non-semantic relational disposition codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelationalNonConclusionCode {
    /// A population has no closure identity.
    MissingPopulationClosure,
    /// A required exact alignment is absent.
    MissingAlignment,
    /// More than one authored alignment applies to one trace pair.
    AmbiguousAlignment,
    /// An alignment is not total or has malformed pairs.
    NonTotalAlignment,
    /// The finite assignment budget was exceeded before evaluation.
    AssignmentBoundExceeded,
    /// Too few distinct trace identities remain for the prefix.
    InsufficientDistinctTraces,
    /// A population repeats one trace identity.
    DuplicateTraceIdentity,
    /// The request has invalid explicit identity, prefix, or value typing.
    InvalidRequest,
    /// Expression values have incompatible types.
    InvalidTypedComparison,
}

/// A typed refusal or non-conclusive outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RelationalNonConclusion {
    /// Exact result contract version.
    pub contract_version: String,
    /// Immutable result identity.
    pub identity: String,
    /// Stable terminal code.
    pub code: RelationalNonConclusionCode,
    /// Human-readable diagnostic.
    pub message: String,
    /// Retained input dependence classification.
    pub oracle_dependence: OracleDependence,
}

/// One terminal relational-provider outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum RelationalMethodOutcome {
    /// Exhaustive finite evaluation proved the expression under the ordered prefix.
    Proved(Box<RelationalResult>),
    /// Exhaustive finite evaluation refuted the expression under the ordered prefix.
    Refuted(Box<RelationalResult>),
    /// A required closure/alignment/distinctness condition was unavailable.
    Incomplete(RelationalNonConclusion),
    /// The explicit finite work bound was insufficient.
    RequiresBound(RelationalNonConclusion),
    /// The request was refused before semantic evaluation.
    Refused(RelationalNonConclusion),
    /// Internal result construction failed without a semantic claim.
    Failed(RelationalNonConclusion),
}

impl RelationalMethodOutcome {
    /// Returns the immutable terminal outcome identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        match self {
            Self::Proved(result) | Self::Refuted(result) => &result.identity,
            Self::Incomplete(result)
            | Self::RequiresBound(result)
            | Self::Refused(result)
            | Self::Failed(result) => &result.identity,
        }
    }

    /// Returns the exact dependence classification retained by this outcome.
    #[must_use]
    pub fn oracle_dependence(&self) -> Option<&OracleDependence> {
        match self {
            Self::Proved(result) | Self::Refuted(result) => Some(&result.oracle_dependence),
            Self::Incomplete(result)
            | Self::RequiresBound(result)
            | Self::Refused(result)
            | Self::Failed(result) => Some(&result.oracle_dependence),
        }
    }
}

/// Evaluates one bounded finite relational request without local-name correlation.
#[must_use]
pub fn evaluate_relational(request: &RelationalRequest) -> RelationalMethodOutcome {
    let request_identity = match identity(request) {
        Ok(identity) => identity,
        Err(message) => {
            return non_conclusive(
                request,
                "invalid",
                RelationalNonConclusionCode::InvalidRequest,
                message,
                Terminal::Failed,
            );
        }
    };
    let populations = match prepare_populations(request) {
        Ok(populations) => populations,
        Err((code, message, terminal)) => {
            return non_conclusive(request, "prepare", code, message, terminal);
        }
    };
    let possible = match assignment_count(request, &populations) {
        Ok(count) => count,
        Err((code, message, terminal)) => {
            return non_conclusive(request, "count", code, message, terminal);
        }
    };
    if possible > request.finite_assignment_bound {
        return non_conclusive(
            request,
            "bound",
            RelationalNonConclusionCode::AssignmentBoundExceeded,
            "finite assignment bound is insufficient".to_owned(),
            Terminal::RequiresBound,
        );
    }
    let mut evaluated = Vec::new();
    let mut assignments = BTreeMap::new();
    let evaluation = match quantify(request, &populations, 0, &mut assignments, &mut evaluated) {
        Ok(evaluation) => evaluation,
        Err((code, message, terminal)) => {
            return non_conclusive(request, "evaluate", code, message, terminal);
        }
    };
    let alignment_identities = match alignment_identities(request, &evaluation.decisive) {
        Ok(identities) => identities,
        Err((code, message, terminal)) => {
            return non_conclusive(request, "alignment", code, message, terminal);
        }
    };
    let witness = RelationalWitness {
        decisive_assignment: evaluation.decisive,
        alignment_identities,
        evaluated_assignments: evaluated,
    };
    let result = RelationalResult {
        contract_version: RELATIONAL_RESULT_VERSION.to_owned(),
        identity: match identity(&(
            request_identity.clone(),
            evaluation.value,
            &witness,
            &request.oracle_dependence,
        )) {
            Ok(identity) => identity,
            Err(message) => {
                return non_conclusive(
                    request,
                    "result",
                    RelationalNonConclusionCode::InvalidRequest,
                    message,
                    Terminal::Failed,
                );
            }
        },
        request_identity,
        witness,
        oracle_dependence: request.oracle_dependence.clone(),
    };
    if evaluation.value {
        RelationalMethodOutcome::Proved(Box::new(result))
    } else {
        RelationalMethodOutcome::Refuted(Box::new(result))
    }
}

#[derive(Clone, Copy)]
enum Terminal {
    Incomplete,
    RequiresBound,
    Refused,
    Failed,
}

fn prepare_populations(
    request: &RelationalRequest,
) -> Result<BTreeMap<String, Vec<Trace>>, (RelationalNonConclusionCode, String, Terminal)> {
    if request.contract_version != RELATIONAL_REQUEST_VERSION
        || request.provider_identity.is_empty()
        || request.semantic_subject_identity.is_empty()
        || request.finite_assignment_bound == 0
        || request.prefix.is_empty()
    {
        return Err((
            RelationalNonConclusionCode::InvalidRequest,
            "request version, identities, bound, and prefix are required".to_owned(),
            Terminal::Refused,
        ));
    }
    let mut result = BTreeMap::new();
    let mut trace_ids = BTreeSet::new();
    let population_ids = request
        .populations
        .iter()
        .map(|population| population.identity.as_str())
        .collect::<BTreeSet<_>>();
    if population_ids.len() != request.populations.len()
        || population_ids.contains("")
        || request.assumptions.iter().any(|assumption| {
            assumption.identity.is_empty()
                || assumption.attribute.is_empty()
                || !population_ids.contains(assumption.population_identity.as_str())
        })
    {
        return Err((
            RelationalNonConclusionCode::InvalidRequest,
            "population and assumption identities must be declared and nonempty".to_owned(),
            Terminal::Refused,
        ));
    }
    for population in &request.populations {
        if population.identity.is_empty() || result.contains_key(&population.identity) {
            return Err((
                RelationalNonConclusionCode::InvalidRequest,
                "population identities must be nonempty and distinct".to_owned(),
                Terminal::Refused,
            ));
        }
        if population
            .closure_identity
            .as_deref()
            .is_none_or(str::is_empty)
        {
            return Err((
                RelationalNonConclusionCode::MissingPopulationClosure,
                "declared population has no exact closure identity".to_owned(),
                Terminal::Incomplete,
            ));
        }
        let mut retained = Vec::new();
        for trace in &population.traces {
            if !trace_ids.insert(trace.identity.clone()) {
                return Err((
                    RelationalNonConclusionCode::DuplicateTraceIdentity,
                    "trace identities must be globally distinct".to_owned(),
                    Terminal::Refused,
                ));
            }
            if assumptions_hold(request, population, trace) {
                retained.push(trace.clone());
            }
        }
        result.insert(population.identity.clone(), retained);
    }
    let mut variables = BTreeSet::new();
    for item in &request.prefix {
        if item.variable.is_empty()
            || !variables.insert(item.variable.as_str())
            || !result.contains_key(&item.population_identity)
        {
            return Err((
                RelationalNonConclusionCode::InvalidRequest,
                "prefix variables must be distinct and name declared populations".to_owned(),
                Terminal::Refused,
            ));
        }
    }
    Ok(result)
}

fn assumptions_hold(
    request: &RelationalRequest,
    population: &TracePopulation,
    trace: &Trace,
) -> bool {
    request
        .assumptions
        .iter()
        .filter(|assumption| assumption.population_identity == population.identity)
        .all(|assumption| {
            !assumption.identity.is_empty()
                && trace.attributes.get(&assumption.attribute) == Some(&assumption.equals)
        })
}

fn assignment_count(
    request: &RelationalRequest,
    populations: &BTreeMap<String, Vec<Trace>>,
) -> Result<usize, (RelationalNonConclusionCode, String, Terminal)> {
    let mut used_per_population = BTreeMap::<String, usize>::new();
    let mut count = 1_usize;
    for item in &request.prefix {
        let used = used_per_population
            .entry(item.population_identity.clone())
            .or_default();
        let available = populations
            .get(&item.population_identity)
            .map_or(0, Vec::len)
            .saturating_sub(*used);
        if available == 0 {
            return Err((
                RelationalNonConclusionCode::InsufficientDistinctTraces,
                "the ordered prefix requires more distinct traces than remain in a population"
                    .to_owned(),
                Terminal::Incomplete,
            ));
        }
        count = count.checked_mul(available).ok_or_else(|| {
            (
                RelationalNonConclusionCode::AssignmentBoundExceeded,
                "finite assignment count overflowed".to_owned(),
                Terminal::RequiresBound,
            )
        })?;
        *used += 1;
    }
    Ok(count)
}

struct Evaluation {
    value: bool,
    decisive: BTreeMap<String, TraceIdentity>,
}

fn quantify(
    request: &RelationalRequest,
    populations: &BTreeMap<String, Vec<Trace>>,
    index: usize,
    assignments: &mut BTreeMap<String, TraceIdentity>,
    evaluated: &mut Vec<BTreeMap<String, TraceIdentity>>,
) -> Result<Evaluation, (RelationalNonConclusionCode, String, Terminal)> {
    if index == request.prefix.len() {
        let value = evaluate_expression(request, assignments)?;
        evaluated.push(assignments.clone());
        return Ok(Evaluation {
            value,
            decisive: assignments.clone(),
        });
    }
    let binding = &request.prefix[index];
    let candidates = populations
        .get(&binding.population_identity)
        .ok_or_else(|| {
            (
                RelationalNonConclusionCode::InvalidRequest,
                "prefix references an undeclared population".to_owned(),
                Terminal::Refused,
            )
        })?;
    let used = assignments.values().collect::<BTreeSet<_>>();
    let candidates = candidates
        .iter()
        .filter(|trace| !used.contains(&trace.identity))
        .collect::<Vec<_>>();
    let target = matches!(binding.quantifier, Quantifier::ForAll);
    let mut first = None;
    for trace in candidates {
        assignments.insert(binding.variable.clone(), trace.identity.clone());
        let child = quantify(request, populations, index + 1, assignments, evaluated)?;
        assignments.remove(&binding.variable);
        if first.is_none() {
            first = Some(child.decisive.clone());
        }
        if child.value != target {
            return Ok(child);
        }
    }
    Ok(Evaluation {
        value: target,
        decisive: first.unwrap_or_default(),
    })
}

fn evaluate_expression(
    request: &RelationalRequest,
    assignments: &BTreeMap<String, TraceIdentity>,
) -> Result<bool, (RelationalNonConclusionCode, String, Terminal)> {
    let RelationalExpression::Compare {
        left,
        operator,
        right,
    } = &request.expression;
    if left.variable == right.variable || left.field.is_empty() || right.field.is_empty() {
        return Err((
            RelationalNonConclusionCode::InvalidRequest,
            "expression must compare explicit fields from two distinct variables".to_owned(),
            Terminal::Refused,
        ));
    }
    let left_trace = trace_for(request, assignments.get(&left.variable))?;
    let right_trace = trace_for(request, assignments.get(&right.variable))?;
    let alignment = resolve_alignment(request, left_trace, right_trace)?;
    let reverse = alignment.left_trace != left_trace.identity;
    let mut result = true;
    for pair in &alignment.pairs {
        let (left_index, right_index) = if reverse {
            (pair.right, pair.left)
        } else {
            (pair.left, pair.right)
        };
        let left_value = left_trace.points[left_index]
            .values
            .get(&left.field)
            .ok_or_else(|| {
                (
                    RelationalNonConclusionCode::InvalidRequest,
                    "left expression field is absent".to_owned(),
                    Terminal::Refused,
                )
            })?;
        let right_value = right_trace.points[right_index]
            .values
            .get(&right.field)
            .ok_or_else(|| {
                (
                    RelationalNonConclusionCode::InvalidRequest,
                    "right expression field is absent".to_owned(),
                    Terminal::Refused,
                )
            })?;
        if std::mem::discriminant(left_value) != std::mem::discriminant(right_value) {
            return Err((
                RelationalNonConclusionCode::InvalidTypedComparison,
                "aligned values have different declared types".to_owned(),
                Terminal::Refused,
            ));
        }
        let comparison = match operator {
            ComparisonOperator::Equal => left_value == right_value,
            ComparisonOperator::NotEqual => left_value != right_value,
            ComparisonOperator::LessThan => left_value < right_value,
            ComparisonOperator::LessThanOrEqual => left_value <= right_value,
        };
        result &= comparison;
    }
    Ok(result)
}

fn trace_for<'a>(
    request: &'a RelationalRequest,
    identity: Option<&TraceIdentity>,
) -> Result<&'a Trace, (RelationalNonConclusionCode, String, Terminal)> {
    let Some(identity) = identity else {
        return Err((
            RelationalNonConclusionCode::InvalidRequest,
            "expression names an unbound variable".to_owned(),
            Terminal::Refused,
        ));
    };
    request
        .populations
        .iter()
        .flat_map(|population| &population.traces)
        .find(|trace| &trace.identity == identity)
        .ok_or_else(|| {
            (
                RelationalNonConclusionCode::InvalidRequest,
                "bound trace identity is unavailable".to_owned(),
                Terminal::Refused,
            )
        })
}

fn resolve_alignment<'a>(
    request: &'a RelationalRequest,
    left: &Trace,
    right: &Trace,
) -> Result<&'a TraceAlignment, (RelationalNonConclusionCode, String, Terminal)> {
    let candidates = request
        .alignments
        .iter()
        .filter(|alignment| {
            (alignment.left_trace == left.identity && alignment.right_trace == right.identity)
                || (alignment.left_trace == right.identity
                    && alignment.right_trace == left.identity)
        })
        .collect::<Vec<_>>();
    let alignment = match candidates.as_slice() {
        [] => {
            return Err((
                RelationalNonConclusionCode::MissingAlignment,
                "no exact authored alignment exists for the trace pair".to_owned(),
                Terminal::Incomplete,
            ));
        }
        [alignment] => *alignment,
        _ => {
            return Err((
                RelationalNonConclusionCode::AmbiguousAlignment,
                "more than one authored alignment applies to the trace pair".to_owned(),
                Terminal::Incomplete,
            ));
        }
    };
    validate_alignment(alignment, left, right)?;
    Ok(alignment)
}

fn validate_alignment(
    alignment: &TraceAlignment,
    left: &Trace,
    right: &Trace,
) -> Result<(), (RelationalNonConclusionCode, String, Terminal)> {
    if alignment.identity.is_empty()
        || alignment.pairs.len() != left.points.len()
        || alignment.pairs.len() != right.points.len()
    {
        return Err((
            RelationalNonConclusionCode::NonTotalAlignment,
            "alignment must map every position exactly once".to_owned(),
            Terminal::Incomplete,
        ));
    }
    let reverse = alignment.left_trace != left.identity;
    let mut left_positions = BTreeSet::new();
    let mut right_positions = BTreeSet::new();
    for pair in &alignment.pairs {
        let (left_index, right_index) = if reverse {
            (pair.right, pair.left)
        } else {
            (pair.left, pair.right)
        };
        if left_index >= left.points.len()
            || right_index >= right.points.len()
            || !left_positions.insert(left_index)
            || !right_positions.insert(right_index)
        {
            return Err((
                RelationalNonConclusionCode::NonTotalAlignment,
                "alignment contains a duplicate or out-of-range position".to_owned(),
                Terminal::Incomplete,
            ));
        }
        if matches!(alignment.kind, AlignmentKind::ExactEventIdentity)
            && left.points[left_index].event_identity != right.points[right_index].event_identity
        {
            return Err((
                RelationalNonConclusionCode::NonTotalAlignment,
                "event-identity alignment maps unequal event identities".to_owned(),
                Terminal::Incomplete,
            ));
        }
    }
    Ok(())
}

fn alignment_identities(
    request: &RelationalRequest,
    assignments: &BTreeMap<String, TraceIdentity>,
) -> Result<Vec<String>, (RelationalNonConclusionCode, String, Terminal)> {
    let RelationalExpression::Compare { left, right, .. } = &request.expression;
    let left_trace = trace_for(request, assignments.get(&left.variable))?;
    let right_trace = trace_for(request, assignments.get(&right.variable))?;
    Ok(vec![
        resolve_alignment(request, left_trace, right_trace)?
            .identity
            .clone(),
    ])
}

fn non_conclusive(
    request: &RelationalRequest,
    stage: &str,
    code: RelationalNonConclusionCode,
    message: String,
    terminal: Terminal,
) -> RelationalMethodOutcome {
    let identity = identity(&(stage, code, &message, request)).unwrap_or_else(|_| {
        let fallback = format!("{stage}\0{code:?}\0{message}");
        format!("sha256:{:x}", Sha256::digest(fallback.as_bytes()))
    });
    let result = RelationalNonConclusion {
        contract_version: RELATIONAL_RESULT_VERSION.to_owned(),
        identity,
        code,
        message,
        oracle_dependence: request.oracle_dependence.clone(),
    };
    match terminal {
        Terminal::Incomplete => RelationalMethodOutcome::Incomplete(result),
        Terminal::RequiresBound => RelationalMethodOutcome::RequiresBound(result),
        Terminal::Refused => RelationalMethodOutcome::Refused(result),
        Terminal::Failed => RelationalMethodOutcome::Failed(result),
    }
}

fn identity<T: Serialize>(value: &T) -> Result<String, String> {
    let value = serde_json::to_value(value).map_err(|error| error.to_string())?;
    jcs_sha256(&value).map_err(|error| error.to_string())
}
