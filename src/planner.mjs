// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  CONTRACT_VERSION,
  INPUT_KINDS,
  requireEnum,
  requireRecord,
  requireString,
  validateSelectionPolicy,
  validateToolCapability,
  validateVerificationPlan,
} from "./contracts.mjs";

const FAULT_SAFETY_PREREQUISITES = [
  { kind: "environment-authorization", id: "non-production-fault-scope" },
  { kind: "injection-binding", id: "confirmed-injection-site" },
  { kind: "observation-binding", id: "independent-effect-observation" },
];

export function buildVerificationPlan(request) {
  const input = requireRecord(request, "plan request");
  requireString(input.planId, "planId");
  const policy = validateSelectionPolicy(input.policy);
  if (!Array.isArray(input.properties) || !Array.isArray(input.selections)) {
    throw new TypeError("properties and selections must be arrays");
  }
  const properties = new Map(
    input.properties.map((property) => {
      requireString(property.propertyId, "property.propertyId");
      requireEnum(property.input.kind, INPUT_KINDS, "property.input.kind");
      return [property.propertyId, property];
    }),
  );
  const capabilities = new Map();
  for (const capability of input.capabilities ?? []) {
    validateToolCapability(capability);
    const list = capabilities.get(capability.techniqueId) ?? [];
    list.push(capability);
    capabilities.set(capability.techniqueId, list);
  }
  const available = new Set(
    (input.availablePrerequisites ?? []).map(prerequisiteKey),
  );

  const tasks = [];
  for (const selection of input.selections) {
    const property = properties.get(selection.propertyId);
    if (!property) {
      throw new Error(`selection names unknown property ${selection.propertyId}`);
    }
    const taskId = `verify:${property.propertyId}:${selection.techniqueId}`;
    const capability = chooseCapability(
      capabilities.get(selection.techniqueId) ?? [],
      property,
      selection.capabilityId,
    );
    let readiness = "ready";
    const gaps = [];
    let prerequisites = [];

    if (!capability) {
      readiness = "unsupported";
      gaps.push({
        kind: "unsupported-capability",
        detail: `no capability supports ${property.input.kind}/${property.profile}`,
      });
    } else {
      prerequisites = [...capability.prerequisites];
      if (property.input.kind === "fault") {
        prerequisites = mergePrerequisites(
          prerequisites,
          FAULT_SAFETY_PREREQUISITES,
        );
      }
      if (capability.qualification !== "qualified") {
        prerequisites = mergePrerequisites(prerequisites, [
          {
            kind: "capability-qualification",
            id: capability.capabilityId,
          },
        ]);
      }
      const missing = prerequisites.filter(
        (prerequisite) => prerequisite.required !== false && !available.has(prerequisiteKey(prerequisite)),
      );
      if (missing.length > 0) readiness = "blocked";
      for (const prerequisite of missing) {
        const setupTaskId = `setup:${property.propertyId}:${selection.techniqueId}:${prerequisite.kind}:${prerequisite.id}`;
        tasks.push({
          taskId: setupTaskId,
          kind: "manual-prerequisite",
          propertyId: property.propertyId,
          techniqueId: selection.techniqueId,
          input: {
            kind: "manual",
            action: "satisfy-prerequisite",
            prerequisite,
            reviewProcedure: `Resolve ${prerequisite.kind}:${prerequisite.id} and record its source and disposition.`,
            requiredDisposition: ["satisfied", "deferred", "rejected"],
          },
          applicability: selection.applicability,
          readiness: "blocked",
          priority: selection.priority,
          dependencies: [],
          missingPrerequisites: [prerequisite],
          runnerRequest: null,
          invocationAllowed: false,
        });
      }
    }

    tasks.push({
      taskId,
      kind: "verification",
      propertyId: property.propertyId,
      techniqueId: selection.techniqueId,
      input: property.input,
      applicability: selection.applicability,
      readiness,
      priority: selection.priority,
      reasons: selection.reasons ?? [],
      dependencies: tasks
        .filter(
          (task) =>
            task.kind === "manual-prerequisite" &&
            task.propertyId === property.propertyId &&
            task.techniqueId === selection.techniqueId,
        )
        .map((task) => task.taskId),
      missingPrerequisites: prerequisites.filter(
        (prerequisite) => prerequisite.required !== false && !available.has(prerequisiteKey(prerequisite)),
      ),
      gaps,
      runnerRequest: capability
        ? {
            ...capability.runner,
            capabilityId: capability.capabilityId,
            environmentPermissions: capability.environmentPermissions ?? [],
            budget: property.budget ?? input.budget ?? null,
            nativeArtifactInterface: capability.nativeArtifactInterface,
          }
        : null,
      invocationAllowed: readiness === "ready" && selection.applicability === "applicable",
    });
  }

  tasks.sort((a, b) => compare(a.taskId, b.taskId));
  const plan = {
    contractVersion: CONTRACT_VERSION,
    kind: "VerificationPlan",
    planId: input.planId,
    policyRef: {
      policyId: policy.policyId,
      version: policy.version,
      sourceRef: policy.sourceRef,
    },
    inventory: [...properties.values()],
    tasks,
    budget: input.budget ?? null,
    evidenceOwner: "Quoin",
    sufficiencyOwner: "authored-human-decision",
  };
  validateVerificationPlan(plan);
  return plan;
}

function chooseCapability(capabilities, property, requestedId) {
  const supported = capabilities.filter(
    (capability) =>
      capability.inputKinds.includes(property.input.kind) &&
      capability.profiles.includes(property.profile),
  );
  if (requestedId) {
    return supported.find((capability) => capability.capabilityId === requestedId);
  }
  if (supported.length > 1) {
    throw new Error(
      `multiple capabilities support ${property.propertyId}; selection must name capabilityId`,
    );
  }
  return supported[0];
}

function mergePrerequisites(left, right) {
  const merged = new Map();
  for (const prerequisite of [...left, ...right]) {
    merged.set(prerequisiteKey(prerequisite), {
      required: true,
      ...prerequisite,
    });
  }
  return [...merged.values()].sort((a, b) =>
    compare(prerequisiteKey(a), prerequisiteKey(b)),
  );
}

function prerequisiteKey(prerequisite) {
  return `${prerequisite.kind}:${prerequisite.id}`;
}

function compare(a, b) {
  return a === b ? 0 : a < b ? -1 : 1;
}
