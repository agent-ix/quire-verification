// SPDX-License-Identifier: AGPL-3.0-or-later

export const CONTRACT_VERSION = "quire-verification/e02-draft-1";

export const INPUT_KINDS = Object.freeze([
  "state",
  "relational",
  "trace",
  "fault",
  "manual",
]);

export const APPLICABILITY = Object.freeze([
  "applicable",
  "not-applicable",
  "unknown",
]);

export const READINESS = Object.freeze(["ready", "blocked", "unsupported"]);

export const EXECUTION_STATUSES = Object.freeze([
  "not-run",
  "running",
  "completed",
  "failed",
  "cancelled",
  "exhausted",
  "invalid-input",
  "unsupported",
]);

export const LOGICAL_OUTCOMES = Object.freeze([
  "satisfied",
  "violated",
  "unknown",
  "not-evaluated",
  "inconclusive",
]);

export function requireRecord(value, label) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`${label} must be an object`);
  }
  return value;
}

export function requireString(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(`${label} must be a non-empty string`);
  }
  return value;
}

export function requireEnum(value, allowed, label) {
  if (!allowed.includes(value)) {
    throw new TypeError(`${label} must be one of ${allowed.join(", ")}`);
  }
  return value;
}

export function validateArtifactRef(value, label = "artifactRef") {
  const ref = requireRecord(value, label);
  for (const field of ["authority", "identity", "revision", "digest", "profile"]) {
    requireString(ref[field], `${label}.${field}`);
  }
  return ref;
}

export function validateTechniqueDefinition(value) {
  const definition = requireRecord(value, "TechniqueDefinition");
  requireEnum(definition.contractVersion, [CONTRACT_VERSION], "contractVersion");
  for (const field of ["techniqueId", "name", "class", "purpose"]) {
    requireString(definition[field], `TechniqueDefinition.${field}`);
  }
  validateArtifactRef(definition.sourceRef, "TechniqueDefinition.sourceRef");
  requireRecord(definition.applicability, "TechniqueDefinition.applicability");
  return definition;
}

export function validateToolCapability(value) {
  const capability = requireRecord(value, "ToolCapability");
  requireEnum(capability.contractVersion, [CONTRACT_VERSION], "contractVersion");
  for (const field of ["capabilityId", "techniqueId"]) {
    requireString(capability[field], `ToolCapability.${field}`);
  }
  requireEnum(
    capability.qualification,
    ["qualified", "provisional", "unqualified"],
    "ToolCapability.qualification",
  );
  requireRecord(capability.runner, "ToolCapability.runner");
  requireString(capability.runner.kind, "ToolCapability.runner.kind");
  requireString(capability.runner.adapterId, "ToolCapability.runner.adapterId");
  requireString(capability.runner.adapterVersion, "ToolCapability.runner.adapterVersion");
  requireRecord(
    capability.nativeArtifactInterface,
    "ToolCapability.nativeArtifactInterface",
  );
  if (!Array.isArray(capability.environmentPermissions)) {
    throw new TypeError("ToolCapability.environmentPermissions must be an array");
  }
  if (!Array.isArray(capability.inputKinds) || capability.inputKinds.length === 0) {
    throw new TypeError("ToolCapability.inputKinds must be a non-empty array");
  }
  for (const kind of capability.inputKinds) {
    requireEnum(kind, INPUT_KINDS, "ToolCapability.inputKinds[]");
  }
  if (!Array.isArray(capability.profiles) || capability.profiles.length === 0) {
    throw new TypeError("ToolCapability.profiles must be a non-empty array");
  }
  if (!Array.isArray(capability.prerequisites)) {
    throw new TypeError("ToolCapability.prerequisites must be an array");
  }
  capability.prerequisites.forEach((prerequisite, index) => {
    requireRecord(prerequisite, `ToolCapability.prerequisites[${index}]`);
    requireString(prerequisite.kind, `ToolCapability.prerequisites[${index}].kind`);
    requireString(prerequisite.id, `ToolCapability.prerequisites[${index}].id`);
  });
  return capability;
}

