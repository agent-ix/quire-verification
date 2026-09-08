// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
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
assert.deepEqual(actual, fixture.catalog);
console.log(
  `Quoin baseline parity: ${actual.methods.length} methods, ` +
    `${actual.duplicates.length} duplicates, ${actual.unreadable.length} unreadable roots`,
);
