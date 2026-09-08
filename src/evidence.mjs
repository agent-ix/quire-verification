// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  exactValue,
  validateInventoryProperty,
  validateTechniqueResult,
} from "./contracts.mjs";

export function assessEvidence({ inventory, results, asOf }) {
  if (!Array.isArray(inventory) || !Array.isArray(results)) {
    throw new TypeError("inventory and results must be arrays");
  }
  const at = Date.parse(asOf);
  if (!Number.isFinite(at)) throw new TypeError("asOf must be an ISO timestamp");
  for (const result of results) validateTechniqueResult(result);

  const obligations = inventory.map((property) => {
    validateInventoryProperty(property);
    const candidates = results.filter(
      (result) => result.propertyRef.identity === property.propertyRef.identity,
    );
    const exact = candidates
      .filter((result) => scopeDifferences(property, result).length === 0)
      .sort((a, b) => Date.parse(b.producedAt) - Date.parse(a.producedAt));
    if (exact.length === 0) {
      return {
        propertyId: property.propertyId,
        status: "missing",
        resultId: null,
        gaps: candidates.flatMap((result) =>
          scopeDifferences(property, result).map((dimension) => ({
            kind: "scope-mismatch",
            dimension,
            resultId: result.resultId,
          })),
        ),
      };
    }
    const result = exact[0];
    if (isStale(property, result, at)) {
      return view(property, result, "stale", [{ kind: "freshness-expired" }]);
    }
    const status = classify(result);
    return view(property, result, status, gapsFor(result, status));
  });

  return {
    asOf,
    denominator: inventory.length,
    obligations,
    evidenceOwner: "Quoin",
    sufficiencyDecision: "not-made",
  };
}

export function scopeDifferences(property, result) {
  const pairs = [
    ["techniqueId", property.techniqueId, result.techniqueId],
    ["propertyRef", property.propertyRef, result.propertyRef],
    ["subjectRefs", property.subjectRefs, result.subjectRefs],
    ["profile", property.profile, result.profile],
    ["bindingRefs", property.bindingRefs, result.bindingRefs],
    ["dependencyClosure", property.dependencyClosure, result.dependencyClosure],
    ["executionScope", property.executionScope, result.executionScope],
  ];
  return pairs
    .filter(([, expected, actual]) => exactValue(expected) !== exactValue(actual))
    .map(([dimension]) => dimension);
}

function isStale(property, result, asOf) {
  if (result.validUntil) return asOf > Date.parse(result.validUntil);
  const maxAgeSeconds = property.freshness?.maxAgeSeconds;
  if (maxAgeSeconds === undefined) return false;
  return asOf - Date.parse(result.producedAt) > maxAgeSeconds * 1000;
}

function classify(result) {
  if (result.execution.status === "unsupported") return "unsupported";
  if (result.execution.status === "failed") return "failed";
  if (result.execution.status !== "completed") return "incomplete";
  if (result.activation.status === "not-activated") return "untriggered";
  if (["inadequate", "unknown", "not-assessed"].includes(result.adequacy.status)) {
    return "inadequate";
  }
  if (result.activation.status === "unknown") return "incomplete";
  if (["partial", "not-started", "unknown"].includes(result.participation.status)) {
    return "partial";
  }
  if (result.assumptions.some((assumption) => assumption.status === "not-met")) {
    return "assumption-unmet";
  }
  if (result.assumptions.some((assumption) => assumption.status !== "met")) {
    return "inconclusive";
  }
  if (result.logical.outcome === "violated") return "violated";
  if (result.logical.outcome === "satisfied") return "supported";
  return "inconclusive";
}

function gapsFor(result, status) {
  const gaps = [];
  if (status === "untriggered") gaps.push({ kind: "required-activation-absent" });
  if (status === "inadequate") {
    gaps.push({
      kind: "observation-inadequate",
      missing: result.adequacy.missingObservations ?? [],
    });
  }
  if (status === "partial") gaps.push({ kind: "partial-participation" });
  if (status === "assumption-unmet") gaps.push({ kind: "assumption-unmet" });
  if (["failed", "incomplete", "unsupported", "inconclusive"].includes(status)) {
    gaps.push({ kind: status, detail: result.execution.detail ?? null });
  }
  return gaps;
}

function view(property, result, status, gaps) {
  return {
    propertyId: property.propertyId,
    status,
    freshness: status === "stale" ? "stale" : "fresh",
    resultId: result.resultId,
    execution: result.execution,
    logical: result.logical,
    activation: result.activation,
    participation: result.participation,
    assumptions: result.assumptions,
    adequacy: result.adequacy,
    provenance: result.provenance,
    nativeArtifacts: result.nativeArtifacts,
    gaps,
  };
}