export function validateSelectionPolicy(value) {
  const policy = requireRecord(value, "SelectionPolicy");
  requireEnum(policy.contractVersion, [CONTRACT_VERSION], "contractVersion");
  requireString(policy.policyId, "SelectionPolicy.policyId");
  requireString(policy.version, "SelectionPolicy.version");
  validateArtifactRef(policy.sourceRef, "SelectionPolicy.sourceRef");
  if (!Array.isArray(policy.rules)) {
    throw new TypeError("SelectionPolicy.rules must be an array");
  }
  for (const rule of policy.rules) {
    requireString(rule.ruleId, "SelectionPolicy.rules[].ruleId");
    requireString(rule.explanation, "SelectionPolicy.rules[].explanation");
    requireEnum(
      rule.priority,
      ["required", "recommended", "optional"],
      "SelectionPolicy.rules[].priority",
    );
    if ("score" in rule) {
      throw new TypeError("SelectionPolicy rules must not contain an ordinal score");
    }
  }
  const provenance = requireRecord(policy.provenance, "SelectionPolicy.provenance");
  if (provenance.standardDerived === true) {
    for (const field of ["standard", "edition", "objectives", "interpretation", "rightsDisposition"]) {
      if (!(field in provenance)) {
        throw new TypeError(`SelectionPolicy.provenance.${field} is required for standards-derived policy`);
      }
    }
  }
  return policy;
}

export function validateVerificationPlan(value) {
  const plan = requireRecord(value, "VerificationPlan");
  requireEnum(plan.contractVersion, [CONTRACT_VERSION], "contractVersion");
  requireString(plan.planId, "VerificationPlan.planId");
  if (!Array.isArray(plan.inventory) || !Array.isArray(plan.tasks)) {
    throw new TypeError("VerificationPlan inventory and tasks must be arrays");
  }
  for (const property of plan.inventory) validateInventoryProperty(property);
  const taskIds = new Set();
  for (const task of plan.tasks) {
    requireString(task.taskId, "task.taskId");
    if (taskIds.has(task.taskId)) throw new TypeError(`duplicate task id ${task.taskId}`);
    taskIds.add(task.taskId);
    requireEnum(task.applicability, APPLICABILITY, "task.applicability");
    requireEnum(task.readiness, READINESS, "task.readiness");
    validateInput(task.input, "task.input");
    if (!Array.isArray(task.dependencies)) {
      throw new TypeError("task.dependencies must be an array");
    }
    if (
      task.invocationAllowed === true &&
      (task.applicability !== "applicable" || task.readiness !== "ready")
    ) {
      throw new TypeError("only an applicable, ready task may allow invocation");
    }
  }
  for (const task of plan.tasks) {
    for (const dependency of task.dependencies) {
      if (!taskIds.has(dependency)) throw new TypeError(`unknown task dependency ${dependency}`);
    }
  }
  assertAcyclic(plan.tasks);
  return plan;
}

export function validateTechniqueResult(value) {
  const result = requireRecord(value, "TechniqueResult");
  requireEnum(result.contractVersion, [CONTRACT_VERSION], "contractVersion");
  for (const field of ["resultId", "planId", "taskId", "techniqueId", "producedAt"]) {
    requireString(result[field], `TechniqueResult.${field}`);
  }
  const execution = requireRecord(result.execution, "execution");
  const logical = requireRecord(result.logical, "logical");
  const activation = requireRecord(result.activation, "activation");
  const adequacy = requireRecord(result.adequacy, "adequacy");
  requireRecord(result.participation, "participation");
  requireRecord(result.executionScope, "executionScope");
  requireRecord(result.provenance, "provenance");
  for (const field of ["producer", "producerVersion", "adapter", "sourceRevision"]) {
    requireString(result.provenance[field], `provenance.${field}`);
  }
  requireEnum(execution.status, EXECUTION_STATUSES, "execution.status");
  requireEnum(logical.outcome, LOGICAL_OUTCOMES, "logical.outcome");
  requireEnum(
    activation.status,
    ["activated", "not-activated", "unknown", "not-required"],
    "activation.status",
  );
  requireEnum(
    adequacy.status,
    ["adequate", "inadequate", "unknown", "not-assessed"],
    "adequacy.status",
  );
  validateArtifactRef(result.propertyRef, "TechniqueResult.propertyRef");
  if (!Array.isArray(result.subjectRefs) || !Array.isArray(result.bindingRefs)) {
    throw new TypeError("TechniqueResult subjectRefs and bindingRefs must be arrays");
  }
  if (!Array.isArray(result.assumptions) || !Array.isArray(result.nativeArtifacts)) {
    throw new TypeError("TechniqueResult assumptions and nativeArtifacts must be arrays");
  }
  for (const [index, artifact] of result.nativeArtifacts.entries()) {
    requireRecord(artifact, `TechniqueResult.nativeArtifacts[${index}]`);
    requireString(artifact.uri, `TechniqueResult.nativeArtifacts[${index}].uri`);
    requireString(artifact.digest, `TechniqueResult.nativeArtifacts[${index}].digest`);
  }
  requireString(result.profile, "TechniqueResult.profile");
  requireString(result.dependencyClosure, "TechniqueResult.dependencyClosure");
  result.subjectRefs.forEach((ref, index) =>
    validateArtifactRef(ref, `TechniqueResult.subjectRefs[${index}]`),
  );
  result.bindingRefs.forEach((ref, index) =>
    validateArtifactRef(ref, `TechniqueResult.bindingRefs[${index}]`),
  );
  if (!Number.isFinite(Date.parse(result.producedAt))) {
    throw new TypeError("TechniqueResult.producedAt must be an ISO timestamp");
  }
  if (result.validUntil !== undefined && !Number.isFinite(Date.parse(result.validUntil))) {
    throw new TypeError("TechniqueResult.validUntil must be an ISO timestamp");
  }
  if (
    result.execution.status !== "completed" &&
    ["satisfied", "violated"].includes(result.logical.outcome)
  ) {
    throw new TypeError("a non-completed execution cannot report a logical verdict");
  }
  return result;
}

