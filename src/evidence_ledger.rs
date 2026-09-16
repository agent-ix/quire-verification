// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Typed, append-only qualification evidence records for FR-032 through FR-034.
//!
//! This I18--I21 boundary validates producer-supplied records. It neither runs
//! producers nor creates an evidence store owned by Quoin.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

/// Closed categories for identities at the evidence boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceKind {
    /// Qualification claim.
    Claim,
    /// Source revision.
    Source,
    /// Package revision.
    Package,
    /// Semantic model.
    Model,
    /// Model binding.
    Binding,
    /// Requirement.
    Requirement,
    /// Requirement clause.
    Clause,
    /// Semantic subject.
    Subject,
    /// Assumption.
    Assumption,
    /// Implementation revision.
    Implementation,
    /// Verification method.
    Method,
    /// Provider identity.
    Provider,
    /// Provider run.
    ProviderRun,
    /// Provider result.
    Result,
    /// Immutable artifact.
    Artifact,
    /// Ecosystem lock.
    EcosystemLock,
    /// Conformance corpus.
    Corpus,
    /// Human decision.
    Decision,
}

/// Closed, append-only graph relations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceRelation {
    /// Source derivation.
    DerivedFrom,
    /// Evaluation.
    Evaluates,
    /// Implementation.
    Implements,
    /// Producer generation.
    GeneratedBy,
    /// Replay.
    Replays,
    /// Support.
    Supports,
    /// Contradiction.
    Contradicts,
    /// Attributed decision.
    Decides,
}

/// An exact identity whose category is never inferred from text.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvidenceIdentity {
    /// Closed category.
    pub kind: EvidenceKind,
    /// Producer-owned opaque identifier.
    pub value: String,
}

/// One immutable relation with exact scope and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceEdge {
    /// Relation origin.
    pub from: EvidenceIdentity,
    /// Relation target.
    pub to: EvidenceIdentity,
    /// Relation kind.
    pub relation: EvidenceRelation,
    /// Exact relation scope.
    pub scope: String,
    /// Immutable provenance.
    pub provenance: String,
}

/// Producer bytes accepted without executing or reinterpreting the producer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceIntake {
    /// Record/artifact identity.
    pub identity: EvidenceIdentity,
    /// Declared SHA-256 digest.
    pub declared_digest: String,
    /// Exact producer bytes inspected at intake; they are never executed.
    pub producer_bytes: Vec<u8>,
    /// Acquisition provenance.
    pub provenance: String,
}

/// A limitation and the exact claims it narrows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Limitation {
    /// Limitation category.
    pub kind: LimitationKind,
    /// Affected claims only.
    pub affected_claims: Vec<EvidenceIdentity>,
    /// Removed fields for a redaction.
    pub removed_fields: Vec<String>,
}

/// Evidence limitations that never broaden claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LimitationKind {
    /// Data was redacted.
    Redacted,
    /// Data was lost.
    Lost,
    /// Data is inaccessible.
    Inaccessible,
    /// Data cannot be replayed.
    NonReplayable,
    /// Future production material is absent.
    FutureProductionEvidence,
}

/// A typed graph emitted by this repository and retained by Quoin.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceGraph {
    /// Append-only edges.
    pub edges: Vec<EvidenceEdge>,
    /// Byte-validated intake records.
    pub intake: Vec<EvidenceIntake>,
    /// Visible claim limitations.
    pub limitations: Vec<Limitation>,
}

/// Exact replay result of a counterexample revision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayOutcome {
    /// Exact named failure replayed.
    SameFailure,
    /// Replay material unavailable.
    Unavailable,
}

/// One immutable, ordered counterexample revision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CounterexampleRevision {
    /// Revision identity.
    pub identity: String,
    /// Parent identity, or none for the original.
    pub parent: Option<String>,
    /// Exact subject.
    pub subject: EvidenceIdentity,
    /// Exact failing claim.
    pub claim: EvidenceIdentity,
    /// Typed values/trace digest.
    pub values_or_trace: String,
    /// Provider identity.
    pub provider: EvidenceIdentity,
    /// Provider run.
    pub run: EvidenceIdentity,
    /// Transform/replay operation.
    pub operation: String,
    /// Explicit correspondence to parent values/execution.
    pub correspondence: Option<String>,
    /// Domain validity for the named claim.
    pub domain_valid: bool,
    /// Replay disposition.
    pub replay: ReplayOutcome,
    /// Retained material limitations.
    pub limitations: Vec<Limitation>,
}

