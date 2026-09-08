// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

import {
  APPLICABILITY,
  CONTRACT_SCHEMA,
  CONTRACT_VERSION,
  EXECUTION_STATUSES,
  INPUT_KINDS,
  LOGICAL_OUTCOMES,
  READINESS,
  adaptQuoinCatalog,
  buildVerificationPlan,
  validateContract,
  validateTechniqueResult,
  validateToolCapability,
} from "../src/index.mjs";

const portfolio = JSON.parse(
  await readFile(new URL("../fixtures/portfolio.json", import.meta.url)),
);
const results = JSON.parse(
  await readFile(new URL("../fixtures/results.json", import.meta.url)),
);
const catalog = JSON.parse(
  await readFile(new URL("../fixtures/quoin-catalog-export.json", import.meta.url)),
);

test("runtime constants are derived from the authoritative schema", () => {
  const definitions = CONTRACT_SCHEMA.$defs;
  assert.equal(
    CONTRACT_VERSION,
    definitions.TechniqueDefinition.properties.contractVersion.const,
  );
  assert.deepEqual(
    INPUT_KINDS,
    definitions.Input.oneOf.map((entry) => entry.properties.kind.const),
  );
  assert.deepEqual(
    APPLICABILITY,
    definitions.VerificationPlan.properties.tasks.items.properties.applicability.enum,
  );
  assert.deepEqual(
    READINESS,
    definitions.VerificationPlan.properties.tasks.items.properties.readiness.enum,
  );
  assert.deepEqual(
    EXECUTION_STATUSES,
    definitions.TechniqueResult.properties.execution.properties.status.enum,
  );
  assert.deepEqual(
    LOGICAL_OUTCOMES,
    definitions.TechniqueResult.properties.logical.properties.outcome.enum,
  );
});

test("the root schema accepts each of the five public contracts", () => {
  const registry = adaptQuoinCatalog(catalog);
  const plan = buildVerificationPlan(portfolio.planRequest);
  for (const value of [
    registry.techniques[0],
    portfolio.planRequest.capabilities[0],
    portfolio.planRequest.policy,
    plan,
    results.results[0],
  ]) {
    assert.equal(validateContract(value), value);
  }
  assert.throws(() => validateContract(results.inventory[0]), /oneOf/);
});

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
    "runRef",
    "provenance",
    "nativeArtifacts",
  ]) {
    const result = structuredClone(results.results[0]);
    delete result[field];
    assert.throws(() => validateTechniqueResult(result), new RegExp(field));
  }
});

test("requires versioned adapter provenance", () => {
  const result = structuredClone(results.results[0]);
  delete result.provenance.adapterVersion;
  assert.throws(() => validateTechniqueResult(result), /adapterVersion/);
});

test("schema prevents a verdict when execution did not complete", () => {
  const result = structuredClone(results.results[0]);
  result.execution.status = "failed";
  assert.throws(() => validateTechniqueResult(result), /logical|must NOT be valid/);
});
