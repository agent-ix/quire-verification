# Qualification fuzz targets

The three Rust targets are generated from the retained QUOIN `spec-fuzz` selection. The
root Cargo workspace owns their dependency closure, lints, formatting, and compilation;
there is no independent `fuzz/Cargo.lock`.

For a bounded local diagnostic, pass the retained seed directory explicitly and supply a
libFuzzer run bound. For example:

```console
cargo fuzz run fuzz_fr_016_ac_3_strict_shared_reference fuzz/seeds/fuzz_fr_016_ac_3_strict_shared_reference -- -runs=1000
cargo fuzz run fuzz_nfr_005_ac_1_bounded_json fuzz/seeds/fuzz_nfr_005_ac_1_bounded_json -- -runs=1000
cargo fuzz run fuzz_nfr_006_ac_5_native_response fuzz/seeds/fuzz_nfr_006_ac_5_native_response -- -runs=1000
```

These commands are diagnostics. Qualification execution and evidence production use the
accepted Engineering Assurance Rust interface with an owner-supplied budget and stopping
rule. Compiling a target or exhausting a local diagnostic run does not discharge an
obligation.
