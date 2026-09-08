// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  buildVerificationPlan,
  validateSelectionPolicy,
} from "../src/index.mjs";

const fixture = JSON.parse(
  await readFile(new URL("../fixtures/portfolio.json", import.meta.url)),
);

test("builds a five-input-kind plan and keeps missing prerequisites visible", () => {
  const plan = buildVerificationPlan(fixture.planRequest);
  const verification = plan.tasks.filter((task) => task.kind === "verification");
  assert.equal(verification.length, 6);
  assert.deepEqual(
    [...new Set(verification.map((task) => task.input.kind))].sort(),
    ["fault", "manual", "relational", "state", "trace"],
  );
  assert.equal(plan.inventory.length, 5);
  assert.equal(plan.inventory[0].classification, "unknown");
  assert.equal(plan.evidenceOwner, "Quoin");
  assert.equal(plan.sufficiencyOwner, "authored-human-decision");

  const byTechnique = Object.fromEntries(
    verification.map((task) => [task.techniqueId, task]),
  );
  assert.equal(byTechnique["property-based-testing"].readiness, "ready");
  assert.equal(byTechnique.fuzzing.readiness, "ready");
  assert.equal(byTechnique.inspection.readiness, "ready");
  assert.equal(byTechnique["metamorphic-testing"].readiness, "blocked");
  assert.equal(byTechnique["runtime-monitoring"].readiness, "blocked");
  assert.equal(byTechnique["fault-injection"].readiness, "blocked");
  assert.equal(byTechnique["fault-injection"].invocationAllowed, false);

  const faultGaps = byTechnique["fault-injection"].missingPrerequisites.map(
    (item) => `${item.kind}:${item.id}`,
  );
  assert.deepEqual(faultGaps, [
    "environment-authorization:non-production-fault-scope",
    "injection-binding:confirmed-injection-site",
    "observation-binding:independent-effect-observation",
  ]);
  assert.equal(
    plan.tasks.filter((task) => task.kind === "manual-prerequisite").length,
    8,
  );
});

test("applicability and readiness remain independent", () => {
  const request = structuredClone(fixture.planRequest);
  request.selections = [
    {
      ...request.selections.find(
        (selection) => selection.techniqueId === "property-based-testing",
      ),
      applicability: "unknown",
    },
  ];
  const [task] = buildVerificationPlan(request).tasks.filter(
    (item) => item.kind === "verification",
  );
  assert.equal(task.applicability, "unknown");
  assert.equal(task.readiness, "ready");
  assert.equal(task.invocationAllowed, false);
});

test("explicit non-applicability remains separate from ready capability", () => {
  const request = structuredClone(fixture.planRequest);
  request.selections = [
    {
      ...request.selections.find(
        (selection) => selection.techniqueId === "property-based-testing",
      ),
      applicability: "not-applicable",
    },
  ];
  const [task] = buildVerificationPlan(request).tasks.filter(
    (item) => item.kind === "verification",
  );
  assert.equal(task.applicability, "not-applicable");
  assert.equal(task.readiness, "ready");
  assert.equal(task.invocationAllowed, false);
});

test("an absent capability is unsupported rather than not applicable", () => {
  const request = structuredClone(fixture.planRequest);
  request.capabilities = request.capabilities.filter(
    (capability) => capability.techniqueId !== "fuzzing",
  );
  request.selections = request.selections.filter(
    (selection) => selection.techniqueId === "fuzzing",
  );
  const [task] = buildVerificationPlan(request).tasks.filter(
    (item) => item.kind === "verification",
  );
  assert.equal(task.applicability, "applicable");
  assert.equal(task.readiness, "unsupported");
  assert.equal(task.invocationAllowed, false);
});

test("selection policy rejects invented ordinal scores", () => {
  const policy = structuredClone(fixture.planRequest.policy);
  policy.rules[0].score = 42;
  assert.throws(
    () => validateSelectionPolicy(policy),
    /must not contain an ordinal score/,
  );
});

test("standards-derived policy requires exact provenance and rights", () => {
  const policy = structuredClone(fixture.planRequest.policy);
  policy.provenance.standardDerived = true;
  assert.throws(
    () => validateSelectionPolicy(policy),
    /provenance\.standard/,
  );
});
