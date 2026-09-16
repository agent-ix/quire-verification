// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! IT-005-SC-04 through SC-07 development integration coverage.

// Integration-test fixture helpers are private and intentionally not documented.
#![allow(missing_docs)]

use ix_trace_rs::trace;
use quire_verification::{
    evidence_ledger::{
        CapabilityIdentity, ClaimClass, DecisionDisposition, EvidenceEdge, EvidenceGraph,
        EvidenceIdentity, EvidenceKind, EvidenceRelation, HumanDecision, LedgerOutcome,
        LedgerRefusalCode, LedgerRow, QualificationVector, VectorState, account_qualification,
        validate_evidence_graph,
    },
    hybrid_synthesis_provider::{
        ANALYZE_METHOD_RESULT_VERSION, ANALYZE_PROVIDER_REVISION, AnalyzeAssumptionsIdentity,
        AnalyzeBoundsIdentity, AnalyzeContentIdentity, AnalyzeImplementationIdentity,
        AnalyzeMethod, AnalyzeOptionsIdentity, AnalyzeOutcome, AnalyzeRequestIdentity,
        AnalyzeResult, AnalyzeResultIdentity, AnalyzeRunIdentity, AnalyzeSubjectIdentity,
        AnalyzeToolchainIdentity, ProviderConsumption, ProviderRevision, consume_analyze_result,
    },
};

struct FixtureAuthorizationVerifier;

impl quire_verification::evidence_ledger::DecisionAuthorizationVerifier
    for FixtureAuthorizationVerifier
{
    fn verifies(
        &self,
        decision: &HumanDecision,
        authorization: &quire_verification::evidence_ledger::VerifiedDecisionAuthorization,
    ) -> bool {
        decision.identity == id(EvidenceKind::Decision, "decision:it-005")
            && authorization.attestation == id(EvidenceKind::Artifact, "authentication:it-005")
    }
}

fn verifier() -> FixtureAuthorizationVerifier {
    FixtureAuthorizationVerifier
}

fn id(kind: EvidenceKind, value: &str) -> EvidenceIdentity {
    EvidenceIdentity {
        kind,
        value: value.into(),
    }
}

fn graph_for(claim: &EvidenceIdentity) -> EvidenceGraph {
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
    EvidenceGraph {
        edges: kinds
            .into_iter()
            .map(|kind| EvidenceEdge {
                from: id(kind, &format!("{kind:?}:1")),
                to: claim.clone(),
                relation: if kind == EvidenceKind::Decision {
                    EvidenceRelation::Decides
                } else {
                    EvidenceRelation::Supports
                },
                scope: "it-005".into(),
                provenance: "fixture".into(),
            })
            .collect(),
        intake: vec![],
        limitations: vec![],
    }
}

fn rejected_candidate() -> AnalyzeResult {
    AnalyzeResult {
        contract_version: ANALYZE_METHOD_RESULT_VERSION.into(),
        provider_revision: ProviderRevision(ANALYZE_PROVIDER_REVISION.into()),
        method: AnalyzeMethod::CanonicalSynthesis,
        outcome: AnalyzeOutcome::ValidationRejected,
        subject_identity: AnalyzeSubjectIdentity("subject:it-005".into()),
        run_identity: AnalyzeRunIdentity("run:it-005".into()),
        request_identity: AnalyzeRequestIdentity("request:it-005".into()),
        implementation_identity: AnalyzeImplementationIdentity("implementation:analyze".into()),
        toolchain_identity: AnalyzeToolchainIdentity("toolchain:rust-1.98.1".into()),
        options_identity: AnalyzeOptionsIdentity("options:it-005".into()),
        assumptions_identity: AnalyzeAssumptionsIdentity("assumptions:it-005".into()),
        bounds_identity: AnalyzeBoundsIdentity("bounds:it-005".into()),
        content_identity: AnalyzeContentIdentity("content:it-005".into()),
        result_identity: AnalyzeResultIdentity("result:it-005".into()),
    }
}

fn lock() -> EvidenceIdentity {
    id(EvidenceKind::EcosystemLock, "lock:it-005")
}

fn rows() -> Vec<LedgerRow> {
    (0..176)
        .map(|number| LedgerRow {
            capability_identity: CapabilityIdentity(format!("capability:{number}")),
            required_claim_classes: vec![ClaimClass::Analysis],
            vectors: vec![QualificationVector {
                claim_class: ClaimClass::Analysis,
                source: id(EvidenceKind::Source, "source:it-005"),
                package: id(EvidenceKind::Package, "package:it-005"),
                implementation: id(EvidenceKind::Implementation, "implementation:it-005"),
                method: id(EvidenceKind::Method, "method:it-005"),
                corpus: id(EvidenceKind::Corpus, "corpus:it-005"),
                ecosystem_lock: lock(),
                run: id(EvidenceKind::ProviderRun, &format!("run:it-005:{number}")),
                result: id(EvidenceKind::Result, &format!("result:it-005:{number}")),
                state: VectorState::Planned,
            }],
        })
        .collect()
}

fn passed_rows() -> Vec<LedgerRow> {
    let mut rows = rows();
    for row in &mut rows {
        row.vectors[0].state = VectorState::Passed;
    }
    rows
}

