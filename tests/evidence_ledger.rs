// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

#![allow(missing_docs)]

use ix_trace_rs::trace;
use quire_verification::evidence_ledger::*;

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
fn accepted() -> Vec<String> {
    (0..176).map(|n| format!("capability:{n}")).collect()
}
fn rows(state: VectorState) -> Vec<LedgerRow> {
    accepted()
        .into_iter()
        .map(|capability_identity| LedgerRow {
            capability_identity,
            required_claim_classes: vec![ClaimClass::Grammar],
            vectors: vec![vector(ClaimClass::Grammar, state)],
        })
        .collect()
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
    let mut digest_mismatch = graph.clone();
    digest_mismatch.intake[0].declared_digest = "0".repeat(64);
    assert!(matches!(
        validate_evidence_graph(&digest_mismatch, &claim),
        LedgerOutcome::Refused { .. }
    ));
    edges.push(EvidenceEdge {
        from: claim.clone(),
        to: id(EvidenceKind::Requirement, "Requirement:1"),
        relation: EvidenceRelation::Supports,
        scope: "scope".into(),
        provenance: "producer".into(),
    });
    graph.edges = edges;
    assert!(matches!(
        validate_evidence_graph(&graph, &claim),
        LedgerOutcome::Refused { .. }
    ));
    assert_eq!(graph.edges.len(), 13);
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
    assert!(matches!(
        validate_lineage(&[root, child]),
        LedgerOutcome::Refused { .. }
    ));
}

/// Tracing: TC-051, FR-034-AC-1, FR-034-AC-2, FR-034-AC-3, IT-005.
#[test]
#[trace("TC-051", "FR-034-AC-1", "FR-034-AC-2", "FR-034-AC-3", "IT-005")]
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
            &graph,
            &rows(VectorState::Planned),
            None,
            "scope:1"
        ),
        LedgerOutcome::Incomplete {
            limitations: vec![LimitationKind::FutureProductionEvidence]
        }
    );
    let decision = HumanDecision {
        identity: id(EvidenceKind::Decision, "decision:1"),
        actor: "human:1".into(),
        authority: "qualification-owner".into(),
        ledger_scope: "scope:1".into(),
        disposition: DecisionDisposition::Accepted,
        rationale: "reviewed".into(),
    };
    assert_eq!(
        account_qualification(
            &accepted(),
            &lock(),
            &graph,
            &rows(VectorState::Passed),
            Some(&decision),
            "scope:1"
        ),
        LedgerOutcome::Accounted
    );
    let mut duplicate = rows(VectorState::Passed);
    duplicate[175].capability_identity = "capability:0".into();
    assert!(matches!(
        account_qualification(&accepted(), &lock(), &graph, &duplicate, None, "scope:1"),
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
            "scope:1"
        ),
        LedgerOutcome::Refused { .. }
    ));
}
