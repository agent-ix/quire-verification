---
id: NFR-006
title: "Bound native process execution"
type: NFR
quality_attribute: security
relationships:
  - target: "ix://agent-ix/quire-verification/FR-019"
    type: "constrains"
---
# NFR-006: Bound native process execution

## Statement

When the VP04 runner invokes a native candidate, the process executor SHALL use
an exact direct-process request with finite time, output, environment, and
working-directory bounds.

## Scope

This requirement constrains only the repository-owned VP04 native process
adapter in the exact reviewed Linux execution environment. It does not redefine
the parser CLI, operate a shell, invoke a network service, authorize production
execution, or claim another operating-system profile.

## Rationale

An exact reviewed binary can still hang, flood pipes, inherit unstable or secret
environment state, receive ambiguous shell text, or escape the selected run
directory. The runner must turn those conditions into bounded typed results.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Shell-mediated native invocations | 0 | 0 | Test (TC-038) |
| Per-invocation wall-clock budget | At most 10 seconds | 10 seconds | Test (TC-038) |
| Captured standard output | At most 16 MiB | 16 MiB | Test (TC-038) |
| Captured standard error | At most 16 MiB | 16 MiB | Test (TC-038) |
| Undeclared inherited environment entries | 0 | 0 | Test (TC-038) |

## Verification

A Rust integration harness invokes controlled native child fixtures that expose
arguments and environment, exceed each output and time boundary separately and
simultaneously, exit by code or signal, and emit invalid UTF-8 or malformed
protocol output.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-006-AC-1 | The executor invokes an exact absolute executable with a separate argument vector, no shell, a fresh run directory, and only explicitly declared environment entries. | Test (TC-038) |
| NFR-006-AC-2 | A child still running at 10 seconds is terminated and reaped, and the returned failed result carries `execution_timeout` without a logical outcome claim. | Test (TC-038) |
| NFR-006-AC-3 | Standard output or error is accepted through exactly 16 MiB; the first excess byte terminates and reaps the child and returns `output_limit_exceeded` with the stream name, count, and limit but not the excess bytes. | Test (TC-038) |
| NFR-006-AC-4 | A signal, unexpected exit code, invalid UTF-8, or malformed method response returns a distinct typed failed result without panic, orphan process, partial success, or logical outcome claim. | Test (TC-038) |
| NFR-006-AC-5 | A candidate path that is relative, outside the selected execution root, absent, non-regular, or resolves through a symbolic link is refused before child launch. | Test (TC-038) |

## Dependencies

- Constrains the native execution behavior in [FR-019](../functional/FR-019-execute-bounded-parser-portfolio.md).
- Uses the existing Rust-only and bounded-input controls in [NFR-004](./NFR-004-rust-qualified-path.md) and [NFR-005](./NFR-005-bounded-json-ingestion.md).
