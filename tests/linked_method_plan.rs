// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! TC-049 linked-plan execution coverage.

use ix_trace_rs::trace;
use quire_verification::linked_method_plan::*;
use std::collections::{BTreeMap, BTreeSet};

struct Fixed(ProviderOutcome);
impl MethodProvider for Fixed {
    fn execute(&self, _: &PlanNode) -> ProviderOutcome {
        self.0.clone()
    }
}
fn node(id: &str, provider: &str) -> PlanNode {
    PlanNode {
        identity: id.into(),
        claim_identity: format!("claim:{id}"),
        semantic_subject_identity: "subject:1".into(),
        method_identity: "method:1".into(),
        provider_identity: provider.into(),
        assumptions_identity: "assumptions:1".into(),
        semantic_bound_identity: "bound:1".into(),
        work_limit: 1,
        expected_evidence_class: "test".into(),
        aggregation: None,
    }
}
fn plan(edges: Vec<PlanEdge>) -> LinkedPlan {
    LinkedPlan {
        contract_version: LINKED_PLAN_VERSION.into(),
        identity: "plan:1".into(),
        ecosystem_lock_identity: "lock:1".into(),
        nodes: vec![node("a", "good"), node("b", "bad"), node("c", "good")],
        edges,
        providers: vec![
            ProviderManifest {
                identity: "good".into(),
                methods: BTreeSet::from(["method:1".into()]),
                available: true,
            },
            ProviderManifest {
                identity: "bad".into(),
                methods: BTreeSet::from(["method:1".into()]),
                available: true,
            },
        ],
    }
}
fn disposition<'a>(account: &'a LinkedPlanAccount, id: &str) -> &'a NodeDisposition {
    &account
        .nodes
        .iter()
        .find(|node| node.node_identity == id)
        .unwrap()
        .disposition
}

/// Tracing: TC-049, FR-031-AC-1, FR-031-CON-2, IT-005-SC-03.
#[trace("TC-049", "FR-031-AC-1", "FR-031-CON-2", "IT-005")]
#[test]
fn tc_049_accounts_acyclic_nodes_in_deterministic_order() {
    let p = plan(vec![PlanEdge {
        from: "a".into(),
        to: "c".into(),
        kind: DependencyKind::Validates,
    }]);
    let good = Fixed(ProviderOutcome::Proved);
    let bad = Fixed(ProviderOutcome::Tested);
    let providers = BTreeMap::from([
        ("good".into(), &good as &dyn MethodProvider),
        ("bad".into(), &bad as &dyn MethodProvider),
    ]);
    let result = execute_linked_plan(&p, &providers);
    assert_eq!(
        result
            .nodes
            .iter()
            .map(|n| n.node_identity.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "b", "c"]
    );
    assert!(matches!(
        disposition(&result, "c"),
        NodeDisposition::Executed {
            outcome: ProviderOutcome::Proved,
            ..
        }
    ));
}

/// Tracing: TC-049, FR-031-AC-2, FR-031-CON-2, IT-005-SC-03.
#[trace("TC-049", "FR-031-AC-2", "FR-031-CON-2", "IT-005")]
#[test]
fn tc_049_blocks_only_requires_descendants_and_keeps_siblings() {
    let p = plan(vec![PlanEdge {
        from: "b".into(),
        to: "c".into(),
        kind: DependencyKind::Requires,
    }]);
    let good = Fixed(ProviderOutcome::Proved);
    let bad = Fixed(ProviderOutcome::Failed);
    let providers = BTreeMap::from([
        ("good".into(), &good as &dyn MethodProvider),
        ("bad".into(), &bad as &dyn MethodProvider),
    ]);
    let result = execute_linked_plan(&p, &providers);
    assert!(matches!(
        disposition(&result, "b"),
        NodeDisposition::Executed {
            outcome: ProviderOutcome::Failed,
            ..
        }
    ));
    assert!(
        matches!(disposition(&result,"c"),NodeDisposition::Blocked { prerequisite } if prerequisite=="b")
    );
    assert!(matches!(
        disposition(&result, "a"),
        NodeDisposition::Executed {
            outcome: ProviderOutcome::Proved,
            ..
        }
    ));
}

/// Tracing: TC-049, FR-031-AC-2, FR-031-AC-3, FR-031-CON-1.
#[trace("TC-049", "FR-031-AC-2", "FR-031-AC-3", "FR-031-CON-1")]
#[test]
fn tc_049_refuses_cycles_and_preserves_unsupported_per_node() {
    let mut cyclic = plan(vec![
        PlanEdge {
            from: "a".into(),
            to: "c".into(),
            kind: DependencyKind::Validates,
        },
        PlanEdge {
            from: "c".into(),
            to: "a".into(),
            kind: DependencyKind::Validates,
        },
    ]);
    let good = Fixed(ProviderOutcome::Proved);
    let providers = BTreeMap::from([("good".into(), &good as &dyn MethodProvider)]);
    let refused = execute_linked_plan(&cyclic, &providers);
    assert!(
        refused
            .nodes
            .iter()
            .all(|n| matches!(n.disposition, NodeDisposition::Refused { .. }))
    );
    cyclic.edges.clear();
    let result = execute_linked_plan(&cyclic, &providers);
    assert!(
        matches!(disposition(&result,"b"),NodeDisposition::Unsupported { provider } if provider=="bad")
    );
    assert!(matches!(
        disposition(&result, "a"),
        NodeDisposition::Executed { .. }
    ));
}

/// Tracing: TC-049, FR-031-AC-3, FR-031-CON-1.
#[trace("TC-049", "FR-031-AC-3", "FR-031-CON-1")]
#[test]
fn tc_049_refuses_self_justifying_aggregation_without_relabeling_siblings() {
    let mut authored = plan(vec![]);
    authored.nodes[2].aggregation = Some(AggregationRule {
        identity: "aggregate:c".into(),
        required_inputs: vec!["c".into()],
    });
    let good = Fixed(ProviderOutcome::Refuted);
    let bad = Fixed(ProviderOutcome::Tested);
    let providers = BTreeMap::from([
        ("good".into(), &good as &dyn MethodProvider),
        ("bad".into(), &bad as &dyn MethodProvider),
    ]);
    let result = execute_linked_plan(&authored, &providers);
    assert!(matches!(
        disposition(&result, "c"),
        NodeDisposition::Refused { .. }
    ));
    assert!(matches!(
        disposition(&result, "a"),
        NodeDisposition::Executed {
            outcome: ProviderOutcome::Refuted,
            ..
        }
    ));
}
