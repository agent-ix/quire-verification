// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Deterministic prerequisite-aware verification planning (FR-005).

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, FixedOffset};
use serde_json::{Value, json};

use crate::contracts::{
    VerificationError, VerificationErrorCode, contract_version, jcs_equal, validate_contract,
    validate_plan_value, validate_schema_definition,
};
use crate::value::{arrays_field, field, object, optional_array, string_field};

const FAULT_SAFETY_PREREQUISITES: [(&str, &str); 3] = [
    ("environment-authorization", "non-production-fault-scope"),
    ("injection-binding", "confirmed-injection-site"),
    ("observation-binding", "independent-effect-observation"),
];

/// Builds and validates a deterministic verification plan from a JSON request.
///
/// # Errors
/// Returns an input, timestamp, schema, capability, or plan-graph error.
pub fn build_verification_plan(request: &Value) -> Result<Value, VerificationError> {
    object(request, "plan request")?;
    let plan_id = string_field(request, "planId", "plan request")?;
    let policy = field(request, "policy", "plan request")?;
    validate_schema_definition("SelectionPolicy", policy)?;
    let as_of = parse_timestamp(
        string_field(request, "asOf", "plan request")?,
        "plan request.asOf",
    )?;
    let policy_stale = policy
        .get("validUntil")
        .and_then(Value::as_str)
        .map(|value| parse_timestamp(value, "policy.validUntil").map(|until| as_of > until))
        .transpose()?
        .unwrap_or(false);
    let properties = collect_properties(request)?;
    let inventory = normalized_inventory(&properties)?;
    let capabilities = collect_capabilities(request)?;
    let available = optional_array(request, "availablePrerequisites", "plan request")?
        .iter()
        .map(prerequisite_key)
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut tasks = Vec::new();
    for selection in arrays_field(request, "selections", "plan request")? {
        tasks.extend(build_selection_tasks(
            selection,
            request,
            policy,
            policy_stale,
            &properties,
            &capabilities,
            &available,
        )?);
    }
    let mut tasks = coalesce_plan_tasks(tasks)?;
    tasks.sort_by(|left, right| {
        left.get("taskId")
            .and_then(Value::as_str)
            .cmp(&right.get("taskId").and_then(Value::as_str))
    });
    let plan = json!({
        "contractVersion": contract_version(),
        "kind": "VerificationPlan",
        "planId": plan_id,
        "policyRef": {
            "policyId": string_field(policy, "policyId", "policy")?,
            "version": string_field(policy, "version", "policy")?,
            "sourceRef": field(policy, "sourceRef", "policy")?,
            "validUntil": policy.get("validUntil").cloned().unwrap_or(Value::Null),
            "freshness": if policy_stale { "stale" } else { "current" },
        },
        "inventory": inventory,
        "tasks": tasks,
        "budget": request.get("budget").cloned().unwrap_or(Value::Null),
        "evidenceOwner": "Quoin",
        "sufficiencyOwner": "authored-human-decision",
    });
    validate_plan_value(&plan)?;
    Ok(plan)
}

fn collect_properties(request: &Value) -> Result<BTreeMap<String, &Value>, VerificationError> {
    let mut properties = BTreeMap::new();
    for property in arrays_field(request, "properties", "plan request")? {
        validate_schema_definition("PropertyInventoryItem", property)?;
        let property_id = string_field(property, "propertyId", "property")?.to_owned();
        if properties.insert(property_id.clone(), property).is_some() {
            return Err(VerificationError::new(
                VerificationErrorCode::DuplicatePropertyId,
                "duplicate property identifier",
            )
            .with_context("property_id", property_id));
        }
    }
    Ok(properties)
}

fn normalized_inventory(
    properties: &BTreeMap<String, &Value>,
) -> Result<Vec<Value>, VerificationError> {
    properties
        .values()
        .map(|property| {
            let mut normalized = (*property).clone();
            let fields = object_mut(&mut normalized)?;
            fields
                .entry("classification".to_owned())
                .or_insert_with(|| Value::String("unknown".to_owned()));
            Ok(normalized)
        })
        .collect()
}

fn collect_capabilities(
    request: &Value,
) -> Result<BTreeMap<String, Vec<&Value>>, VerificationError> {
    let mut capabilities = BTreeMap::<String, Vec<&Value>>::new();
    for capability in optional_array(request, "capabilities", "plan request")? {
        validate_contract(capability.clone())?;
        capabilities
            .entry(string_field(capability, "techniqueId", "capability")?.to_owned())
            .or_default()
            .push(capability);
    }
    Ok(capabilities)
}

