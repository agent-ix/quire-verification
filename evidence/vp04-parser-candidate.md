# VP04 native parser candidate evidence

Observed 2026-09-08 against the merged `agent-ix/quire-spec-language` LC01
syntax candidate.

## Selected candidate

- Repository revision: `2be73504edff1224d6396da15edca3775905d37b`
  (merge commit for language PR 7).
- Reviewed implementation revision: `2492ef3ddf4996f7806c8137e1f150b9e65d5e25`.
- Review evidence: language SR-028 is the final code/Rust review and SR-029 is
  the completed plan gap analysis. The PR records owner authorization to land.
- Qualified use here: the `quire-spec` LC01 `parse` and `format` syntax
  commands only. Imports remain unresolved; this candidate supplies no model
  binding, type checking, evaluation, or sufficiency decision.

## Dependency and environment identity

- `Cargo.lock` SHA-256:
  `26c8dc235777ff79a5882e01a6c700d2feab9fe4d9f4397662490aa37fb9d444`.
- Locally built release `quire-spec` SHA-256:
  `c6bd75613f85dfd38aaf37169aed0a3d7409466d4b2f34dcc5397896c46579e5`.
- Rust: `rustc 1.94.1 (e408947bf 2026-03-25)`, host
  `x86_64-unknown-linux-gnu`, LLVM 21.1.8.
- Execution host: Linux x86_64, kernel
  `6.18.33.2-microsoft-standard-WSL2`.
- Cargo was invoked with `--locked`, `--no-default-features`, and the isolated
  target directory `/tmp/quire-spec-language-agent-b-target`.

`Cargo.toml` identifies the first-party package as `AGPL-3.0-only` and pins the
direct dependency versions. The language repository's `docs/dependencies.md`
and `Cargo.lock` retain the resolved dependency inventory and grants.

## Fixture identity and rights

- Selected fixture: `tests/fixtures/parent.native` at the selected repository
  revision.
- Fixture SHA-256:
  `bb7a187662ee136b70c27a82204ecd41b446dcac56fbc183d32bc94b8ff28018`.
- The file carries `SPDX-License-Identifier: AGPL-3.0-only` and identifies
  itself as a newly authored syntax fixture. `LICENSE-DECISION.md` explicitly
  assigns AGPL-3.0-only to newly authored implementation test fixtures.
- Formatted output SHA-256:
  `4df9a865a1fd75e6b10b7fe9cea6118018bac19eca9552d9f6feb0a87ceecde4`.
- Declared mutation SHA-256:
  `2f6c5dde78cb3c1c666ebb21a0c993da26b8ac6dfd762c83ef63203d9b5085fa`.
  It changes line 6 `invariant` to `invarianx` and retains the fixture notice.

These bytes are private qualification inputs. This record does not authorize a
public release or change the deferred license for normative standard artifacts.

## Reproduced checks

The exact merged revision passed:

```text
cargo fmt --all -- --check
cargo clippy --locked --target-dir /tmp/quire-spec-language-agent-b-target --all-targets --no-default-features -- -D warnings
cargo test --locked --target-dir /tmp/quire-spec-language-agent-b-target --no-default-features
cargo build --locked --release --no-default-features --target-dir /tmp/quire-spec-language-agent-b-target
```

The default test run reported 38 passed and 3 explicitly ignored private-packet
tests. The three private-packet tests had already passed independently against
the pinned Agent A packet during B's candidate inspection; they are unrelated to
the selected LC01 syntax commands.

Direct parse command:

```text
/tmp/quire-spec-language-agent-b-target/release/quire-spec parse test:parent fixture:1 tests/fixtures/parent.native
```

It exited 0 with `status: parsed`, four clauses, one import, and the exact source
digest above. Formatting the same bytes and parsing the formatted output at
revision `fixture:formatted` also exited 0 with four clauses and one import.
The exact roundtrip commands were:

```text
/tmp/quire-spec-language-agent-b-target/release/quire-spec format test:parent fixture:1 tests/fixtures/parent.native
/tmp/quire-spec-language-agent-b-target/release/quire-spec parse test:parent fixture:formatted /tmp/quire-spec-formatted.native
```

Mutation command:

```text
/tmp/quire-spec-language-agent-b-target/release/quire-spec parse test:parent fixture:invalid /tmp/quire-spec-parent-invalid.native
```

It exited 1 with `status: refused`, code `invalid_syntax`, phase `parse`, and the
expected line 6 span from byte 249 through byte 258 (columns 1 through 10).

The earlier hosted syntax run on checkpoint `8b739de` passed. The merged PR
intentionally retained manual-dispatch-only CI and used the accepted final local
review candidate, so this evidence distinguishes the reproduced exact merged
revision from that earlier hosted checkpoint.

## Disposition finalizer input

The ignored disposition-finalization lane reads `VP04_PARSER` when supplied and
otherwise uses `.vp04-build/release/quire-spec` under this repository. The
`.vp04-build/` directory is ignored. Populate it from the selected upstream
revision with an absolute target directory, for example:

```text
cargo build --locked --release --no-default-features --target-dir /absolute/path/to/quire-verification/.vp04-build
```

The lane rechecks that the selected executable has SHA-256 digest
`c6bd75613f85dfd38aaf37169aed0a3d7409466d4b2f34dcc5397896c46579e5`;
another binary at either the default path or `VP04_PARSER` refuses admission.
