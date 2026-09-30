// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

#![allow(missing_docs)]

use ix_trace_rs::trace;
use quire_verification::hybrid_synthesis_provider::{
    ANALYZE_METHOD_RESULT_VERSION, AnalyzeAssumptionsIdentity, AnalyzeBoundsIdentity,
    AnalyzeContentIdentity, AnalyzeMethod, AnalyzeOptionsIdentity, AnalyzeOutcome,
    AnalyzeRequestIdentity, AnalyzeResult, AnalyzeResultIdentity, AnalyzeRunIdentity,
    AnalyzeSubjectIdentity, ProviderConsumption, consume_analyze_result,
};

fn result(method: AnalyzeMethod, outcome: AnalyzeOutcome) -> AnalyzeResult {
    AnalyzeResult {
        contract_version: ANALYZE_METHOD_RESULT_VERSION.into(),
        method,
        outcome,
        subject_identity: AnalyzeSubjectIdentity("subject:hybrid-fixture".into()),
        run_identity: AnalyzeRunIdentity("run:analyze-fixture".into()),
        request_identity: AnalyzeRequestIdentity("request:hybrid-fixture".into()),
        options_identity: AnalyzeOptionsIdentity("options:hybrid-fixture".into()),
        assumptions_identity: AnalyzeAssumptionsIdentity("assumptions:hybrid-fixture".into()),
        bounds_identity: AnalyzeBoundsIdentity("bounds:hybrid-fixture".into()),
        content_identity: AnalyzeContentIdentity("content:hybrid-fixture".into()),
        result_identity: AnalyzeResultIdentity("result:analyze-fixture".into()),
    }
}

/// Covers provider intake classification under the IT-005 integration family.
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

/// Covers typed provider failure classification under the IT-005 integration family.
#[test]
#[trace("IT-005")]
fn it_005_sc_04_retains_nonconclusions_and_refuses_malformed_results() {
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::HybridReachability,
            AnalyzeOutcome::Incomplete
        )),
        ProviderConsumption::Incomplete
    );
    let mut incomplete_envelope =
        result(AnalyzeMethod::HybridReachability, AnalyzeOutcome::Enclosure);
    incomplete_envelope.bounds_identity = AnalyzeBoundsIdentity(String::new());
    assert_eq!(
        consume_analyze_result(&incomplete_envelope),
        ProviderConsumption::Refused
    );
    let mut wrong_contract = result(AnalyzeMethod::HybridReachability, AnalyzeOutcome::Enclosure);
    wrong_contract.contract_version = "quire.method-result/v0".into();
    assert_eq!(
        consume_analyze_result(&wrong_contract),
        ProviderConsumption::Refused
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::HybridReachability,
            AnalyzeOutcome::Candidate
        )),
        ProviderConsumption::Refused
    );
}

/// Covers the provider-side IT-005 scenario.
#[test]
#[trace("IT-005")]
fn it_005_sc_04_keeps_every_terminal_outcome_typed() {
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::NoCandidate
        )),
        ProviderConsumption::NoCandidate
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::Incomplete
        )),
        ProviderConsumption::Incomplete
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::Failed
        )),
        ProviderConsumption::Failed
    );
    assert_eq!(
        consume_analyze_result(&result(
            AnalyzeMethod::CanonicalSynthesis,
            AnalyzeOutcome::Refused
        )),
        ProviderConsumption::ProviderRefused
    );
}
