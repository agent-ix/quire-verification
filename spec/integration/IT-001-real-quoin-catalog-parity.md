---
id: IT-001
title: "Real Quoin method-catalog parity"
type: IT
relationships:
  - target: "ix://agent-ix/quire-verification/FR-004"
    type: "verifies"
  - target: "ix://agent-ix/quire-verification/FR-009"
    type: "verifies"
---
# IT-001: Real Quoin method-catalog parity

## Objective

Verify that the retained Quoin catalog fixture matches output produced by the
real Quoin catalog loader at the pinned Quoin and module revisions. This catches
producer drift that fixture-only tests cannot detect.

## Target Integration

The system under test is the Rust `check-quoin-baseline` binary. The external target
is the real `loadMethodCatalog` implementation in a pinned checkout of the
`agent-ix/quoin` repository, reading the real `spec-artifacts-process` module
directory through Quoin's normal filesystem catalog-loading path.

## Preconditions

- Independent Quoin and `spec-artifacts-process` checkouts are at the revisions recorded in `fixtures/quoin-catalog-export.json`.
- Quoin's locked development dependencies are installed.
- Node.js 22 or newer is available.
- The retained fixture is present and unchanged in this repository.
- No catalog loader, filesystem access, or input stream is mocked.

## Inputs

- The absolute path to the pinned `spec_artifacts_process` module root.
- The real JSON `MethodCatalog` emitted by Quoin's `loadMethodCatalog`.
- `fixtures/quoin-catalog-export.json` as the retained comparison baseline.

## Test Procedure

1. Read the Quoin and module revisions recorded in the retained fixture and check out those exact revisions.
   - IT-001-SC-01: both working trees report the expected revisions.
2. Invoke Quoin's real `loadMethodCatalog` with the pinned module root and serialize its complete JSON output.
   - IT-001-SC-02: the command exits zero and emits a parseable `MethodCatalog` document.
3. Pipe the emitted document to `cargo run --quiet --bin check-quoin-baseline` in this repository.
   - IT-001-SC-03: the comparison exits zero after matching complete catalog content with methods keyed by technique identifier.
4. Record the comparison summary from standard output.
   - IT-001-SC-04: the summary reports 33 methods, zero duplicates, and zero unreadable roots.

## Expected Results

The real Quoin loader reads the pinned module through its actual filesystem path,
emits 33 unique readable method definitions, and the baseline checker confirms
semantic parity with the retained fixture. Any revision mismatch, unreadable
module, duplicate method, missing or extra method, or field change fails the test.

## Metadata

- Priority: P0
- Target Integration: Quoin method-catalog loader and spec-artifacts-process module
- Automation: Command-line automated with real local checkouts

## Dependencies

- Verifies [FR-004](../functional/FR-004-adapt-quoin-catalog.md) and [FR-009](../functional/FR-009-retain-portfolio-failures.md).
- Requires the pinned external revisions named by the fixture.

## Notes

This integration test uses real Quoin code and real filesystem input. It does not
mock the loader, module directory, or JSON stream.

## Traceability

The integration test closes the external-producer boundary for the catalog
compatibility behavior while making no claim about native verification tools.
