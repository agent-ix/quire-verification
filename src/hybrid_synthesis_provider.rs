// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Pinned, non-executing intake of Analyze hybrid/synthesis provider results.
//!
//! Governing requirements: Task-039 and IT-005-SC-04. The Analyze crate is not
//! linked here while its transitive dependency graph has incompatible exact
//! patch pins. This boundary consumes typed producer output without invoking,
//! reinterpreting, or upgrading a candidate into a proof.

/// Immutable upstream provider revision identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProviderRevision(pub String);

/// Exact semantic subject identity supplied by the Analyze provider.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnalyzeSubjectIdentity(pub String);

/// Exact upstream provider run identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnalyzeRunIdentity(pub String);

/// Exact immutable upstream result identity.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AnalyzeResultIdentity(pub String);

/// Merged Analyze revision supplying hybrid/synthesis development results.
pub const ANALYZE_PROVIDER_REVISION: &str = "26a46969c4c9163ba80b5dedb3059bdcd4031946";

/// The distinct provider methods consumed at this boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalyzeMethod {
    /// Sound bounded hybrid reachability enclosure.
    HybridReachability,
    /// Bounded canonical synthesis search.
    CanonicalSynthesis,
}

/// Typed upstream terminal outcome, never inferred from human-readable output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalyzeOutcome {
    /// An enclosure with declared error; it remains analysis evidence.
    Enclosure,
    /// A discovered candidate requiring independent checking.
    Candidate,
    /// A candidate has upstream validation evidence, not a general proof.
    ValidatedCandidate,
    /// The finite candidate space was exhausted.
    NoCandidate,
    /// A bounded analysis/search did not finish.
    Incomplete,
    /// The provider failed without a semantic conclusion.
    Failed,
    /// The provider refused malformed input.
    Refused,
}

/// Exact producer output supplied to the Verification provider boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalyzeResult {
    /// Revision that produced this result.
    pub provider_revision: ProviderRevision,
    /// Exact provider method.
    pub method: AnalyzeMethod,
    /// Typed terminal outcome.
    pub outcome: AnalyzeOutcome,
    /// Exact semantic subject identity.
    pub subject_identity: AnalyzeSubjectIdentity,
    /// Exact provider run identity.
    pub run_identity: AnalyzeRunIdentity,
    /// Immutable result/artifact identity.
    pub result_identity: AnalyzeResultIdentity,
}

/// Outcome of consuming one producer result locally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderConsumption {
    /// Evidence is retained, but no proof is promoted.
    RetainedAnalysis,
    /// A candidate is retained for independent checking.
    CandidateRequiresIndependentCheck,
    /// Upstream validation is retained without promotion to a general proof.
    ValidatedCandidateRequiresEvidenceReview,
    /// No candidate is a finite search result, not satisfaction.
    NoCandidate,
    /// Non-conclusive provider states remain visible.
    NonConclusive,
    /// The upstream result cannot be consumed at this pin/boundary.
    Refused,
}

/// Consumes one exact result without executing Analyze or promoting its claim.
#[must_use]
pub fn consume_analyze_result(result: &AnalyzeResult) -> ProviderConsumption {
    if result.provider_revision.0 != ANALYZE_PROVIDER_REVISION
        || result.subject_identity.0.is_empty()
        || result.run_identity.0.is_empty()
        || result.result_identity.0.is_empty()
        || (result.method == AnalyzeMethod::HybridReachability
            && matches!(
                result.outcome,
                AnalyzeOutcome::Candidate
                    | AnalyzeOutcome::ValidatedCandidate
                    | AnalyzeOutcome::NoCandidate
            ))
        || (result.method == AnalyzeMethod::CanonicalSynthesis
            && result.outcome == AnalyzeOutcome::Enclosure)
    {
        return ProviderConsumption::Refused;
    }
    match result.outcome {
        AnalyzeOutcome::Enclosure => ProviderConsumption::RetainedAnalysis,
        AnalyzeOutcome::Candidate => ProviderConsumption::CandidateRequiresIndependentCheck,
        AnalyzeOutcome::ValidatedCandidate => {
            ProviderConsumption::ValidatedCandidateRequiresEvidenceReview
        }
        AnalyzeOutcome::NoCandidate => ProviderConsumption::NoCandidate,
        AnalyzeOutcome::Incomplete | AnalyzeOutcome::Failed | AnalyzeOutcome::Refused => {
            ProviderConsumption::NonConclusive
        }
    }
}
