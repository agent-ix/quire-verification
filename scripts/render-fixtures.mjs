// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFile } from "node:fs/promises";

import { assessEvidence, buildVerificationPlan } from "../src/index.mjs";

const portfolio = JSON.parse(
  await readFile(new URL("../fixtures/portfolio.json", import.meta.url), "utf8"),
);
const results = JSON.parse(
  await readFile(new URL("../fixtures/results.json", import.meta.url), "utf8"),
);

console.log(
  JSON.stringify(
    {
      schemaVersion: "quire-verification/e02-evidence-packet/v1",
      claimLimit:
        "Fixture planning and intake only; no represented external verification or fault tool executed.",
      plan: buildVerificationPlan(portfolio.planRequest),
      assessment: assessEvidence(results),
    },
    null,
    2,
  ),
);
