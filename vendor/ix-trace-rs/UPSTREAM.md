# ix-trace-rs snapshot provenance

- Upstream repository: `https://github.com/agent-ix/ix-trace-rs`
- Upstream revision: `2ce4ebf47f726b9d76388220545cd0abda8a5cfb`
- Imported for: test-only compiler-checked `#[trace(...)]` attributes
- Imported files: `LICENSE` and `src/lib.rs`
- Local packaging metadata: `Cargo.toml` retains the upstream package and lib
  metadata and omits upstream-only test dependencies and release settings.
- Local modifications to imported source and license: none

SHA-256 hashes at the recorded revision:

| File | SHA-256 |
|---|---|
| Upstream `Cargo.toml` | `d4a9e7cefe64aa7bd418a57b65b1a64562413868af0d8255d6d1be6b909830c0` |
| `LICENSE` | `0d96a4ff68ad6d4b6f1f30f713b18d5184912ba8dd389f86aa7710db079abcb0` |
| `src/lib.rs` | `d7be2c00ed45ef0ad9bda683ef6a5b9397cca32483e89a7ac1fb745eb841d1c4` |

Refresh this directory only by importing a reviewed upstream revision, then
update these hashes, `Cargo.lock`, the Rust review, and clean-runner CI evidence.
