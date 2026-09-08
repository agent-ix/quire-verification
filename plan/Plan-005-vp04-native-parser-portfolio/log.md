---
type: log
title: "Plan-005 — Update Log"
description: "Chronological log of changes to the Plan-005 bundle."
---
# Plan-005 — Update Log

## History

* **2026-09-08** — QUOIN `spec-to-plan` created the VP04 bundle from reviewed revision `21c894d` and all-lens review `03a7dce`; Task-020 is ready, while Tasks 021 through 024 retain the accepted VP02/VP03, shared-role, exact-parser, and external-disposition gates.
* **2026-09-08** — Task-020 moved to `in_progress` after SR-058 recorded the honest 0/5 initial gap state; implementation begins test-first against the reviewed NFR-006 and TC-038 boundary.
* **2026-09-08** — Exact Rust review found a pathname substitution race and descendant-process escape in the first Task-020 implementation. QUOIN `/specify` and all-lens `/spec-review` revised NFR-006/TC-038 at `d3dbf5b`/`fa11e0e`; Task-020 now requires descriptor-pinned admission and whole-process-group cleanup.
* **2026-09-08** — Task-020 completed at `be1d45b` after all 37 first-party Rust tests, 50 repeated native-process runs, formatting, denied-warning Clippy, fixture validation, dependency policy, and NFR-006 coverage passed. Exact Rust review SR-059 accepted the boundary at `ac2af2c`; NFR-006 is 7/7 backed and TC-038 is complete.
* **2026-09-08** — Hosted CI run `34277066256` contradicted Task-020 completion with a transient write-busy fixture executable and a descendant still live after group signaling. QUOIN `/specify` reopened NFR-006/TC-038 at `743afca`; owner-selected all-lens `/spec-review` SR-071 through SR-078 accepted observed group disappearance, an explicit containment-failure outcome, a separate one-second cleanup budget, and immutable linked test fixtures. Task-020 returned to `in_progress` before remediation implementation.
* **2026-09-08** — Task-020 remediation completed at `8d0aa4c`: group cleanup now distinguishes signaling, direct-child reap, group-disappearance observation, and containment failure; parallel tests hard-link the completed Cargo fixture. All 38 first-party tests, strict Clippy, formatting, fixture and dependency gates pass, and the five native-process tests passed 100 repeated runs. Exact Rust review SR-079 passes at `6762a33`; Task-020 returned to `done` pending required hosted CI on the pushed revision.
* **2026-09-08** — Hosted CI run `34280735151` passed the remediated Task-020 at `a788858`. The exact merged LC01 syntax candidate `2be7350` was then independently reproduced: formatting, strict Clippy, 38 default Rust tests, locked release build, direct parse, format/reparse, and the located syntax-refusal mutation all passed. `evidence/vp04-parser-candidate.md` retains the revision, lock/binary/fixture hashes, Linux/Rust profile, commands, review evidence, and AGPL-3.0-only fixture rights. This clears only Task-024's parser/fixture entrance subtask; Task-024 remains blocked on Task-023 and its transitive gates.