/// Independent qualification vectors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ClaimClass {
    /// Grammar.
    Grammar,
    /// Semantic kernel.
    SemanticKernel,
    /// Runtime/monitor.
    RuntimeMonitor,
    /// Analysis.
    Analysis,
    /// Backend.
    Backend,
    /// Tooling.
    Tooling,
    /// Consumer integration.
    ConsumingIntegration,
}

/// A vector's visible execution state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VectorState {
    /// Explicit future work.
    Planned,
    /// Not executed.
    Unexecuted,
    /// Passed.
    Passed,
    /// Failed.
    Failed,
    /// Unsupported.
    Unsupported,
    /// Human waiver, never a pass.
    Waived,
}

/// Exact provenance binding for one independent vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationVector {
    /// Independent class.
    pub claim_class: ClaimClass,
    /// Source identity.
    pub source: EvidenceIdentity,
    /// Package identity.
    pub package: EvidenceIdentity,
    /// Implementation identity.
    pub implementation: EvidenceIdentity,
    /// Method identity.
    pub method: EvidenceIdentity,
    /// Corpus identity.
    pub corpus: EvidenceIdentity,
    /// Ecosystem lock identity.
    pub ecosystem_lock: EvidenceIdentity,
    /// Provider run identity.
    pub run: EvidenceIdentity,
    /// Result identity.
    pub result: EvidenceIdentity,
    /// Vector state.
    pub state: VectorState,
}

/// One primary row in the closed capability denominator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerRow {
    /// Accepted capability identity.
    pub capability_identity: CapabilityIdentity,
    /// Claim classes applicable to this capability under the selected profile.
    pub required_claim_classes: Vec<ClaimClass>,
    /// Applicable independent vectors.
    pub vectors: Vec<QualificationVector>,
}

/// Exact capability identity in the closed qualification denominator.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CapabilityIdentity(pub String);

/// Immutable attributed human decision over an unchanged ledger scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HumanDecision {
    /// Decision identity.
    pub identity: EvidenceIdentity,
    /// Authenticated actor.
    pub actor: String,
    /// Decision authority.
    pub authority: String,
    /// Externally verified authorization bound to this exact decision.
    pub authorization: VerifiedDecisionAuthorization,
    /// Exact ledger scope digest.
    pub ledger_scope: String,
    /// Disposition.
    pub disposition: DecisionDisposition,
    /// Human rationale.
    pub rationale: String,
}

/// An externally verified authorization result for one immutable decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedDecisionAuthorization {
    /// Exact decision authorized by the external verifier.
    pub decision_identity: EvidenceIdentity,
    /// Verified actor.
    pub actor: String,
    /// Verified authority.
    pub authority: String,
    /// Verified unchanged ledger scope.
    pub ledger_scope: String,
    /// Immutable verifier attestation artifact.
    pub attestation: EvidenceIdentity,
}

/// External trust boundary for decision-attestation verification.
///
/// Implementations verify the attestation signature or authorization protocol;
/// this ledger only consumes their boolean result and retains the exact bytes.
pub trait DecisionAuthorizationVerifier {
    /// Returns whether this authorization attests this exact decision.
    fn verifies(
        &self,
        decision: &HumanDecision,
        authorization: &VerifiedDecisionAuthorization,
    ) -> bool;
}

/// Eventual decision disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecisionDisposition {
    /// Accept.
    Accepted,
    /// Reject.
    Rejected,
    /// Limit scope.
    Limited,
}

