// SPDX-License-Identifier: AGPL-3.0-or-later

import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

const schema = JSON.parse(
  await readFile(
    new URL("../contracts/e02-draft-1.schema.json", import.meta.url),
    "utf8",
  ),
);

test("the versioned packet contains all five E02 contracts", () => {
  assert.deepEqual(
    Object.keys(schema.$defs)
      .filter((name) =>
        [
          "TechniqueDefinition",
          "ToolCapability",
          "SelectionPolicy",
          "VerificationPlan",
          "TechniqueResult",
        ].includes(name),
      )
      .sort(),
    [
      "SelectionPolicy",
      "TechniqueDefinition",
      "TechniqueResult",
      "ToolCapability",
      "VerificationPlan",
    ],
  );
  assert.match(schema.$id, /private\.agent-ix\.local/);
});

test("the input contract keeps all five subject shapes explicit", () => {
  assert.deepEqual(
    schema.$defs.Input.oneOf.map((entry) => entry.properties.kind.const),
    ["state", "relational", "trace", "fault", "manual"],
  );
});

test("result intake exposes exact plan, task, run, producer, and adapter binding", () => {
  assert.deepEqual(schema.$defs.ResultBinding.required, [
    "planId",
    "taskId",
    "runRef",
    "producer",
    "producerVersion",
    "adapter",
    "adapterVersion",
  ]);
  assert.ok(schema.$defs.TechniqueResult.required.includes("runRef"));
  assert.ok(
    schema.$defs.TechniqueResult.properties.provenance.required.includes(
      "adapterVersion",
    ),
  );
});
