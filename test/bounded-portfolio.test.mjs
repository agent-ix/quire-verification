// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  adaptQuoinCatalog,
  runBoundedPortfolio,
  validateTechniqueResult,
  validateVerificationPlan,
} from "../src/index.mjs";

const catalog = JSON.parse(
  await readFile(
    new URL("../fixtures/quoin-catalog-export.json", import.meta.url),
    "utf8",
  ),
);

test("runs two complementary local methods and retains manual work", async () => {
  const report = await runBoundedPortfolio({
    sourceRevision: "test-revision",
    producedAt: "2026-09-07T21:00:00Z",
  });
  assert.equal(report.plan.tasks.filter((task) => task.kind === "verification").length, 3);
  assert.deepEqual(
    report.results.map((result) => [result.techniqueId, result.execution.status]),
    [
      ["unit-testing", "completed"],
      ["metamorphic-testing", "completed"],
      ["inspection", "not-run"],
    ],
  );
  assert.deepEqual(
    report.assessment.obligations.map((obligation) => obligation.status),
    ["supported", "supported", "incomplete"],
  );
  assert.equal(report.assessment.sufficiencyDecision, "not-made");
  assert.match(report.claimLimit, /no external method/);
});

test("retained local portfolio names its executable revision and incomplete review", async () => {
  const report = JSON.parse(
    await readFile(
      new URL("../evidence/local-catalog-portfolio.json", import.meta.url),
      "utf8",
    ),
  );
  assert.equal(
    report.sourceRevision,
    "cc42165317ec0658b5c8eb4c85ae009143db6d3f",
  );
  validateVerificationPlan(report.plan);
  report.results.forEach(validateTechniqueResult);
  assert.deepEqual(
    report.assessment.obligations.map((obligation) => obligation.status),
    ["supported", "supported", "incomplete"],
  );
  assert.equal(report.assessment.sufficiencyDecision, "not-made");
});

test("catalog parity is keyed by techniqueId rather than fixture order", async () => {
  const shuffled = structuredClone(catalog);
  shuffled.catalog.methods.reverse();
  const report = await runBoundedPortfolio({
    sourceRevision: "test-revision",
    producedAt: "2026-09-07T21:00:00Z",
    catalogSnapshot: shuffled,
  });
  assert.equal(report.results[0].execution.status, "completed");
  assert.equal(report.results[0].logical.outcome, "satisfied");
});

test("a parity violation is retained as evidence instead of thrown", async () => {
  const corruptingAdapter = (snapshot) => {
    const adapted = adaptQuoinCatalog(snapshot);
    adapted.techniques[0].name = "corrupted name";
    return adapted;
  };
  const report = await runBoundedPortfolio({
    sourceRevision: "test-revision",
    producedAt: "2026-09-07T21:00:00Z",
    catalogSnapshot: catalog,
    catalogAdapter: corruptingAdapter,
  });
  assert.equal(report.results[0].execution.status, "completed");
  assert.equal(report.results[0].logical.outcome, "violated");
  assert.equal(report.assessment.obligations[0].status, "violated");
});

test("an adapter exception is retained as a failed TechniqueResult", async () => {
  const report = await runBoundedPortfolio({
    sourceRevision: "test-revision",
    producedAt: "2026-09-07T21:00:00Z",
    catalogSnapshot: catalog,
    catalogAdapter() {
      throw new Error("injected adapter failure");
    },
  });
  assert.deepEqual(
    report.results.slice(0, 2).map((result) => [
      result.execution.status,
      result.logical.outcome,
    ]),
    [
      ["failed", "not-evaluated"],
      ["failed", "not-evaluated"],
    ],
  );
  assert.deepEqual(
    report.assessment.obligations.slice(0, 2).map(({ status }) => status),
    ["failed", "failed"],
  );
});
