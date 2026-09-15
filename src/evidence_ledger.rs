// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Append-only typed evidence intake, counterexample lineage, and ledger accounting.
//! Governing requirements: FR-032 through FR-034 and TC-050 through TC-052.

use std::collections::BTreeSet;

/// Closed identity categories; equal text never permits cross-kind substitution.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EvidenceKind {
    Claim,
    Requirement,
    Subject,
    Assumption,
    Implementation,
    Method,
    ProviderRun,
    Artifact,
    Decision,
}
/// Closed append-only evidence relations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceRelation {
    DerivedFrom,
    Evaluates,
    Implements,
    GeneratedBy,
    Replays,
    Supports,
    Contradicts,
    Decides,
}
/// Exact typed evidence identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EvidenceIdentity {
    pub kind: EvidenceKind,
    pub value: String,
}
/// One immutable evidence edge with exact provenance and scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceEdge {
    pub from: EvidenceIdentity,
    pub to: EvidenceIdentity,
    pub relation: EvidenceRelation,
    pub scope: String,
    pub provenance: String,
}
/// Visible impact of absent or weakened evidence; never a semantic verdict.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Limitation {
    Redacted,
    Lost,
    Inaccessible,
    NonReplayable,
    FutureProductionEvidence,
}
/// Immutable counterexample revision linked to one prior revision or root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CounterexampleRevision {
    pub identity: String,
    pub parent: Option<String>,
    pub subject: EvidenceIdentity,
    pub replay_identity: Option<String>,
}
/// One primary inventory capability row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerRow {
    pub capability_identity: String,
    pub claim_class: String,
    pub evidence: Vec<EvidenceIdentity>,
    pub limitations: Vec<Limitation>,
    pub decision: Option<EvidenceIdentity>,
}
/// Typed intake result, deliberately without evidence-storage ownership.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LedgerOutcome {
    Accounted,
    Incomplete { limitation: Limitation },
    Refused { reason: String },
}
/// Validates graph and ledger invariants without executing providers or deciding release acceptance.
#[must_use]
pub fn account_ledger(
    edges: &[EvidenceEdge],
    revisions: &[CounterexampleRevision],
    rows: &[LedgerRow],
) -> LedgerOutcome {
    if rows.len() != 176
        || rows
            .iter()
            .map(|row| row.capability_identity.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != 176
    {
        return LedgerOutcome::Refused {
            reason: "ledger requires exactly 176 unique primary capability rows".into(),
        };
    }
    if edges.iter().any(|edge| {
        edge.scope.is_empty()
            || edge.provenance.is_empty()
            || edge.from.kind == edge.to.kind && edge.from.value == edge.to.value
    }) {
        return LedgerOutcome::Refused {
            reason: "empty provenance or self evidence edge".into(),
        };
    }
    if revisions.iter().any(|revision| {
        revision.identity.is_empty()
            || revision.parent.as_deref() == Some(revision.identity.as_str())
    }) {
        return LedgerOutcome::Refused {
            reason: "counterexample lineage is not immutable".into(),
        };
    }
    if rows.iter().any(|row| !row.limitations.is_empty()) {
        return LedgerOutcome::Incomplete {
            limitation: rows
                .iter()
                .flat_map(|row| row.limitations.iter())
                .next()
                .copied()
                .unwrap_or(Limitation::FutureProductionEvidence),
        };
    }
    if rows.iter().any(|row| {
        row.decision
            .as_ref()
            .is_some_and(|decision| decision.kind != EvidenceKind::Decision)
    }) {
        return LedgerOutcome::Refused {
            reason: "only attributed decision identities may decide a claim".into(),
        };
    }
    LedgerOutcome::Accounted
}
