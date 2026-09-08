// SPDX-License-Identifier: AGPL-3.0-or-later

import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";
import canonicalize from "canonicalize";

export const CONTRACT_SCHEMA = deepFreeze(
  JSON.parse(
    readFileSync(
      new URL("../contracts/e02-draft-1.schema.json", import.meta.url),
      "utf8",
    ),
  ),
);

const definitions = CONTRACT_SCHEMA.$defs;
const ajv = new Ajv2020({
  allErrors: true,
  allowUnionTypes: true,
  strict: true,
});
addFormats(ajv);
const validateRoot = ajv.compile(CONTRACT_SCHEMA);
const validators = new Map();

export const CONTRACT_VERSION =
  definitions.TechniqueDefinition.properties.contractVersion.const;

export const INPUT_KINDS = Object.freeze(
  definitions.Input.oneOf.map((entry) => entry.properties.kind.const),
);

export const APPLICABILITY = Object.freeze(
  definitions.VerificationPlan.properties.tasks.items.properties.applicability.enum,
);

export const READINESS = Object.freeze(
  definitions.VerificationPlan.properties.tasks.items.properties.readiness.enum,
);

export const EXECUTION_STATUSES = Object.freeze(
  definitions.TechniqueResult.properties.execution.properties.status.enum,
);

export const LOGICAL_OUTCOMES = Object.freeze(
  definitions.TechniqueResult.properties.logical.properties.outcome.enum,
);

export const CONTRACT_DEFINITIONS = Object.freeze([
  "TechniqueDefinition",
  "ToolCapability",
  "SelectionPolicy",
  "VerificationPlan",
  "TechniqueResult",
]);

export function validateContract(value) {
  return assertValid(validateRoot, value, "E02 contract");
}

export function validateSchemaDefinition(name, value, label = name) {
  if (!(name in definitions)) {
    throw new TypeError(`unknown schema definition ${name}`);
  }
  let validate = validators.get(name);
  if (!validate) {
    validate = ajv.compile({
      $ref: `${CONTRACT_SCHEMA.$id}#/$defs/${name}`,
    });
    validators.set(name, validate);
  }
  return assertValid(validate, value, label);
}

export function validateArtifactRef(value, label = "artifactRef") {
  return validateSchemaDefinition("ArtifactRef", value, label);
}

export function validateTechniqueDefinition(value) {
  return validateSchemaDefinition(
    "TechniqueDefinition",
    value,
    "TechniqueDefinition",
  );
}

export function validateToolCapability(value) {
  return validateSchemaDefinition("ToolCapability", value, "ToolCapability");
}

export function validateSelectionPolicy(value) {
  return validateSchemaDefinition("SelectionPolicy", value, "SelectionPolicy");
}

export function validateVerificationPlan(value) {
  const plan = validateSchemaDefinition(
    "VerificationPlan",
    value,
    "VerificationPlan",
  );
  const taskIds = new Set();
  for (const task of plan.tasks) {
    if (taskIds.has(task.taskId)) {
      throw new TypeError(`duplicate task id ${task.taskId}`);
    }
    taskIds.add(task.taskId);
  }
  for (const task of plan.tasks) {
    for (const dependency of task.dependencies) {
      if (!taskIds.has(dependency)) {
        throw new TypeError(`unknown task dependency ${dependency}`);
      }
    }
  }
  assertAcyclic(plan.tasks);
  return plan;
}

export function validateTechniqueResult(value) {
  return validateSchemaDefinition("TechniqueResult", value, "TechniqueResult");
}

export function validateInventoryProperty(
  value,
  { requireResultBinding = false } = {},
) {
  return validateSchemaDefinition(
    requireResultBinding ? "EvidenceObligation" : "PropertyInventoryItem",
    value,
    requireResultBinding ? "EvidenceObligation" : "PropertyInventoryItem",
  );
}

export function validateResultBinding(value) {
  return validateSchemaDefinition("ResultBinding", value, "ResultBinding");
}

export function validateInput(value, label = "input") {
  return validateSchemaDefinition("Input", value, label);
}

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

export function jcsCanonicalize(value) {
  assertJsonValue(value, "$", new Set());
  const result = canonicalize(value);
  if (typeof result !== "string") {
    throw new TypeError("RFC 8785 JCS input must be a JSON value");
  }
  return result;
}

export function jcsEqual(left, right) {
  return jcsCanonicalize(left) === jcsCanonicalize(right);
}

export function jcsSha256(value) {
  return `sha256-jcs:${createHash("sha256")
    .update(jcsCanonicalize(value), "utf8")
    .digest("hex")}`;
}

function assertValid(validate, value, label) {
  if (validate(value)) return value;
  const details = (validate.errors ?? []).map(formatSchemaError).join("; ");
  throw new TypeError(`${label} failed JSON Schema validation: ${details}`);
}

function formatSchemaError(error) {
  const path = error.instancePath
    .split("/")
    .filter(Boolean)
    .map(unescapeJsonPointer)
    .join(".");
  const location = path.length === 0 ? "" : `.${path}`;
  if (error.keyword === "required") {
    return `${location}.${error.params.missingProperty} is required`;
  }
  if (location.endsWith(".score") && error.message === "boolean schema is false") {
    return `${location} must not contain an ordinal score`;
  }
  return `${location || "."} ${error.message}`;
}

function unescapeJsonPointer(value) {
  return value.replaceAll("~1", "/").replaceAll("~0", "~");
}

function assertJsonValue(value, path, ancestors) {
  if (
    value === null ||
    typeof value === "boolean" ||
    (typeof value === "string" && value.isWellFormed())
  ) {
    return;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw new TypeError(`${path} contains a non-finite number`);
    }
    return;
  }
  if (typeof value === "string") {
    throw new TypeError(`${path} contains an ill-formed Unicode string`);
  }
  if (typeof value !== "object") {
    throw new TypeError(`${path} contains a non-JSON value`);
  }
  if (ancestors.has(value)) {
    throw new TypeError(`${path} contains a cycle`);
  }
  ancestors.add(value);
  if (Array.isArray(value)) {
    for (let index = 0; index < value.length; index += 1) {
      if (!Object.hasOwn(value, index)) {
        throw new TypeError(`${path} contains a sparse array`);
      }
      assertJsonValue(value[index], `${path}[${index}]`, ancestors);
    }
  } else {
    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) {
      throw new TypeError(`${path} contains a non-JSON object`);
    }
    for (const [key, item] of Object.entries(value)) {
      if (!key.isWellFormed()) {
        throw new TypeError(`${path} contains an ill-formed property name`);
      }
      assertJsonValue(item, `${path}.${key}`, ancestors);
    }
  }
  ancestors.delete(value);
}

function assertAcyclic(tasks) {
  const byId = new Map(tasks.map((task) => [task.taskId, task]));
  const visiting = new Set();
  const visited = new Set();
  const visit = (taskId) => {
    if (visiting.has(taskId)) {
      throw new TypeError(`task dependency cycle at ${taskId}`);
    }
    if (visited.has(taskId)) return;
    visiting.add(taskId);
    for (const dependency of byId.get(taskId).dependencies) visit(dependency);
    visiting.delete(taskId);
    visited.add(taskId);
  };
  for (const task of tasks) visit(task.taskId);
}

function deepFreeze(value) {
  if (value !== null && typeof value === "object" && !Object.isFrozen(value)) {
    for (const item of Object.values(value)) deepFreeze(item);
    Object.freeze(value);
  }
  return value;
}
