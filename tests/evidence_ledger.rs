// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

#![allow(missing_docs)]

use ix_trace_rs::trace;
use quire_verification::evidence_ledger::*;

struct FixtureAuthorizationVerifier;

impl DecisionAuthorizationVerifier for FixtureAuthorizationVerifier {
    fn verifies(
        &self,
        decision: &HumanDecision,
        authorization: &VerifiedDecisionAuthorization,
    ) -> bool {
        decision.identity == id(EvidenceKind::Decision, "decision:1")
            && authorization.attestation == id(EvidenceKind::Artifact, "authentication:1")
    }
}

fn verifier() -> FixtureAuthorizationVerifier {
    FixtureAuthorizationVerifier
}

struct RejectingAuthorizationVerifier;

impl DecisionAuthorizationVerifier for RejectingAuthorizationVerifier {
    fn verifies(
        &self,
        _decision: &HumanDecision,
        _authorization: &VerifiedDecisionAuthorization,
    ) -> bool {
        false
    }
}

fn id(kind: EvidenceKind, value: &str) -> EvidenceIdentity {
    EvidenceIdentity {
        kind,
        value: value.into(),
    }
}
fn lock() -> EvidenceIdentity {
    id(EvidenceKind::EcosystemLock, "lock:qspec-8d0fbad")
}
fn vector(class: ClaimClass, state: VectorState) -> QualificationVector {
    QualificationVector {
        claim_class: class,
        source: id(EvidenceKind::Source, "source:1"),
        package: id(EvidenceKind::Package, "package:1"),
        implementation: id(EvidenceKind::Implementation, "implementation:1"),
        method: id(EvidenceKind::Method, "method:1"),
        corpus: id(EvidenceKind::Corpus, "corpus:1"),
        ecosystem_lock: lock(),
        run: id(EvidenceKind::ProviderRun, "run:1"),
        result: id(EvidenceKind::Result, "result:1"),
        state,
    }
}
fn accepted() -> Vec<CapabilityIdentity> {
    (0..176)
        .map(|n| CapabilityIdentity(format!("capability:{n}")))
        .collect()
}
fn rows(state: VectorState) -> Vec<LedgerRow> {
    accepted()
        .into_iter()
        .map(|capability_identity| LedgerRow {
            vectors: vec![QualificationVector {
                result: id(
                    EvidenceKind::Result,
                    &format!("result:{}", capability_identity.0),
                ),
                run: id(
                    EvidenceKind::ProviderRun,
                    &format!("run:{}", capability_identity.0),
                ),
                ..vector(ClaimClass::Grammar, state)
            }],
            capability_identity,
            required_claim_classes: vec![ClaimClass::Grammar],
        })
        .collect()
}

fn passed_vector_graph(_rows: &[LedgerRow]) -> EvidenceGraph {
    let claim = id(EvidenceKind::Claim, "claim:ledger");
    let shared_identities = [
        id(EvidenceKind::Source, "source:1"),
        id(EvidenceKind::Package, "package:1"),
        id(EvidenceKind::Implementation, "implementation:1"),
        id(EvidenceKind::Method, "method:1"),
        id(EvidenceKind::Corpus, "corpus:1"),
        lock(),
        id(EvidenceKind::ProviderRun, "run:1"),
        id(EvidenceKind::Result, "result:1"),
    ];
    EvidenceGraph {
        edges: shared_identities
            .into_iter()
            .map(|from| EvidenceEdge {
                from,
                to: claim.clone(),
                relation: EvidenceRelation::Supports,
                scope: "scope:1".into(),
                provenance: "fixture".into(),
            })
            .collect(),
        intake: vec![],
        limitations: vec![],
    }
}

fn authenticated_decision(scope: &str) -> HumanDecision {
    HumanDecision {
        identity: id(EvidenceKind::Decision, "decision:1"),
        actor: "human:1".into(),
        authority: "qualification-owner".into(),
        authorization: VerifiedDecisionAuthorization {
            decision_identity: id(EvidenceKind::Decision, "decision:1"),
            actor: "human:1".into(),
            authority: "qualification-owner".into(),
            ledger_scope: scope.into(),
            attestation: id(EvidenceKind::Artifact, "authentication:1"),
        },
        ledger_scope: scope.into(),
        disposition: DecisionDisposition::Accepted,
        rationale: "reviewed".into(),
    }
}

