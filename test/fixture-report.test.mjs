// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import { assessEvidence, buildVerificationPlan } from "../src/index.mjs";

const read = async (path) =>
  JSON.parse(await readFile(new URL(path, import.meta.url), "utf8"));

test("the retained evidence packet is reproducible from its inputs", async () => {
  const portfolio = await read("../fixtures/portfolio.json");
  const results = await read("../fixtures/results.json");
  const report = await read("../evidence/e02-fixture-report.json");
  assert.deepEqual(report.plan, buildVerificationPlan(portfolio.planRequest));
  assert.deepEqual(report.assessment, assessEvidence(results));
  assert.match(report.claimLimit, /no represented external/);
});
