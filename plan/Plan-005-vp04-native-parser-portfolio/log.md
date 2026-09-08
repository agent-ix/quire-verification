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
