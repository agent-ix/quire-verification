// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdir, readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";

import {
  adaptQuoinAdvice,
  adaptQuoinCatalog,
  buildVerificationPlan,
  validateContract,
  validateInventoryProperty,
  validateSchemaDefinition,
} from "../src/index.mjs";

const fixtureDirectory = new URL("../fixtures/", import.meta.url);

export async function validateAllFixtures() {
  const handlers = new Map([
    ["portfolio.json", validatePortfolioFixture],
    ["quoin-advice-export.json", validateAdviceFixture],
    ["quoin-catalog-export.json", validateCatalogFixture],
    ["results.json", validateResultsFixture],
  ]);
  const files = (await readdir(fixtureDirectory))
    .filter((name) => name.endsWith(".json"))
    .sort();
  const unhandled = files.filter((name) => !handlers.has(name));
  if (unhandled.length > 0) {
    throw new Error(`fixture JSON has no schema handler: ${unhandled.join(", ")}`);
  }
  const summary = [];
  for (const name of files) {
    const value = JSON.parse(await readFile(new URL(name, fixtureDirectory), "utf8"));
    summary.push({ name, contracts: handlers.get(name)(value) });
  }
  return summary;
}

function validatePortfolioFixture(value) {
  validateSchemaDefinition("PortfolioFixture", value, "portfolio fixture");
  validateContract(value.planRequest.policy);
  value.planRequest.capabilities.forEach(validateContract);
  value.planRequest.properties.forEach((property) =>
    validateInventoryProperty(property),
  );
  validateContract(buildVerificationPlan(value.planRequest));
  return 2 + value.planRequest.capabilities.length;
}

function validateResultsFixture(value) {
  validateSchemaDefinition("ResultsFixture", value, "results fixture");
  value.inventory.forEach((property) =>
    validateInventoryProperty(property, { requireResultBinding: true }),
  );
  value.results.forEach(validateContract);
  return value.results.length;
}

function validateCatalogFixture(value) {
  validateSchemaDefinition("QuoinCatalogFixture", value, "Quoin catalog fixture");
  const snapshot = adaptQuoinCatalog(value);
  snapshot.techniques.forEach(validateContract);
  return snapshot.techniques.length;
}

function validateAdviceFixture(value) {
  validateSchemaDefinition("QuoinAdviceFixture", value, "Quoin advice fixture");
  if (value.cases.length !== value.advice.length) {
    throw new Error("Quoin advice fixture cases and advice must have equal lengths");
  }
  value.advice.forEach(adaptQuoinAdvice);
  return 0;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const summary = await validateAllFixtures();
  const contractCount = summary.reduce((sum, item) => sum + item.contracts, 0);
  console.log(
    `validated ${summary.length} fixtures and ${contractCount} public contract instances against e02-draft-1.schema.json`,
  );
}
