// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFile } from "node:fs/promises";

import { jcsEqual } from "../src/contracts.mjs";
const fixture = JSON.parse(
  await readFile(
    new URL("../fixtures/quoin-catalog-export.json", import.meta.url),
    "utf8",
  ),
);

let input = "";
for await (const chunk of process.stdin) input += chunk;
if (input.trim().length === 0) {
  throw new Error("expected a Quoin MethodCatalog JSON document on stdin");
}
const actual = JSON.parse(input);
if (!jcsEqual(byTechniqueId(actual), byTechniqueId(fixture.catalog))) {
  throw new Error("Quoin baseline differs from the pinned catalog fixture");
}
console.log(
  `Quoin baseline parity: ${actual.methods.length} methods, ` +
    `${actual.duplicates.length} duplicates, ${actual.unreadable.length} unreadable roots`,
);

function byTechniqueId(catalog) {
  return {
    ...catalog,
    methods: [...catalog.methods].sort((left, right) =>
      left.id === right.id ? 0 : left.id < right.id ? -1 : 1,
    ),
  };
}
