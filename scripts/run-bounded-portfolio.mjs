// SPDX-License-Identifier: AGPL-3.0-or-later

import { runBoundedPortfolio } from "../src/index.mjs";

const [sourceRevision, producedAt] = process.argv.slice(2);
if (!sourceRevision || !producedAt) {
  throw new Error(
    "usage: node scripts/run-bounded-portfolio.mjs <source-revision> <produced-at>",
  );
}
console.log(
  JSON.stringify(await runBoundedPortfolio({ sourceRevision, producedAt }), null, 2),
);
