// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFile } from "node:fs/promises";
import { performance } from "node:perf_hooks";

import { adaptQuoinCatalog } from "./catalog-adapter.mjs";
import {
  CONTRACT_VERSION,
  jcsEqual,
  jcsSha256,
  requireString,
  validateTechniqueResult,
} from "./contracts.mjs";
import { assessEvidence } from "./evidence.mjs";
import { buildVerificationPlan } from "./planner.mjs";

export async function runBoundedPortfolio({
  sourceRevision,
  producedAt,
  catalogSnapshot,
  catalogAdapter = adaptQuoinCatalog,
}) {
  requireString(sourceRevision, "sourceRevision");
  requireString(producedAt, "producedAt");
  if (!Number.isFinite(Date.parse(producedAt))) {
    throw new TypeError("producedAt must be an ISO timestamp");
  }
  if (typeof catalogAdapter !== "function") {
    throw new TypeError("catalogAdapter must be a function");
  }
  const catalog = catalogSnapshot ?? JSON.parse(
    await readFile(
      new URL("../fixtures/quoin-catalog-export.json", import.meta.url),
      "utf8",
    ),
  );
  const profile = "quire-verification/local-catalog-portfolio-v1";
  const sourceDigest = jcsSha256({
    sourceRevision,
    catalogRevision: catalog.source.quoinRevision,
    moduleRevision: catalog.source.moduleRevision,
    catalog: catalog.catalog,
  });
  const ref = (identity, revision = sourceRevision) => ({
    authority: "github.com/agent-ix/quire-verification",
    identity,
    revision,
    digest: sourceDigest,
    profile,
  });
  const common = {
    subjectRefs: [ref("repository:quire-verification")],
    profile,
    dependencyClosure: sourceDigest,
    executionScope: {
      repository: "agent-ix/quire-verification",
      revision: sourceRevision,
      paths: ["src/catalog-adapter.mjs", "fixtures/quoin-catalog-export.json"],
    },
    freshness: { maxAgeSeconds: 2_592_000 },
  };
  const runRef = ref("run:local-catalog-portfolio");
  const properties = [
    {
      ...common,
      propertyId: "LOCAL-CATALOG-PARITY",
      propertyRef: ref("property:quoin-catalog-field-parity"),
      bindingRefs: [ref("binding:pinned-quoin-catalog-export")],
      input: {
        kind: "state",
        snapshotRef: ref("snapshot:pinned-quoin-catalog"),
        property: "all pinned method IDs and fields survive adaptation",
      },
    },
    {
      ...common,
      propertyId: "LOCAL-CATALOG-PERMUTATION",
      propertyRef: ref("property:catalog-permutation-invariance"),
      bindingRefs: [ref("binding:catalog-adapter")],
      input: {
        kind: "relational",
        leftSubject: "catalog:original-order",
        rightSubject: "catalog:permuted-order",
        relation: "adapted technique registry is bytewise equal",
      },
    },
    {
      ...common,
      propertyId: "LOCAL-HUMAN-REVIEW",
      propertyRef: ref("property:e02-boundary-human-review"),
      bindingRefs: [],
      input: {
        kind: "manual",
        reviewProcedure:
          "Review identity ownership, compatibility behavior, claim limits, and rights disposition.",
        requiredDisposition: ["accept", "revise", "reject"],
      },
    },
  ];
  const policy = {
    contractVersion: CONTRACT_VERSION,
    kind: "SelectionPolicy",
    policyId: "POLICY-LOCAL-CATALOG-PORTFOLIO",
    version: "1",
    validUntil: "2026-10-07T00:00:00Z",
    sourceRef: ref("policy:local-catalog-portfolio"),
    provenance: {
      kind: "project-authored",
      standardDerived: false,
      rights: "AGPL-3.0-or-later",
    },
    rules: [
      {
        ruleId: "fixed-parity",
        priority: "required",
        explanation: "Compare every field with the pinned existing behavior.",
      },
      {
        ruleId: "permutation-invariance",
        priority: "recommended",
        explanation: "Compare related executions under input-order changes.",
      },
      {
        ruleId: "human-boundary-review",
        priority: "required",
        explanation: "A human owner decides whether the provisional boundary is accepted.",
      },
    ],
  };
  const capabilities = [
    capability("CAP-LOCAL-UNIT", "unit-testing", "state", profile),
    capability(
      "CAP-LOCAL-METAMORPHIC",
      "metamorphic-testing",
      "relational",
      profile,
    ),
    {
      ...capability("CAP-HUMAN-REVIEW", "inspection", "manual", profile),
      runner: {
        kind: "manual",
        adapterId: "human-review-record",
        adapterVersion: "1",
      },
      prerequisites: [{ kind: "reviewer", id: "verification-owner", required: true }],
      environmentPermissions: [],
    },
  ];
  const selections = [
    selection("LOCAL-CATALOG-PARITY", "unit-testing", "required"),
    selection(
      "LOCAL-CATALOG-PERMUTATION",
      "metamorphic-testing",
      "recommended",
    ),
    selection("LOCAL-HUMAN-REVIEW", "inspection", "required"),
  ];
  for (const [index, property] of properties.entries()) {
    property.techniqueId = selections[index].techniqueId;
    property.resultBinding = resultBinding({
      planId: "PLAN-LOCAL-CATALOG-PORTFOLIO",
      taskId: `verify:${property.propertyId}:${selections[index].techniqueId}`,
      runRef,
    });
  }
  const plan = buildVerificationPlan({
    planId: "PLAN-LOCAL-CATALOG-PORTFOLIO",
    asOf: producedAt,
    policy,
    properties,
    capabilities,
    selections,
    availablePrerequisites: [],
    budget: { wallClockSeconds: 10, parallelism: 1 },
  });

  const parity = observeTechnique(() => checkCatalogParity(catalog, catalogAdapter));
  const permutation = observeTechnique(() =>
    checkPermutationInvariance(catalog, catalogAdapter),
  );
  const unitMilliseconds = parity.elapsedMilliseconds;
  const metamorphicMilliseconds = permutation.elapsedMilliseconds;

  const evidenceInventory = properties;
  const results = [
    observedResult({
      property: evidenceInventory[0],
      techniqueId: "unit-testing",
      resultId: "RESULT-LOCAL-UNIT",
      producedAt,
      sourceRevision,
      runRef,
      observation: parity,
    }),
    observedResult({
      property: evidenceInventory[1],
      techniqueId: "metamorphic-testing",
      resultId: "RESULT-LOCAL-METAMORPHIC",
      producedAt,
      sourceRevision,
      runRef,
      observation: permutation,
    }),
    incompleteManualResult({
      property: evidenceInventory[2],
      producedAt,
      sourceRevision,
      runRef,
    }),
  ];
  results.forEach(validateTechniqueResult);
  return {
    schemaVersion: "quire-verification/local-portfolio-report/v1",
    claimLimit:
      "Actual local adapter checks only; no external method, parser, fault, or human review is claimed.",
    sourceRevision,
    producedAt,
    setupEffort: {
      kind: "one-time",
      measuredSeparatelyIn: "docs/evaluation-report.md",
    },
    runEffort: {
      unitMilliseconds,
      metamorphicMilliseconds,
      manual: "pending verification-owner",
    },
    plan,
    results,
    assessment: assessEvidence({
      inventory: evidenceInventory,
      results,
      asOf: producedAt,
    }),
  };
}

