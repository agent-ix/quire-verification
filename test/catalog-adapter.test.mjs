// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { adaptQuoinAdvice, adaptQuoinCatalog } from "../src/index.mjs";

const catalogFixture = JSON.parse(
  await readFile(new URL("../fixtures/quoin-catalog-export.json", import.meta.url)),
);
const adviceFixture = JSON.parse(
  await readFile(new URL("../fixtures/quoin-advice-export.json", import.meta.url)),
);

test("preserves every pinned Quoin method ID and descriptive field", () => {
  const adapted = adaptQuoinCatalog(catalogFixture);
  assert.equal(adapted.qualified, true);
  assert.equal(adapted.techniques.length, 33);
  assert.deepEqual(
    adapted.techniques.map((method) => method.techniqueId),
    catalogFixture.catalog.methods.map((method) => method.id),
  );
  for (const [index, method] of catalogFixture.catalog.methods.entries()) {
    const technique = adapted.techniques[index];
    assert.equal(technique.techniqueId, method.id);
    assert.equal(technique.name, method.name);
    assert.equal(technique.class, method.class);
    assert.equal(technique.purpose, method.definition);
    assert.equal(technique.evidenceKind, method.evidenceKind ?? null);
    assert.deepEqual(technique.applicability, method.applicability);
    assert.deepEqual(technique.legacyToolingHints, method.tooling);
  }
});

test("preserves the active first-wins view but refuses to qualify conflicts or unreadable inputs", () => {
  const adverse = structuredClone(catalogFixture);
  adverse.catalog.duplicates.push({
    id: "fuzzing",
    modules: ["spec-artifacts-process", "conflicting-module"],
  });
  adverse.catalog.unreadable.push({
    moduleRoot: "/fixture/unreadable",
    reason: "invalid yaml",
  });
  const adapted = adaptQuoinCatalog(adverse);
  assert.equal(adapted.qualified, false);
  assert.equal(adapted.techniques.length, 33);
  assert.deepEqual(
    adapted.diagnostics.map((diagnostic) => diagnostic.kind).sort(),
    ["conflicting-definition", "unreadable-source"],
  );
  assert.match(
    adapted.diagnostics.find((item) => item.kind === "conflicting-definition").effect,
    /first-wins-definition-preserved/,
  );
});

test("maps positive advice to applicable and inconclusive advice to unknown", () => {
  const adapted = adviceFixture.advice.map(adaptQuoinAdvice);
  for (const [index, advice] of adviceFixture.advice.entries()) {
    assert.deepEqual(
      adapted[index].selections.map((selection) => selection.techniqueId),
      advice.recommended.map((recommendation) => recommendation.method),
    );
    assert.ok(
      adapted[index].selections.every(
        (selection) => selection.applicability === "applicable",
      ),
    );
  }
  const unknown = adapted.find((item) => item.propertyId === "unknown");
  assert.equal(unknown.residualApplicability, "unknown");
  assert.equal(unknown.selections.length, 0);
});
