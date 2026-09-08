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
an admitted executable descriptor and a dedicated process group with finite
time, output, environment, and working-directory bounds.

## Scope

This requirement constrains only the repository-owned VP04 native process
adapter in the exact reviewed Linux execution environment. That profile requires
Linux `openat2`, `/proc/self/fd`, and process-group signaling. If those facilities
are unavailable or refuse the required flags, the adapter refuses with
`unsupported_execution_profile` before launching a candidate. It does not
redefine the parser CLI, operate a shell, invoke a network service, authorize
production execution, or claim another operating-system profile.

## Rationale

An exact reviewed binary can still hang, flood pipes, inherit unstable or secret
environment state, receive ambiguous shell text, escape the selected run
directory, change between path validation and execution, or leave descendants
holding pipes after the direct child exits. The runner must pin the admitted
file, contain the whole process group, and turn those conditions into bounded
typed results.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Shell-mediated native invocations | 0 | 0 | Test (TC-038) |
| Maximum accepted per-invocation wall-clock budget | At most 10 seconds | 10 seconds | Test (TC-038) |
| Captured standard output | At most 16 MiB | 16 MiB | Test (TC-038) |
| Captured standard error | At most 16 MiB | 16 MiB | Test (TC-038) |
| Undeclared inherited environment entries | 0 | 0 | Test (TC-038) |
| Request-path resolutions after executable admission | 0 | 0 | Test (TC-038) |
| Live process-group members when the executor reports successful containment | 0 | 0 | Test (TC-038) |
| Post-signal containment observation budget | At most 1 second | 1 second | Test (TC-038) |

## Verification

A Rust integration harness invokes controlled native child fixtures that expose
arguments and environment, exceed each output and time boundary separately and
simultaneously, exit by code or signal, emit invalid UTF-8 or malformed protocol
output, replace the admitted pathname, and leave descendants with both retained
and closed pipes. It observes process-group disappearance after the direct child
is reaped and runs fixture cases concurrently from immutable links to a completed
Cargo-built fixture binary.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-006-AC-1 | The executor invokes an exact absolute executable with a separate argument vector, no shell, a fresh run directory, and only explicitly declared environment entries. | Test (TC-038) |
| NFR-006-AC-2 | A request accepts a positive declared deadline no greater than 10 seconds and refuses an absent, zero, negative, non-finite, or larger deadline with `invalid_execution_deadline` before child launch. | Test (TC-038) |
| NFR-006-AC-3 | A process group still live at its accepted declared deadline is signaled, the direct child is reaped, and group disappearance is observed before returning `execution_timeout`; failure to send the signal or observe disappearance returns `process_containment_failure` with trigger `execution_timeout` and never claims the group terminated. Neither result carries a logical outcome claim. | Test (TC-038) |
| NFR-006-AC-4 | Standard output or error is accepted through exactly 16 MiB; the first excess byte signals the process group, reaps the direct child, and returns `output_limit_exceeded` with the stream name, count, and limit but not the excess bytes only after group disappearance is observed. Failed signaling or observation returns `process_containment_failure` with trigger `output_limit_exceeded`. | Test (TC-038) |
| NFR-006-AC-5 | A signal, unexpected exit code, invalid UTF-8, malformed method response, or descendant surviving the direct child returns a distinct typed failed result without panic, unreported containment failure, partial success, or logical outcome claim. | Test (TC-038) |
| NFR-006-AC-6 | A candidate path that is relative, outside the selected execution root, absent, non-regular, or resolves through a symbolic link is refused before child launch; lack of the selected Linux descriptor/process-group profile refuses with `unsupported_execution_profile`. | Test (TC-038) |
| NFR-006-AC-7 | The executor opens the root and root-relative candidate with `openat2` using `RESOLVE_BENEATH`, `RESOLVE_NO_MAGICLINKS`, and `RESOLVE_NO_SYMLINKS`, retains the admitted descriptor through launch, and invokes only its trusted descriptor-backed `/proc/self/fd` path; replacing the request pathname after admission cannot substitute the executed bytes. | Test (TC-038) |
| NFR-006-AC-8 | After requesting group termination, the executor reaps the direct child before waiting at most one second for process-group disappearance; it reports `process_group_terminated: true` only after observing disappearance, and otherwise returns `process_containment_failure` with the original failure code retained as `trigger`. | Test (TC-038) |

## Dependencies

- Constrains the native execution behavior in [FR-019](../functional/FR-019-execute-bounded-parser-portfolio.md).
- Uses the existing Rust-only and bounded-input controls in [NFR-004](./NFR-004-rust-qualified-path.md) and [NFR-005](./NFR-005-bounded-json-ingestion.md).
