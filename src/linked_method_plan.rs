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
    /// The target may run only after the source has a conclusive supported outcome.
    Requires,
    /// The source establishes an ordering-only validation dependency for the target.
    Validates,
    /// The source establishes an ordering-only replay dependency for the target.
    Replays,
    /// The source establishes an ordering-only aggregation dependency for the target.
    Aggregates,
}

/// A semantic/provider terminal outcome; it is never rewritten by aggregation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderOutcome {
    /// The provider established the submitted claim.
    Proved,
    /// The provider disproved the submitted claim.
    Refuted,
    /// The provider completed a test without a stronger proof classification.
    Tested,
    /// The provider retained an observation without a stronger proof classification.
    Observed,
    /// The provider ran but could not reach a conclusive outcome.
    Inconclusive,
    /// The provider does not support the requested semantic operation.
    Unsupported,
    /// The provider could not complete the requested operation.
    Incomplete,
    /// The provider operation failed.
    Failed,
}

/// One immutable provider capability declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProviderManifest {
    /// Stable identity used by plan nodes to select this provider.
    pub identity: String,
    /// Method identities the provider declares it can execute.
    pub methods: BTreeSet<String>,
    /// Whether the provider is admitted for dispatch in this plan.
    pub available: bool,
}

/// One linked method-plan node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlanNode {
    /// Stable identity of this node within the plan.
    pub identity: String,
    /// Identity of the claim evaluated by this node.
    pub claim_identity: String,
    /// Identity of the semantic subject shared across related evaluations.
    pub semantic_subject_identity: String,
    /// Identity of the verification method to dispatch.
    pub method_identity: String,
    /// Identity of the provider selected for dispatch.
    pub provider_identity: String,
    /// Identity of the immutable assumption set used by the method.
    pub assumptions_identity: String,
    /// Identity of the method's semantic execution bound.
    pub semantic_bound_identity: String,
    /// Maximum authored work units available to the provider.
    pub work_limit: usize,
    /// Evidence class the provider result is expected to retain.
    pub expected_evidence_class: String,
    /// Optional rule that combines already-terminal authored inputs.
    pub aggregation: Option<AggregationRule>,
}

/// An authored, non-relabeling aggregation rule.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AggregationRule {
    /// Stable identity of the authored aggregation rule.
    pub identity: String,
    /// Node identities that must already have terminal accounts.
    pub required_inputs: Vec<String>,
}

/// A typed edge between authored node identities.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PlanEdge {
    /// Identity of the dependency source node.
    pub from: String,
    /// Identity of the dependent target node.
    pub to: String,
    /// Typed dependency semantics applied to the edge.
    pub kind: DependencyKind,
}

/// The complete externally supplied plan.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LinkedPlan {
    /// Version of the supplied plan contract.
    pub contract_version: String,
    /// Stable identity of the complete authored plan.
    pub identity: String,
    /// Identity of the exact ecosystem dependency lock used by the plan.
    pub ecosystem_lock_identity: String,
    /// Nodes requested by the plan in authored result order.
    pub nodes: Vec<PlanNode>,
    /// Typed dependencies between declared nodes.
    pub edges: Vec<PlanEdge>,
    /// Provider capabilities admitted for the plan.
    pub providers: Vec<ProviderManifest>,
}

/// The exact terminal state for an accounted plan node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeDisposition {
    /// The selected provider ran and returned an immutable semantic outcome.
    Executed {
        /// Semantic outcome returned by the provider.
        outcome: ProviderOutcome,
        /// Content identity binding the plan, node, and outcome.
        result_identity: String,
    },
    /// A required predecessor lacked a conclusive supported outcome.
    Blocked {
        /// Identity of the predecessor that blocked this node.
        prerequisite: String,
    },
    /// No admitted provider can execute the node's requested method.
    Unsupported {
        /// Identity of the unavailable or incapable provider.
        provider: String,
    },
    /// The coordinator could not reach the otherwise valid node.
    NotRun {
        /// Stable explanation of why the node was not reached.
        reason: String,
    },
    /// The node or entire plan was rejected before provider execution.
    Refused {
        /// Stable explanation of the refusal.
        reason: String,
    },
}

/// Immutable accounted node result.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AccountedNode {
    /// Identity of the node receiving this terminal account.
    pub node_identity: String,
    /// Exactly one terminal disposition for the node.
    pub disposition: NodeDisposition,
}

/// Complete deterministic plan execution account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LinkedPlanAccount {
    /// Version of the emitted node-accounting contract.
    pub contract_version: String,
    /// Content identity of the complete plan account.
    pub identity: String,
    /// Terminal accounts in the plan's authored node order.
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
        return refused_account(plan, &reason);
    }
    let order = topological_order(plan);
    let Some(order) = order else {
        return refused_account(plan, "dependency cycle without accepted fixed-point rule");
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
        results.insert(id, execute_node(plan, node, &manifest, providers, &results));
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

fn execute_node(
    plan: &LinkedPlan,
    node: &PlanNode,
    manifest: &BTreeMap<&str, &ProviderManifest>,
    providers: &BTreeMap<String, &dyn MethodProvider>,
    results: &BTreeMap<String, AccountedNode>,
) -> AccountedNode {
    let disposition = if let Some(prerequisite) = plan
        .edges
        .iter()
        .filter(|edge| edge.to == node.identity && edge.kind == DependencyKind::Requires)
        .map(|edge| edge.from.as_str())
        .find(|source| !requires_satisfied(results.get(*source)))
    {
        NodeDisposition::Blocked {
            prerequisite: prerequisite.to_owned(),
        }
    } else if let Some(rule) = &node.aggregation
        && rule
            .required_inputs
            .iter()
            .any(|input| input == &node.identity || !results.contains_key(input))
    {
        NodeDisposition::Refused {
            reason: "aggregation inputs are self-referential or non-terminal".to_owned(),
        }
    } else if let (Some(declared), Some(provider)) = (
        manifest.get(node.provider_identity.as_str()),
        providers.get(node.provider_identity.as_str()),
    ) {
        if declared.available && declared.methods.contains(&node.method_identity) {
            let outcome = provider.execute(node);
            let result_identity = identity(&(plan.identity.as_str(), node, &outcome));
            NodeDisposition::Executed {
                outcome,
                result_identity,
            }
        } else {
            unsupported(node)
        }
    } else {
        unsupported(node)
    };
    AccountedNode {
        node_identity: node.identity.clone(),
        disposition,
    }
}

fn unsupported(node: &PlanNode) -> NodeDisposition {
    NodeDisposition::Unsupported {
        provider: node.provider_identity.clone(),
    }
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

fn refused_account(plan: &LinkedPlan, reason: &str) -> LinkedPlanAccount {
    account(
        plan,
        plan.nodes
            .iter()
            .map(|node| AccountedNode {
                node_identity: node.identity.clone(),
                disposition: NodeDisposition::Refused {
                    reason: reason.to_owned(),
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
