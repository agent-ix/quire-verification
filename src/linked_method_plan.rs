// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Deterministic, finite linked-method plan coordination for FR-031 and TC-049.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::contracts::jcs_sha256;

/// Version of the linked-plan request contract.
pub const LINKED_PLAN_VERSION: &str = "quire.verification-plan/v1";
/// Version of the terminal node-accounting contract.
pub const LINKED_RESULT_VERSION: &str = "quire.method-result/v1";

/// A declared dependency edge kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DependencyKind {
    Requires,
    Validates,
    Replays,
    Aggregates,
}

/// A semantic/provider terminal outcome; it is never rewritten by aggregation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderOutcome {
    Proved,
    Refuted,
    Tested,
    Observed,
    Inconclusive,
    Unsupported,
    Incomplete,
    Failed,
}

/// One immutable provider capability declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProviderManifest {
    pub identity: String,
    pub methods: BTreeSet<String>,
    pub available: bool,
}

/// One linked method-plan node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlanNode {
    pub identity: String,
    pub claim_identity: String,
    pub semantic_subject_identity: String,
    pub method_identity: String,
    pub provider_identity: String,
    pub assumptions_identity: String,
    pub semantic_bound_identity: String,
    pub work_limit: usize,
    pub expected_evidence_class: String,
    pub aggregation: Option<AggregationRule>,
}

/// An authored, non-relabeling aggregation rule.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AggregationRule {
    pub identity: String,
    pub required_inputs: Vec<String>,
}

/// A typed edge between authored node identities.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlanEdge {
    pub from: String,
    pub to: String,
    pub kind: DependencyKind,
}

/// The complete externally supplied plan.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LinkedPlan {
    pub contract_version: String,
    pub identity: String,
    pub ecosystem_lock_identity: String,
    pub nodes: Vec<PlanNode>,
    pub edges: Vec<PlanEdge>,
    pub providers: Vec<ProviderManifest>,
}

/// The exact terminal state for an accounted plan node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeDisposition {
    Executed {
        outcome: ProviderOutcome,
        result_identity: String,
    },
    Blocked {
        prerequisite: String,
    },
    Unsupported {
        provider: String,
    },
    NotRun {
        reason: String,
    },
    Refused {
        reason: String,
    },
}

/// Immutable accounted node result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountedNode {
    pub node_identity: String,
    pub disposition: NodeDisposition,
}

/// Complete deterministic plan execution account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LinkedPlanAccount {
    pub contract_version: String,
    pub identity: String,
    pub nodes: Vec<AccountedNode>,
}

/// Provider dispatch seam. The coordinator never replaces this with a production call.
pub trait MethodProvider {
    /// Executes a fully resolved node and returns its immutable terminal outcome.
    fn execute(&self, node: &PlanNode) -> ProviderOutcome;
}

/// Executes a linked plan with one terminal disposition per node.
#[must_use]
pub fn execute_linked_plan(
    plan: &LinkedPlan,
    providers: &BTreeMap<String, &dyn MethodProvider>,
) -> LinkedPlanAccount {
    let invalid = validate(plan);
    if let Some(reason) = invalid {
        return refused_account(plan, reason);
    }
    let order = topological_order(plan);
    let Some(order) = order else {
        return refused_account(
            plan,
            "dependency cycle without accepted fixed-point rule".to_owned(),
        );
    };
    let manifest = plan
        .providers
        .iter()
        .map(|item| (item.identity.as_str(), item))
        .collect::<BTreeMap<_, _>>();
    let mut results = BTreeMap::<String, AccountedNode>::new();
    for id in order {
        let Some(node) = plan.nodes.iter().find(|item| item.identity == id) else {
            continue;
        };
        let requires = plan
            .edges
            .iter()
            .filter(|edge| edge.to == id && edge.kind == DependencyKind::Requires)
            .map(|edge| edge.from.as_str());
        if let Some(prerequisite) = requires
            .filter(|source| !requires_satisfied(results.get(*source)))
            .next()
        {
            results.insert(
                id.clone(),
                AccountedNode {
                    node_identity: id,
                    disposition: NodeDisposition::Blocked {
                        prerequisite: prerequisite.to_owned(),
                    },
                },
            );
            continue;
        }
        if let Some(rule) = &node.aggregation {
            if rule
                .required_inputs
                .iter()
                .any(|input| input == &node.identity || !results.contains_key(input))
            {
                results.insert(
                    id.clone(),
                    AccountedNode {
                        node_identity: id,
                        disposition: NodeDisposition::Refused {
                            reason: "aggregation inputs are self-referential or non-terminal"
                                .to_owned(),
                        },
                    },
                );
                continue;
            }
        }
        let Some(declared) = manifest.get(node.provider_identity.as_str()) else {
            results.insert(
                id.clone(),
                AccountedNode {
                    node_identity: id,
                    disposition: NodeDisposition::Unsupported {
                        provider: node.provider_identity.clone(),
                    },
                },
            );
            continue;
        };
        if !declared.available
            || !declared.methods.contains(&node.method_identity)
            || !providers.contains_key(&node.provider_identity)
        {
            results.insert(
                id.clone(),
                AccountedNode {
                    node_identity: id,
                    disposition: NodeDisposition::Unsupported {
                        provider: node.provider_identity.clone(),
                    },
                },
            );
            continue;
        }
        let outcome = providers[&node.provider_identity].execute(node);
        let result_identity = identity(&(plan.identity.as_str(), node, &outcome));
        results.insert(
            id.clone(),
            AccountedNode {
                node_identity: id,
                disposition: NodeDisposition::Executed {
                    outcome,
                    result_identity,
                },
            },
        );
    }
    account(
        plan,
        plan.nodes
            .iter()
            .map(|node| {
                results
                    .remove(&node.identity)
                    .unwrap_or_else(|| AccountedNode {
                        node_identity: node.identity.clone(),
                        disposition: NodeDisposition::NotRun {
                            reason: "coordinator did not reach node".to_owned(),
                        },
                    })
            })
            .collect(),
    )
}

