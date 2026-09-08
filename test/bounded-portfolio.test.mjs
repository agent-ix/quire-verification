// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import test from "node:test";

import { runBoundedPortfolio } from "../src/index.mjs";

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
