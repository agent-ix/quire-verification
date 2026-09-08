// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  runBoundedPortfolio,
  validateTechniqueResult,
  validateVerificationPlan,
} from "../src/index.mjs";

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
    "46900c2f793fc7a3bcc0f6f84aa112846093b9c8",
  );
  validateVerificationPlan(report.plan);
  report.results.forEach(validateTechniqueResult);
  assert.deepEqual(
    report.assessment.obligations.map((obligation) => obligation.status),
    ["supported", "supported", "incomplete"],
  );
  assert.equal(report.assessment.sufficiencyDecision, "not-made");
});