fn validate(plan: &LinkedPlan) -> Option<String> {
    if plan.contract_version != LINKED_PLAN_VERSION
        || plan.identity.is_empty()
        || plan.ecosystem_lock_identity.is_empty()
    {
        return Some("invalid contract version or plan identity".to_owned());
    }
    let nodes = plan
        .nodes
        .iter()
        .map(|node| node.identity.as_str())
        .collect::<BTreeSet<_>>();
    if nodes.len() != plan.nodes.len()
        || nodes.contains("")
        || plan.nodes.iter().any(|node| {
            node.claim_identity.is_empty()
                || node.semantic_subject_identity.is_empty()
                || node.method_identity.is_empty()
                || node.provider_identity.is_empty()
                || node.work_limit == 0
        })
    {
        return Some("nodes must be complete and uniquely identified".to_owned());
    }
    if plan.edges.iter().any(|edge| {
        !nodes.contains(edge.from.as_str())
            || !nodes.contains(edge.to.as_str())
            || edge.from == edge.to
    }) {
        return Some("edges must reference distinct declared nodes".to_owned());
    }
    let provider_ids = plan
        .providers
        .iter()
        .map(|provider| provider.identity.as_str())
        .collect::<BTreeSet<_>>();
    if provider_ids.len() != plan.providers.len() || provider_ids.contains("") {
        return Some("provider identities must be nonempty and distinct".to_owned());
    }
    None
}

fn topological_order(plan: &LinkedPlan) -> Option<Vec<String>> {
    let mut remaining = plan
        .nodes
        .iter()
        .map(|node| node.identity.clone())
        .collect::<BTreeSet<_>>();
    let mut resolved = BTreeSet::new();
    let mut order = Vec::new();
    while !remaining.is_empty() {
        let next = remaining
            .iter()
            .find(|id| {
                plan.edges
                    .iter()
                    .filter(|edge| &edge.to == *id)
                    .all(|edge| resolved.contains(&edge.from))
            })
            .cloned();
        let next = next?;
        remaining.remove(&next);
        resolved.insert(next.clone());
        order.push(next);
    }
    Some(order)
}

fn refused_account(plan: &LinkedPlan, reason: String) -> LinkedPlanAccount {
    account(
        plan,
        plan.nodes
            .iter()
            .map(|node| AccountedNode {
                node_identity: node.identity.clone(),
                disposition: NodeDisposition::Refused {
                    reason: reason.clone(),
                },
            })
            .collect(),
    )
}
fn account(plan: &LinkedPlan, nodes: Vec<AccountedNode>) -> LinkedPlanAccount {
    LinkedPlanAccount {
        contract_version: LINKED_RESULT_VERSION.to_owned(),
        identity: identity(&(plan, &nodes)),
        nodes,
    }
}
fn requires_satisfied(node: Option<&AccountedNode>) -> bool {
    matches!(
        node.map(|node| &node.disposition),
        Some(NodeDisposition::Executed {
            outcome: ProviderOutcome::Proved
                | ProviderOutcome::Refuted
                | ProviderOutcome::Tested
                | ProviderOutcome::Observed,
            ..
        })
    )
}
fn identity<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| jcs_sha256(&value).ok())
        .unwrap_or_else(|| {
            format!(
                "sha256:{:x}",
                Sha256::digest(b"linked-plan-identity-unavailable")
            )
        })
}