fn complete_passed_graph(decision: &HumanDecision) -> EvidenceGraph {
    let claim = id(EvidenceKind::Claim, "claim:ledger");
    let mut graph = passed_vector_graph(&rows(VectorState::Passed));
    graph
        .edges
        .extend(rows(VectorState::Passed).iter().flat_map(|row| {
            let vector = &row.vectors[0];
            [vector.run.clone(), vector.result.clone()]
                .into_iter()
                .map(|from| EvidenceEdge {
                    from,
                    to: claim.clone(),
                    relation: EvidenceRelation::Supports,
                    scope: "scope:1".into(),
                    provenance: "fixture".into(),
                })
        }));
    graph.edges.extend([
        EvidenceEdge {
            from: id(EvidenceKind::Model, "model:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Binding, "binding:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Requirement, "requirement:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Subject, "subject:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Assumption, "assumption:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Provider, "provider:1"),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: decision.authorization.attestation.clone(),
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: decision.identity.clone(),
            to: claim.clone(),
            relation: EvidenceRelation::Decides,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
    ]);
    graph.intake = [
        decision.identity.clone(),
        decision.authorization.attestation.clone(),
    ]
    .into_iter()
    .map(|identity| EvidenceIntake {
        identity,
        declared_digest: evidence_digest(b"attested fixture"),
        producer_bytes: b"attested fixture".to_vec(),
        provenance: "fixture".into(),
    })
    .collect();
    graph
}

/// Tracing: TC-050, FR-032-AC-1, FR-032-AC-2, IT-005.
#[test]
#[trace("TC-050", "FR-032-AC-1", "FR-032-AC-2", "IT-005")]
fn tc_050_refuses_circular_discharge_without_erasing_graph() {
    let claim = id(EvidenceKind::Claim, "claim:1");
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
    let mut edges: Vec<_> = kinds
        .into_iter()
        .map(|kind| EvidenceEdge {
            from: id(kind, &format!("{kind:?}:1")),
            to: claim.clone(),
            relation: if kind == EvidenceKind::Decision {
                EvidenceRelation::Decides
            } else {
                EvidenceRelation::Supports
            },
            scope: "scope".into(),
            provenance: "producer".into(),
        })
        .collect();
    let mut graph = EvidenceGraph {
        edges: edges.clone(),
        intake: vec![EvidenceIntake {
            identity: id(EvidenceKind::Artifact, "artifact:1"),
            declared_digest: evidence_digest(b"artifact bytes"),
            producer_bytes: b"artifact bytes".to_vec(),
            provenance: "producer".into(),
        }],
        limitations: vec![],
    };
    assert_eq!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Accounted
    );
    let mut contradictory = graph.clone();
    contradictory.edges[0].relation = EvidenceRelation::Contradicts;
    assert!(matches!(
        validate_evidence_graph(&contradictory, &claim),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::MissingEvidencePath,
            ..
        }
    ));
    let mut unsupported_relation = graph.clone();
    unsupported_relation.edges.push(EvidenceEdge {
        from: id(EvidenceKind::Artifact, "artifact:unsupported"),
        to: claim.clone(),
        relation: EvidenceRelation::Implements,
        scope: "scope".into(),
        provenance: "producer".into(),
    });
    assert!(matches!(
        validate_evidence_graph(&unsupported_relation, &claim),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::InvalidRelation,
            ..
        }
    ));
    let mut digest_mismatch = graph.clone();
    digest_mismatch.intake[0].declared_digest = "0".repeat(64);
    assert!(matches!(
        validate_evidence_graph(&digest_mismatch, &claim),
        LedgerOutcome::Refused { .. }
    ));
    let cycle = id(EvidenceKind::Claim, "claim:cycle");
    edges.extend([
        EvidenceEdge {
            from: claim.clone(),
            to: cycle.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope".into(),
            provenance: "producer".into(),
        },
        EvidenceEdge {
            from: cycle,
            to: claim.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope".into(),
            provenance: "producer".into(),
        },
    ]);
    graph.edges = edges;
    assert!(matches!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::CircularEvidence,
            ..
        }
    ));
    assert_eq!(graph.edges.len(), 14);
}

/// Tracing: TC-050, FR-032-AC-3, IT-005.
#[test]
#[trace("TC-050", "FR-032-AC-3", "IT-005")]
fn tc_050_keeps_redaction_claim_specific() {
    let claim = id(EvidenceKind::Claim, "claim:1");
    let graph = EvidenceGraph {
        edges: vec![],
        intake: vec![],
        limitations: vec![Limitation {
            kind: LimitationKind::Redacted,
            affected_claims: vec![claim.clone()],
            removed_fields: vec!["trace.secret".into()],
        }],
    };
    assert_eq!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::Redacted]
        }
    );
}