/// Result of a graph, lineage, or ledger operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerOutcome {
    /// Valid local accounting.
    Accounted,
    /// Development can proceed; exact claims are incomplete.
    Incomplete {
        /// Limiting states.
        limitations: Vec<LimitationKind>,
    },
    /// An authenticated decision explicitly rejected the candidate.
    Rejected {
        /// Exact rejecting decision identity.
        decision: EvidenceIdentity,
    },
    /// An authenticated decision explicitly limited the candidate scope.
    Limited {
        /// Exact limiting decision identity.
        decision: EvidenceIdentity,
    },
    /// Invalid discharge/acceptance.
    Refused {
        /// Stable refusal category.
        code: LedgerRefusalCode,
        /// Stable explanatory reason.
        reason: String,
    },
}

/// Stable categories for rejected graph, lineage, and ledger operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LedgerRefusalCode {
    /// Identity, digest, scope, or provenance was malformed.
    MalformedEvidence,
    /// An edge relation cannot support its declared identity kinds.
    InvalidRelation,
    /// Evidence would discharge itself circularly.
    CircularEvidence,
    /// A required evidence path is absent.
    MissingEvidencePath,
    /// Lineage provenance or ordering is invalid.
    InvalidLineage,
    /// The closed qualification denominator or vectors are invalid.
    InvalidLedger,
    /// A decision is absent, invalid, or cannot accept this scope.
    InvalidDecision,
}

/// Validates exact graph intake and discharge without executing providers.
#[must_use]
pub fn validate_evidence_graph(graph: &EvidenceGraph, claim: &EvidenceIdentity) -> LedgerOutcome {
    if !valid_kind(claim, EvidenceKind::Claim) {
        return refused(
            LedgerRefusalCode::MalformedEvidence,
            "discharge requires an exact claim identity",
        );
    }
    if graph.edges.iter().any(invalid_edge)
        || graph.edges.iter().any(|edge| !valid_relation(edge))
        || graph.intake.iter().any(invalid_intake)
        || has_cross_kind_reuse(&graph.edges, &graph.intake)
    {
        return refused(
            LedgerRefusalCode::InvalidRelation,
            "malformed, stale, cross-kind, digest-mismatched, or unsupported evidence relation",
        );
    }
    if graph.limitations.iter().any(invalid_limitation) {
        return refused(
            LedgerRefusalCode::MalformedEvidence,
            "limitations must name exact affected claims and redacted fields",
        );
    }
    let limitations: Vec<_> = graph
        .limitations
        .iter()
        .filter(|l| l.affected_claims.iter().any(|c| c == claim))
        .map(|l| l.kind)
        .collect();
    if !limitations.is_empty() {
        return LedgerOutcome::Incomplete { limitations };
    }
    if has_cycle_reaching_claim(&graph.edges, claim) {
        return refused(
            LedgerRefusalCode::CircularEvidence,
            "circular self-evidence cannot discharge a claim",
        );
    }
    let incoming: BTreeSet<_> = graph
        .edges
        .iter()
        .filter(|e| supports_claim(e, claim))
        .map(|e| e.from.kind)
        .collect();
    let kinds = [
        EvidenceKind::Source,
        EvidenceKind::Model,
        EvidenceKind::Binding,
        EvidenceKind::Requirement,
        EvidenceKind::Subject,
        EvidenceKind::Assumption,
        EvidenceKind::Implementation,
        EvidenceKind::Method,
        EvidenceKind::Provider,
        EvidenceKind::ProviderRun,
        EvidenceKind::Artifact,
        EvidenceKind::Decision,
    ];
    if kinds.iter().any(|kind| !incoming.contains(kind)) {
        return refused(
            LedgerRefusalCode::MissingEvidencePath,
            "claim lacks an independently attributed exact evidence path",
        );
    }
    LedgerOutcome::Accounted
}