export function validateInventoryProperty(value) {
  const property = requireRecord(value, "inventory property");
  requireString(property.propertyId, "inventory property.propertyId");
  validateArtifactRef(property.propertyRef, "inventory property.propertyRef");
  if (!Array.isArray(property.subjectRefs) || !Array.isArray(property.bindingRefs)) {
    throw new TypeError("inventory property subjectRefs and bindingRefs must be arrays");
  }
  property.subjectRefs.forEach((ref, index) =>
    validateArtifactRef(ref, `inventory property.subjectRefs[${index}]`),
  );
  property.bindingRefs.forEach((ref, index) =>
    validateArtifactRef(ref, `inventory property.bindingRefs[${index}]`),
  );
  requireString(property.profile, "inventory property.profile");
  requireString(property.dependencyClosure, "inventory property.dependencyClosure");
  requireRecord(property.executionScope, "inventory property.executionScope");
  validateInput(property.input, "inventory property.input");
  return property;
}

export function validateInput(value, label = "input") {
  const input = requireRecord(value, label);
  requireEnum(input.kind, INPUT_KINDS, `${label}.kind`);
  const requiredByKind = {
    state: ["snapshotRef", "property"],
    relational: ["leftSubject", "rightSubject", "relation"],
    trace: ["traceRef", "clockProfile", "completion"],
    fault: ["faultModelRef", "requestedInjectionSite", "mitigationOracle"],
    manual: ["reviewProcedure", "requiredDisposition"],
  };
  for (const field of requiredByKind[input.kind]) {
    if (!(field in input)) throw new TypeError(`${label}.${field} is required`);
  }
  if (input.kind === "state") validateArtifactRef(input.snapshotRef, `${label}.snapshotRef`);
  if (input.kind === "trace") validateArtifactRef(input.traceRef, `${label}.traceRef`);
  if (input.kind === "fault") validateArtifactRef(input.faultModelRef, `${label}.faultModelRef`);
  if (input.kind === "manual") {
    requireString(input.reviewProcedure, `${label}.reviewProcedure`);
    if (!Array.isArray(input.requiredDisposition) || input.requiredDisposition.length === 0) {
      throw new TypeError(`${label}.requiredDisposition must be a non-empty array`);
    }
  }
  return input;
}

export function canonicalJson(value) {
  return `${JSON.stringify(sortKeys(value), null, 2)}\n`;
}

export function exactValue(value) {
  return JSON.stringify(sortKeys(value));
}

function sortKeys(value) {
  if (Array.isArray(value)) return value.map(sortKeys);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.keys(value)
      .sort()
      .map((key) => [key, sortKeys(value[key])]),
  );
}

function assertAcyclic(tasks) {
  const byId = new Map(tasks.map((task) => [task.taskId, task]));
  const visiting = new Set();
  const visited = new Set();
  const visit = (taskId) => {
    if (visiting.has(taskId)) throw new TypeError(`task dependency cycle at ${taskId}`);
    if (visited.has(taskId)) return;
    visiting.add(taskId);
    for (const dependency of byId.get(taskId).dependencies) visit(dependency);
    visiting.delete(taskId);
    visited.add(taskId);
  };
  for (const task of tasks) visit(task.taskId);
}
