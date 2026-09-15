// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

#![allow(missing_docs)]

use ix_trace_rs::trace;
use quire_verification::hybrid_synthesis_provider::{
    ANALYZE_METHOD_RESULT_VERSION, ANALYZE_PROVIDER_REVISION, AnalyzeAssumptionsIdentity,
    AnalyzeBoundsIdentity, AnalyzeContentIdentity, AnalyzeImplementationIdentity, AnalyzeMethod,
    AnalyzeOptionsIdentity, AnalyzeOutcome, AnalyzeRequestIdentity, AnalyzeResult,
    AnalyzeResultIdentity, AnalyzeRunIdentity, AnalyzeSubjectIdentity, AnalyzeToolchainIdentity,
    ProviderConsumption, ProviderRevision, consume_analyze_result,
};

fn result(method: AnalyzeMethod, outcome: AnalyzeOutcome) -> AnalyzeResult {
    AnalyzeResult {
        contract_version: ANALYZE_METHOD_RESULT_VERSION.into(),
        provider_revision: ProviderRevision(ANALYZE_PROVIDER_REVISION.into()),
        method,
        outcome,
        subject_identity: AnalyzeSubjectIdentity("subject:hybrid-fixture".into()),
        run_identity: AnalyzeRunIdentity("run:analyze-fixture".into()),
        request_identity: AnalyzeRequestIdentity("request:hybrid-fixture".into()),
        implementation_identity: AnalyzeImplementationIdentity("implementation:analyze".into()),
        toolchain_identity: AnalyzeToolchainIdentity("toolchain:rust-1.98.1".into()),
        options_identity: AnalyzeOptionsIdentity("options:hybrid-fixture".into()),
        assumptions_identity: AnalyzeAssumptionsIdentity("assumptions:hybrid-fixture".into()),
        bounds_identity: AnalyzeBoundsIdentity("bounds:hybrid-fixture".into()),
        content_identity: AnalyzeContentIdentity("content:hybrid-fixture".into()),
        result_identity: AnalyzeResultIdentity("result:analyze-fixture".into()),
    }
}

/// Tracing: IT-005, Task-039.
#[test]
#[trace("IT-005")]
fn it_005_sc_04_keeps_candidate_and_validation_distinct_from_proof() {
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::Candidate
        )),
        ProviderConsumption::CandidateRequiresIndependentCheck
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::ValidatedCandidate
        )),
        ProviderConsumption::ValidatedCandidateRequiresEvidenceReview
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::ValidationRejected
        )),
        ProviderConsumption::CandidateValidationRejected
    );
}

/// Tracing: IT-005, Task-039.
#[test]
#[trace("IT-005")]
fn it_005_sc_04_retains_nonconclusions_and_refuses_wrong_revision() {
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::HybridReachability,
            AnalyzeOutcome::Incomplete
        )),
        ProviderConsumption::NonConclusive
    );
    let mut stale = result(AnalyzeMethod::HybridReachability, AnalyzeOutcome::Enclosure);
    stale.provider_revision = ProviderRevision("stale".into());
    assert_eq!(consume_analyze_result(&stale), ProviderConsumption::Refused);
    let mut incomplete_envelope =
        result(AnalyzeMethod::HybridReachability, AnalyzeOutcome::Enclosure);
    incomplete_envelope.bounds_identity = AnalyzeBoundsIdentity(String::new());
    assert_eq!(
        consume_analyze_result(&incomplete_envelope),
        ProviderConsumption::Refused
    );
}