/// Validates ordered immutable, claim-relative counterexample revisions.
#[must_use]
pub fn validate_lineage(revisions: &[CounterexampleRevision]) -> LedgerOutcome {
    let mut seen = BTreeMap::<&str, &CounterexampleRevision>::new();
    for rev in revisions {
        if rev.identity.is_empty()
            || rev.values_or_trace.is_empty()
            || rev.operation.is_empty()
            || !valid_kind(&rev.subject, EvidenceKind::Subject)
            || !valid_kind(&rev.claim, EvidenceKind::Claim)
            || !valid_kind(&rev.provider, EvidenceKind::Provider)
            || !valid_kind(&rev.run, EvidenceKind::ProviderRun)
        {
            return refused(
                LedgerRefusalCode::InvalidLineage,
                "counterexample revision has malformed exact provenance",
            );
        }
        if rev.limitations.iter().any(invalid_limitation) {
            return refused(
                LedgerRefusalCode::InvalidLineage,
                "counterexample limitation must name exact affected claims",
            );
        }
        if let Some(parent_id) = rev.parent.as_deref() {
            let Some(parent) = seen.get(parent_id) else {
                return refused(
                    LedgerRefusalCode::InvalidLineage,
                    "lineage parent is absent or unordered",
                );
            };
            if rev.correspondence.as_deref().is_none_or(str::is_empty)
                || rev.subject != parent.subject
                || rev.claim != parent.claim
                || rev.domain_valid != parent.domain_valid
                || (rev.replay != parent.replay && rev.replay != ReplayOutcome::Unavailable)
            {
                return refused(
                    LedgerRefusalCode::InvalidLineage,
                    "transformation changes domain, claim, correspondence, or verdict",
                );
            }
        }
        if rev.replay == ReplayOutcome::SameFailure
            && rev
                .limitations
                .iter()
                .any(|l| l.kind != LimitationKind::FutureProductionEvidence)
        {
            return refused(
                LedgerRefusalCode::InvalidLineage,
                "limited replay material cannot be replay-qualified",
            );
        }
        if rev.replay == ReplayOutcome::Unavailable && rev.limitations.is_empty() {
            return refused(
                LedgerRefusalCode::InvalidLineage,
                "unavailable replay must retain an explicit limitation",
            );
        }
        if seen.insert(&rev.identity, rev).is_some() {
            return refused(
                LedgerRefusalCode::InvalidLineage,
                "counterexample revisions are immutable and unique",
            );
        }
    }
    LedgerOutcome::Accounted
}

/// Accounts the exact denominator and independent vectors without gating local development on production artifacts.
#[must_use]
pub fn account_qualification(
    accepted: &[CapabilityIdentity],
    lock: &EvidenceIdentity,
    graph: &EvidenceGraph,
    rows: &[LedgerRow],
    decision: Option<&HumanDecision>,
    authorization_verifier: &dyn DecisionAuthorizationVerifier,
    _scope: &str,
) -> LedgerOutcome {
    let expected: BTreeSet<_> = accepted.iter().collect();
    let actual: BTreeSet<_> = rows.iter().map(|r| &r.capability_identity).collect();
    if accepted.len() != 176
        || expected.len() != 176
        || rows.len() != 176
        || actual.len() != 176
        || expected != actual
    {
        return refused(
            LedgerRefusalCode::InvalidLedger,
            "ledger requires the exact closed 176-capability denominator",
        );
    }
    if !valid_kind(lock, EvidenceKind::EcosystemLock) {
        return refused(
            LedgerRefusalCode::InvalidLedger,
            "ledger requires one exact ecosystem lock and scope",
        );
    }
    if graph.edges.iter().any(invalid_edge)
        || graph.edges.iter().any(|edge| !valid_relation(edge))
        || graph.intake.iter().any(invalid_intake)
        || graph.limitations.iter().any(invalid_limitation)
        || has_cross_kind_reuse(&graph.edges, &graph.intake)
    {
        return refused(
            LedgerRefusalCode::InvalidRelation,
            "malformed, cross-kind, unsupported, or circular evidence cannot discharge a row",
        );
    }
    if has_any_cycle(&graph.edges) {
        return refused(
            LedgerRefusalCode::CircularEvidence,
            "self-produced or circular evidence cannot discharge a row",
        );
    }
    let incomplete = match validate_ledger_rows(rows, lock) {
        Ok(incomplete) => incomplete,
        Err(outcome) => return outcome,
    };
    if incomplete {
        return LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence],
        };
    }
    let scope = qualification_scope(accepted, lock, rows);
    if decision
        .is_some_and(|decision| !valid_decision(decision, graph, authorization_verifier, &scope))
    {
        return refused(
            LedgerRefusalCode::InvalidDecision,
            "decision is not authenticated, attributed, or scoped to this unchanged ledger",
        );
    }
    let Some(decision) = decision else {
        return LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence],
        };
    };
    let Some(claim) = decision_claim(graph, decision) else {
        return refused(
            LedgerRefusalCode::InvalidDecision,
            "decision does not decide one exact qualification claim",
        );
    };
    if let outcome @ LedgerOutcome::Refused { .. } = validate_evidence_graph(graph, claim) {
        return outcome;
    }
    if let LedgerOutcome::Incomplete { .. } = validate_evidence_graph(graph, claim) {
        return LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence],
        };
    }
    if rows
        .iter()
        .flat_map(|row| &row.vectors)
        .any(|vector| !has_vector_evidence(graph, vector, claim))
    {
        return refused(
            LedgerRefusalCode::MissingEvidencePath,
            "a passed vector lacks an exact path to the qualification claim",
        );
    }
    decision_outcome(decision)
}

