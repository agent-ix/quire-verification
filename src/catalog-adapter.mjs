// SPDX-License-Identifier: AGPL-3.0-or-later

import { createHash } from "node:crypto";

import {
  CONTRACT_VERSION,
  canonicalJson,
  requireRecord,
  requireString,
  validateArtifactRef,
  validateTechniqueDefinition,
} from "./contracts.mjs";

export function adaptQuoinCatalog(snapshot) {
  const input = requireRecord(snapshot, "Quoin catalog snapshot");
  const source = requireRecord(input.source, "snapshot.source");
  const catalog = requireRecord(input.catalog, "snapshot.catalog");
  validateArtifactRef(source.sourceRef, "snapshot.source.sourceRef");
  if (!Array.isArray(catalog.methods)) {
    throw new TypeError("snapshot.catalog.methods must be an array");
  }

  const techniques = catalog.methods.map((method, index) => {
    requireRecord(method, `catalog.methods[${index}]`);
    const techniqueId = requireString(method.id, `catalog.methods[${index}].id`);
    const definition = {
      contractVersion: CONTRACT_VERSION,
      kind: "TechniqueDefinition",
      techniqueId,
      name: requireString(method.name, `${techniqueId}.name`),
      class: requireString(method.class, `${techniqueId}.class`),
      purpose: requireString(method.definition, `${techniqueId}.definition`),
      evidenceKind: method.evidenceKind ?? null,
      applicability: normalizeRules(method.applicability),
      possibleClaims: [],
      legacyToolingHints: Array.isArray(method.tooling)
        ? method.tooling.map(String)
        : [],
      sourceModule: requireString(method.moduleName, `${techniqueId}.moduleName`),
      sourceRef: source.sourceRef,
      definitionDigest: `sha256:${createHash("sha256")
        .update(canonicalJson(method))
        .digest("hex")}`,
    };
    validateTechniqueDefinition(definition);
    return definition;
  });

  const ids = techniques.map((definition) => definition.techniqueId);
  if (new Set(ids).size !== ids.length) {
    throw new Error("post-merge Quoin snapshot contains duplicate active method IDs");
  }
  techniques.sort((a, b) => compare(a.techniqueId, b.techniqueId));

  const diagnostics = [];
  for (const duplicate of catalog.duplicates ?? []) {
    diagnostics.push({
      kind: "conflicting-definition",
      techniqueId: String(duplicate.id),
      modules: [...(duplicate.modules ?? [])].map(String),
      effect: "active-first-wins-definition-preserved; snapshot-unqualified",
    });
  }
  for (const unreadable of catalog.unreadable ?? []) {
    diagnostics.push({
      kind: "unreadable-source",
      source: String(unreadable.moduleRoot),
      reason: String(unreadable.reason),
      effect: "available-definitions-preserved; snapshot-unqualified",
    });
  }
  diagnostics.sort((a, b) => compare(canonicalJson(a), canonicalJson(b)));

  return {
    contractVersion: CONTRACT_VERSION,
    kind: "TechniqueRegistrySnapshot",
    sourceRef: source.sourceRef,
    qualified: diagnostics.length === 0,
    techniques,
    diagnostics,
  };
}

export function adaptQuoinAdvice(advice) {
  const input = requireRecord(advice, "Quoin advice");
  requireString(input.obligation, "advice.obligation");
  if (!Array.isArray(input.recommended)) {
    throw new TypeError("advice.recommended must be an array");
  }
  const selections = input.recommended.map((recommendation) => ({
    propertyId: input.obligation,
    techniqueId: requireString(recommendation.method, "recommendation.method"),
    applicability: "applicable",
    reasons: (recommendation.reasons ?? []).map(({ rule, value }) => ({
      rule: String(rule),
      value: String(value),
    })),
  }));
  return {
    propertyId: input.obligation,
    selections,
    residualApplicability:
      selections.length === 0 && input.inconclusive === true ? "unknown" : null,
    authored: input.authored ?? null,
    mismatch: input.mismatch === true,
    uncatalogued: input.uncatalogued === true,
  };
}

function normalizeRules(value) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return {};
  return Object.fromEntries(
    Object.entries(value)
      .map(([axis, values]) => [
        axis,
        Array.isArray(values) ? values.map(String) : [],
      ])
      .sort(([a], [b]) => compare(a, b)),
  );
}

function compare(a, b) {
  return a === b ? 0 : a < b ? -1 : 1;
}
