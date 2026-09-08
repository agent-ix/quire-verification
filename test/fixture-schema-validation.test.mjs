// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import test from "node:test";

import { validateAllFixtures } from "../scripts/validate-fixtures.mjs";

test("executes the schema against every JSON fixture", async () => {
  const summary = await validateAllFixtures();
  assert.deepEqual(
    summary.map(({ name }) => name),
    [
      "portfolio.json",
      "quoin-advice-export.json",
      "quoin-catalog-export.json",
      "results.json",
    ],
  );
  assert.ok(summary.reduce((sum, item) => sum + item.contracts, 0) > 0);
});
