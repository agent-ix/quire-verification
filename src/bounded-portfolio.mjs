// SPDX-License-Identifier: AGPL-3.0-or-later

import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { performance } from "node:perf_hooks";

import { adaptQuoinCatalog } from "./catalog-adapter.mjs";
import { CONTRACT_VERSION, exactValue } from "./contracts.mjs";
import { assessEvidence } from "./evidence.mjs";
import { buildVerificationPlan } from "./planner.mjs";

export async function runBoundedPortfolio({ sourceRevision, producedAt }) {
  const catalog = JSON.parse(
    await readFile(
      new URL("../fixtures/quoin-catalog-export.json", import.meta.url),
      "utf8",
    ),
  );
  const profile = "quire-verification/local-catalog-portfolio-v1";
  const sourceDigest = digest(
    JSON.stringify({
      sourceRevision,
      catalogRevision: catalog.source.quoinRevision,
      moduleRevision: catalog.source.moduleRevision,
    }),
  );
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

  const unitStart = performance.now();
  const baseline = adaptQuoinCatalog(catalog);
  if (!baseline.qualified || baseline.techniques.length !== catalog.catalog.methods.length) {
    throw new Error("catalog parity precondition failed");
  }
  for (const [index, technique] of baseline.techniques.entries()) {
    const method = catalog.catalog.methods[index];
    if (
      technique.techniqueId !== method.id ||
      technique.name !== method.name ||
      technique.class !== method.class ||
      technique.purpose !== method.definition ||
      technique.evidenceKind !== method.evidenceKind ||
      exactValue(technique.applicability) !== exactValue(method.applicability) ||
      exactValue(technique.legacyToolingHints) !== exactValue(method.tooling) ||
      technique.sourceModule !== method.moduleName
    ) {
      throw new Error(`catalog field parity failed for ${method.id}`);
    }
  }
  const unitMilliseconds = performance.now() - unitStart;

  const metamorphicStart = performance.now();
  const reversed = structuredClone(catalog);
  reversed.catalog.methods.reverse();
  const rotated = structuredClone(catalog);
  rotated.catalog.methods.push(rotated.catalog.methods.shift());
  const expected = exactValue(baseline.techniques);
  if (
    exactValue(adaptQuoinCatalog(reversed).techniques) !== expected ||
    exactValue(adaptQuoinCatalog(rotated).techniques) !== expected
  ) {
    throw new Error("catalog adaptation changed under input permutation");
  }
  const metamorphicMilliseconds = performance.now() - metamorphicStart;

  const evidenceInventory = properties.map((property, index) => ({
    ...property,
    techniqueId: selections[index].techniqueId,
  }));
  const results = [
    successfulResult({
      property: evidenceInventory[0],
      techniqueId: "unit-testing",
      resultId: "RESULT-LOCAL-UNIT",
      producedAt,
      sourceRevision,
      milliseconds: unitMilliseconds,
      detail: `checked ${baseline.techniques.length} method definitions`,
    }),
    successfulResult({
      property: evidenceInventory[1],
      techniqueId: "metamorphic-testing",
      resultId: "RESULT-LOCAL-METAMORPHIC",
      producedAt,
      sourceRevision,
      milliseconds: metamorphicMilliseconds,
      detail: "checked reversed and rotated catalog orders",
    }),
    incompleteManualResult({
      property: evidenceInventory[2],
      producedAt,
      sourceRevision,
    }),
  ];
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

function successfulResult({
  property,
  techniqueId,
  resultId,
  producedAt,
  sourceRevision,
  milliseconds,
  detail,
}) {
  return resultBase({
    property,
    techniqueId,
    resultId,
    producedAt,
    sourceRevision,
    execution: { status: "completed", detail, elapsedMilliseconds: milliseconds },
    logical: { outcome: "satisfied" },
    activation: { status: "not-required" },
    participation: { status: "complete", participants: ["catalog-adapter"] },
    assumptions: [{ assumptionId: "pinned-source-fixture", status: "met" }],
    adequacy: { status: "adequate", observations: [detail] },
  });
}

function incompleteManualResult({ property, producedAt, sourceRevision }) {
  return resultBase({
    property,
    techniqueId: "inspection",
    resultId: "RESULT-LOCAL-HUMAN-REVIEW-PENDING",
    producedAt,
    sourceRevision,
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
      sourceRevision,
      nodeVersion: process.version,
      executionMode: "native-local",
    },
    nativeArtifacts: [],
  };
}

function digest(value) {
  return `sha256:${createHash("sha256").update(value).digest("hex")}`;
}
