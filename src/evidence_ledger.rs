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
    /// Exact ledger scope digest.
    pub ledger_scope: String,
    /// Disposition.
    pub disposition: DecisionDisposition,
    /// Human rationale.
    pub rationale: String,
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
    if graph.edges.iter().any(invalid_edge) || graph.intake.iter().any(invalid_intake) {
        return refused(
            LedgerRefusalCode::MalformedEvidence,
            "malformed, stale, cross-kind, or digest-mismatched evidence",
        );
    }
    if graph.edges.iter().any(|e| {
        e.relation == EvidenceRelation::Decides
            && (!valid_kind(&e.from, EvidenceKind::Decision) || e.to.kind != EvidenceKind::Claim)
    }) {
        return refused(
            LedgerRefusalCode::InvalidRelation,
            "only a decision may decide an exact claim",
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
    scope: &str,
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
    if !valid_kind(lock, EvidenceKind::EcosystemLock) || scope.is_empty() {
        return refused(
            LedgerRefusalCode::InvalidLedger,
            "ledger requires one exact ecosystem lock and scope",
        );
    }
    if graph.edges.iter().any(invalid_edge) || has_any_cycle(&graph.edges) {
        return refused(
            LedgerRefusalCode::CircularEvidence,
            "self-produced or circular evidence cannot discharge a row",
        );
    }
    let mut incomplete = false;
    for row in rows {
        let classes: BTreeSet<_> = row.vectors.iter().map(|v| v.claim_class).collect();
        let required: BTreeSet<_> = row.required_claim_classes.iter().copied().collect();
        if classes.len() != row.vectors.len()
            || required.len() != row.required_claim_classes.len()
            || classes != required
        {
            return refused(
                LedgerRefusalCode::InvalidLedger,
                "every applicable claim-class vector occurs exactly once per row",
            );
        }
        for vector in &row.vectors {
            if !valid_vector(vector, lock) {
                return refused(
                    LedgerRefusalCode::InvalidLedger,
                    "vector lacks an exact source/package/implementation/method/corpus/lock/run/result binding",
                );
            }
            if vector.state == VectorState::Passed && !has_vector_evidence(graph, vector) {
                return refused(
                    LedgerRefusalCode::MissingEvidencePath,
                    "a passed vector lacks retained graph evidence",
                );
            }
            incomplete |= vector.state != VectorState::Passed;
        }
    }
    if incomplete {
        return LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence],
        };
    }
    if let Some(d) = decision
        && (!valid_kind(&d.identity, EvidenceKind::Decision)
            || d.actor.is_empty()
            || d.authority.is_empty()
            || d.rationale.is_empty()
            || d.ledger_scope != scope)
    {
        return refused(
            LedgerRefusalCode::InvalidDecision,
            "decision is not authenticated, attributed, or scoped to this unchanged ledger",
        );
    }
    match decision {
        Some(HumanDecision {
            disposition: DecisionDisposition::Accepted,
            ..
        }) => LedgerOutcome::Accounted,
        Some(_) | None => LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence],
        },
    }
}

fn valid_identity(id: &EvidenceIdentity) -> bool {
    !id.value.is_empty()
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

fn has_vector_evidence(graph: &EvidenceGraph, vector: &QualificationVector) -> bool {
    let identities = [
        &vector.source,
        &vector.package,
        &vector.implementation,
        &vector.method,
        &vector.corpus,
        &vector.run,
        &vector.result,
    ];
    identities.iter().all(|identity| {
        graph
            .edges
            .iter()
            .any(|edge| edge.from == **identity || edge.to == **identity)
    })
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
            let degree = indegree.get_mut(next).expect("edge target is indexed");
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