fn decision_outcome(decision: &HumanDecision) -> LedgerOutcome {
    match decision.disposition {
        DecisionDisposition::Accepted => LedgerOutcome::Accounted,
        DecisionDisposition::Rejected => LedgerOutcome::Rejected {
            decision: decision.identity.clone(),
        },
        DecisionDisposition::Limited => LedgerOutcome::Limited {
            decision: decision.identity.clone(),
        },
    }
}

/// Computes the exact immutable scope identity for a qualification ledger.
#[must_use]
pub fn qualification_scope(
    accepted: &[CapabilityIdentity],
    lock: &EvidenceIdentity,
    rows: &[LedgerRow],
) -> String {
    let mut entries: Vec<_> = accepted
        .iter()
        .map(|capability| capability.0.clone())
        .collect();
    entries.sort();
    entries.push(format!("lock:{:?}:{}", lock.kind, lock.value));
    entries.extend(rows.iter().map(ledger_row_identity));
    entries.sort();
    evidence_digest(entries.join("\n").as_bytes())
}

fn ledger_row_identity(row: &LedgerRow) -> String {
    let mut classes: Vec<_> = row
        .required_claim_classes
        .iter()
        .map(|class| format!("{class:?}"))
        .collect();
    classes.sort();
    let mut vectors: Vec<_> = row.vectors.iter().map(vector_identity).collect();
    vectors.sort();
    format!(
        "{}|{}|{}",
        row.capability_identity.0,
        classes.join(","),
        vectors.join(",")
    )
}

fn vector_identity(vector: &QualificationVector) -> String {
    let identities = [
        &vector.source,
        &vector.package,
        &vector.implementation,
        &vector.method,
        &vector.corpus,
        &vector.ecosystem_lock,
        &vector.run,
        &vector.result,
    ];
    let values: Vec<_> = identities
        .iter()
        .map(|identity| format!("{:?}:{}", identity.kind, identity.value))
        .collect();
    format!(
        "{:?}|{:?}|{}",
        vector.claim_class,
        vector.state,
        values.join("|")
    )
}

fn valid_identity(id: &EvidenceIdentity) -> bool {
    !id.value.is_empty()
}

fn valid_decision(
    decision: &HumanDecision,
    graph: &EvidenceGraph,
    authorization_verifier: &dyn DecisionAuthorizationVerifier,
    scope: &str,
) -> bool {
    valid_kind(&decision.identity, EvidenceKind::Decision)
        && !decision.actor.is_empty()
        && !decision.authority.is_empty()
        && !decision.rationale.is_empty()
        && decision.ledger_scope == scope
        && decision.authorization.decision_identity == decision.identity
        && decision.authorization.actor == decision.actor
        && decision.authorization.authority == decision.authority
        && decision.authorization.ledger_scope == decision.ledger_scope
        && valid_kind(&decision.authorization.attestation, EvidenceKind::Artifact)
        && graph
            .intake
            .iter()
            .any(|intake| intake.identity == decision.identity)
        && graph
            .intake
            .iter()
            .any(|intake| intake.identity == decision.authorization.attestation)
        && authorization_verifier.verifies(decision, &decision.authorization)
}