fn build_selection_tasks(
    selection: &Value,
    request: &Value,
    policy: &Value,
    policy_stale: bool,
    properties: &BTreeMap<String, &Value>,
    capabilities: &BTreeMap<String, Vec<&Value>>,
    available: &BTreeSet<(String, String)>,
) -> Result<Vec<Value>, VerificationError> {
    let property_id = string_field(selection, "propertyId", "selection")?;
    let technique_id = string_field(selection, "techniqueId", "selection")?;
    let applicability = string_field(selection, "applicability", "selection")?;
    let priority = string_field(selection, "priority", "selection")?;
    let property = properties.get(property_id).copied().ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::UnknownProperty,
            "selection names unknown property",
        )
        .with_context("property_id", property_id)
    })?;
    let capability = choose_capability(
        capabilities.get(technique_id).map_or(&[], Vec::as_slice),
        property,
        selection.get("capabilityId").and_then(Value::as_str),
    )?;
    let (mut readiness, prerequisites, gaps) =
        capability_prerequisites(capability, property, policy, policy_stale)?;
    let missing = missing_prerequisites(&prerequisites, available)?;
    if !missing.is_empty() {
        readiness = "blocked";
    }
    let verification_id = verification_task_id(property_id, technique_id);
    let (mut tasks, dependencies) = setup_tasks(&missing, &verification_id)?;
    tasks.push(json!({
        "taskId": verification_id, "kind": "verification",
        "propertyId": property_id, "techniqueId": technique_id,
        "input": field(property, "input", "property")?, "applicability": applicability,
        "readiness": readiness, "priority": priority,
        "reasons": selection.get("reasons").cloned().unwrap_or_else(|| json!([])),
        "dependencies": dependencies, "missingPrerequisites": missing, "gaps": gaps,
        "runnerRequest": runner_request(capability, property, request),
        "invocationAllowed": readiness == "ready" && applicability == "applicable",
    }));
    Ok(tasks)
}

fn capability_prerequisites(
    capability: Option<&Value>,
    property: &Value,
    policy: &Value,
    policy_stale: bool,
) -> Result<(&'static str, Vec<Value>, Vec<Value>), VerificationError> {
    let Some(capability) = capability else {
        let input_kind = string_field(
            field(property, "input", "property")?,
            "kind",
            "property.input",
        )?;
        let profile = string_field(property, "profile", "property")?;
        return Ok((
            "unsupported",
            Vec::new(),
            vec![
                json!({"kind": "unsupported-capability", "detail": format!("no capability supports {input_kind}/{profile}")}),
            ],
        ));
    };
    let mut prerequisites = optional_array(capability, "prerequisites", "capability")?.to_vec();
    if string_field(
        field(property, "input", "property")?,
        "kind",
        "property.input",
    )? == "fault"
    {
        prerequisites.extend(
            FAULT_SAFETY_PREREQUISITES
                .iter()
                .map(|(kind, id)| json!({"kind": kind, "id": id})),
        );
    }
    if string_field(capability, "qualification", "capability")? != "qualified" {
        prerequisites.push(json!({"kind": "capability-qualification", "id": string_field(capability, "capabilityId", "capability")?}));
    }
    if policy_stale {
        prerequisites.push(json!({
            "kind": "policy-freshness",
            "id": format!("{}@{}", string_field(policy, "policyId", "policy")?, string_field(policy, "version", "policy")?),
        }));
    }
    Ok(("ready", merge_prerequisites(prerequisites)?, Vec::new()))
}

fn missing_prerequisites(
    prerequisites: &[Value],
    available: &BTreeSet<(String, String)>,
) -> Result<Vec<Value>, VerificationError> {
    let mut missing = Vec::new();
    for prerequisite in prerequisites {
        let required = prerequisite.get("required").and_then(Value::as_bool) != Some(false);
        let forced_stale =
            prerequisite.get("kind").and_then(Value::as_str) == Some("policy-freshness");
        if required && (forced_stale || !available.contains(&prerequisite_key(prerequisite)?)) {
            missing.push(prerequisite.clone());
        }
    }
    Ok(missing)
}

fn setup_tasks(
    missing: &[Value],
    verification_id: &str,
) -> Result<(Vec<Value>, Vec<Value>), VerificationError> {
    let mut tasks = Vec::with_capacity(missing.len());
    let mut dependencies = Vec::with_capacity(missing.len());
    for prerequisite in missing {
        let kind = string_field(prerequisite, "kind", "prerequisite")?;
        let id = string_field(prerequisite, "id", "prerequisite")?;
        let setup_id = setup_task_id(kind, id);
        dependencies.push(Value::String(setup_id.clone()));
        tasks.push(json!({
            "taskId": setup_id, "kind": "manual-prerequisite",
            "prerequisite": prerequisite, "requiredBy": [verification_id],
            "input": {"kind": "manual", "action": "satisfy-prerequisite", "prerequisite": prerequisite,
                "reviewProcedure": format!("Resolve {kind}:{id} and record its source and disposition."),
                "requiredDisposition": ["satisfied", "deferred", "rejected"]},
            "readiness": "blocked",
            "dependencies": [], "missingPrerequisites": [prerequisite], "runnerRequest": null,
            "invocationAllowed": false,
        }));
    }
    Ok((tasks, dependencies))
}