function capability(capabilityId, techniqueId, inputKind, profile) {
  return {
    contractVersion: CONTRACT_VERSION,
    kind: "ToolCapability",
    capabilityId,
    techniqueId,
    qualification: "qualified",
    inputKinds: [inputKind],
    profiles: [profile],
    prerequisites: [],
    runner: {
      kind: "native-producer",
      adapterId: "quire-verification-local-portfolio",
      adapterVersion: "1",
    },
    environmentPermissions: ["read-repository-fixtures"],
    nativeArtifactInterface: { mode: "stdout-json", mediaTypes: ["application/json"] },
    limits: { subject: "catalog adapter at one source revision" },
  };
}

function selection(propertyId, techniqueId, priority) {
  return {
    propertyId,
    techniqueId,
    applicability: "applicable",
    priority,
    reasons: [{ rule: "project-authored", value: propertyId }],
  };
}

function observedResult({
  property,
  techniqueId,
  resultId,
  producedAt,
  sourceRevision,
  runRef,
  observation,
}) {
  const completed = observation.executionStatus === "completed";
  return resultBase({
    property,
    techniqueId,
    resultId,
    producedAt,
    sourceRevision,
    runRef,
    execution: {
      status: observation.executionStatus,
      detail: observation.detail,
      elapsedMilliseconds: observation.elapsedMilliseconds,
    },
    logical: {
      outcome: completed
        ? observation.passed
          ? "satisfied"
          : "violated"
        : "not-evaluated",
    },
    activation: { status: "not-required" },
    participation: { status: "complete", participants: ["catalog-adapter"] },
    assumptions: [{ assumptionId: "pinned-source-fixture", status: "met" }],
    adequacy: completed
      ? { status: "adequate", observations: [observation.detail] }
      : { status: "not-assessed", observations: [] },
  });
}