fn validate_ledger_rows(
    rows: &[LedgerRow],
    lock: &EvidenceIdentity,
) -> Result<bool, LedgerOutcome> {
    let mut incomplete = false;
    let mut result_identities = BTreeSet::new();
    for row in rows {
        let classes: BTreeSet<_> = row.vectors.iter().map(|v| v.claim_class).collect();
        let required: BTreeSet<_> = row.required_claim_classes.iter().copied().collect();
        if classes.len() != row.vectors.len()
            || required.len() != row.required_claim_classes.len()
            || classes != required
        {
            return Err(refused(
                LedgerRefusalCode::InvalidLedger,
                "every applicable claim-class vector occurs exactly once per row",
            ));
        }
        for vector in &row.vectors {
            if !valid_vector(vector, lock) {
                return Err(refused(
                    LedgerRefusalCode::InvalidLedger,
                    "vector lacks an exact source/package/implementation/method/corpus/lock/run/result binding",
                ));
            }
            if !result_identities.insert(&vector.result) {
                return Err(refused(
                    LedgerRefusalCode::InvalidLedger,
                    "an independent capability vector cannot reuse another vector result",
                ));
            }
            incomplete |= vector.state != VectorState::Passed;
        }
    }
    Ok(incomplete)
}
fn valid_kind(id: &EvidenceIdentity, kind: EvidenceKind) -> bool {
    id.kind == kind && valid_identity(id)
}
fn invalid_edge(e: &EvidenceEdge) -> bool {
    !valid_identity(&e.from)
        || !valid_identity(&e.to)
        || e.from == e.to
        || e.scope.is_empty()
        || e.provenance.is_empty()
}
fn invalid_intake(i: &EvidenceIntake) -> bool {
    !valid_identity(&i.identity)
        || i.declared_digest.len() != 64
        || i.declared_digest != evidence_digest(&i.producer_bytes)
        || i.provenance.is_empty()
}

fn invalid_limitation(limitation: &Limitation) -> bool {
    limitation.affected_claims.is_empty()
        || limitation
            .affected_claims
            .iter()
            .any(|claim| !valid_kind(claim, EvidenceKind::Claim))
        || (limitation.kind == LimitationKind::Redacted && limitation.removed_fields.is_empty())
}

/// Returns the lowercase SHA-256 digest of producer bytes inspected at intake.
#[must_use]
pub fn evidence_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
fn supports_claim(edge: &EvidenceEdge, claim: &EvidenceIdentity) -> bool {
    edge.to == *claim
        && match edge.from.kind {
            EvidenceKind::Decision => edge.relation == EvidenceRelation::Decides,
            _ => edge.relation == EvidenceRelation::Supports,
        }
}

fn has_vector_evidence(
    graph: &EvidenceGraph,
    vector: &QualificationVector,
    claim: &EvidenceIdentity,
) -> bool {
    let identities = [
        &vector.source,
        &vector.package,
        &vector.implementation,
        &vector.method,
        &vector.corpus,
        &vector.ecosystem_lock,
        &vector.run,
        &vector.result,
    ];
    identities.iter().all(|identity| {
        graph.edges.iter().any(|edge| {
            edge.from == **identity
                && edge.to == *claim
                && edge.relation == EvidenceRelation::Supports
        })
    })
}

fn decision_claim<'a>(
    graph: &'a EvidenceGraph,
    decision: &HumanDecision,
) -> Option<&'a EvidenceIdentity> {
    let mut claims = graph.edges.iter().filter_map(|edge| {
        (edge.from == decision.identity && edge.relation == EvidenceRelation::Decides)
            .then_some(&edge.to)
    });
    let claim = claims.next()?;
    claims.all(|other| other == claim).then_some(claim)
}