/// Tracing: TC-050, FR-032-AC-2, FR-032-AC-3, IT-005.
#[test]
#[trace("TC-050", "FR-032-AC-2", "FR-032-AC-3", "IT-005")]
fn tc_050_retains_sibling_claim_when_an_unrelated_cycle_exists() {
    let claim = id(EvidenceKind::Claim, "claim:target");
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
    let mut edges: Vec<_> = kinds
        .into_iter()
        .map(|kind| EvidenceEdge {
            from: id(kind, &format!("{kind:?}:target")),
            to: claim.clone(),
            relation: if kind == EvidenceKind::Decision {
                EvidenceRelation::Decides
            } else {
                EvidenceRelation::Supports
            },
            scope: "scope".into(),
            provenance: "p".into(),
        })
        .collect();
    edges.extend([
        EvidenceEdge {
            from: id(EvidenceKind::Claim, "cycle:a"),
            to: id(EvidenceKind::Claim, "cycle:b"),
            relation: EvidenceRelation::Supports,
            scope: "scope".into(),
            provenance: "p".into(),
        },
        EvidenceEdge {
            from: id(EvidenceKind::Claim, "cycle:b"),
            to: id(EvidenceKind::Claim, "cycle:a"),
            relation: EvidenceRelation::Supports,
            scope: "scope".into(),
            provenance: "p".into(),
        },
    ]);
    let graph = EvidenceGraph {
        edges,
        intake: vec![],
        limitations: vec![],
    };
    assert_eq!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Accounted
    );
    assert_eq!(graph.edges.len(), 14);
}

/// Tracing: TC-052, FR-033-AC-1, FR-033-AC-2, FR-033-AC-3, IT-005.
#[test]
#[trace("TC-052", "FR-033-AC-1", "FR-033-AC-2", "FR-033-AC-3", "IT-005")]
fn tc_052_preserves_ordered_lineage_and_refuses_verdict_change() {
    let root = CounterexampleRevision {
        identity: "root".into(),
        parent: None,
        subject: id(EvidenceKind::Subject, "subject:1"),
        claim: id(EvidenceKind::Claim, "claim:1"),
        values_or_trace: "trace:root".into(),
        provider: id(EvidenceKind::Provider, "provider:1"),
        run: id(EvidenceKind::ProviderRun, "run:1"),
        operation: "original".into(),
        correspondence: None,
        domain_valid: true,
        replay: ReplayOutcome::SameFailure,
        limitations: vec![],
    };
    let mut child = root.clone();
    child.identity = "child".into();
    child.parent = Some("root".into());
    child.values_or_trace = "trace:shrunk".into();
    child.operation = "shrink".into();
    child.correspondence = Some("map:v1".into());
    assert_eq!(
        validate_lineage(&[root.clone(), child.clone()]),
        LedgerOutcome::Accounted
    );
    child.replay = ReplayOutcome::Unavailable;
    child.limitations = vec![Limitation {
        kind: LimitationKind::NonReplayable,
        affected_claims: vec![child.claim.clone()],
        removed_fields: vec![],
    }];
    assert_eq!(
        validate_lineage(&[root.clone(), child.clone()]),
        LedgerOutcome::Accounted
    );
    child.limitations.clear();
    assert!(matches!(
        validate_lineage(&[root, child]),
        LedgerOutcome::Refused { .. }
    ));
}

