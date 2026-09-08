// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  jcsCanonicalize,
  jcsEqual,
  validateInventoryProperty,
  validateTechniqueResult,
} from "./contracts.mjs";

export function assessEvidence({ inventory, results, asOf }) {
  if (!Array.isArray(inventory) || !Array.isArray(results)) {
    throw new TypeError("inventory and results must be arrays");
  }
  const at = Date.parse(asOf);
  if (!Number.isFinite(at)) throw new TypeError("asOf must be an ISO timestamp");
  const resultIds = new Set();
  for (const result of results) {
    validateTechniqueResult(result);
    if (resultIds.has(result.resultId)) {
      throw new TypeError(`duplicate result id ${result.resultId}`);
    }
    resultIds.add(result.resultId);
  }

  const obligations = inventory.map((property) => {
    validateInventoryProperty(property, { requireResultBinding: true });
    const candidates = results.filter(
      (result) => result.propertyRef.identity === property.propertyRef.identity,
    );
    const exact = candidates
      .filter((result) => scopeDifferences(property, result).length === 0)
      .sort((a, b) => compare(a.resultId, b.resultId));
    if (exact.length === 0) {
      return {
        propertyId: property.propertyId,
        status: "missing",
        resultId: null,
        gaps: candidates
          .flatMap((result) =>
            scopeDifferences(property, result).map((dimension) => ({
              kind: "scope-mismatch",
              dimension,
              resultId: result.resultId,
            })),
          )
          .sort((a, b) => compare(jcsCanonicalize(a), jcsCanonicalize(b))),
      };
    }
    if (exact.length > 1) {
      return {
        propertyId: property.propertyId,
        status: "ambiguous",
        resultId: null,
        candidateResultIds: exact.map((result) => result.resultId),
        gaps: [
          {
            kind: "ambiguous-results",
            resultIds: exact.map((result) => result.resultId),
          },
        ],
      };
    }
    const result = exact[0];
    const findings = findingsFor(result);
    if (isStale(property, result, at)) {
      return view(property, result, "stale", [
        { kind: "freshness-expired" },
        ...findings,
      ]);
    }
    const status = classify(result);
    return view(property, result, status, findings);
  });

  return {
    asOf,
    denominator: inventory.length,
    obligations,
    evidenceOwner: "Quoin",
    sufficiencyDecision: "not-made",
    statusSemantics:
      "compatibility summary only; gaps enumerate every applicable independent result finding",
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
    ["planId", property.resultBinding.planId, result.planId],
    ["taskId", property.resultBinding.taskId, result.taskId],
    ["runRef", property.resultBinding.runRef, result.runRef],
    ["producer", property.resultBinding.producer, result.provenance.producer],
    [
      "producerVersion",
      property.resultBinding.producerVersion,
      result.provenance.producerVersion,
    ],
    ["adapter", property.resultBinding.adapter, result.provenance.adapter],
    [
      "adapterVersion",
      property.resultBinding.adapterVersion,
      result.provenance.adapterVersion,
    ],
  ];
  return pairs
    .filter(([, expected, actual]) => !jcsEqual(expected, actual))
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

export function findingsFor(result) {
  const gaps = [];
  if (result.execution.status === "unsupported") {
    gaps.push({ kind: "unsupported", detail: result.execution.detail ?? null });
  } else if (result.execution.status === "failed") {
    gaps.push({ kind: "failed", detail: result.execution.detail ?? null });
  } else if (result.execution.status !== "completed") {
    gaps.push({ kind: "incomplete", detail: result.execution.detail ?? null });
  }
  if (result.activation.status === "not-activated") {
    gaps.push({ kind: "required-activation-absent" });
  } else if (result.activation.status === "unknown") {
    gaps.push({ kind: "activation-unknown" });
  }
  if (["inadequate", "unknown", "not-assessed"].includes(result.adequacy.status)) {
    gaps.push({
      kind: "observation-inadequate",
      missing: result.adequacy.missingObservations ?? [],
    });
  }
  if (["partial", "not-started", "unknown"].includes(result.participation.status)) {
    gaps.push({
      kind: "partial-participation",
      status: result.participation.status,
      missing: result.participation.missingParticipants ?? [],
    });
  }
  for (const assumption of result.assumptions) {
    if (assumption.status === "not-met") {
      gaps.push({ kind: "assumption-unmet", assumptionId: assumption.assumptionId });
    } else if (assumption.status !== "met") {
      gaps.push({
        kind: "assumption-inconclusive",
        assumptionId: assumption.assumptionId,
        status: assumption.status,
      });
    }
  }
  if (result.logical.outcome === "violated") {
    gaps.push({ kind: "logical-violation" });
  } else if (
    result.execution.status === "completed" &&
    !["satisfied", "violated"].includes(result.logical.outcome)
  ) {
    gaps.push({ kind: "logical-inconclusive", outcome: result.logical.outcome });
  }
  return gaps.sort((a, b) => compare(jcsCanonicalize(a), jcsCanonicalize(b)));
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

function compare(a, b) {
  return a === b ? 0 : a < b ? -1 : 1;
}