function incompleteManualResult({ property, producedAt, sourceRevision, runRef }) {
  return resultBase({
    property,
    techniqueId: "inspection",
    resultId: "RESULT-LOCAL-HUMAN-REVIEW-PENDING",
    producedAt,
    sourceRevision,
    runRef,
    execution: { status: "not-run", detail: "verification-owner review pending" },
    logical: { outcome: "not-evaluated" },
    activation: { status: "not-required" },
    participation: { status: "not-started", participants: [] },
    assumptions: [],
    adequacy: { status: "not-assessed", observations: [] },
  });
}

function resultBase({
  property,
  techniqueId,
  resultId,
  producedAt,
  sourceRevision,
  runRef,
  ...dimensions
}) {
  return {
    contractVersion: CONTRACT_VERSION,
    kind: "TechniqueResult",
    resultId,
    planId: "PLAN-LOCAL-CATALOG-PORTFOLIO",
    taskId: `verify:${property.propertyId}:${techniqueId}`,
    techniqueId,
    producedAt,
    runRef,
    propertyRef: property.propertyRef,
    subjectRefs: property.subjectRefs,
    profile: property.profile,
    bindingRefs: property.bindingRefs,
    dependencyClosure: property.dependencyClosure,
    executionScope: property.executionScope,
    ...dimensions,
    provenance: {
      producer: "quire-verification-local-portfolio",
      producerVersion: "1",
      adapter: "native-node",
      adapterVersion: "1",
      sourceRevision,
      nodeVersion: process.version,
      executionMode: "native-local",
    },
    nativeArtifacts: [],
  };
}

function resultBinding({ planId, taskId, runRef }) {
  return {
    planId,
    taskId,
    runRef,
    producer: "quire-verification-local-portfolio",
    producerVersion: "1",
    adapter: "native-node",
    adapterVersion: "1",
  };
}

function observeTechnique(check) {
  const started = performance.now();
  try {
    const observation = check();
    return {
      executionStatus: "completed",
      passed: observation.passed,
      detail: observation.detail,
      elapsedMilliseconds: performance.now() - started,
    };
  } catch (error) {
    return {
      executionStatus: "failed",
      passed: false,
      detail: error instanceof Error ? error.message : String(error),
      elapsedMilliseconds: performance.now() - started,
    };
  }
}

function checkCatalogParity(catalog, catalogAdapter) {
  const adapted = catalogAdapter(catalog);
  const methods = catalog.catalog.methods;
  const sourceById = uniqueBy(methods, (method) => method.id, "source method");
  const adaptedById = uniqueBy(
    adapted.techniques,
    (technique) => technique.techniqueId,
    "adapted technique",
  );
  const failures = [];
  if (!adapted.qualified) failures.push("adapted catalog is unqualified");
  for (const [techniqueId, method] of sourceById) {
    const technique = adaptedById.get(techniqueId);
    if (!technique) {
      failures.push(`missing adapted technique ${techniqueId}`);
      continue;
    }
    const expected = {
      techniqueId: method.id,
      name: method.name,
      class: method.class,
      purpose: method.definition,
      evidenceKind: method.evidenceKind,
      applicability: method.applicability,
      legacyToolingHints: method.tooling,
      sourceModule: method.moduleName,
    };
    const actual = Object.fromEntries(
      Object.keys(expected).map((field) => [field, technique[field]]),
    );
    if (!jcsEqual(actual, expected)) failures.push(`field mismatch for ${techniqueId}`);
  }
  for (const techniqueId of adaptedById.keys()) {
    if (!sourceById.has(techniqueId)) failures.push(`unexpected adapted technique ${techniqueId}`);
  }
  return failures.length === 0
    ? { passed: true, detail: `checked ${sourceById.size} method definitions by techniqueId` }
    : { passed: false, detail: failures.join("; ") };
}

function checkPermutationInvariance(catalog, catalogAdapter) {
  const baseline = catalogAdapter(catalog).techniques;
  const reversed = structuredClone(catalog);
  reversed.catalog.methods.reverse();
  const rotated = structuredClone(catalog);
  if (rotated.catalog.methods.length > 0) {
    rotated.catalog.methods.push(rotated.catalog.methods.shift());
  }
  const passed =
    jcsEqual(catalogAdapter(reversed).techniques, baseline) &&
    jcsEqual(catalogAdapter(rotated).techniques, baseline);
  return {
    passed,
    detail: passed
      ? "checked reversed and rotated catalog orders"
      : "catalog adaptation changed under input permutation",
  };
}

function uniqueBy(values, key, label) {
  if (!Array.isArray(values)) throw new TypeError(`${label}s must be an array`);
  const byKey = new Map();
  for (const value of values) {
    const id = key(value);
    if (byKey.has(id)) throw new Error(`duplicate ${label} ID ${id}`);
    byKey.set(id, value);
  }
  return byKey;
}