/// Tracing: TC-051, FR-034-AC-1, FR-034-AC-2, FR-034-AC-3, IT-005.
#[test]
#[trace("TC-051", "FR-034-AC-1", "FR-034-AC-2", "FR-034-AC-3", "IT-005")]
// One traced adversarial scenario keeps all FR-034 acceptance mutations together.
#[allow(clippy::too_many_lines)]
fn tc_051_accounts_closed_vectors_without_production_gate() {
    let graph = EvidenceGraph {
        edges: vec![],
        intake: vec![],
        limitations: vec![],
    };
    assert_eq!(
        account_qualification(
            &accepted(),
            &lock(),
            &passed_vector_graph(&rows(VectorState::Planned)),
            &rows(VectorState::Planned),
            None,
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence]
        }
    );
    let passed = rows(VectorState::Passed);
    let scope = qualification_scope(&accepted(), &lock(), &passed);
    let decision = authenticated_decision(&scope);
    assert_eq!(
        account_qualification(
            &accepted(),
            &lock(),
            &complete_passed_graph(&decision),
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Accounted
    );
    let mut limited = decision.clone();
    limited.disposition = DecisionDisposition::Limited;
    assert_eq!(
        account_qualification(
            &accepted(),
            &lock(),
            &complete_passed_graph(&limited),
            &rows(VectorState::Passed),
            Some(&limited),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Limited {
            decision: limited.identity.clone()
        }
    );
    let mut missing_package = complete_passed_graph(&decision);
    missing_package
        .edges
        .retain(|edge| edge.from != id(EvidenceKind::Package, "package:1"));
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &missing_package,
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::MissingEvidencePath,
            ..
        }
    ));
    let mut missing_lock = complete_passed_graph(&decision);
    missing_lock.edges.retain(|edge| edge.from != lock());
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &missing_lock,
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::MissingEvidencePath,
            ..
        }
    ));
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &complete_passed_graph(&decision),
            &rows(VectorState::Passed),
            Some(&decision),
            &RejectingAuthorizationVerifier,
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::InvalidDecision,
            ..
        }
    ));
    let mut cyclic = complete_passed_graph(&decision);
    let cycle_a = id(EvidenceKind::Claim, "claim:cycle-a");
    let cycle_b = id(EvidenceKind::Claim, "claim:cycle-b");
    cyclic.edges.extend([
        EvidenceEdge {
            from: cycle_a.clone(),
            to: cycle_b.clone(),
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
        EvidenceEdge {
            from: cycle_b,
            to: cycle_a,
            relation: EvidenceRelation::Supports,
            scope: "scope:1".into(),
            provenance: "fixture".into(),
        },
    ]);
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &cyclic,
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::CircularEvidence,
            ..
        }
    ));
    let mut duplicate_result = rows(VectorState::Passed);
    duplicate_result[1].vectors[0].result = duplicate_result[0].vectors[0].result.clone();
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &complete_passed_graph(&decision),
            &duplicate_result,
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::InvalidLedger,
            ..
        }
    ));
    let mut unsupported_relation = complete_passed_graph(&decision);
    unsupported_relation.edges.push(EvidenceEdge {
        from: id(EvidenceKind::Artifact, "artifact:unsupported"),
        to: id(EvidenceKind::Claim, "claim:ledger"),
        relation: EvidenceRelation::Implements,
        scope: "scope:1".into(),
        provenance: "fixture".into(),
    });
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &unsupported_relation,
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::InvalidRelation,
            ..
        }
    ));
    let mut missing_authentication = complete_passed_graph(&decision);
    missing_authentication
        .intake
        .retain(|record| record.identity != decision.authorization.attestation);
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &missing_authentication,
            &rows(VectorState::Passed),
            Some(&decision),
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused {
            code: LedgerRefusalCode::InvalidDecision,
            ..
        }
    ));
    let mut duplicate = rows(VectorState::Passed);
    duplicate[175].capability_identity = CapabilityIdentity("capability:0".into());
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &graph,
            &duplicate,
            None,
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused { .. }
    ));
    let mut missing_vector = rows(VectorState::Passed);
    missing_vector[0]
        .required_claim_classes
        .push(ClaimClass::Backend);
    assert!(matches!(
        account_qualification(
            &accepted(),
            &lock(),
            &graph,
            &missing_vector,
            None,
            &verifier(),
            "scope:1"
        ),
        LedgerOutcome::Refused { .. }
    ));
}

/// Trace: TC-051, FR-034-AC-3.
#[test]
fn tc_051_scope_framing_separates_delimited_opaque_identities() {
    let mut left = vector(ClaimClass::Grammar, VectorState::Passed);
    left.source = id(EvidenceKind::Source, "a|Package:b");
    left.package = id(EvidenceKind::Package, "c");
    let mut right = vector(ClaimClass::Grammar, VectorState::Passed);
    right.source = id(EvidenceKind::Source, "a");
    right.package = id(EvidenceKind::Package, "b|Package:c");
    let capability = CapabilityIdentity("capability:scope".into());
    let left_rows = vec![LedgerRow {
        capability_identity: capability.clone(),
        required_claim_classes: vec![ClaimClass::Grammar],
        vectors: vec![left],
    }];
    let right_rows = vec![LedgerRow {
        capability_identity: capability.clone(),
        required_claim_classes: vec![ClaimClass::Grammar],
        vectors: vec![right],
    }];
    assert_ne!(
        qualification_scope(std::slice::from_ref(&capability), &lock(), &left_rows),
        qualification_scope(&[capability], &lock(), &right_rows),
    );
}
