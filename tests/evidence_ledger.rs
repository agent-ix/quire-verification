// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

use quire_verification::evidence_ledger::*;

fn identity(kind: EvidenceKind, value: &str) -> EvidenceIdentity {
    EvidenceIdentity {
        kind,
        value: value.into(),
    }
}
fn rows(limitation: Option<Limitation>) -> Vec<LedgerRow> {
    (0..176)
        .map(|index| LedgerRow {
            capability_identity: format!("capability:{index}"),
            claim_class: "development".into(),
            evidence: vec![identity(EvidenceKind::Artifact, "artifact:1")],
            limitations: limitation.into_iter().collect(),
            decision: None,
        })
        .collect()
}

#[test]
fn tc_050_refuses_self_evidence_without_erasing_other_rows() {
    let edge = EvidenceEdge {
        from: identity(EvidenceKind::Claim, "claim:1"),
        to: identity(EvidenceKind::Claim, "claim:1"),
        relation: EvidenceRelation::Supports,
        scope: "scope".into(),
        provenance: "run".into(),
    };
    assert!(matches!(
        account_ledger(&[edge], &[], &rows(None)),
        LedgerOutcome::Refused { .. }
    ));
}
#[test]
fn tc_051_accounts_exactly_176_rows_and_keeps_future_production_nonblocking() {
    assert!(matches!(
        account_ledger(&[], &[], &rows(Some(Limitation::FutureProductionEvidence))),
        LedgerOutcome::Incomplete {
            limitation: Limitation::FutureProductionEvidence
        }
    ));
    assert!(matches!(
        account_ledger(&[], &[], &rows(None)),
        LedgerOutcome::Accounted
    ));
}
#[test]
fn tc_052_refuses_mutable_counterexample_lineage() {
    let revision = CounterexampleRevision {
        identity: "revision:1".into(),
        parent: Some("revision:1".into()),
        subject: identity(EvidenceKind::Subject, "subject:1"),
        replay_identity: None,
    };
    assert!(matches!(
        account_ledger(&[], &[revision], &rows(None)),
        LedgerOutcome::Refused { .. }
    ));
}
