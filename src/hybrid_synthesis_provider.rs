// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Non-executing intake of Analyze hybrid/synthesis provider results.
//!
//! Governing requirements: Task-039 and IT-005-SC-04. The Analyze crate is not
//! linked here while its transitive dependency graph has incompatible exact
//! patch pins. This boundary consumes typed producer output without invoking,
//! reinterpreting, or upgrading a candidate into a proof.

pub use quire_analyze_method_contract::{
    AnalyzeAssumptionsIdentity, AnalyzeBoundsIdentity, AnalyzeContentIdentity,
    AnalyzeImplementationIdentity, AnalyzeMethod, AnalyzeOptionsIdentity, AnalyzeOutcome,
    AnalyzeRequestIdentity, AnalyzeResult, AnalyzeResultIdentity, AnalyzeRunIdentity,
    AnalyzeSubjectIdentity, AnalyzeToolchainIdentity, ProviderRevision,
};

/// Shared method-result contract accepted at this provider boundary.
pub const ANALYZE_METHOD_RESULT_VERSION: &str =
    quire_analyze_method_contract::METHOD_RESULT_VERSION;

/// Outcome of consuming one producer result locally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderConsumption {
    /// Evidence is retained, but no proof is promoted.
    RetainedAnalysis,
    /// A candidate is retained for independent checking.
    CandidateRequiresIndependentCheck,
    /// Upstream validation is retained without promotion to a general proof.
    ValidatedCandidateRequiresEvidenceReview,
    /// A candidate was rejected by its independent validator.
    CandidateValidationRejected,
    /// No candidate is a finite search result, not satisfaction.
    NoCandidate,
    /// A bounded provider search did not finish.
    Incomplete,
    /// Provider execution failed without a semantic conclusion.
    Failed,
    /// Provider refused malformed input.
    ProviderRefused,
    /// The upstream result cannot be consumed at this boundary.
    Refused,
}

/// Consumes one exact result without executing Analyze or promoting its claim.
#[must_use]
pub fn consume_analyze_result(result: &AnalyzeResult) -> ProviderConsumption {
    if result.contract_version != ANALYZE_METHOD_RESULT_VERSION
        || result.subject_identity.0.is_empty()
        || result.run_identity.0.is_empty()
        || result.request_identity.0.is_empty()
        || result.implementation_identity.0.is_empty()
        || result.toolchain_identity.0.is_empty()
        || result.options_identity.0.is_empty()
        || result.assumptions_identity.0.is_empty()
        || result.bounds_identity.0.is_empty()
        || result.content_identity.0.is_empty()
        || result.result_identity.0.is_empty()
        || (result.method == AnalyzeMethod::HybridReachability
            && matches!(
                result.outcome,
                AnalyzeOutcome::Candidate
                    | AnalyzeOutcome::ValidatedCandidate
                    | AnalyzeOutcome::ValidationRejected
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
        AnalyzeOutcome::ValidationRejected => ProviderConsumption::CandidateValidationRejected,
        AnalyzeOutcome::NoCandidate => ProviderConsumption::NoCandidate,
        AnalyzeOutcome::Incomplete => ProviderConsumption::Incomplete,
        AnalyzeOutcome::Failed => ProviderConsumption::Failed,
        AnalyzeOutcome::Refused => ProviderConsumption::ProviderRefused,
    }
}
