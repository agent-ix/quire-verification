// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { assessEvidence, scopeDifferences } from "../src/index.mjs";

const fixture = JSON.parse(
  await readFile(new URL("../fixtures/results.json", import.meta.url)),
);

test("accounts for every obligation without collapsing result states", () => {
  const report = assessEvidence(fixture);
  assert.equal(report.denominator, fixture.inventory.length);
  assert.equal(report.obligations.length, fixture.inventory.length);
  assert.equal(report.evidenceOwner, "Quoin");
  assert.equal(report.sufficiencyDecision, "not-made");
  const statuses = Object.fromEntries(
    report.obligations.map((obligation) => [
      obligation.propertyId,
      obligation.status,
    ]),
  );
  assert.deepEqual(statuses, fixture.expectedStatuses);
});

test("reports exact scope mismatches rather than accepting a tool-name match", () => {
  const report = assessEvidence(fixture);
  const scope = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-SCOPE",
  );
  assert.equal(scope.status, "missing");
  assert.deepEqual(scope.gaps, [
    {
      kind: "scope-mismatch",
      dimension: "executionScope",
      resultId: "RESULT-SCOPE-MISMATCH",
    },
  ]);

  const expected = fixture.inventory.find(
    (obligation) => obligation.propertyId === "OBL-STATE-PROPERTY",
  );
  const actual = structuredClone(
    fixture.results.find((result) => result.resultId === "RESULT-SUPPORTED"),
  );
  actual.techniqueId = "fuzzing";
  assert.deepEqual(scopeDifferences(expected, actual), ["techniqueId"]);
});

test("keeps activation and observation adequacy distinct from logical truth", () => {
  const report = assessEvidence(fixture);
  const untriggered = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-FAULT",
  );
  assert.equal(untriggered.logical.outcome, "satisfied");
  assert.equal(untriggered.activation.status, "not-activated");
  assert.equal(untriggered.status, "untriggered");

  const inadequate = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-TRACE",
  );
  assert.equal(inadequate.logical.outcome, "unknown");
  assert.equal(inadequate.adequacy.status, "inadequate");
  assert.deepEqual(inadequate.gaps[0].missing, ["parse-finish"]);
});

test("keeps partial participation and unmet assumptions visible", () => {
  const report = assessEvidence(fixture);
  const partial = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-PARTIAL",
  );
  assert.equal(partial.logical.outcome, "satisfied");
  assert.equal(partial.participation.status, "partial");
  assert.equal(partial.status, "partial");

  const assumption = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-ASSUMPTION",
  );
  assert.equal(assumption.logical.outcome, "satisfied");
  assert.equal(assumption.assumptions[0].status, "not-met");
  assert.equal(assumption.status, "assumption-unmet");
});

test("joins every required identity and scope dimension exactly", () => {
  const expected = fixture.inventory.find(
    (obligation) => obligation.propertyId === "OBL-STATE-PROPERTY",
  );
  const source = fixture.results.find(
    (result) => result.resultId === "RESULT-SUPPORTED",
  );
  const mutations = [
    ["techniqueId", (result) => (result.techniqueId = "fuzzing")],
    ["propertyRef", (result) => (result.propertyRef.revision = "property-r2")],
    ["subjectRefs", (result) => (result.subjectRefs[0].revision = "parser-r2")],
    ["profile", (result) => (result.profile = "quire-state/other")],
    ["bindingRefs", (result) => (result.bindingRefs[0].revision = "binding-r2")],
    ["dependencyClosure", (result) => (result.dependencyClosure = "sha256:other")],
    ["executionScope", (result) => (result.executionScope.revision = "parser-r2")],
  ];
  for (const [dimension, mutate] of mutations) {
    const result = structuredClone(source);
    mutate(result);
    assert.ok(scopeDifferences(expected, result).includes(dimension), dimension);
  }
});

test("retains native artifacts as references in the assessment view", () => {
  const report = assessEvidence(fixture);
  const supported = report.obligations.find(
    (obligation) => obligation.propertyId === "OBL-STATE-PROPERTY",
  );
  assert.equal(supported.nativeArtifacts.length, 1);
  assert.match(supported.nativeArtifacts[0].uri, /^fixture:\/\/native\//);
  assert.equal(supported.provenance.executionMode, "fixture-replay");
});