fn valid_relation(edge: &EvidenceEdge) -> bool {
    match edge.relation {
        EvidenceRelation::DerivedFrom => {
            matches!(
                edge.from.kind,
                EvidenceKind::Model
                    | EvidenceKind::Binding
                    | EvidenceKind::Artifact
                    | EvidenceKind::Result
            ) && edge.to.kind == EvidenceKind::Source
        }
        EvidenceRelation::Evaluates => {
            edge.from.kind == EvidenceKind::Method
                && matches!(
                    edge.to.kind,
                    EvidenceKind::Subject | EvidenceKind::Requirement
                )
        }
        EvidenceRelation::Implements => {
            edge.from.kind == EvidenceKind::Implementation
                && matches!(
                    edge.to.kind,
                    EvidenceKind::Requirement | EvidenceKind::Subject
                )
        }
        EvidenceRelation::GeneratedBy => {
            matches!(
                edge.from.kind,
                EvidenceKind::ProviderRun | EvidenceKind::Result | EvidenceKind::Artifact
            ) && matches!(
                edge.to.kind,
                EvidenceKind::Provider | EvidenceKind::ProviderRun
            )
        }
        EvidenceRelation::Replays => {
            edge.from.kind == EvidenceKind::ProviderRun && edge.to.kind == EvidenceKind::Artifact
        }
        EvidenceRelation::Supports | EvidenceRelation::Contradicts => {
            edge.from.kind != EvidenceKind::Decision && edge.to.kind == EvidenceKind::Claim
        }
        EvidenceRelation::Decides => {
            edge.from.kind == EvidenceKind::Decision && edge.to.kind == EvidenceKind::Claim
        }
    }
}

fn has_cross_kind_reuse(edges: &[EvidenceEdge], intake: &[EvidenceIntake]) -> bool {
    let mut kinds = BTreeMap::<&str, EvidenceKind>::new();
    edges
        .iter()
        .flat_map(|edge| [&edge.from, &edge.to])
        .chain(intake.iter().map(|record| &record.identity))
        .any(
            |identity| match kinds.insert(&identity.value, identity.kind) {
                Some(kind) => kind != identity.kind,
                None => false,
            },
        )
}

fn has_any_cycle(edges: &[EvidenceEdge]) -> bool {
    let mut indegree = BTreeMap::<EvidenceIdentity, usize>::new();
    let mut outgoing = BTreeMap::<EvidenceIdentity, Vec<EvidenceIdentity>>::new();
    for edge in edges {
        indegree.entry(edge.from.clone()).or_default();
        *indegree.entry(edge.to.clone()).or_default() += 1;
        outgoing
            .entry(edge.from.clone())
            .or_default()
            .push(edge.to.clone());
    }
    let mut ready = BTreeSet::new();
    for (identity, degree) in &indegree {
        if *degree == 0 {
            ready.insert(identity.clone());
        }
    }
    let mut visited = 0usize;
    while let Some(identity) = ready.pop_first() {
        visited = visited.saturating_add(1);
        for next in outgoing.get(&identity).into_iter().flatten() {
            let Some(degree) = indegree.get_mut(next) else {
                return true;
            };
            *degree -= 1;
            if *degree == 0 {
                ready.insert(next.clone());
            }
        }
    }
    visited != indegree.len()
}

fn has_cycle_reaching_claim(edges: &[EvidenceEdge], claim: &EvidenceIdentity) -> bool {
    let mut relevant = BTreeSet::from([claim.clone()]);
    loop {
        let before = relevant.len();
        for edge in edges {
            if relevant.contains(&edge.to) {
                relevant.insert(edge.from.clone());
            }
        }
        if relevant.len() == before {
            break;
        }
    }
    let upstream: Vec<_> = edges
        .iter()
        .filter(|edge| relevant.contains(&edge.from) && relevant.contains(&edge.to))
        .cloned()
        .collect();
    has_any_cycle(&upstream)
}
fn valid_vector(v: &QualificationVector, lock: &EvidenceIdentity) -> bool {
    valid_kind(&v.source, EvidenceKind::Source)
        && valid_kind(&v.package, EvidenceKind::Package)
        && valid_kind(&v.implementation, EvidenceKind::Implementation)
        && valid_kind(&v.method, EvidenceKind::Method)
        && valid_kind(&v.corpus, EvidenceKind::Corpus)
        && v.ecosystem_lock == *lock
        && valid_kind(&v.run, EvidenceKind::ProviderRun)
        && valid_kind(&v.result, EvidenceKind::Result)
}
fn refused(code: LedgerRefusalCode, reason: &str) -> LedgerOutcome {
    LedgerOutcome::Refused {
        code,
        reason: reason.into(),
    }
}
