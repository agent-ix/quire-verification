// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  validateTechniqueResult,
  validateToolCapability,
} from "../src/index.mjs";

const portfolio = JSON.parse(
  await readFile(new URL("../fixtures/portfolio.json", import.meta.url)),
);
const results = JSON.parse(
  await readFile(new URL("../fixtures/results.json", import.meta.url)),
);

test("rejects a capability that pretends all subjects use a unary state input", () => {
  const capability = structuredClone(portfolio.planRequest.capabilities[0]);
  capability.inputKinds = ["universal-predicate"];
  assert.throws(() => validateToolCapability(capability), /inputKinds/);
});

test("requires execution, logical outcome, activation, and adequacy separately", () => {
  const result = structuredClone(results.results[0]);
  delete result.activation;
  assert.throws(() => validateTechniqueResult(result), /activation/);
});

test("requires participation, assumptions, scope, provenance, and native artifacts", () => {
  for (const field of [
    "participation",
    "assumptions",
    "executionScope",
    "provenance",
    "nativeArtifacts",
  ]) {
    const result = structuredClone(results.results[0]);
    delete result[field];
    assert.throws(() => validateTechniqueResult(result), new RegExp(field));
  }
});