// This fixture spells out the complete cross-boundary evidence graph for IT-005.
#[allow(clippy::too_many_lines)]
fn graph_for_passed_rows(rows: &[LedgerRow], decision: &HumanDecision) -> EvidenceGraph {
    let claim = id(EvidenceKind::Claim, "claim:ledger");
    let vector = &rows[0].vectors[0];
    let shared_identities = [
        vector.source.clone(),
        vector.package.clone(),
        vector.implementation.clone(),
        vector.method.clone(),
        vector.corpus.clone(),
        vector.ecosystem_lock.clone(),
        vector.run.clone(),
        vector.result.clone(),
    ];
    let mut edges: Vec<_> = shared_identities
        .into_iter()
        .map(|from| EvidenceEdge {
            from,
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        })
        .collect();
    edges.extend(rows.iter().flat_map(|row| {
        let vector = &row.vectors[0];
        [vector.run.clone(), vector.result.clone()]
            .into_iter()
            .map(|from| EvidenceEdge {
                from,
                to: claim.clone(),
                relation: EvidenceRelation::Supports,
                scope: "it-005".into(),
                provenance: "fixture".into(),
            })
    }));
    edges.extend([
        EvidenceEdge {
            from: id(EvidenceKind::Model, "model:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Binding, "binding:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Requirement, "requirement:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Subject, "subject:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Assumption, "assumption:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Provider, "provider:it-005"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: decision.authorization.attestation.clone(),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: decision.identity.clone(),
            to: claim,
            relation: EvidenceRelation::Decides,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
    ]);
    EvidenceGraph {
        edges,
        intake: [
            decision.identity.clone(),
            decision.authorization.attestation.clone(),
        ]
        .into_iter()
        .map(
            |identity| quire_verification::evidence_ledger::EvidenceIntake {
                identity,
                declared_digest: quire_verification::evidence_ledger::evidence_digest(
                    b"attested fixture",
                ),
                producer_bytes: b"attested fixture".to_vec(),
                provenance: "fixture".into(),
            },
        )
        .collect(),
        limitations: vec![],
    }
}

/// Covers the candidate-rejection and circular-discharge integration cases.
#[test]
#[trace("IT-005", "FR-032-AC-2")]
fn it_005_sc_04_05_rejects_candidate_and_refuses_circular_discharge() {
    assert_eq!(
        consume_analyze_result(&rejected_candidate()),
        ProviderConsumption::CandidateValidationRejected
    );
    let claim = id(EvidenceKind::Claim, "claim:it-005");
    let mut graph = graph_for(&claim);
    assert_eq!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Accounted
    );
    let cycle = id(EvidenceKind::Claim, "claim:it-005-cycle");
    graph.edges.extend([
        EvidenceEdge {
            from: claim.clone(),
            to: cycle.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: cycle,
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "it-005".into(),
            provenance: "fixture".into(),
        },
    ]);
    assert!(matches!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::CircularEvidence,
            ..
        }
    ));
    assert_eq!(graph.edges.len(), 14);
}

/// Covers all-row retention and non-blocking future-evidence integration cases.
#[test]
#[trace("IT-005", "FR-034-AC-1", "FR-034-AC-3")]
// The integration scenario keeps its sequential qualification state transitions observable.
#[allow(clippy::too_many_lines)]
fn it_005_sc_06_07_keeps_all_rows_and_future_evidence_nonblocking() {
    let accepted: Vec<_> = (0..176)
        .map(|number| CapabilityIdentity(format!("capability:{number}")))
        .collect();
    let graph = EvidenceGraph {
        edges: vec![],
        intake: vec![],
        limitations: vec![],
    };
    let retained = rows();
    assert_eq!(
        account_qualification(
            &accepted,
            &lock(),
            &graph,
            &retained,
            None,
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Incomplete {
            limitations: vec![
                quire_verification::evidence_ledger::LimitationKind::FutureProductionEvidence
            ]
        }
    );
    assert_eq!(retained.len(), 176);
    let mut duplicate = retained.clone();
    duplicate[175].capability_identity = CapabilityIdentity("capability:0".into());
    assert!(matches!(
        account_qualification(
            &accepted,
            &lock(),
            &graph,
            &duplicate,
            None,
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Refused { .. }
    ));
    let mut passed = passed_rows();
    let scope =
        quire_verification::evidence_ledger::qualification_scope(&accepted, &lock(), &passed)
            .expect("fixture ledger scope fits canonical framing");
    let decision = HumanDecision {
        identity: id(EvidenceKind::Decision, "decision:it-005"),
        actor: "reviewer".into(),
        authority: "owner".into(),
        authorization: quire_verification::evidence_ledger::VerifiedDecisionAuthorization {
            decision_identity: id(EvidenceKind::Decision, "decision:it-005"),
            actor: "reviewer".into(),
            authority: "owner".into(),
            ledger_scope: scope.clone(),
            attestation: id(EvidenceKind::Artifact, "authentication:it-005"),
        },
        ledger_scope: scope,
        disposition: DecisionDisposition::Accepted,
        rationale: "fixture".into(),
    };
    let evidence = graph_for_passed_rows(&passed, &decision);
    assert_eq!(
        account_qualification(
            &accepted,
            &lock(),
            &evidence,
            &passed,
            Some(&decision),
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Accounted
    );
    passed[0].vectors.clear();
    assert!(matches!(
        account_qualification(
            &accepted,
            &lock(),
            &evidence,
            &passed,
            Some(&decision),
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Refused { .. }
    ));
    let mut wrong_lock = passed_rows();
    wrong_lock[0].vectors[0].ecosystem_lock = id(EvidenceKind::EcosystemLock, "lock:wrong");
    assert!(matches!(
        account_qualification(
            &accepted,
            &lock(),
            &evidence,
            &wrong_lock,
            Some(&decision),
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Refused { .. }
    ));
    let rejected = HumanDecision {
        disposition: DecisionDisposition::Rejected,
        ..decision
    };
    assert!(matches!(
        account_qualification(
            &accepted,
            &lock(),
            &evidence,
            &passed_rows(),
            Some(&rejected),
            &verifier(),
            "it-005"
        ),
        LedgerOutcome::Rejected { .. }
    ));
}