fn coalesce_plan_tasks(tasks: Vec<Value>) -> Result<Vec<Value>, VerificationError> {
    let mut by_id = BTreeMap::<String, Value>::new();
    for task in tasks {
        let task_id = string_field(&task, "taskId", "plan task")?.to_owned();
        let Some(existing) = by_id.get_mut(&task_id) else {
            by_id.insert(task_id, task);
            continue;
        };
        if existing.get("kind").and_then(Value::as_str) != Some("manual-prerequisite")
            || task.get("kind").and_then(Value::as_str) != Some("manual-prerequisite")
        {
            return Err(VerificationError::new(
                VerificationErrorCode::DuplicateTaskId,
                "duplicate task identifier",
            )
            .with_context("task_id", task_id));
        }
        if !jcs_equal(
            field(existing, "prerequisite", "setup task")?,
            field(&task, "prerequisite", "setup task")?,
        )? {
            return Err(VerificationError::new(
                VerificationErrorCode::InvalidInput,
                "shared prerequisite identity has conflicting definitions",
            )
            .with_context("task_id", task_id));
        }
        let required_by = existing
            .get_mut("requiredBy")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| {
                VerificationError::new(
                    VerificationErrorCode::InvalidInput,
                    "setup task requiredBy must be an array",
                )
            })?;
        required_by.extend(
            arrays_field(&task, "requiredBy", "setup task")?
                .iter()
                .cloned(),
        );
        required_by.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
        required_by.dedup();
    }
    Ok(by_id.into_values().collect())
}

fn runner_request(capability: Option<&Value>, property: &Value, request: &Value) -> Value {
    capability.map_or(Value::Null, |item| json!({
        "kind": item.pointer("/runner/kind").cloned().unwrap_or(Value::Null),
        "adapterId": item.pointer("/runner/adapterId").cloned().unwrap_or(Value::Null),
        "adapterVersion": item.pointer("/runner/adapterVersion").cloned().unwrap_or(Value::Null),
        "capabilityId": item.get("capabilityId").cloned().unwrap_or(Value::Null),
        "environmentPermissions": item.get("environmentPermissions").cloned().unwrap_or_else(|| json!([])),
        "budget": property.get("budget").or_else(|| request.get("budget")).cloned().unwrap_or(Value::Null),
        "nativeArtifactInterface": item.get("nativeArtifactInterface").cloned().unwrap_or(Value::Null),
    }))
}

fn choose_capability<'a>(
    capabilities: &'a [&Value],
    property: &Value,
    requested: Option<&str>,
) -> Result<Option<&'a Value>, VerificationError> {
    let kind = string_field(
        field(property, "input", "property")?,
        "kind",
        "property.input",
    )?;
    let profile = string_field(property, "profile", "property")?;
    let supported = capabilities
        .iter()
        .copied()
        .filter(|capability| {
            contains_string(capability.get("inputKinds"), kind)
                && contains_string(capability.get("profiles"), profile)
        })
        .collect::<Vec<_>>();
    if let Some(requested_id) = requested {
        return Ok(supported
            .into_iter()
            .find(|item| item.get("capabilityId").and_then(Value::as_str) == Some(requested_id)));
    }
    if supported.len() > 1 {
        return Err(VerificationError::new(
            VerificationErrorCode::AmbiguousCapability,
            "multiple capabilities support property; selection must name capabilityId",
        )
        .with_context(
            "property_id",
            string_field(property, "propertyId", "property")?,
        ));
    }
    Ok(supported.into_iter().next())
}

fn contains_string(value: Option<&Value>, expected: &str) -> bool {
    value
        .and_then(Value::as_array)
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn merge_prerequisites(values: Vec<Value>) -> Result<Vec<Value>, VerificationError> {
    let mut merged = BTreeMap::new();
    for mut prerequisite in values {
        let key = prerequisite_key(&prerequisite)?;
        if prerequisite.get("required").is_none() {
            object_mut(&mut prerequisite)?.insert("required".to_owned(), Value::Bool(true));
        }
        merged.insert(key, prerequisite);
    }
    Ok(merged.into_values().collect())
}

fn prerequisite_key(value: &Value) -> Result<(String, String), VerificationError> {
    Ok((
        string_field(value, "kind", "prerequisite")?.to_owned(),
        string_field(value, "id", "prerequisite")?.to_owned(),
    ))
}

fn setup_task_id(kind: &str, id: &str) -> String {
    composite_task_id("setup", kind, id)
}

pub(crate) fn verification_task_id(property_id: &str, technique_id: &str) -> String {
    composite_task_id("verify", property_id, technique_id)
}

fn composite_task_id(prefix: &str, left: &str, right: &str) -> String {
    format!("{prefix}:{}:{left}:{}:{right}", left.len(), right.len())
}

fn object_mut(value: &mut Value) -> Result<&mut serde_json::Map<String, Value>, VerificationError> {
    value.as_object_mut().ok_or_else(|| {
        VerificationError::new(
            VerificationErrorCode::InvalidInput,
            "prerequisite must be an object",
        )
    })
}

fn parse_timestamp(
    value: &str,
    field_name: &str,
) -> Result<DateTime<FixedOffset>, VerificationError> {
    DateTime::parse_from_rfc3339(value).map_err(|error| {
        VerificationError::new(VerificationErrorCode::InvalidTimestamp, error.to_string())
            .with_context("field", field_name)
    })
}
